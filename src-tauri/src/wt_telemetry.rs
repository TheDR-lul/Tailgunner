/// War Thunder Telemetry Reader
/// Reads data from localhost:8111 (official WT API)
/// EAC-Safe: only HTTP requests, no memory injection

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const WT_TELEMETRY_URL: &str = "http://127.0.0.1:8111";
const POLL_INTERVAL_MS: u64 = 100; // 10 times per second

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudMessage {
    pub id: u32,
    pub msg: String,
    pub time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HudEvent {
    Kill(String),              // Killed enemy vehicle name
    Crashed,
    EngineOverheated,
    OilOverheated,
    SetAfire(String),          // Set enemy on fire (enemy name)
    TakeDamage(String),        // Taking damage from attacker (attacker name)
    SeverelyDamaged(String),   // Severely damaged by attacker
    ShotDown(String),          // Shot down by attacker
    Achievement(String),       // Achievement unlocked (achievement name)
    ChatMessage(String),       // Any chat message (full text)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub valid: bool,
    pub type_: VehicleType,
    pub vehicle_name: String,
    pub indicators: Indicators,
    pub state: Vec<String>,
    
    // HUD events detected this frame
    #[serde(skip)]
    pub hud_events: Vec<HudEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VehicleType {
    Tank,
    Aircraft,
    Helicopter,
    Ship,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Indicators {
    // Basic
    pub speed: f32,           // IAS km/h
    pub altitude: f32,        // m
    pub climb: f32,           // m/s
    
    // Engine
    pub engine_rpm: f32,
    pub engine_temp: f32,
    pub oil_temp: f32,
    pub water_temp: f32,
    pub manifold_pressure: f32,
    pub throttle: f32,
    pub mixture: f32,
    pub radiator: f32,
    pub compressor_stage: i32,
    pub magneto: i32,
    
    // Controls
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub aileron: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
    pub gear: f32,
    pub airbrake: f32,
    
    // Aerodynamics
    pub aoa: f32,             // Angle of Attack
    pub slip: f32,            // Sideslip
    pub g_load: f32,          // G-load
    pub mach: f32,            // Mach number
    pub tas: f32,             // TAS km/h
    pub ias: f32,             // IAS km/h
    
    // Weapons
    pub cannon_ready: bool,
    pub machine_gun_ready: bool,
    pub rockets_ready: i32,
    pub bombs_ready: i32,
    pub torpedoes_ready: i32,
    
    // Ammo
    pub ammo_count: i32,
    pub rocket_count: i32,
    pub bomb_count: i32,
    
    // Fuel
    pub fuel: f32,            // kg
    pub fuel_max: f32,
    pub fuel_time: f32,       // minutes
    
    // Damage
    pub engine_damage: f32,   // 0.0-1.0
    pub controls_damage: f32,
    pub gear_damage: f32,
    pub flaps_damage: f32,
    
    // Tank-specific
    pub stabilizer: f32,      // 0.0 or 1.0
    pub crew_total: i32,
    pub crew_current: i32,
    pub gunner_state: i32,    // 0=alive, 1=wounded, 2=dead
    pub driver_state: i32,
    pub cruise_control: f32,  // Speed setpoint
    pub driving_direction: i32, // 0=forward, 1=backward
}

pub struct WTTelemetryReader {
    client: reqwest::Client,
    last_state: Option<GameState>,
    last_fetch_time: Option<std::time::Instant>,
    cache_duration_ms: u64,
    last_hud_evt_id: u32,
    last_hud_dmg_id: u32,
    player_names: Vec<String>,  // Player names for filtering own events
    clan_tags: Vec<String>,      // Player clan tags for filtering
    enemy_names: Vec<String>,    // Enemy player names to track
    enemy_clans: Vec<String>,    // Enemy clan tags to track
    hud_initialized: bool,  // Flag to skip old events on first connect
    last_battle_time: u32,  // Last battle time (seconds) to detect old events
    processed_messages: std::collections::HashSet<String>, // Cache to prevent duplicates
    last_message: Option<String>, // Last processed message for immediate duplicate check
}

impl WTTelemetryReader {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(200))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            last_state: None,
            last_fetch_time: None,
            cache_duration_ms: 50, // Cache for 50ms (20 Hz max poll rate)
            last_hud_evt_id: 0,
            last_hud_dmg_id: 0,
            player_names: Vec::new(),
            clan_tags: Vec::new(),
            enemy_names: Vec::new(),
            enemy_clans: Vec::new(),
            hud_initialized: false,
            last_battle_time: 0,
            processed_messages: std::collections::HashSet::new(),
            last_message: None,
        }
    }
    
    /// Get current player names
    pub fn get_player_names(&self) -> Vec<String> {
        self.player_names.clone()
    }
    
    /// Set player names
    pub fn set_player_names(&mut self, names: Vec<String>) {
        self.player_names = names.clone();
        if !names.is_empty() {
            log::info!("[HUD] 🎮 Player names set: {:?}", names);
        } else {
            log::info!("[HUD] 🎮 Player names cleared");
        }
    }
    
    /// Get current clan tags
    pub fn get_clan_tags(&self) -> Vec<String> {
        self.clan_tags.clone()
    }
    
    /// Set clan tags
    pub fn set_clan_tags(&mut self, tags: Vec<String>) {
        self.clan_tags = tags.clone();
        if !tags.is_empty() {
            log::info!("[HUD] 🏷️ Clan tags set: {:?}", tags);
        } else {
            log::info!("[HUD] 🏷️ Clan tags cleared");
        }
    }
    
    /// Get current enemy names
    pub fn get_enemy_names(&self) -> Vec<String> {
        self.enemy_names.clone()
    }
    
    /// Set enemy names (for tracking specific enemies)
    pub fn set_enemy_names(&mut self, names: Vec<String>) {
        self.enemy_names = names.clone();
        if !names.is_empty() {
            log::info!("[HUD] 🎯 Enemy names set: {:?}", names);
        } else {
            log::info!("[HUD] 🎯 Enemy names cleared");
        }
    }
    
    /// Get current enemy clans
    pub fn get_enemy_clans(&self) -> Vec<String> {
        self.enemy_clans.clone()
    }
    
    /// Set enemy clans (for tracking specific enemy clans)
    pub fn set_enemy_clans(&mut self, clans: Vec<String>) {
        self.enemy_clans = clans.clone();
        if !clans.is_empty() {
            log::info!("[HUD] 🎯 Enemy clans set: {:?}", clans);
        } else {
            log::info!("[HUD] 🎯 Enemy clans cleared");
        }
    }
    
    /// Check if message contains enemy identity
    fn is_enemy_message(&self, msg: &str) -> bool {
        // Check if any of our enemy names match
        for name in &self.enemy_names {
            if msg.contains(name) {
                return true;
            }
        }
        
        // Check if any of our enemy clans match
        for tag in &self.enemy_clans {
            if msg.contains(tag) {
                return true;
            }
        }
        
        false
    }

    /// Check War Thunder availability
    #[allow(dead_code)]
    pub async fn is_game_running(&self) -> bool {
        let url = format!("{}/state", WT_TELEMETRY_URL);
        self.client
            .get(&url)
            .send()
            .await
            .is_ok()
    }

    /// Get current game state
    /// WT API is split into 2 endpoints:
    /// - /indicators → type, army, valid (vehicle information)
    /// - /state → flight indicators (IAS, AoA, G-load, etc.)
    pub async fn get_state(&mut self) -> Result<GameState> {
        // Check cache first
        if let Some(last_time) = self.last_fetch_time {
            let elapsed = last_time.elapsed().as_millis() as u64;
            if elapsed < self.cache_duration_ms {
                if let Some(ref cached_state) = self.last_state {
                    return Ok(cached_state.clone());
                }
            }
        }
        // 1. Request /indicators for vehicle type
        let indicators_url = format!("{}/indicators", WT_TELEMETRY_URL);
        let indicators_response = self.client
            .get(&indicators_url)
            .send()
            .await
            .context("Failed to connect to War Thunder /indicators")?;

        let indicators_json: serde_json::Value = indicators_response
            .json()
            .await
            .context("Failed to parse WT /indicators")?;

        // 2. Request /state for flight indicators
        let state_url = format!("{}/state", WT_TELEMETRY_URL);
        let state_response = self.client
            .get(&state_url)
            .send()
            .await
            .context("Failed to connect to War Thunder /state")?;

        let state_json: serde_json::Value = state_response
            .json()
            .await
            .context("Failed to parse WT /state")?;

        // 3. Combine data from both endpoints
        let mut state = self.parse_combined_state(indicators_json, state_json)?;
        
        // 4. Get HUD events (non-blocking, errors are logged but don't fail the request)
        match self.get_hud_events().await {
            Ok(hud_events) => {
                state.hud_events = hud_events;
            }
            Err(e) => {
                log::debug!("[WT API] Failed to get HUD events: {}", e);
                state.hud_events = Vec::new();
            }
        }
        
        self.last_state = Some(state.clone());
        self.last_fetch_time = Some(std::time::Instant::now());
        
        Ok(state)
    }

    /// Get indicators
    #[allow(dead_code)]
    pub async fn get_indicators(&self) -> Result<Indicators> {
        let url = format!("{}/indicators", WT_TELEMETRY_URL);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get indicators")?;

        let indicators: Indicators = response
            .json()
            .await
            .context("Failed to parse indicators")?;

        Ok(indicators)
    }
    
    /// Get HUD messages (events and damage) and parse for player events
    pub async fn get_hud_events(&mut self) -> Result<Vec<HudEvent>> {
        let url = format!("{}/hudmsg?lastEvt={}&lastDmg={}", 
            WT_TELEMETRY_URL, self.last_hud_evt_id, self.last_hud_dmg_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get HUD messages")?;
        
        let json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse HUD messages")?;
        
        let mut events = Vec::new();
        
        // Parse event messages
        // Note: API returns "events" (plural), not "event"
        if let Some(event_array) = json.get("events").and_then(|v| v.as_array()) {
            for msg_value in event_array {
                let id = msg_value.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                
                // Update last seen ID
                if id > self.last_hud_evt_id {
                    self.last_hud_evt_id = id;
                }
            }
        }
        
        // Parse damage messages (contain kill feed, crashes, overheats)
        if let Some(damage_array) = json.get("damage").and_then(|v| v.as_array()) {
            if damage_array.len() > 0 {
                log::info!("[HUD] 📥 Received {} damage messages", damage_array.len());
            }
            
            // INIT PHASE: Find max time and ID to establish baseline
            if !self.hud_initialized && !damage_array.is_empty() {
                let max_time = damage_array.iter()
                    .filter_map(|v| v.get("time").and_then(|t| t.as_u64()))
                    .max()
                    .unwrap_or(0) as u32;
                
                let max_id = damage_array.iter()
                    .filter_map(|v| v.get("id").and_then(|i| i.as_u64()))
                    .max()
                    .unwrap_or(0) as u32;
                
                // Process recent events (within last 15 seconds)
                // This allows catching events that happened just before app started
                let recent_threshold = max_time.saturating_sub(15);
                
                log::info!("[HUD] 🔧 Initializing: max_time={}s, max_id={}, processing events after {}s", 
                    max_time, max_id, recent_threshold);
                
                // First pass: process recent events
                for msg_value in damage_array {
                    let _id = msg_value.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let msg = msg_value.get("msg").and_then(|v| v.as_str()).unwrap_or("");
                    let time = msg_value.get("time").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    
                    // Only process recent events during initialization
                    if time >= recent_threshold {
                        let message_key = format!("{}:{}", time, msg);
                        if !self.processed_messages.contains(&message_key) {
                            if let Some(event) = self.parse_hud_message(msg) {
                                log::info!("[HUD] ✅ INIT event at {}s: {:?}", time, event);
                                events.push(event);
                                self.processed_messages.insert(message_key);
                                self.last_message = Some(msg.to_string());
                            }
                        }
                    }
                }
                
                // Set baseline to max ID after processing recent events
                self.last_hud_dmg_id = max_id;
                self.hud_initialized = true;
                log::info!("[HUD] ✅ Initialized with baseline ID={}", max_id);
                
                // Return early - we've processed everything
                return Ok(events);
            }
            
            // NORMAL PHASE: Only process NEW events (ID > baseline)
            for msg_value in damage_array {
                let id = msg_value.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let msg = msg_value.get("msg").and_then(|v| v.as_str()).unwrap_or("");
                let time = msg_value.get("time").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                
                if id <= self.last_hud_dmg_id {
                    continue; // Skip old events
                }
                
                // Update baseline for next check
                self.last_hud_dmg_id = id;
                
                // Filter by battle time: skip old events from previous battles
                if time < self.last_battle_time {
                    log::debug!("[HUD] ⏭️ Skipping old event (time {}s < last {}s)", time, self.last_battle_time);
                    continue;
                }
                
                // Update last battle time
                if time > self.last_battle_time {
                    self.last_battle_time = time;
                    // Clean old messages from cache (keep only last 10 seconds)
                    self.processed_messages.retain(|m| {
                        // Parse time from cached message key (format: "time:msg")
                        if let Some(cached_time) = m.split(':').next().and_then(|t| t.parse::<u32>().ok()) {
                            // Use saturating_sub to prevent overflow when starting new battle
                            time.saturating_sub(cached_time) < 10
                        } else {
                            false
                        }
                    });
                }
                
                // IMMEDIATE duplicate check: same message as last one
                if let Some(ref last_msg) = self.last_message {
                    if last_msg == msg {
                        // log::debug!("[HUD DEBUG] ⏭️ BLOCKED IMMEDIATE duplicate: '{}'", msg);
                        continue;
                    }
                }
                
                // Check for duplicates (same message within 10 seconds)
                let message_key = format!("{}:{}", time, msg);
                if self.processed_messages.contains(&message_key) {
                    // log::debug!("[HUD DEBUG] ⏭️ BLOCKED CACHED duplicate: '{}' (key: {})", msg, message_key);
                    continue;
                }
                
                // Mark as processed
                // log::debug!("[HUD DEBUG] ✅ PASSED filter, marking as processed: '{}'", msg);
                self.processed_messages.insert(message_key.clone());
                self.last_message = Some(msg.to_string());
                // log::debug!("[HUD DEBUG] 📝 Cached messages count: {}, cache key: {}", self.processed_messages.len(), message_key);
                
                // Try to parse as specific player event
                if let Some(event) = self.parse_hud_message(msg) {
                    log::info!("[HUD] ✅ Parsed event at {}s: {:?} from msg '{}'", time, event, msg);
                    events.push(event);
                } else if !msg.is_empty() {
                    // If not a specific event, treat as generic chat message
                    // Skip system messages (disconnects, etc)
                    if !msg.contains("has disconnected") && 
                       !msg.contains("NET_PLAYER_") &&
                       !msg.contains("td! kd?") {
                        log::debug!("[HUD Event] 💬 CHAT at {}s: {}", time, msg);
                        events.push(HudEvent::ChatMessage(msg.to_string()));
                    }
                }
            }
        }
        
        // Mark HUD as initialized after first call
        if !self.hud_initialized {
            self.hud_initialized = true;
            // log::debug!("[HUD DEBUG] ✅ INITIALIZED - baseline IDs set: evt={}, dmg={}", self.last_hud_evt_id, self.last_hud_dmg_id);
            log::info!("[HUD] ✅ Initialized - will only process NEW events (ID > {})", self.last_hud_dmg_id);
        }
        
        Ok(events)
    }
    
    /// Check if message contains player identity
    fn is_player_message(&self, msg: &str) -> bool {
        // Check if any of our player names match
        for name in &self.player_names {
            if msg.contains(name) {
                return true;
            }
        }
        
        // Check if any of our clan tags match
        for tag in &self.clan_tags {
            if msg.contains(tag) {
                return true;
            }
        }
        
        false
    }
    
    /// Parse HUD message to detect player events
    fn parse_hud_message(&self, msg: &str) -> Option<HudEvent> {
        // Parse event type - these are always player events
        if msg.contains("Engine overheated") {
            return Some(HudEvent::EngineOverheated);
        }
        
        if msg.contains("Oil overheated") {
            return Some(HudEvent::OilOverheated);
        }
        
        if msg.contains("has crashed") {
            // Only detect player crashes
            if self.is_player_message(msg) {
                return Some(HudEvent::Crashed);
            }
            return None;
        }
        
        // Achievements: "has achieved" or "has delivered"
        if msg.contains("has achieved") || msg.contains("has delivered") {
            if self.is_player_message(msg) {
                // Extract achievement name
                let achievement = if msg.contains("has achieved") {
                    msg.split("has achieved").nth(1).unwrap_or("").trim()
                } else {
                    msg.split("has delivered").nth(1).unwrap_or("").trim()
                };
                
                if !achievement.is_empty() {
                    log::info!("[HUD Event] 🏆 ACHIEVEMENT: {}", achievement);
                    return Some(HudEvent::Achievement(achievement.to_string()));
                }
            }
            return None;
        }
        
        // Set enemy on fire
        if msg.contains("set afire") {
            if !self.player_names.is_empty() || !self.clan_tags.is_empty() {
                let parts: Vec<&str> = msg.split("set afire").collect();
                if parts.len() == 2 {
                    let attacker = parts[0].trim();
                    let victim = parts[1].trim();
                    
                    // Player set enemy on fire
                    if self.is_player_message(attacker) {
                        log::info!("[HUD Event] 🔥 SET AFIRE: {}", victim);
                        return Some(HudEvent::SetAfire(victim.to_string()));
                    }
                    
                    // Player was set on fire
                    if self.is_player_message(victim) {
                        log::info!("[HUD Event] 💥 TAKING FIRE from: {}", attacker);
                        return Some(HudEvent::TakeDamage(attacker.to_string()));
                    }
                }
            }
            return None;
        }
        
        // Severely damaged
        if msg.contains("severely damaged") {
            if !self.player_names.is_empty() || !self.clan_tags.is_empty() {
                let parts: Vec<&str> = msg.split("severely damaged").collect();
                if parts.len() == 2 {
                    let attacker = parts[0].trim();
                    let victim = parts[1].trim();
                    
                    // Player was severely damaged
                    if self.is_player_message(victim) {
                        log::info!("[HUD Event] 💔 SEVERELY DAMAGED by: {}", attacker);
                        return Some(HudEvent::SeverelyDamaged(attacker.to_string()));
                    }
                }
            }
            return None;
        }
        
        // Shot down
        if msg.contains("shot down") {
            if !self.player_names.is_empty() || !self.clan_tags.is_empty() {
                let parts: Vec<&str> = msg.split("shot down").collect();
                if parts.len() == 2 {
                    let attacker = parts[0].trim();
                    let victim = parts[1].trim();
                    
                    // Player was shot down
                    if self.is_player_message(victim) {
                        log::info!("[HUD Event] ✈️💥 SHOT DOWN by: {}", attacker);
                        return Some(HudEvent::ShotDown(attacker.to_string()));
                    }
                }
            }
            return None;
        }
        
    // Destroyed (kill)
    if msg.contains("destroyed") {
        // Check if player identity is set
        if !self.player_names.is_empty() || !self.clan_tags.is_empty() {
            // ONLY count kills if our player identity is the attacker
            if let Some(before_destroyed) = msg.split("destroyed").next() {
                if self.is_player_message(before_destroyed) {
                    // Extract victim name (after "destroyed")
                    if let Some(destroyed_part) = msg.split("destroyed").nth(1) {
                        let victim = destroyed_part.trim().to_string();
                        
                        // Check if victim is in our enemy tracking list
                        let is_priority = self.is_enemy_message(&victim);
                        
                        if is_priority {
                            log::info!("[HUD Event] 🎯💀 PRIORITY KILL (tracked enemy player): {}", victim);
                        } else {
                            log::info!("[HUD Event] 🎯 KILL (bot/random): {}", victim);
                        }
                        
                        // Return kill event with victim name
                        // Frontend can filter by enemy list if needed
                        return Some(HudEvent::Kill(victim));
                    }
                }
            }
            // If player identity doesn't match, this is someone else's kill - ignore
            return None;
        } else {
            // Without player identity, we can't reliably filter
            // Skip to avoid false positives from other players' kills
            return None;
        }
    }
    
    // "has been wrecked" (alternative kill message)
    if msg.contains("has been wrecked") {
        if !self.player_names.is_empty() || !self.clan_tags.is_empty() {
            if let Some(before_wrecked) = msg.split("has been wrecked").next() {
                if self.is_player_message(before_wrecked) {
                    if let Some(victim_part) = msg.split("has been wrecked").nth(1) {
                        let victim = victim_part.trim().to_string();
                        
                        let is_priority = self.is_enemy_message(&victim);
                        if is_priority {
                            log::info!("[HUD Event] 🎯💀 PRIORITY KILL (wrecked, tracked enemy): {}", victim);
                        } else {
                            log::info!("[HUD Event] 🎯 KILL (wrecked): {}", victim);
                        }
                        
                        return Some(HudEvent::Kill(victim));
                    }
                }
            }
            return None;
        } else {
            return None;
        }
    }
        
        None
    }

    /// Parse combined data from /indicators and /state
    fn parse_combined_state(&self, indicators_json: serde_json::Value, state_json: serde_json::Value) -> Result<GameState> {
        // 1. Parse type and army from /indicators
        let valid = indicators_json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
        let type_str = indicators_json.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        let army_str = indicators_json.get("army").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        // Reduced logging spam - only log on vehicle changes
        // log::info!("[WT Parser] From /indicators: type='{}', army='{}', valid={}", type_str, army_str, valid);
        
        // Extract vehicle name from type string
        // Examples:
        // - "tankModels/sw_cv_90105_tml" -> "sw_cv_90105_tml"
        // - "rafale_c_f3" -> "rafale_c_f3"
        let vehicle_name = if type_str.contains('/') {
            let name = type_str.split('/').last().unwrap_or(type_str).to_string();
            name
        } else {
            type_str.to_string()
        };
        
        // Detect vehicle type from army and type string
        let type_ = match army_str {
            "air" => VehicleType::Aircraft,
            "ground" | "tank" => VehicleType::Tank,  // API uses both "ground" and "tank"
            "ship" => VehicleType::Ship,
            "helicopter" => VehicleType::Helicopter,
            _ => {
                // Fallback: try to detect from type_str
                if type_str.contains("tankModels") || type_str.contains("tank_") {
                    VehicleType::Tank
                } else if type_str.contains("ship") || type_str.contains("boat") {
                    VehicleType::Ship
                } else if type_str.contains("helicopter") || type_str.contains("heli_") {
                    VehicleType::Helicopter
                } else if !type_str.is_empty() && type_str != "unknown" {
                    VehicleType::Aircraft
                } else {
                    VehicleType::Unknown
                }
            }
        };

        let state: Vec<String> = state_json.get("state")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // 2. Parse indicators from correct source
        // TANKS: all data in /indicators
        // AIRCRAFT: flight data in /state
        let mut indicators = if matches!(type_, VehicleType::Tank) {
            self.parse_indicators(indicators_json.clone())
        } else {
            self.parse_indicators(state_json.clone())
        };

        // 3. Parse damage from state array
        // State contains strings like: ["damaged", "engine_damaged", "controls_damaged", "gear_damaged", "flaps_damaged"]
        indicators.engine_damage = if state.iter().any(|s| s.contains("engine")) { 1.0 } else { 0.0 };
        indicators.controls_damage = if state.iter().any(|s| s.contains("controls") || s.contains("aileron") || s.contains("elevator") || s.contains("rudder")) { 1.0 } else { 0.0 };
        indicators.gear_damage = if state.iter().any(|s| s.contains("gear")) { 1.0 } else { 0.0 };
        indicators.flaps_damage = if state.iter().any(|s| s.contains("flaps")) { 1.0 } else { 0.0 };

        // 4. Calculate fuel time remaining (minutes)
        // Use cached fuel consumption rate from previous state
        if let Some(last_state) = &self.last_state {
            let fuel_diff = last_state.indicators.fuel - indicators.fuel;
            if fuel_diff > 0.0 && indicators.fuel > 0.0 {
                // fuel_diff is per tick (0.1 sec), multiply by 10 to get per second
                let consumption_per_sec = fuel_diff * 10.0;
                if consumption_per_sec > 0.01 {
                    indicators.fuel_time = (indicators.fuel / consumption_per_sec) / 60.0; // minutes
                }
            }
        }

        // Reduced log spam - only log on vehicle change
        use std::sync::Mutex;
        use std::sync::OnceLock;
        static LAST_VEHICLE: OnceLock<Mutex<String>> = OnceLock::new();
        
        let last_vehicle = LAST_VEHICLE.get_or_init(|| Mutex::new(String::new()));
        let mut last = last_vehicle.lock()
            .expect("LAST_VEHICLE mutex poisoned");
        
        if *last != vehicle_name {
            log::error!("[WT Parser] ✅ Vehicle detected: '{}' ({:?}, army={})", 
                vehicle_name, type_, army_str);
            *last = vehicle_name.clone();
        }

        Ok(GameState {
            valid,
            type_,
            vehicle_name,
            indicators,
            state,
            hud_events: Vec::new(),
        })
    }

    /// Parse raw WT data to struct (LEGACY - used for tests)
    /// /state contains ALL data in one JSON
    #[allow(dead_code)]
    fn parse_state(&self, json: serde_json::Value) -> Result<GameState> {
        let valid = json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
        
        let type_str = json.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        // Use error! for visibility (temporary debug)
        log::error!("[WT Parser DEBUG] 📋 Extracted type_str: '{}', valid: {}", type_str, valid);
        
        // Extract vehicle name from type string
        // Examples:
        // - "tankModels/sw_cv_90105_tml" -> "sw_cv_90105_tml"
        // - "rafale_c_f3" -> "rafale_c_f3"
        let vehicle_name = if type_str.contains('/') {
            let name = type_str.split('/').last().unwrap_or(type_str).to_string();
            log::error!("[WT Parser DEBUG] ✂️ Extracted from path: '{}'", name);
            name
        } else {
            log::error!("[WT Parser DEBUG] ➡️ Using as-is: '{}'", type_str);
            type_str.to_string()
        };
        
        // Detect vehicle type from string
        let type_ = if type_str.contains("tankModels") || type_str.contains("tank_") {
            VehicleType::Tank
        } else if type_str.contains("ship") || type_str.contains("boat") {
            VehicleType::Ship
        } else if type_str.contains("helicopter") || type_str.contains("heli_") {
            VehicleType::Helicopter
        } else if !type_str.is_empty() && type_str != "unknown" {
            // If it's not a tank/ship/heli and has a name, it's likely an aircraft
            VehicleType::Aircraft
        } else {
            VehicleType::Unknown
        };

        let state = json.get("state")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // Parse indicators from SAME JSON (all data in /state)
        let indicators = self.parse_indicators(json.clone());

        // Final debug log - use error! for visibility
        log::error!("[WT Parser DEBUG] ✅ FINAL => Vehicle: '{}', Type: {:?}, Speed: {:.0} km/h", 
            vehicle_name, type_, indicators.speed);

        Ok(GameState {
            valid,
            type_,
            vehicle_name,
            indicators,
            state,
            hud_events: Vec::new(),
        })
    }

    /// Parse indicators from JSON
    /// WT API field names contain commas, spaces, and units!
    /// Supports BOTH aircraft and ground vehicle formats
    fn parse_indicators(&self, json: serde_json::Value) -> Indicators {
        let get_f32 = |key: &str| json.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let get_i32 = |key: &str| json.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let get_bool = |key: &str| json.get(key).and_then(|v| v.as_bool()).unwrap_or(false);

        // AIRCRAFT vs TANK detection: 
        // - Tanks have "speed" field (no commas, no units)
        // - Aircraft have "IAS, km/h" field (with commas and units)
        // - Tanks have "army": "tank" or "army": "ground"
        let is_tank = json.get("speed").is_some() || 
                      json.get("army").and_then(|v| v.as_str())
                          .map(|s| s == "tank" || s == "ground")
                          .unwrap_or(false);
        
        let (speed, ias, tas) = if is_tank {
            // TANK: direct "speed" field
            let spd = get_f32("speed");
            (spd, spd, spd)
        } else {
            // AIRCRAFT: "IAS, km/h" and "TAS, km/h"
            let ias_val = get_f32("IAS, km/h");
            let tas_val = get_f32("TAS, km/h");
            (ias_val.max(tas_val), ias_val, tas_val)
        };
        
        let altitude = get_f32("H, m");
        let fuel = get_f32("Mfuel, kg");
        let fuel_max = get_f32("Mfuel0, kg");
        
        // ENGINE RPM: tanks use "rpm", aircraft use "RPM 1"/"RPM 2"
        let engine_rpm = if is_tank {
            get_f32("rpm")
        } else {
            get_f32("RPM 1").max(get_f32("RPM 2"))
        };
        
        // GEAR: tanks use "gear" (gear number), aircraft use "gear, %" (percentage)
        let gear_value = if is_tank {
            get_f32("gear") // Gear number (1-7)
        } else {
            get_f32("gear, %") // Percentage
        };
        
        // Only log if in battle (crew > 0 or speed > 0)
        let in_battle = (is_tank && get_i32("crew_total") > 0) || (!is_tank && speed > 1.0);
        
        if in_battle {
            // Reduced logging spam
            // log::debug!("[WT Parser] 🎮 Type={}, Speed={:.0} km/h, RPM={:.0}, Gear={:.0}", 
            //     if is_tank { "TANK" } else { "AIRCRAFT" }, speed, engine_rpm, gear_value);
            // if is_tank {
            //     log::debug!("[WT Parser] 🚜 Tank data: Crew={}/{}, Ammo={}, Stabilizer={}", 
            //         get_i32("crew_current"), get_i32("crew_total"), 
            //         get_i32("first_stage_ammo"), get_f32("stabilizer"));
            // }
        }

        Indicators {
            // Basic
            speed,
            altitude,
            climb: get_f32("Vy, m/s"),
            
            // Engine
            engine_rpm,
            engine_temp: get_f32("engine temp 1, C").max(get_f32("engine temp 2, C")),
            oil_temp: get_f32("oil temp 1, C").max(get_f32("oil temp 2, C")),
            water_temp: get_f32("water temp 1, C").max(get_f32("water temp 2, C")),
            manifold_pressure: get_f32("manifold pressure 1, atm").max(get_f32("manifold pressure 2, atm")),
            throttle: get_f32("throttle 1, %").max(get_f32("throttle 2, %")),
            mixture: get_f32("mixture 1, %").max(get_f32("mixture 2, %")),
            radiator: get_f32("radiator 1, %").max(get_f32("radiator 2, %")),
            compressor_stage: get_i32("compressor stage 1").max(get_i32("compressor stage 2")),
            magneto: get_i32("magneto 1").max(get_i32("magneto 2")),
            
            // Controls (percentage)
            pitch: get_f32("stick_elevator"),
            roll: get_f32("stick_ailerons"),
            yaw: get_f32("pedals1"),
            aileron: get_f32("aileron, %"),
            elevator: get_f32("elevator, %"),
            rudder: get_f32("rudder, %"),
            flaps: get_f32("flaps, %"),
            gear: gear_value,
            airbrake: get_f32("airbrake, %"),
            
            // Aerodynamics
            aoa: get_f32("AoA, deg"),
            slip: get_f32("AoS, deg"),
            g_load: get_f32("Ny"),  // Vertical G-load
            mach: get_f32("M"),
            tas,
            ias,
            
            // Weapons
            cannon_ready: get_bool("cannon_ready"),
            machine_gun_ready: get_bool("machine_gun_ready"),
            rockets_ready: get_i32("rockets_ready"),
            bombs_ready: get_i32("bombs_ready"),
            torpedoes_ready: get_i32("torpedoes_ready"),
            
            // Ammo (different fields for tanks vs aircraft)
            ammo_count: if is_tank {
                get_i32("first_stage_ammo")  // Tank ammo in ready rack
            } else {
                get_i32("cannon ammo 1").max(get_i32("cannon ammo 2"))  // Aircraft cannon ammo
            },
            rocket_count: get_i32("rockets"),
            bomb_count: get_i32("bombs"),
            
            // Fuel (kg)
            fuel,
            fuel_max,
            fuel_time: 0.0,
            
            // Damage
            engine_damage: 0.0,
            controls_damage: 0.0,
            gear_damage: 0.0,
            flaps_damage: 0.0,
            
            // Tank-specific
            stabilizer: get_f32("stabilizer"),
            crew_total: get_i32("crew_total"),
            crew_current: get_i32("crew_current"),
            gunner_state: get_i32("gunner_state"),
            driver_state: get_i32("driver_state"),
            cruise_control: get_f32("cruise_control"),
            driving_direction: get_i32("driving_direction_mode"),
        }
    }

    pub fn get_poll_interval() -> Duration {
        Duration::from_millis(POLL_INTERVAL_MS)
    }
}

impl Default for WTTelemetryReader {
    fn default() -> Self {
        Self::new()
    }
}

