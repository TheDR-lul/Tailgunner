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
use std::collections::VecDeque;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub trigger_name: String,
    pub event_type: String,
    pub entity: String,
    pub timestamp: String,
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
    recent_trigger_events: Arc<RwLock<VecDeque<TriggerEvent>>>,
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
            recent_trigger_events: Arc::new(RwLock::new(VecDeque::with_capacity(10))),
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
        let recent_trigger_events = Arc::clone(&self.recent_trigger_events);

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
                            
                            // Reduced logging spam - only log on significant changes
                            // log::debug!("[WT] Vehicle: {:?}, Speed: {:.0} km/h, Alt: {:.0}m, Fuel: {:.0}/{:.0} kg", 
                            //     state.type_, state.indicators.speed, state.indicators.altitude,
                            //     state.indicators.fuel, state.indicators.fuel_max);
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
                        
                        // Update built-in dynamic triggers with vehicle-specific limits
                        let limit_triggers = vehicle_limits_manager.generate_limit_triggers().await;
                        if !limit_triggers.is_empty() {
                            log::info!("[Dynamic Triggers] 🔧 Updating {} built-in triggers for {}", 
                                limit_triggers.len(), game_state.vehicle_name);
                            
                            let mut tm = trigger_manager.write().await;
                            
                            // Update existing built-in triggers with vehicle-specific values
                            for trigger in &limit_triggers {
                                log::info!("[Dynamic Triggers] 🔄 Updating built-in trigger: {} → {}", 
                                    trigger.id, trigger.name);
                                
                                if let Err(e) = tm.update_trigger_condition(
                                    &trigger.id,
                                    trigger.name.clone(),
                                    trigger.description.clone(),
                                    trigger.condition.clone()
                                ) {
                                    log::warn!("[Dynamic Triggers] ⚠️ Failed to update {}: {}", trigger.id, e);
                                }
                            }
                        } else {
                            log::warn!("[Dynamic Triggers] ⚠️ No vehicle limits found for {}, using defaults", 
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
                
                // Check indicator-based triggers (overspeed, over-G, etc.)
                // Event-based triggers are automatically skipped in check_triggers() and handled separately below
                let trigger_events = {
                    let mut tm = trigger_manager.write().await;
                    let events = tm.check_triggers(&game_state);
                    
                    if !events.is_empty() {
                        log::info!("[Triggers] 🎯 {} indicator triggers fired", events.len());
                        for (event, pattern) in &events {
                            log::info!("[Triggers]   - {:?} (pattern: {})", 
                                event, pattern.is_some());
                        }
                    }
                    
                    events
                };
                
                // Process HUD events and filter by event-based triggers
                let telemetry_lock = telemetry.read().await;
                let player_names = telemetry_lock.get_player_names().to_vec();
                let clan_tags = telemetry_lock.get_clan_tags().to_vec();
                let enemy_names = telemetry_lock.get_enemy_names().to_vec();
                let enemy_clans = telemetry_lock.get_enemy_clans().to_vec();
                drop(telemetry_lock);
                
                let trigger_manager_lock = trigger_manager.read().await;
                let all_triggers = trigger_manager_lock.get_triggers().to_vec();
                drop(trigger_manager_lock);
                
                let mut hud_events_with_patterns: Vec<(GameEvent, Option<VibrationPattern>)> = Vec::new();
                
                for hud_evt in &game_state.hud_events {
                    use crate::wt_telemetry::HudEvent;
                    
                    // Extract entity name and game event
                    let (entity_name, game_event) = match hud_evt {
                        HudEvent::Kill(enemy) => {
                            log::info!("[HUD] 🎯 Kill: {}", enemy);
                            (enemy.as_str(), GameEvent::TargetDestroyed)
                        }
                        HudEvent::Crashed => {
                            log::warn!("[HUD] 💥 Crashed!");
                            ("", GameEvent::Crashed)
                        }
                        HudEvent::EngineOverheated => {
                            log::warn!("[HUD] 🔥 Engine overheated!");
                            ("", GameEvent::EngineOverheat)
                        }
                        HudEvent::OilOverheated => {
                            log::warn!("[HUD] 🛢️ Oil overheated!");
                            ("", GameEvent::OilOverheated)
                        }
                        HudEvent::SetAfire(victim) => {
                            log::info!("[HUD] 🔥 Set enemy on fire: {}", victim);
                            (victim.as_str(), GameEvent::EnemySetAfire)
                        }
                        HudEvent::TakeDamage(attacker) => {
                            log::warn!("[HUD] 💥 Taking damage from: {}", attacker);
                            (attacker.as_str(), GameEvent::TakingDamage)
                        }
                        HudEvent::SeverelyDamaged(attacker) => {
                            log::warn!("[HUD] 💔 Severely damaged by: {}", attacker);
                            (attacker.as_str(), GameEvent::SeverelyDamaged)
                        }
                        HudEvent::ShotDown(attacker) => {
                            log::warn!("[HUD] ✈️💥 Shot down by: {}", attacker);
                            (attacker.as_str(), GameEvent::ShotDown)
                        }
                        HudEvent::Achievement(name) => {
                            log::info!("[HUD] 🏆 Achievement: {}", name);
                            (name.as_str(), GameEvent::Achievement)
                        }
                        HudEvent::ChatMessage(text) => {
                            log::debug!("[HUD] 💬 Chat message: {}", text);
                            (text.as_str(), GameEvent::ChatMessage)
                        }
                    };
                    
                    // Find matching event-based triggers
                    for trigger in &all_triggers {
                        if !trigger.enabled || !trigger.is_event_based {
                            continue;
                        }
                        
                        if trigger.event != game_event {
                            continue;
                        }
                        
                        // Apply filter
                        if !crate::event_triggers::TriggerManager::should_fire_event_trigger(
                            trigger,
                            entity_name,
                            &player_names,
                            &clan_tags,
                            &enemy_names,
                            &enemy_clans,
                        ) {
                            log::debug!("[HUD] ❌ Trigger '{}' filtered out for entity '{}'", trigger.name, entity_name);
                            continue;
                        }
                        
                        log::info!("[HUD] ✅ Trigger '{}' matched for entity '{}'", trigger.name, entity_name);
                        
                        // Record trigger event for debug console
                        {
                            let mut events = recent_trigger_events.write().await;
                            if events.len() >= 10 {
                                events.pop_front();
                            }
                            events.push_back(TriggerEvent {
                                trigger_name: trigger.name.clone(),
                                event_type: format!("{:?}", game_event),
                                entity: entity_name.to_string(),
                                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                            });
                        }
                        
                        hud_events_with_patterns.push((game_event.clone(), trigger.pattern.clone()));
                    }
                }
                
                // Combine events: basic, triggers (including dynamic), and HUD events
                let mut all_events: Vec<(GameEvent, Option<VibrationPattern>)> = trigger_events;
                all_events.extend(basic_events.into_iter().map(|e| (e, None)));
                all_events.extend(hud_events_with_patterns);
                
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
                        log::info!("[Pattern] Executing pattern for event {:?}: Attack={:.0}ms, Hold={:.0}ms, Decay={:.0}ms, Repeat={}, Pause={}ms", 
                            event, pattern.attack.duration_ms, pattern.hold.duration_ms, pattern.decay.duration_ms,
                            pattern.burst.repeat_count, pattern.burst.pause_between_ms);
                        Self::execute_pattern_async(
                            Arc::clone(&device_manager),
                            Arc::clone(&rate_limiter),
                            Arc::clone(&current_intensity),
                            Arc::clone(&running),
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
        
        // Immediately stop all devices (send 0.0 multiple times)
        for _ in 0..5 {
            let _ = self.device_manager.stop_all().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        
        // Reset current intensity
        *self.current_intensity.write().await = 0.0;
        
        // Clear trigger cooldowns to prevent stuck state
        {
            let mut tm = self.trigger_manager.write().await;
            tm.clear_cooldowns();
        }
        
        log::info!("✅ All vibrations stopped, cooldowns cleared");
        Ok(())
    }

    /// Execute pattern asynchronously
    fn execute_pattern_async(
        device_manager: Arc<DeviceManager>,
        rate_limiter: Arc<RateLimiter>,
        current_intensity: Arc<RwLock<f32>>,
        running: Arc<RwLock<bool>>,
        pattern: VibrationPattern,
    ) {
        tokio::spawn(async move {
            let points = pattern.generate_points(20); // 20 Hz sampling
            
            let mut last_time = Duration::from_millis(0);
            
            for (absolute_time, intensity) in points {
                // Check if engine is still running
                if !*running.read().await {
                    log::info!("[Pattern] ⏹️ Pattern interrupted by Stop");
                    break;
                }
                
                // Sleep for the DIFFERENCE between current and last time point
                let sleep_duration = absolute_time.saturating_sub(last_time);
                tokio::time::sleep(sleep_duration).await;
                last_time = absolute_time;
                
                // Check again after sleep
                if !*running.read().await {
                    log::info!("[Pattern] ⏹️ Pattern interrupted by Stop (after sleep)");
                    break;
                }
                
                *current_intensity.write().await = intensity;
                
                if rate_limiter.try_send() {
                    if let Err(e) = device_manager.send_vibration(intensity).await {
                        log::warn!("Failed to execute pattern: {}", e);
                        break;
                    }
                }
            }
            
            // Smooth fade out at the end (or force 0.0 if stopped)
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
    
    /// Get recent trigger events for debug console
    pub async fn get_recent_trigger_events(&self) -> Vec<TriggerEvent> {
        let events = self.recent_trigger_events.read().await;
        events.iter().cloned().collect()
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
    
    /// Get current enemy names (for tracking kills)
    pub async fn get_enemy_names(&self) -> Vec<String> {
        self.telemetry.read().await.get_enemy_names()
    }
    
    /// Set enemy names (for tracking kills)
    pub async fn set_enemy_names(&self, names: Vec<String>) {
        self.telemetry.write().await.set_enemy_names(names);
    }
    
    /// Get current enemy clans (for tracking kills)
    pub async fn get_enemy_clans(&self) -> Vec<String> {
        self.telemetry.read().await.get_enemy_clans()
    }
    
    /// Set enemy clans (for tracking kills)
    pub async fn set_enemy_clans(&self, clans: Vec<String>) {
        self.telemetry.write().await.set_enemy_clans(clans);
    }

    /// Get managers (for UI)
    pub fn get_profile_manager(&self) -> Arc<RwLock<ProfileManager>> {
        Arc::clone(&self.profile_manager)
    }
    
    /// Get telemetry reader (for loading player identity from DB)
    pub fn get_telemetry(&self) -> Arc<RwLock<crate::wt_telemetry::WTTelemetryReader>> {
        Arc::clone(&self.telemetry)
    }
    
    /// Get vehicle limits manager (for % of max value calculations in patterns)
    #[allow(dead_code)]
    pub fn get_vehicle_limits_manager(&self) -> Arc<VehicleLimitsManager> {
        Arc::clone(&self.vehicle_limits_manager)
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

