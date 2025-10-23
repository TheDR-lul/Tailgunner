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

use haptic_engine::HapticEngine;
use pattern_engine::VibrationPattern;
use profile_manager::Profile;
use device_manager::DeviceInfo;
use tokio::sync::Mutex;
use std::sync::Arc;

// Глобальное состояние приложения
pub struct AppState {
    engine: Arc<Mutex<HapticEngine>>,
}

// Tauri команды
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

#[tauri::command]
async fn test_vibration(
    state: tauri::State<'_, AppState>,
    intensity: f32,
    duration_ms: u64,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.test_vibration(intensity, duration_ms).await
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Инициализация логгера
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
