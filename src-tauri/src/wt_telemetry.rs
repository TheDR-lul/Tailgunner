/// War Thunder Telemetry Reader
/// Reads data from localhost:8111 (official WT API)
/// EAC-Safe: only HTTP requests, no memory injection

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const WT_TELEMETRY_URL: &str = "http://127.0.0.1:8111";
const POLL_INTERVAL_MS: u64 = 100; // 10 times per second

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub valid: bool,
    pub type_: VehicleType,
    pub vehicle_name: String,
    pub indicators: Indicators,
    pub state: Vec<String>,
    
    // Combat events (detected from state changes)
    pub hit_received: bool,        // Detected hit this frame
    pub critical_damage: bool,     // Critical damage this frame
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
}

impl WTTelemetryReader {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(200))
            .build()
            .unwrap();
        
        Self {
            client,
            last_state: None,
            last_fetch_time: None,
            cache_duration_ms: 50, // Cache for 50ms (20 Hz max poll rate)
        }
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

        log::debug!("[WT API] /indicators response:\n{}", 
            serde_json::to_string_pretty(&indicators_json).unwrap_or_else(|_| "Failed to serialize".to_string()));

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
        let state = self.parse_combined_state(indicators_json, state_json)?;
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

    /// Parse combined data from /indicators and /state
    fn parse_combined_state(&self, indicators_json: serde_json::Value, state_json: serde_json::Value) -> Result<GameState> {
        // 1. Parse type and army from /indicators
        let valid = indicators_json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
        let type_str = indicators_json.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        let army_str = indicators_json.get("army").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        log::info!("[WT Parser] From /indicators: type='{}', army='{}', valid={}", type_str, army_str, valid);
        
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
        let mut last = last_vehicle.lock().unwrap();
        
        if *last != vehicle_name {
            log::error!("[WT Parser] ✅ Vehicle detected: '{}' ({:?}, army={})", 
                vehicle_name, type_, army_str);
            *last = vehicle_name.clone();
        }

        // 5. Detect combat events (Hit, CriticalDamage) by comparing with last_state
        let mut hit_received = false;
        let mut critical_damage = false;
        
        if let Some(last) = &self.last_state {
            // Hit detection: new damage entries in state array
            let new_damage_entries: Vec<_> = state.iter()
                .filter(|s| s.contains("damaged") || s.contains("broken") || s.contains("fire"))
                .filter(|s| !last.state.contains(s))
                .collect();
            
            if !new_damage_entries.is_empty() {
                hit_received = true;
                log::error!("[Combat Event] 🎯 HIT DETECTED: {:?}", new_damage_entries);
            }
            
            // Critical damage detection: multiple new damage entries OR engine/controls damage
            if new_damage_entries.len() >= 2 || 
               new_damage_entries.iter().any(|s| s.contains("engine") || s.contains("controls") || s.contains("fire")) {
                critical_damage = true;
                log::error!("[Combat Event] 💥 CRITICAL HIT: {:?}", new_damage_entries);
            }
        }

        Ok(GameState {
            valid,
            type_,
            vehicle_name,
            indicators,
            state,
            hit_received,
            critical_damage,
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
            hit_received: false,      // Legacy method doesn't track state changes
            critical_damage: false,
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
            log::debug!("[WT Parser] 🎮 Type={}, Speed={:.0} km/h, RPM={:.0}, Gear={:.0}", 
                if is_tank { "TANK" } else { "AIRCRAFT" }, speed, engine_rpm, gear_value);
            
            if is_tank {
                log::debug!("[WT Parser] 🚜 Tank data: Crew={}/{}, Ammo={}, Stabilizer={}", 
                    get_i32("crew_current"), get_i32("crew_total"), 
                    get_i32("first_stage_ammo"), get_f32("stabilizer"));
            }
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

