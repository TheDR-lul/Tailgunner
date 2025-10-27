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
    pub msg: String,
    pub sender: String,  // ALWAYS present in WT API
    pub enemy: bool,
    pub mode: String,    // ALWAYS present in WT API
    pub time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudMessage {
    pub id: u32,
    pub msg: String,
    pub sender: String,  // Empty string for system messages
    pub enemy: bool,
    pub mode: String,    // Empty string for most messages
    pub time: u32,
}

#[derive(Clone)]
pub struct ServerState {
    pub emulator: Arc<APIEmulator>,
    pub chat_messages: Arc<RwLock<Vec<ChatMessage>>>,
    pub next_chat_id: Arc<RwLock<u32>>,
    pub hud_events: Arc<RwLock<Vec<HudMessage>>>,
    pub hud_damage: Arc<RwLock<Vec<HudMessage>>>,
    pub next_hud_id: Arc<RwLock<u32>>,
    pub start_time: std::time::Instant,  // Battle start time for relative timestamps
}

pub async fn start_server(emulator: Arc<APIEmulator>) {
    let state = ServerState {
        emulator: emulator.clone(),
        chat_messages: Arc::new(RwLock::new(Vec::new())),
        next_chat_id: Arc::new(RwLock::new(1)),
        hud_events: Arc::new(RwLock::new(Vec::new())),
        hud_damage: Arc::new(RwLock::new(Vec::new())),
        next_hud_id: Arc::new(RwLock::new(1)),
        start_time: std::time::Instant::now(),  // Start battle time
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
        .route("/hudmsg/send", post(hudmsg_send_handler))
        
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
    
    // Try to bind with retries (in case port is in TIME_WAIT state from previous run)
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:8112").await {
        Ok(l) => l,
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            log::warn!("[API Server] ⚠️ Port 8112 in use, waiting 1s and retrying...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            match tokio::net::TcpListener::bind("127.0.0.1:8112").await {
                Ok(l) => l,
                Err(e) => {
                    log::error!("[API Server] ❌ Failed to bind after retry: {}", e);
                    log::error!("[API Server] 💡 TIP: Restart the application to free port 8112");
                    return; // Exit gracefully instead of panic
                }
            }
        }
        Err(e) => {
            log::error!("[API Server] ❌ Failed to bind to port 8112: {}", e);
            return; // Exit gracefully instead of panic
        }
    };
    
    log::info!("[API Server] ✅ Successfully bound to port 8112");
    
    if let Err(e) = axum::serve(listener, app).await {
        log::error!("[API Server] ❌ Server error: {}", e);
    }
    
    log::info!("[API Server] 🛑 Server stopped");
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
    let last_dmg = params.get("lastDmg").and_then(|s| s.parse().ok()).unwrap_or(0);
    
    // Get new events since last_evt
    let events_lock = state.hud_events.read().await;
    let new_events: Vec<&HudMessage> = events_lock
        .iter()
        .filter(|m| m.id > last_evt)
        .collect();
    
    // Get new damage messages since last_dmg
    let damage_lock = state.hud_damage.read().await;
    let new_damage: Vec<&HudMessage> = damage_lock
        .iter()
        .filter(|m| m.id > last_dmg)
        .collect();
    
    Json(serde_json::json!({
        "events": new_events,
        "damage": new_damage
    }))
}

#[derive(Debug, Deserialize)]
struct SendHudMessageRequest {
    message: String,
    #[serde(default)]
    event_type: String, // "event" or "damage"
}

async fn hudmsg_send_handler(
    State(state): State<ServerState>,
    Json(payload): Json<SendHudMessageRequest>,
) -> Json<HudMessage> {
    let mut next_id = state.next_hud_id.write().await;
    let id = *next_id;
    *next_id += 1;
    
    let hud_msg = HudMessage {
        id,
        msg: payload.message.clone(),
        sender: String::new(),  // Empty string for system messages
        enemy: false,
        mode: String::new(),    // Empty string
        time: state.start_time.elapsed().as_secs() as u32,  // Seconds from battle start
    };
    
    // Add to appropriate list
    if payload.event_type == "damage" {
        let mut damage = state.hud_damage.write().await;
        damage.push(hud_msg.clone());
        
        // Keep only last 100 messages (like WT does)
        let len = damage.len();
        if len > 100 {
            damage.drain(0..len - 100);
        }
        
        log::info!("[API Server] HUD Damage: {}", payload.message);
    } else {
        let mut events = state.hud_events.write().await;
        events.push(hud_msg.clone());
        
        // Keep only last 100 messages (like WT does)
        let len = events.len();
        if len > 100 {
            events.drain(0..len - 100);
        }
        
        log::info!("[API Server] HUD Event: {}", payload.message);
    }
    
    Json(hud_msg)
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
    sender: Option<String>,
    enemy: Option<bool>,
}

async fn gamechat_send_handler(
    State(state): State<ServerState>,
    Json(payload): Json<SendChatRequest>,
) -> Json<ChatMessage> {
    let mut next_id = state.next_chat_id.write().await;
    let id = *next_id;
    *next_id += 1;
    
    let sender = payload.sender.clone().unwrap_or_else(|| "TestPlayer".to_string());
    let enemy = payload.enemy.unwrap_or(false);
    
    let message = ChatMessage {
        id,
        msg: payload.message.clone(),
        sender: sender.clone(),
        enemy,
        mode: payload.mode.clone().unwrap_or_default(),  // Empty string if not provided
        time: state.start_time.elapsed().as_secs() as u32,  // Seconds from battle start
    };
    
    let mut chat = state.chat_messages.write().await;
    chat.push(message.clone());
    
    // Keep only last 100 messages (like WT does)
    let len = chat.len();
    if len > 100 {
        chat.drain(0..len - 100);
    }
    
    log::info!("[API Server] Chat from {}: {}", sender, payload.message);
    
    Json(message)
}

async fn mission_handler(State(state): State<ServerState>) -> Json<serde_json::Value> {
    use crate::api_emulator::VehicleType;
    
    let emu_state = state.emulator.get_state();
    
    if !emu_state.enabled || !emu_state.in_battle {
        return Json(serde_json::json!({
            "status": "not_started",
            "objectives": []
        }));
    }
    
    // Generate objectives based on vehicle type
    let objectives = match emu_state.vehicle_type {
        VehicleType::Aircraft => vec![
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Destroy all enemy aircraft"
            }),
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Destroy ground targets (0/12)"
            }),
            serde_json::json!({
                "primary": false,
                "status": "in_progress",
                "text": "Win by tickets"
            }),
        ],
        VehicleType::Tank => vec![
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Destroy all enemies"
            }),
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Capture and hold all points"
            }),
        ],
        VehicleType::Ship => vec![
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Destroy the enemy fleet"
            }),
            serde_json::json!({
                "primary": true,
                "status": "in_progress",
                "text": "Do not allow the destruction of the Allied fleet"
            }),
        ],
    };
    
    Json(serde_json::json!({
        "status": "running",
        "objectives": objectives
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

