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
    // Базовые
    pub speed: f32,           // IAS км/ч
    pub altitude: f32,        // м
    pub climb: f32,           // м/с
    
    // Двигатель
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
    
    // Управление
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub aileron: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
    pub gear: f32,
    pub airbrake: f32,
    
    // Аэродинамика
    pub aoa: f32,             // Угол атаки
    pub slip: f32,            // Скольжение
    pub g_load: f32,          // G-перегрузка
    pub mach: f32,            // Число Маха
    pub tas: f32,             // TAS км/ч
    pub ias: f32,             // IAS км/ч
    
    // Вооружение
    pub cannon_ready: bool,
    pub machine_gun_ready: bool,
    pub rockets_ready: i32,
    pub bombs_ready: i32,
    pub torpedoes_ready: i32,
    
    // Боезапас
    pub ammo_count: i32,
    pub rocket_count: i32,
    pub bomb_count: i32,
    
    // Топливо
    pub fuel: f32,            // кг
    pub fuel_max: f32,
    pub fuel_time: f32,       // минуты
    
    // Повреждения
    pub engine_damage: f32,   // 0.0-1.0
    pub controls_damage: f32,
    pub gear_damage: f32,
    pub flaps_damage: f32,
}

pub struct WTTelemetryReader {
    client: reqwest::Client,
    last_state: Option<GameState>,
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
            "ground" => VehicleType::Tank,
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

        let state = state_json.get("state")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // 2. Parse indicators from /state
        let indicators = self.parse_indicators(state_json);

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

        Ok(GameState {
            valid,
            type_,
            vehicle_name,
            indicators,
            state,
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
        })
    }

    /// Parse indicators from JSON
    /// WT API field names contain commas, spaces, and units!
    fn parse_indicators(&self, json: serde_json::Value) -> Indicators {
        let get_f32 = |key: &str| json.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let get_i32 = |key: &str| json.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let get_bool = |key: &str| json.get(key).and_then(|v| v.as_bool()).unwrap_or(false);

        let ias = get_f32("IAS, km/h");
        let tas = get_f32("TAS, km/h");
        let altitude = get_f32("H, m");
        let fuel = get_f32("Mfuel, kg");
        let fuel_max = get_f32("Mfuel0, kg");
        
        log::debug!("[WT Parser] IAS={}, TAS={}, H={}, Fuel={}/{}", ias, tas, altitude, fuel, fuel_max);

        Indicators {
            // Базовые (используем реальные названия из WT API!)
            speed: ias.max(tas), // Берем больше из IAS/TAS
            altitude,
            climb: get_f32("Vy, m/s"),
            
            // Двигатель
            engine_rpm: get_f32("RPM 1").max(get_f32("RPM 2")),
            engine_temp: get_f32("engine temp 1, C").max(get_f32("engine temp 2, C")),
            oil_temp: get_f32("oil temp 1, C").max(get_f32("oil temp 2, C")),
            water_temp: get_f32("water temp 1, C").max(get_f32("water temp 2, C")),
            manifold_pressure: get_f32("manifold pressure 1, atm").max(get_f32("manifold pressure 2, atm")),
            throttle: get_f32("throttle 1, %").max(get_f32("throttle 2, %")),
            mixture: get_f32("mixture 1, %").max(get_f32("mixture 2, %")),
            radiator: get_f32("radiator 1, %").max(get_f32("radiator 2, %")),
            compressor_stage: get_i32("compressor stage 1").max(get_i32("compressor stage 2")),
            magneto: get_i32("magneto 1").max(get_i32("magneto 2")),
            
            // Управление (в процентах)
            pitch: get_f32("stick_elevator"),
            roll: get_f32("stick_ailerons"),
            yaw: get_f32("pedals1"),
            aileron: get_f32("aileron, %"),
            elevator: get_f32("elevator, %"),
            rudder: get_f32("rudder, %"),
            flaps: get_f32("flaps, %"),
            gear: get_f32("gear, %"),
            airbrake: get_f32("airbrake, %"),
            
            // Аэродинамика
            aoa: get_f32("AoA, deg"),
            slip: get_f32("AoS, deg"),
            g_load: get_f32("Ny"),  // Вертикальная перегрузка (G)
            mach: get_f32("M"),
            tas: get_f32("TAS, km/h"),
            ias: get_f32("IAS, km/h"),
            
            // Вооружение (пока не используем, так как названия сложнее)
            cannon_ready: get_bool("cannon_ready"),
            machine_gun_ready: get_bool("machine_gun_ready"),
            rockets_ready: 0, // TODO
            bombs_ready: 0,
            torpedoes_ready: 0,
            
            // Боезапас (пока не используем)
            ammo_count: 0, // TODO
            rocket_count: 0,
            bomb_count: 0,
            
            // Топливо (в кг!)
            fuel: get_f32("Mfuel, kg"),
            fuel_max: get_f32("Mfuel0, kg"),
            fuel_time: 0.0, // TODO: вычислить из Mfuel / расход
            
            // Повреждения (пока не используем, нужно смотреть /state)
            engine_damage: 0.0, // TODO: из /state
            controls_damage: 0.0,
            gear_damage: 0.0,
            flaps_damage: 0.0,
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

