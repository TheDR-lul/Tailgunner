mod wt_telemetry;
mod pattern_engine;
mod device_manager;
mod rate_limiter;
mod event_engine;
mod profile_manager;
mod haptic_engine;
mod event_triggers;
mod ui_patterns;
mod vehicle_limits;
mod state_history;
mod datamine;

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
async fn scan_devices(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.get_device_manager().scan_devices().await
        .map(|_| "Device scan started".to_string())
        .map_err(|e| e.to_string())
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

// Lovense commands
#[tauri::command]
async fn add_lovense_device(
    state: tauri::State<'_, AppState>,
    ip: String,
    port: Option<u16>,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.get_device_manager().add_lovense_device(ip, port).await
        .map(|_| "Lovense device added".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_lovense_device(
    state: tauri::State<'_, AppState>,
    device_id: String,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.get_device_manager().remove_lovense_device(&device_id).await
        .map(|_| "Lovense device removed".to_string())
        .map_err(|e| e.to_string())
}

// Pattern import/export commands
use ui_patterns::UIPattern;

#[tauri::command]
async fn export_pattern(
    state: tauri::State<'_, AppState>,
    pattern_id: String,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let triggers = engine.trigger_manager.read().await;
    
    // Find pattern by ID (user patterns only)
    let pattern = triggers.get_triggers()
        .iter()
        .filter(|t| !t.is_builtin)
        .find(|t| t.id == pattern_id)
        .ok_or_else(|| format!("Pattern not found: {}", pattern_id))?;
    
    // Serialize to JSON
    serde_json::to_string_pretty(&pattern)
        .map_err(|e| format!("Failed to serialize pattern: {}", e))
}

#[tauri::command]
async fn export_all_patterns(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let triggers = engine.trigger_manager.read().await;
    
    // Get user patterns only
    let patterns: Vec<_> = triggers.get_triggers()
        .iter()
        .filter(|t| !t.is_builtin)
        .cloned()
        .collect();
    
    // Serialize to JSON
    serde_json::to_string_pretty(&patterns)
        .map_err(|e| format!("Failed to serialize patterns: {}", e))
}

#[tauri::command]
async fn import_pattern(
    state: tauri::State<'_, AppState>,
    json_data: String,
) -> Result<String, String> {
    // Deserialize from JSON
    let ui_pattern: UIPattern = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to parse pattern JSON: {}", e))?;
    
    // Convert to trigger
    let trigger = ui_pattern.to_trigger()
        .ok_or_else(|| "Failed to convert pattern to trigger".to_string())?;
    
    // Add to engine
    let engine = state.engine.lock().await;
    engine.trigger_manager.write().await.add_trigger(trigger.clone());
    
    log::info!("✅ Pattern imported: {}", trigger.name);
    Ok(format!("Pattern imported: {}", trigger.name))
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
    
    // Get triggers count
    let trigger_manager = engine.trigger_manager.read().await;
    let triggers_count = trigger_manager.get_triggers().len();
    let patterns_active = trigger_manager.get_triggers().iter().filter(|t| t.enabled).count();
    
    Ok(DebugInfo {
        indicators: format!(
            "Vehicle: {}, Speed: {} km/h, Alt: {} m, G: {:.1}, RPM: {}, Fuel: {}%",
            status.vehicle_name, status.speed_kmh, status.altitude_m, status.g_load, 
            status.engine_rpm, status.fuel_percent
        ),
        triggers_count,
        patterns_active,
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
    
    // Auto-save settings
    let config_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let settings_path = config_dir.join("trigger_settings.json");
    let _ = manager.save_settings(&settings_path);
    
    log::info!("[Triggers] Toggle '{}' to {}", id, enabled);
    Ok(format!("Trigger {} toggled to {}", id, enabled))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SimpleVibration {
    intensity: f32,
    duration_ms: u64,
    curve: Option<Vec<pattern_engine::CurvePoint>>,
}

#[tauri::command]
async fn update_trigger(
    state: tauri::State<'_, AppState>, 
    id: String, 
    cooldown_ms: Option<u64>,
    pattern: Option<SimpleVibration>
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    
    // Convert SimpleVibration to VibrationPattern and save curve points
    let (vibration_pattern, curve_points) = match pattern {
        Some(simple) => {
            let curve = simple.curve.clone();
            let pattern = if let Some(ref curve_points) = simple.curve {
                pattern_engine::VibrationPattern::from_curve_points(curve_points.clone(), simple.duration_ms)
            } else {
                pattern_engine::VibrationPattern::simple(simple.intensity, simple.duration_ms)
            };
            (Some(Some(pattern)), curve)
        },
        None => (None, None)  // Don't touch pattern if not provided
    };
    
    manager.update_trigger_with_curve(&id, cooldown_ms, vibration_pattern, curve_points)?;
    
    // Auto-save settings
    let config_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let settings_path = config_dir.join("trigger_settings.json");
    let _ = manager.save_settings(&settings_path);
    
    log::info!("[Triggers] Updated trigger '{}': cooldown={:?}", id, cooldown_ms);
    Ok("Trigger updated".to_string())
}

#[tauri::command]
async fn save_trigger_settings(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let manager = engine.trigger_manager.read().await;
    
    let config_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let settings_path = config_dir.join("trigger_settings.json");
    
    manager.save_settings(&settings_path)?;
    Ok(format!("Settings saved to: {}", settings_path.display()))
}

#[tauri::command]
async fn get_vehicle_info(state: tauri::State<'_, AppState>) -> Result<Option<datamine::VehicleLimits>, String> {
    let engine = state.engine.lock().await;
    let status = engine.get_game_status().await
        .map_err(|e| format!("Failed to get game status: {}", e))?;
    
    if !status.connected || status.vehicle_name.is_empty() || status.vehicle_name == "N/A" {
        log::debug!("[Vehicle Info] No vehicle connected");
        return Ok(None);
    }
    
    log::info!("[Vehicle Info] 🔍 Fetching data for vehicle: '{}'", status.vehicle_name);
    
    // Get vehicle limits from datamine database
    let db = datamine::database::VehicleDatabase::new()
        .map_err(|e| format!("Database error: {}", e))?;
    
    // Use vehicle_name AS IS (fuzzy search will handle variations)
    let identifier = status.vehicle_name.clone();
    
    log::info!("[Vehicle Info] 🔎 Searching for: '{}'", identifier);
    
    let mut vehicle_data = match db.get_limits(&identifier) {
        Some(data) => data,
        None => {
            log::error!("[Vehicle Info] ❌ NO DATA FOUND for vehicle: '{}'", identifier);
            log::error!("[Vehicle Info] Check datamine database or run: datamine_parse");
            return Ok(None);
        }
    };
    
    log::info!("[Vehicle Info] ✅ Found vehicle data!");
    
    // 🌐 LAZY WIKI LOADING - fetch missing data on-demand
    if let datamine::VehicleLimits::Ground(ref mut ground) = vehicle_data {
        let needs_wiki = ground.max_speed_kmh.is_none() || ground.forward_gears.is_none();
        
        if needs_wiki {
            log::info!("[Vehicle Info] 🌐 Fetching missing data from Wiki for '{}'...", identifier);
            
            match datamine::wiki_scraper::scrape_ground_vehicle(&ground.identifier).await {
                Ok(wiki_data) => {
                    let mut updated = false;
                    
                    // Update missing fields
                    if ground.max_speed_kmh.is_none() && wiki_data.max_speed_kmh.is_some() {
                        ground.max_speed_kmh = wiki_data.max_speed_kmh;
                        updated = true;
                    }
                    if ground.max_reverse_speed_kmh.is_none() && wiki_data.max_reverse_speed_kmh.is_some() {
                        ground.max_reverse_speed_kmh = wiki_data.max_reverse_speed_kmh;
                        updated = true;
                    }
                    if ground.forward_gears.is_none() && wiki_data.forward_gears.is_some() {
                        ground.forward_gears = wiki_data.forward_gears;
                        updated = true;
                    }
                    if ground.reverse_gears.is_none() && wiki_data.reverse_gears.is_some() {
                        ground.reverse_gears = wiki_data.reverse_gears;
                        updated = true;
                    }
                    
                    if updated {
                        ground.data_source = "datamine+wiki".to_string();
                        
                        // Save to database for next time
                        if let Err(e) = db.update_ground_wiki_data(
                            &ground.identifier,
                            ground.max_speed_kmh,
                            ground.max_reverse_speed_kmh,
                            ground.forward_gears,
                            ground.reverse_gears,
                        ) {
                            log::warn!("[Vehicle Info] ⚠️ Failed to save Wiki data: {}", e);
                        } else {
                            log::info!("[Vehicle Info] ✅ Wiki data cached for future use");
                        }
                    }
                }
                Err(e) => {
                    log::warn!("[Vehicle Info] ⚠️ Wiki scraping failed: {}", e);
                }
            }
        }
    }
    
    Ok(Some(vehicle_data))
}

#[tauri::command]
async fn add_pattern(
    state: tauri::State<'_, AppState>,
    pattern: ui_patterns::UIPattern
) -> Result<String, String> {
    log::info!("[UI Patterns] Received pattern: '{}' ({} nodes)", pattern.name, pattern.nodes.len());
    
    // Convert UIPattern → EventTrigger
    let trigger = pattern.to_trigger()
        .ok_or_else(|| "Failed to convert pattern to trigger".to_string())?;
    
    log::info!("[UI Patterns] Converted to trigger: '{}' ({:?})", trigger.name, trigger.condition);
    
    // Add to TriggerManager
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    manager.add_trigger(trigger);
    
    log::info!("[UI Patterns] Pattern '{}' added to engine", pattern.name);
    Ok(format!("Pattern '{}' added successfully", pattern.name))
}

#[tauri::command]
async fn remove_pattern(
    state: tauri::State<'_, AppState>,
    id: String
) -> Result<String, String> {
    log::info!("[UI Patterns] Removing pattern: {}", id);
    
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    
    if manager.remove_trigger(&id) {
        Ok(format!("Pattern '{}' removed successfully", id))
    } else {
        Err(format!("Pattern '{}' not found", id))
    }
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
// Datamine commands
#[tauri::command]
async fn datamine_find_game() -> Result<String, String> {
    datamine::Datamine::auto_detect()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn datamine_parse(game_path: String) -> Result<datamine::ParseStats, String> {
    log::info!("[Datamine] Starting parse from: {}", game_path);
    log::info!("[Datamine] Wiki data will be fetched on-demand when vehicles are selected");
    
    let path = std::path::PathBuf::from(game_path);
    
    let mut dm = datamine::Datamine::new(path)
        .map_err(|e| e.to_string())?;
    
    // Parse (now async, without Wiki)
    let result = dm.parse_all().await
        .map_err(|e| e.to_string())?;
    
    log::info!("[Datamine] Parse complete: {} aircraft, {} ground, {} ships",
        result.aircraft_count, result.ground_count, result.ships_count);
    
    Ok(result)
}

#[tauri::command]
async fn datamine_get_limits(identifier: String) -> Result<Option<datamine::VehicleLimits>, String> {
    let db = datamine::database::VehicleDatabase::new()
        .map_err(|e| e.to_string())?;
    
    Ok(db.get_limits(&identifier))
}

#[tauri::command]
async fn datamine_get_stats() -> Result<(usize, usize, usize), String> {
    let db = datamine::database::VehicleDatabase::new()
        .map_err(|e| e.to_string())?;
    
    db.get_stats().map_err(|e| e.to_string())
}

/// Auto-initialize datamine: find game, check database, build if needed
#[tauri::command]
async fn datamine_auto_init() -> Result<String, String> {
    log::info!("[Datamine] Starting auto-initialization");
    
    // Check if database exists and has data
    let db = datamine::database::VehicleDatabase::new()
        .map_err(|e| e.to_string())?;
    
    let (aircraft, ground, ships) = db.get_stats()
        .map_err(|e| e.to_string())?;
    
    if aircraft > 0 || ground > 0 || ships > 0 {
        log::info!("[Datamine] Database exists: {} aircraft, {} ground, {} ships", aircraft, ground, ships);
        return Ok(format!("Database loaded: {} aircraft, {} ground, {} ships", aircraft, ground, ships));
    }
    
    // Database is empty, try to build it
    log::info!("[Datamine] Database empty, attempting auto-build");
    
    // Find War Thunder installation
    let game_path = datamine::Datamine::auto_detect()
        .map_err(|_| "War Thunder installation not found. Please use manual parse.".to_string())?;
    
    log::info!("[Datamine] Found game at: {:?}", game_path);
    
    // Build database
    let stats = datamine_parse(game_path.to_string_lossy().to_string()).await?;
    
    Ok(format!("Database built: {} aircraft, {} ground, {} ships", 
        stats.aircraft_count, stats.ground_count, stats.ships_count))
}

/// Force rebuild database (delete + reparse)
#[tauri::command]
async fn datamine_rebuild() -> Result<datamine::ParseStats, String> {
    log::info!("[Datamine] FORCE REBUILD: deleting old database...");
    
    // Delete database file
    let db_path = dirs::data_local_dir()
        .ok_or_else(|| "Failed to get local data directory".to_string())?
        .join("Tailgunner")
        .join("vehicle_limits.db");
    
    if db_path.exists() {
        std::fs::remove_file(&db_path)
            .map_err(|e| format!("Failed to delete database: {}", e))?;
        log::info!("[Datamine] ✅ Old database deleted");
    }
    
    // Find game
    let game_path = datamine::Datamine::auto_detect()
        .map_err(|_| "War Thunder installation not found".to_string())?;
    
    log::info!("[Datamine] Found game at: {:?}", game_path);
    
    // Rebuild (Wiki data will be fetched on-demand)
    datamine_parse(game_path.to_string_lossy().to_string()).await
}

pub fn run() {
    // Initialize logger
    // Set default log level: DEBUG для нашего кода, WARN для библиотек
    if std::env::var("RUST_LOG").is_err() {
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "butt_thunder_lib=debug,warn");
        
        #[cfg(not(debug_assertions))]
        std::env::set_var("RUST_LOG", "butt_thunder_lib=info,warn");
    }
    env_logger::init();

    let engine = Arc::new(Mutex::new(HapticEngine::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { engine })
        .invoke_handler(tauri::generate_handler![
            init_devices,
            start_engine,
            stop_engine,
            is_running,
            get_devices,
            scan_devices,
            get_profiles,
            test_vibration,
            get_preset_patterns,
            get_game_status,
            get_debug_info,
            get_triggers,
            toggle_trigger,
            update_trigger,
            save_trigger_settings,
            get_vehicle_info,
            add_pattern,
            remove_pattern,
            toggle_pattern,
            add_lovense_device,
            remove_lovense_device,
            export_pattern,
            export_all_patterns,
            import_pattern,
            datamine_find_game,
            datamine_parse,
            datamine_get_limits,
            datamine_get_stats,
            datamine_auto_init,
            datamine_rebuild,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
