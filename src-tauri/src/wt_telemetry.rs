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
    pub async fn get_state(&mut self) -> Result<GameState> {
        let url = format!("{}/state", WT_TELEMETRY_URL);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to War Thunder")?;

        let raw_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse WT telemetry")?;

        let state = self.parse_state(raw_json)?;
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

        let indicators = Indicators::default(); // Заполним позже через get_indicators

        Ok(GameState {
            valid,
            type_,
            indicators,
            state,
        })
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

