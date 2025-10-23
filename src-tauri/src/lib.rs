mod wt_telemetry;
mod pattern_engine;
mod device_manager;
mod rate_limiter;
mod event_engine;
mod profile_manager;
mod haptic_engine;
mod event_triggers;
mod wt_vehicles_api;
mod dynamic_triggers;
mod ui_patterns;

use haptic_engine::{HapticEngine, GameStatusInfo};
use pattern_engine::VibrationPattern;
use profile_manager::Profile;
use device_manager::DeviceInfo;
use tokio::sync::Mutex;
use std::sync::Arc;

// Global application state
pub struct AppState {
    engine: Arc<Mutex<HapticEngine>>,
}

// Tauri commands
#[tauri::command]
async fn init_devices(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.init_devices().await
        .map(|_| "Devices initialized".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_engine(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.start().await
        .map(|_| "Engine started".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_engine(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.stop().await
        .map(|_| "Engine stopped".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn is_running(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let engine = state.engine.lock().await;
    Ok(engine.is_running().await)
}

#[tauri::command]
async fn get_devices(state: tauri::State<'_, AppState>) -> Result<Vec<DeviceInfo>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_device_manager().get_devices().await)
}

#[tauri::command]
async fn get_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<Profile>, String> {
    let engine = state.engine.lock().await;
    let pm = engine.get_profile_manager();
    let profiles = pm.read().await;
    Ok(profiles.get_all_profiles().to_vec())
}

#[derive(serde::Deserialize)]
struct TestVibrationParams {
    intensity: f32,
    #[serde(rename = "durationMs")]
    duration_ms: u64,
}

#[tauri::command]
async fn test_vibration(
    state: tauri::State<'_, AppState>,
    params: TestVibrationParams,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.test_vibration(params.intensity, params.duration_ms).await
        .map(|_| "Test completed".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_preset_patterns() -> Vec<VibrationPattern> {
    vec![
        VibrationPattern::preset_critical_hit(),
        VibrationPattern::preset_simple_hit(),
        VibrationPattern::preset_engine_rumble(),
        VibrationPattern::preset_fire(),
    ]
}

#[tauri::command]
async fn get_game_status(state: tauri::State<'_, AppState>) -> Result<GameStatusInfo, String> {
    let engine = state.engine.lock().await;
    engine.get_game_status().await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct DebugInfo {
    indicators: String,
    triggers_count: usize,
    patterns_active: usize,
}

#[tauri::command]
async fn get_debug_info(state: tauri::State<'_, AppState>) -> Result<DebugInfo, String> {
    let engine = state.engine.lock().await;
    let status = engine.get_game_status().await.map_err(|e| e.to_string())?;
    
    Ok(DebugInfo {
        indicators: format!(
            "Speed: {} km/h, Alt: {} m, G: {:.1}, RPM: {}, Fuel: {}%",
            status.speed_kmh, status.altitude_m, status.g_load, 
            status.engine_rpm, status.fuel_percent
        ),
        triggers_count: 0, // TODO: get from engine
        patterns_active: 0, // TODO: get from engine
    })
}

#[tauri::command]
async fn get_triggers(state: tauri::State<'_, AppState>) -> Result<Vec<event_triggers::EventTrigger>, String> {
    let engine = state.engine.lock().await;
    let manager = engine.trigger_manager.read().await;
    Ok(manager.get_triggers().to_vec())
}

#[tauri::command]
async fn toggle_trigger(state: tauri::State<'_, AppState>, id: String, enabled: bool) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    manager.toggle_trigger(&id, enabled);
    
    log::info!("[Triggers] Toggle '{}' to {}", id, enabled);
    Ok(format!("Trigger {} toggled to {}", id, enabled))
}

#[tauri::command]
async fn add_pattern(
    state: tauri::State<'_, AppState>,
    pattern: ui_patterns::UIPattern
) -> Result<String, String> {
    log::info!("[UI Patterns] Received pattern: {}", pattern.name);
    
    // Convert UIPattern → EventTrigger
    let trigger = pattern.to_trigger()
        .ok_or_else(|| "Failed to convert pattern to trigger".to_string())?;
    
    log::info!("[UI Patterns] Converted to trigger: {:?}", trigger.name);
    
    // Add to TriggerManager
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    manager.add_trigger(trigger);
    
    log::info!("[UI Patterns] ✅ Pattern '{}' added to engine", pattern.name);
    Ok(format!("Pattern '{}' added successfully", pattern.name))
}

#[tauri::command]
async fn remove_pattern(
    _state: tauri::State<'_, AppState>,
    id: String
) -> Result<String, String> {
    log::info!("[UI Patterns] Removing pattern: {}", id);
    
    // TODO: Add remove_trigger method to TriggerManager
    Ok(format!("Pattern '{}' removed", id))
}

#[tauri::command]
async fn toggle_pattern(
    state: tauri::State<'_, AppState>,
    id: String,
    enabled: bool
) -> Result<String, String> {
    // Patterns are triggers, use toggle_trigger
    toggle_trigger(state, id, enabled).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::init();

    let engine = Arc::new(Mutex::new(HapticEngine::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { engine })
        .invoke_handler(tauri::generate_handler![
            init_devices,
            start_engine,
            stop_engine,
            is_running,
            get_devices,
            get_profiles,
            test_vibration,
            get_preset_patterns,
            get_game_status,
            get_debug_info,
            get_triggers,
            toggle_trigger,
            add_pattern,
            remove_pattern,
            toggle_pattern,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
