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
mod player_identity_db;
mod device_history_db;
mod debug_dump;
mod hud_messages;
mod map_module;

use haptic_engine::{HapticEngine, GameStatusInfo};
use pattern_engine::VibrationPattern;
use profile_manager::Profile;
use device_manager::DeviceInfo;
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

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
async fn get_player_names(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_player_names().await)
}

#[tauri::command]
async fn set_player_names(state: tauri::State<'_, AppState>, names: Vec<String>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.set_player_names(names.clone()).await;
    
    // Save to database
    if let Err(e) = save_player_identity_to_db(&engine).await {
        log::warn!("[Player Identity] Failed to save to DB: {}", e);
    }
    
    Ok(format!("Player names set: {:?}", names))
}

#[tauri::command]
async fn get_clan_tags(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_clan_tags().await)
}

#[tauri::command]
async fn set_clan_tags(state: tauri::State<'_, AppState>, tags: Vec<String>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.set_clan_tags(tags.clone()).await;
    
    // Save to database
    if let Err(e) = save_player_identity_to_db(&engine).await {
        log::warn!("[Player Identity] Failed to save to DB: {}", e);
    }
    
    Ok(format!("Clan tags set: {:?}", tags))
}

#[tauri::command]
async fn get_enemy_names(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_enemy_names().await)
}

#[tauri::command]
async fn set_enemy_names(state: tauri::State<'_, AppState>, names: Vec<String>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.set_enemy_names(names.clone()).await;
    
    // Save to database
    if let Err(e) = save_player_identity_to_db(&engine).await {
        log::warn!("[Player Identity] Failed to save to DB: {}", e);
    }
    
    Ok(format!("Enemy names set: {:?}", names))
}

#[tauri::command]
async fn get_enemy_clans(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_enemy_clans().await)
}

#[tauri::command]
async fn set_enemy_clans(state: tauri::State<'_, AppState>, clans: Vec<String>) -> Result<String, String> {
    let engine = state.engine.lock().await;
    engine.set_enemy_clans(clans.clone()).await;
    
    // Save to database
    if let Err(e) = save_player_identity_to_db(&engine).await {
        log::warn!("[Player Identity] Failed to save to DB: {}", e);
    }
    
    Ok(format!("Enemy clans set: {:?}", clans))
}

/// Helper: Save player identity to database
async fn save_player_identity_to_db(engine: &HapticEngine) -> anyhow::Result<()> {
    let player_names = engine.get_player_names().await;
    let clan_tags = engine.get_clan_tags().await;
    let enemy_names = engine.get_enemy_names().await;
    let enemy_clans = engine.get_enemy_clans().await;
    
    let mut db = player_identity_db::PlayerIdentityDB::new()?;
    db.save(&player_names, &clan_tags, &enemy_names, &enemy_clans)?;
    
    Ok(())
}

#[tauri::command]
async fn get_devices(state: tauri::State<'_, AppState>) -> Result<Vec<DeviceInfo>, String> {
    let engine = state.engine.lock().await;
    Ok(engine.get_device_manager().get_devices().await)
}

#[tauri::command]
async fn get_device_history() -> Result<Vec<device_history_db::DeviceRecord>, String> {
    match device_history_db::DeviceHistoryDB::new() {
        Ok(db) => {
            match db.get_all_devices() {
                Ok(devices) => Ok(devices),
                Err(e) => Err(format!("Failed to get device history: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to open device history DB: {}", e))
    }
}

/// DEBUG: Direct HUD API polling
#[tauri::command]
async fn dump_wt_api() -> Result<String, String> {
    log::info!("[API Dump] Starting full API dump...");
    let dumper = debug_dump::ApiDumper::new();
    let data = dumper.dump_all().await;
    
    // Convert to pretty JSON
    match serde_json::to_string_pretty(&data) {
        Ok(json) => {
            log::info!("[API Dump] Successfully dumped {} endpoints", data.len());
            Ok(json)
        }
        Err(e) => {
            log::error!("[API Dump] Failed to serialize: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn debug_hud_raw() -> Result<String, String> {
    log::error!("[DEBUG] 🔍 Fetching HUD API directly...");
    
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8111/hudmsg?lastEvt=0&lastDmg=0")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
        .map_err(|e| format!("❌ HUD API request failed: {}", e))?;
    
    let text = response.text().await.map_err(|e| format!("❌ Failed to read response: {}", e))?;
    
    log::error!("[DEBUG] 📥 HUD API RAW RESPONSE:\n{}", text);
    
    Ok(text)
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
    recent_triggers: Vec<haptic_engine::TriggerEvent>,
}

#[tauri::command]
async fn get_debug_info(state: tauri::State<'_, AppState>) -> Result<DebugInfo, String> {
    let engine = state.engine.lock().await;
    let status = engine.get_game_status().await.map_err(|e| e.to_string())?;
    
    // Get triggers count
    let trigger_manager = engine.trigger_manager.read().await;
    let triggers_count = trigger_manager.get_triggers().len();
    let patterns_active = trigger_manager.get_triggers().iter().filter(|t| t.enabled).count();
    
    // Get recent trigger events
    let recent_triggers = engine.get_recent_trigger_events().await;
    
    Ok(DebugInfo {
        indicators: format!(
            "Vehicle: {}, Speed: {} km/h, Alt: {} m, G: {:.1}, RPM: {}, Fuel: {}%",
            status.vehicle_name, status.speed_kmh, status.altitude_m, status.g_load, 
            status.engine_rpm, status.fuel_percent
        ),
        triggers_count,
        patterns_active,
        recent_triggers,
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
    match vehicle_data {
        // Ground vehicles - fetch speed and gears
        datamine::VehicleLimits::Ground(ref mut ground) => {
            let needs_wiki = ground.max_speed_kmh.is_none() || ground.forward_gears.is_none();
            
            if needs_wiki {
                log::info!("[Vehicle Info] 🌐 Fetching missing ground data from Wiki for '{}'...", identifier);
                
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
        },
        
        // Aircraft - fetch G-load limits and speed limits
        datamine::VehicleLimits::Aircraft(ref mut aircraft) => {
            let needs_wiki = aircraft.max_positive_g.is_none() 
                || aircraft.max_negative_g.is_none()
                || aircraft.flaps_speeds_kmh.is_empty()
                || aircraft.gear_max_speed_kmh.is_none() || aircraft.gear_max_speed_kmh == Some(0.0);
            
            if needs_wiki {
                log::info!("[Vehicle Info] 🌐 Fetching missing aircraft G-load from Wiki for '{}'...", identifier);
                
                match datamine::wiki_scraper::scrape_aircraft_vehicle(&aircraft.identifier).await {
                    Ok(wiki_data) => {
                        let mut updated = false;
                        
                        // Update missing G-load fields
                        if aircraft.max_positive_g.is_none() && wiki_data.max_positive_g.is_some() {
                            aircraft.max_positive_g = wiki_data.max_positive_g;
                            updated = true;
                            log::info!("[Vehicle Info] ✅ Wiki G-load: +{}G / {}G",
                                wiki_data.max_positive_g.unwrap(),
                                wiki_data.max_negative_g.unwrap_or(0.0)
                            );
                        }
                        if aircraft.max_negative_g.is_none() && wiki_data.max_negative_g.is_some() {
                            aircraft.max_negative_g = wiki_data.max_negative_g;
                            updated = true;
                        }
                        
                        // Update missing speed limits
                        if aircraft.flaps_speeds_kmh.is_empty() && !wiki_data.flaps_speeds_kmh.is_empty() {
                            aircraft.flaps_speeds_kmh = wiki_data.flaps_speeds_kmh.clone();
                            updated = true;
                            log::info!("[Vehicle Info] ✅ Wiki Flap Speeds: {:?} km/h",
                                wiki_data.flaps_speeds_kmh
                            );
                        }
                        if (aircraft.gear_max_speed_kmh.is_none() || aircraft.gear_max_speed_kmh == Some(0.0))
                            && wiki_data.gear_max_speed_kmh.is_some() {
                            aircraft.gear_max_speed_kmh = wiki_data.gear_max_speed_kmh;
                            updated = true;
                            log::info!("[Vehicle Info] ✅ Wiki Gear Speed: {} km/h",
                                wiki_data.gear_max_speed_kmh.unwrap()
                            );
                        }
                        
                        if updated {
                            aircraft.data_source = "datamine+wiki".to_string();
                            
                            // Save updated data to database
                            match datamine::database::VehicleDatabase::new() {
                                Ok(mut db) => {
                                    if let Err(e) = db.update_aircraft_wiki_data(&aircraft) {
                                        log::warn!("[Vehicle Info] ⚠️ Failed to save Wiki data: {}", e);
                                    } else {
                                        log::info!("[Vehicle Info] ✅ Saved Wiki data to database");
                                    }
                                },
                                Err(e) => {
                                    log::warn!("[Vehicle Info] ⚠️ Failed to open database: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("[Vehicle Info] ⚠️ Wiki scraping failed: {}", e);
                    }
                }
            }
        },
        
        // Ships - no Wiki scraping yet
        datamine::VehicleLimits::Ship(_) => {
            // No Wiki scraping for ships yet
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

/// Get map objects from War Thunder API
#[tauri::command]
async fn get_map_objects() -> Result<Vec<map_module::MapObject>, String> {
    let url = format!("{}/map_obj.json", "http://127.0.0.1:8111");
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(500))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to War Thunder API: {}", e))?;
    
    let objects: Vec<map_module::MapObject> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse map objects: {}", e))?;
    
    Ok(objects)
}

/// Get map info from War Thunder API
#[tauri::command]
async fn get_map_info() -> Result<map_module::MapInfo, String> {
    let url = format!("{}/map_info.json", "http://127.0.0.1:8111");
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(500))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to War Thunder API: {}", e))?;
    
    let info: map_module::MapInfo = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse map info: {}", e))?;
    
    Ok(info)
}

/// Get combined map data
#[tauri::command]
async fn get_map_data() -> Result<map_module::MapData, String> {
    let objects = get_map_objects().await?;
    let info = get_map_info().await?;
    
    Ok(map_module::MapData::new(objects, info))
}

/// Get map image as base64
#[tauri::command]
async fn get_map_image(map_generation: u32) -> Result<String, String> {
    let url = format!("http://127.0.0.1:8111/map.img?gen={}", map_generation);
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch map image: {}", e))?;
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;
    
    // Convert to base64
    use base64::{Engine as _, engine::general_purpose};
    let base64_image = general_purpose::STANDARD.encode(&bytes);
    
    Ok(format!("data:image/png;base64,{}", base64_image))
}

/// Vehicle mode info for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VehicleModeInfo {
    vehicle_type: String,  // "Ship", "Aircraft", "Tank", "Helicopter", "Unknown"
    mode: String,          // "HudOnly", "FullTelemetry", "Disconnected"
    available_data: Vec<String>,
}

/// Get current vehicle mode and data availability
#[tauri::command]
async fn get_vehicle_mode(state: tauri::State<'_, AppState>) -> Result<VehicleModeInfo, String> {
    let engine = state.engine.lock().await;
    let telemetry = engine.get_telemetry();
    let mut telem = telemetry.write().await;
    
    match telem.get_state().await {
        Ok(game_state) => {
            let vehicle_type = format!("{:?}", game_state.type_);
            let is_ship = matches!(game_state.type_, wt_telemetry::VehicleType::Ship);
            
            let mode = if is_ship {
                "HudOnly".to_string()
            } else if game_state.valid {
                "FullTelemetry".to_string()
            } else {
                "Disconnected".to_string()
            };
            
            let available_data = if is_ship {
                vec![
                    "Combat events (kills, hits)".to_string(),
                    "Damage events (fire, flooding)".to_string(),
                    "Achievements".to_string(),
                    "Map position".to_string(),
                ]
            } else {
                vec![
                    "Speed & altitude".to_string(),
                    "Engine telemetry".to_string(),
                    "Fuel & ammo".to_string(),
                    "Damage state".to_string(),
                    "All combat events".to_string(),
                ]
            };
            
            Ok(VehicleModeInfo {
                vehicle_type,
                mode,
                available_data,
            })
        }
        Err(_) => {
            Ok(VehicleModeInfo {
                vehicle_type: "Unknown".to_string(),
                mode: "Disconnected".to_string(),
                available_data: vec![],
            })
        }
    }
}

pub fn run() {
    // Initialize logger
    // Set default log level: DEBUG for our code, WARN for libraries
    if std::env::var("RUST_LOG").is_err() {
        #[cfg(debug_assertions)]
        std::env::set_var("RUST_LOG", "butt_thunder_lib=debug,warn");
        
        #[cfg(not(debug_assertions))]
        std::env::set_var("RUST_LOG", "butt_thunder_lib=info,warn");
    }
    env_logger::init();

    let engine = HapticEngine::new();
    
    // Load player identity from database
    match player_identity_db::PlayerIdentityDB::new() {
        Ok(db) => {
            match db.load() {
                Ok((player_names, clan_tags, enemy_names, enemy_clans)) => {
                    log::info!("[Startup] 💾 Loaded player identity from database");
                    // Set data in engine (need to do this synchronously before Arc)
                    let telemetry = engine.get_telemetry();
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
                        let mut telem = telemetry.write().await;
                        telem.set_player_names(player_names);
                        telem.set_clan_tags(clan_tags);
                        telem.set_enemy_names(enemy_names);
                        telem.set_enemy_clans(enemy_clans);
                    });
                }
                Err(e) => log::warn!("[Startup] ⚠️ Failed to load player identity: {}", e),
            }
        }
        Err(e) => log::warn!("[Startup] ⚠️ Failed to open player identity DB: {}", e),
    }
    
    let engine = Arc::new(Mutex::new(engine));
    
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
            get_player_names,
            set_player_names,
            get_clan_tags,
            set_clan_tags,
            get_enemy_names,
            set_enemy_names,
            get_enemy_clans,
            set_enemy_clans,
            get_devices,
            get_device_history,
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
            dump_wt_api,
            debug_hud_raw,
            datamine_find_game,
            datamine_parse,
            datamine_get_limits,
            datamine_get_stats,
            datamine_auto_init,
            datamine_rebuild,
            get_map_objects,
            get_map_info,
            get_map_data,
            get_map_image,
            get_vehicle_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
