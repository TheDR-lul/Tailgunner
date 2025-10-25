/// Haptic Engine - Main system coordinator
/// Binds all modules and manages data flow

use crate::{
    device_manager::DeviceManager,
    event_engine::EventEngine,
    event_triggers::TriggerManager,
    vehicle_limits::VehicleLimitsManager,
    pattern_engine::{VibrationPattern, GameEvent},
    profile_manager::ProfileManager,
    rate_limiter::RateLimiter,
    wt_telemetry::WTTelemetryReader,
};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatusInfo {
    pub connected: bool,
    pub vehicle_name: String,
    pub speed_kmh: i32,
    pub altitude_m: i32,
    pub g_load: f32,
    pub engine_rpm: i32,
    pub fuel_percent: i32,
}

impl GameStatusInfo {
    pub fn disconnected() -> Self {
        Self {
            connected: false,
            vehicle_name: "N/A".to_string(),
            speed_kmh: 0,
            altitude_m: 0,
            g_load: 0.0,
            engine_rpm: 0,
            fuel_percent: 0,
        }
    }
}

pub struct HapticEngine {
    telemetry: Arc<RwLock<WTTelemetryReader>>,
    device_manager: Arc<DeviceManager>,
    event_engine: Arc<RwLock<EventEngine>>,
    profile_manager: Arc<RwLock<ProfileManager>>,
    pub trigger_manager: Arc<RwLock<TriggerManager>>,  // pub for access from lib.rs
    vehicle_limits_manager: Arc<VehicleLimitsManager>,
    rate_limiter: Arc<RateLimiter>,
    running: Arc<RwLock<bool>>,
    current_intensity: Arc<RwLock<f32>>,
    last_vehicle_name: Arc<RwLock<String>>,
}

impl HapticEngine {
    pub fn new() -> Self {
        let mut trigger_manager = TriggerManager::new();
        
        // Try to load saved trigger settings
        let config_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."));
        let settings_path = config_dir.join("trigger_settings.json");
        
        if let Err(e) = trigger_manager.load_settings(&settings_path) {
            log::warn!("[Triggers] Failed to load settings: {}", e);
        }
        
        // Initialize vehicle limits manager
        let vehicle_limits_manager = Arc::new(
            VehicleLimitsManager::new()
                .expect("Failed to initialize VehicleLimitsManager (database error)")
        );
        
        Self {
            telemetry: Arc::new(RwLock::new(WTTelemetryReader::new())),
            device_manager: Arc::new(DeviceManager::new()),
            event_engine: Arc::new(RwLock::new(EventEngine::new())),
            profile_manager: Arc::new(RwLock::new(ProfileManager::new())),
            trigger_manager: Arc::new(RwLock::new(trigger_manager)),
            vehicle_limits_manager,
            rate_limiter: Arc::new(RateLimiter::new()),
            running: Arc::new(RwLock::new(false)),
            current_intensity: Arc::new(RwLock::new(0.0)),
            last_vehicle_name: Arc::new(RwLock::new(String::new())),
        }
    }

    /// Initialize devices
    pub async fn init_devices(&self) -> Result<()> {
        self.device_manager.init_buttplug().await?;
        self.device_manager.scan_devices().await?;
        
        // Give time for device discovery
        tokio::time::sleep(Duration::from_secs(3)).await;
        self.device_manager.stop_scanning().await?;
        
        log::info!("Devices initialized");
        Ok(())
    }

    /// Start main loop
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;
        
        let telemetry = Arc::clone(&self.telemetry);
        let device_manager = Arc::clone(&self.device_manager);
        let event_engine = Arc::clone(&self.event_engine);
        let profile_manager = Arc::clone(&self.profile_manager);
        let trigger_manager = Arc::clone(&self.trigger_manager);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let running = Arc::clone(&self.running);
        let current_intensity = Arc::clone(&self.current_intensity);
        let vehicle_limits_manager = Arc::clone(&self.vehicle_limits_manager);
        let last_vehicle_name = Arc::clone(&self.last_vehicle_name);

        tokio::spawn(async move {
            let mut tick_interval = interval(WTTelemetryReader::get_poll_interval());
            let mut connection_lost_counter = 0u32;
            let mut connection_established = false;
            
            while *running.read().await {
                tick_interval.tick().await;

                // Check game connection
                let game_state = {
                    let mut telem = telemetry.write().await;
                    match telem.get_state().await {
                        Ok(state) => {
                            // Connection successful
                            if !connection_established || connection_lost_counter > 0 {
                                log::info!("🎮 War Thunder connected! Vehicle: {:?}", state.type_);
                                connection_established = true;
                                connection_lost_counter = 0;
                            }
                            
                            log::debug!("[WT] Vehicle: {:?}, Speed: {:.0} km/h, Alt: {:.0}m, Fuel: {:.0}/{:.0} kg", 
                                state.type_, 
                                state.indicators.speed,  // Already in km/h from API
                                state.indicators.altitude,
                                state.indicators.fuel,
                                state.indicators.fuel_max
                            );
                            state
                        },
                        Err(_) => {
                            // Game not running or connection lost
                            connection_lost_counter += 1;
                            
                            if connection_lost_counter == 1 {
                                log::warn!("⚠️ War Thunder connection lost! Waiting for reconnect...");
                            } else if connection_lost_counter % 10 == 0 {
                                log::warn!("🔄 Still waiting for War Thunder... ({}s)", connection_lost_counter);
                            }
                            
                            drop(telem);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    }
                };

                // Automatic profile selection
                {
                    let mut pm = profile_manager.write().await;
                    pm.auto_select_profile(&game_state.type_);
                }
                
                // Check if vehicle changed and update dynamic triggers
                {
                    let last_vehicle = last_vehicle_name.read().await;
                    if game_state.vehicle_name != "unknown" && *last_vehicle != game_state.vehicle_name {
                        drop(last_vehicle);
                        log::info!("[Vehicle Limits] 🔄 Vehicle changed: {} (type: {:?})", game_state.vehicle_name, game_state.type_);
                        
                        // Auto-switch profile based on vehicle type
                        {
                            let mut pm = profile_manager.write().await;
                            if let Some(profile) = pm.auto_select_profile(&game_state.type_) {
                                log::info!("[Profile] ✅ Auto-selected profile: {}", profile.name);
                            }
                        }
                        
                        // Update vehicle limits
                        if let Err(e) = vehicle_limits_manager.update_vehicle(&game_state.vehicle_name).await {
                            log::warn!("[Vehicle Limits] ⚠️ Failed to update vehicle: {}", e);
                        }
                        
                        // Generate dynamic triggers based on limits
                        let limit_triggers = vehicle_limits_manager.generate_limit_triggers().await;
                        if !limit_triggers.is_empty() {
                            log::info!("[Dynamic Triggers] 🔧 Generated {} dynamic triggers for {}", 
                                limit_triggers.len(), game_state.vehicle_name);
                            
                            let mut tm = trigger_manager.write().await;
                            
                            // Remove old dynamic triggers
                            tm.get_triggers_mut().retain(|t| !t.id.starts_with("dynamic_"));
                            
                            // Add new dynamic triggers
                            for trigger in &limit_triggers {
                                log::info!("[Dynamic Triggers] ➕ Adding trigger: {} (enabled: {})", 
                                    trigger.name, trigger.enabled);
                                tm.add_trigger(trigger.clone());
                            }
                        } else {
                            log::warn!("[Dynamic Triggers] ⚠️ No dynamic triggers generated for {}", 
                                game_state.vehicle_name);
                        }
                        
                        // Update last vehicle name
                        *last_vehicle_name.write().await = game_state.vehicle_name.clone();
                    }
                }

                // Detect events from state
                let basic_events = {
                    let mut ee = event_engine.write().await;
                    ee.detect_events(&game_state)
                };
                
                // Check custom triggers (overspeed, over-G, etc.)
                let trigger_events = {
                    let mut tm = trigger_manager.write().await;
                    let events = tm.check_triggers(&game_state);
                    
                    if !events.is_empty() {
                        log::info!("[Triggers] 🎯 {} triggers fired", events.len());
                        for (event, pattern) in &events {
                            log::info!("[Triggers]   - {:?} (pattern: {})", 
                                event, pattern.is_some());
                        }
                    }
                    
                    events
                };
                
                // Process HUD events (kills, crashes, overheats)
                let hud_events: Vec<GameEvent> = game_state.hud_events.iter().map(|hud_evt| {
                    use crate::wt_telemetry::HudEvent;
                    match hud_evt {
                        HudEvent::Kill(enemy) => {
                            log::info!("[HUD] 🎯 Kill: {}", enemy);
                            GameEvent::TargetDestroyed
                        }
                        HudEvent::Crashed => {
                            log::warn!("[HUD] 💥 Crashed!");
                            GameEvent::Crashed
                        }
                        HudEvent::EngineOverheated => {
                            log::warn!("[HUD] 🔥 Engine overheated!");
                            GameEvent::EngineOverheat
                        }
                        HudEvent::OilOverheated => {
                            log::warn!("[HUD] 🛢️ Oil overheated!");
                            GameEvent::OilOverheated
                        }
                        HudEvent::SetAfire(victim) => {
                            log::info!("[HUD] 🔥 Set enemy on fire: {}", victim);
                            GameEvent::EnemySetAfire
                        }
                        HudEvent::TakeDamage(attacker) => {
                            log::warn!("[HUD] 💥 Taking damage from: {}", attacker);
                            GameEvent::TakingDamage
                        }
                        HudEvent::SeverelyDamaged(attacker) => {
                            log::warn!("[HUD] 💔 Severely damaged by: {}", attacker);
                            GameEvent::SeverelyDamaged
                        }
                        HudEvent::ShotDown(attacker) => {
                            log::warn!("[HUD] ✈️💥 Shot down by: {}", attacker);
                            GameEvent::ShotDown
                        }
                        HudEvent::Achievement(name) => {
                            log::info!("[HUD] 🏆 Achievement: {}", name);
                            GameEvent::Achievement
                        }
                        HudEvent::ChatMessage(text) => {
                            log::debug!("[HUD] 💬 Chat message: {}", text);
                            GameEvent::ChatMessage
                        }
                    }
                }).collect();
                
                // Combine events: basic, triggers (including dynamic), and HUD events
                let mut all_events: Vec<(GameEvent, Option<VibrationPattern>)> = trigger_events;
                all_events.extend(basic_events.into_iter().map(|e| (e, None)));
                all_events.extend(hud_events.into_iter().map(|e| (e, None)));
                
                // Process each event
                if !all_events.is_empty() {
                    log::info!("[Events] Detected {} events", all_events.len());
                }
                
                for (event, custom_pattern) in all_events {
                    // First check for custom pattern from trigger
                    let pattern = if let Some(p) = custom_pattern {
                        log::info!("[Pattern] Using custom pattern from UI trigger");
                        Some(p)
                    } else {
                        // Otherwise look in profile
                        let pm = profile_manager.read().await;
                        pm.get_pattern_for_event(&event).cloned()
                    };
                    
                    if let Some(pattern) = pattern {
                        log::info!("[Pattern] Executing pattern for event {:?}: Attack={:.0}ms, Hold={:.0}ms", 
                            event, pattern.attack.duration_ms, pattern.hold.duration_ms);
                        Self::execute_pattern_async(
                            Arc::clone(&device_manager),
                            Arc::clone(&rate_limiter),
                            Arc::clone(&current_intensity),
                            pattern,
                        );
                    } else {
                        log::warn!("[Pattern] No pattern found for event {:?}", event);
                    }
                }

                // Check continuous events (engine, etc.)
                let continuous = {
                    let ee = event_engine.read().await;
                    ee.check_continuous_events(&game_state)
                };
                
                for _event in continuous {
                    // For continuous events don't spam, update smoothly
                    if rate_limiter.should_send() {
                        let intensity = *current_intensity.read().await;
                        if let Err(e) = device_manager.send_vibration(intensity * 0.3).await {
                            log::warn!("Failed to send continuous vibration: {}", e);
                        }
                        rate_limiter.mark_sent();
                    }
                }
            }

            // Fail-safe: stop all vibrations on exit
            let _ = device_manager.stop_all().await;
        });

        log::info!("Haptic engine started");
        Ok(())
    }

    /// Stop main loop
    pub async fn stop(&self) -> Result<()> {
        log::warn!("🛑 EMERGENCY STOP - Halting all vibrations!");
        *self.running.write().await = false;
        
        // Immediately stop all devices
        self.device_manager.stop_all().await?;
        
        // Reset current intensity
        *self.current_intensity.write().await = 0.0;
        
        log::info!("✅ All vibrations stopped");
        Ok(())
    }

    /// Execute pattern asynchronously
    fn execute_pattern_async(
        device_manager: Arc<DeviceManager>,
        rate_limiter: Arc<RateLimiter>,
        current_intensity: Arc<RwLock<f32>>,
        pattern: VibrationPattern,
    ) {
        tokio::spawn(async move {
            let points = pattern.generate_points(20); // 20 Hz sampling
            
            for (delay, intensity) in points {
                tokio::time::sleep(delay).await;
                
                *current_intensity.write().await = intensity;
                
                if rate_limiter.try_send() {
                    if let Err(e) = device_manager.send_vibration(intensity).await {
                        log::warn!("Failed to execute pattern: {}", e);
                        break;
                    }
                }
            }
            
            // Smooth fade out at the end
            *current_intensity.write().await = 0.0;
            if rate_limiter.try_send() {
                let _ = device_manager.send_vibration(0.0).await;
            }
        });
    }

    /// Get current game status
    pub async fn get_game_status(&self) -> Result<GameStatusInfo> {
        let mut telem = self.telemetry.write().await;
        match telem.get_state().await {
            Ok(state) => {
                // Calculate fuel in percent (fuel in kg, fuel_max in kg)
                let fuel_percent = if state.indicators.fuel_max > 0.0 {
                    ((state.indicators.fuel / state.indicators.fuel_max * 100.0) as i32).min(100)
                } else {
                    0
                };
                
                Ok(GameStatusInfo {
                    connected: true,
                    vehicle_name: state.vehicle_name.clone(),
                    speed_kmh: state.indicators.speed as i32, // Already in km/h
                    altitude_m: state.indicators.altitude as i32,
                    g_load: state.indicators.g_load,
                    engine_rpm: state.indicators.engine_rpm as i32,
                    fuel_percent,
                })
            },
            Err(_) => Ok(GameStatusInfo::disconnected()),
        }
    }

    /// Check running status
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
    
    /// Get current player names (for filtering HUD events)
    pub async fn get_player_names(&self) -> Vec<String> {
        self.telemetry.read().await.get_player_names()
    }
    
    /// Set player names (for filtering HUD events)
    pub async fn set_player_names(&self, names: Vec<String>) {
        self.telemetry.write().await.set_player_names(names);
    }
    
    /// Get current clan tags (for filtering HUD events)
    pub async fn get_clan_tags(&self) -> Vec<String> {
        self.telemetry.read().await.get_clan_tags()
    }
    
    /// Set clan tags (for filtering HUD events)
    pub async fn set_clan_tags(&self, tags: Vec<String>) {
        self.telemetry.write().await.set_clan_tags(tags);
    }

    /// Get managers (for UI)
    pub fn get_profile_manager(&self) -> Arc<RwLock<ProfileManager>> {
        Arc::clone(&self.profile_manager)
    }

    pub fn get_device_manager(&self) -> Arc<DeviceManager> {
        Arc::clone(&self.device_manager)
    }

    /// Test vibration
    pub async fn test_vibration(&self, intensity: f32, duration_ms: u64) -> Result<()> {
        self.device_manager.send_vibration(intensity).await?;
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;
        self.device_manager.send_vibration(0.0).await?;
        Ok(())
    }
}

impl Default for HapticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HapticEngine {
    fn drop(&mut self) {
        let running = Arc::clone(&self.running);
        tokio::spawn(async move {
            *running.write().await = false;
        });
    }
}

