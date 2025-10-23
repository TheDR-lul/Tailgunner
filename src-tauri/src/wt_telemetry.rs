/// War Thunder Telemetry Reader
/// Читает данные с localhost:8111 (официальный API WT)
/// EAC-Safe: только HTTP-запросы, никакой инъекции в память

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const WT_TELEMETRY_URL: &str = "http://127.0.0.1:8111";
const POLL_INTERVAL_MS: u64 = 100; // 10 раз в секунду

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub valid: bool,
    pub type_: VehicleType,
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

    /// Проверка доступности War Thunder
    pub async fn is_game_running(&self) -> bool {
        let url = format!("{}/state", WT_TELEMETRY_URL);
        self.client
            .get(&url)
            .send()
            .await
            .is_ok()
    }

    /// Получение текущего состояния игры
    /// /state содержит ВСЕ данные (type, valid, indicators)
    pub async fn get_state(&mut self) -> Result<GameState> {
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

        log::trace!("[WT API] /state response: {}", serde_json::to_string_pretty(&state_json).unwrap_or_default());

        // Парсим все данные из ОДНОГО эндпоинта
        let state = self.parse_state(state_json)?;
        self.last_state = Some(state.clone());
        
        Ok(state)
    }

    /// Получение индикаторов
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

    /// Парсинг сырых данных WT в структуру
    /// /state содержит ВСЕ данные в одном JSON
    fn parse_state(&self, json: serde_json::Value) -> Result<GameState> {
        let valid = json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
        
        let type_str = json.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        let type_ = match type_str.to_lowercase().as_str() {
            "tank" | "spaa" | "spg" => VehicleType::Tank,
            "aircraft" | "fighter" | "bomber" | "attacker" => VehicleType::Aircraft,
            "helicopter" => VehicleType::Helicopter,
            "ship" | "boat" => VehicleType::Ship,
            _ => VehicleType::Unknown,
        };

        let state = json.get("state")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // Парсим indicators из ТОГО ЖЕ JSON (все данные в /state)
        let indicators = self.parse_indicators(json);

        Ok(GameState {
            valid,
            type_,
            indicators,
            state,
        })
    }

    /// Парсинг indicators из JSON
    /// Названия полей в WT API содержат запятые, пробелы и единицы измерения!
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

