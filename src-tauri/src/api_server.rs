use axum::{
    extract::{Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

use crate::api_emulator::APIEmulator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u32,
    pub time: u32,
    pub sender: Option<String>,
    pub mode: Option<String>,
    pub msg: String,
    pub enemy: bool,
}

#[derive(Clone)]
pub struct ServerState {
    pub emulator: Arc<APIEmulator>,
    pub chat_messages: Arc<RwLock<Vec<ChatMessage>>>,
    pub next_chat_id: Arc<RwLock<u32>>,
}

pub async fn start_server(emulator: Arc<APIEmulator>) {
    let state = ServerState {
        emulator: emulator.clone(),
        chat_messages: Arc::new(RwLock::new(Vec::new())),
        next_chat_id: Arc::new(RwLock::new(1)),
    };

    let app = Router::new()
        // Status endpoints
        .route("/status", get(status_handler))
        .route("/damage", get(damage_handler))
        
        // Telemetry endpoints
        .route("/indicators", get(indicators_handler))
        .route("/state", get(state_handler))
        
        // HUD endpoints
        .route("/hudmsg", get(hudmsg_handler))
        
        // Map endpoints
        .route("/map_obj.json", get(map_obj_handler))
        .route("/map_info.json", get(map_info_handler))
        .route("/map.img", get(map_img_handler))
        
        // Chat endpoints
        .route("/gamechat", get(gamechat_handler))
        .route("/gamechat/send", post(gamechat_send_handler))
        
        // Mission endpoints
        .route("/mission.json", get(mission_handler))
        
        // Info endpoints
        .route("/info", get(info_handler))
        .route("/gunner_view", get(gunner_view_handler))
        
        // Root endpoint
        .route("/", get(root_handler))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    log::info!("[API Server] Starting emulator server on http://localhost:8112");
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8112")
        .await
        .expect("Failed to bind to port 8112");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

// === Handlers ===

async fn status_handler(State(state): State<ServerState>) -> Response {
    let emu_state = state.emulator.get_state();
    
    if !emu_state.enabled || !emu_state.in_battle {
        return (StatusCode::OK, "").into_response();
    }
    
    Json(serde_json::json!({
        "type": match emu_state.vehicle_type {
            crate::api_emulator::VehicleType::Tank => "tank",
            crate::api_emulator::VehicleType::Aircraft => "aircraft",
            crate::api_emulator::VehicleType::Ship => "ship",
        },
        "state": "in_battle",
        "valid": true
    })).into_response()
}

async fn damage_handler() -> Response {
    (StatusCode::OK, "").into_response()
}

async fn indicators_handler(State(state): State<ServerState>) -> Json<serde_json::Value> {
    Json(state.emulator.generate_indicators())
}

async fn state_handler(State(state): State<ServerState>) -> Json<serde_json::Value> {
    Json(state.emulator.generate_state())
}

async fn hudmsg_handler(
    State(state): State<ServerState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let last_evt = params.get("lastEvt").and_then(|s| s.parse().ok()).unwrap_or(0);
    let _last_dmg = params.get("lastDmg").and_then(|s| s.parse().ok()).unwrap_or(0);
    
    let events: Vec<serde_json::Value> = state.emulator
        .get_events(last_evt as u64)
        .into_iter()
        .map(|e| serde_json::json!({
            "id": e.timestamp / 1000,  // Simple ID from timestamp
            "time": (e.timestamp / 1000) as u32,
            "msg": e.event_type,
        }))
        .collect();
    
    Json(serde_json::json!({
        "events": events,
        "damage": []
    }))
}

async fn map_obj_handler(State(state): State<ServerState>) -> Json<Vec<serde_json::Value>> {
    Json(state.emulator.generate_map_objects())
}

async fn map_info_handler(State(state): State<ServerState>) -> Json<serde_json::Value> {
    Json(state.emulator.generate_map_info())
}

async fn map_img_handler(State(state): State<ServerState>, Query(_params): Query<HashMap<String, String>>) -> Response {
    use base64::{Engine as _, engine::general_purpose};
    
    let emu_state = state.emulator.get_state();
    
    if !emu_state.enabled || !emu_state.in_battle {
        // Return 1x1 transparent PNG
        let png_data = general_purpose::STANDARD.decode(
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
        ).unwrap();
        return ([(header::CONTENT_TYPE, "image/png")], png_data).into_response();
    }
    
    // Generate a simple placeholder map image (for now, just return empty)
    let png_data = general_purpose::STANDARD.decode(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
    ).unwrap();
    ([(header::CONTENT_TYPE, "image/png")], png_data).into_response()
}

async fn gamechat_handler(
    State(state): State<ServerState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<ChatMessage>> {
    let last_id = params.get("lastId").and_then(|s| s.parse().ok()).unwrap_or(0);
    
    let messages = state.chat_messages.read().await;
    let new_messages: Vec<ChatMessage> = messages
        .iter()
        .filter(|m| m.id > last_id)
        .cloned()
        .collect();
    
    Json(new_messages)
}

#[derive(Debug, Deserialize)]
struct SendChatRequest {
    message: String,
    mode: Option<String>,
}

async fn gamechat_send_handler(
    State(state): State<ServerState>,
    Json(payload): Json<SendChatRequest>,
) -> Json<ChatMessage> {
    let mut next_id = state.next_chat_id.write().await;
    let id = *next_id;
    *next_id += 1;
    
    let message = ChatMessage {
        id,
        time: (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32),
        sender: Some("TestPlayer".to_string()),
        mode: payload.mode.clone(),
        msg: payload.message.clone(),
        enemy: false,
    };
    
    state.chat_messages.write().await.push(message.clone());
    
    log::info!("[API Server] Chat message sent: {}", payload.message);
    
    Json(message)
}

async fn mission_handler(State(state): State<ServerState>) -> Json<serde_json::Value> {
    let emu_state = state.emulator.get_state();
    
    if !emu_state.enabled || !emu_state.in_battle {
        return Json(serde_json::json!({
            "status": "not_started",
            "objectives": []
        }));
    }
    
    Json(serde_json::json!({
        "status": "running",
        "objectives": [
            {
                "primary": true,
                "status": "in_progress",
                "text": "Test Objective: Destroy enemy forces"
            }
        ]
    }))
}

async fn info_handler() -> Response {
    (StatusCode::OK, "").into_response()
}

async fn gunner_view_handler() -> Response {
    (StatusCode::OK, "").into_response()
}

async fn root_handler() -> Response {
    (StatusCode::OK, "<html><body><h1>War Thunder API Emulator</h1><p>Server is running</p></body></html>").into_response()
}

