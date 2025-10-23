/// WT Vehicles API Integration
/// Интеграция через REST API (БЕЗ GPL зависимости!)
/// Источник: https://github.com/Sgambe33/WarThunder-Vehicles-API
/// Публичный API: www.wtvehiclesapi.sgambe.serv00.net

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const WT_VEHICLES_API_BASE: &str = "https://www.wtvehiclesapi.sgambe.serv00.net";

/// Информация о технике из WT Vehicles API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleData {
    pub identifier: String,
    pub wikiname: String,
    pub display_name: String,
    pub vehicle_type: String,
    pub country: String,
    pub rank: i32,
    pub battle_rating: HashMap<String, f32>,
    
    // Характеристики двигателя
    pub max_speed_kmh: Option<f32>,
    pub max_altitude_meters: Option<f32>,
    pub engine_power_hp: Option<f32>,
    
    // Пределы G-перегрузки
    pub max_positive_g: Option<f32>,
    pub max_negative_g: Option<f32>,
    
    // Топливо
    pub fuel_capacity_kg: Option<f32>,
    
    // Вооружение
    pub weapons: Option<Vec<String>>,
}

/// Клиент для WT Vehicles API
pub struct WTVehiclesAPI {
    client: reqwest::Client,
    cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, VehicleData>>>,
}

impl WTVehiclesAPI {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Получить данные о технике по имени
    pub async fn get_vehicle(&self, identifier: &str) -> anyhow::Result<VehicleData> {
        // Проверяем кэш
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(identifier) {
                return Ok(data.clone());
            }
        }
        
        // Запрос к API
        let url = format!("{}/api/vehicles/{}", WT_VEHICLES_API_BASE, identifier);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "ButtThunder/0.2.0")
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch vehicle data: {}", response.status()));
        }
        
        let data: VehicleData = response.json().await?;
        
        // Кэшируем
        {
            let mut cache = self.cache.write().await;
            cache.insert(identifier.to_string(), data.clone());
        }
        
        Ok(data)
    }
    
    /// Получить список всей техники (с лимитом)
    pub async fn get_all_vehicles(&self, limit: Option<usize>) -> anyhow::Result<Vec<VehicleData>> {
        let url = format!("{}/api/vehicles", WT_VEHICLES_API_BASE);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "ButtThunder/0.2.0")
            .send()
            .await?;
        
        let mut vehicles: Vec<VehicleData> = response.json().await?;
        
        if let Some(limit) = limit {
            vehicles.truncate(limit);
        }
        
        Ok(vehicles)
    }
    
    /// Получить максимальную скорость для конкретной техники
    pub async fn get_max_speed(&self, identifier: &str) -> Option<f32> {
        self.get_vehicle(identifier).await.ok()?.max_speed_kmh
    }
    
    /// Получить максимальную G-перегрузку
    pub async fn get_max_g_limits(&self, identifier: &str) -> Option<(f32, f32)> {
        let vehicle = self.get_vehicle(identifier).await.ok()?;
        Some((
            vehicle.max_positive_g?,
            vehicle.max_negative_g?,
        ))
    }
}

impl Default for WTVehiclesAPI {
    fn default() -> Self {
        Self::new()
    }
}

/// Упрощенная версия для быстрого доступа к данным
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLimits {
    pub identifier: String,
    pub max_speed_kmh: f32,
    pub max_positive_g: f32,
    pub max_negative_g: f32,
}

impl VehicleLimits {
    /// Create from full vehicle data
    pub fn from_vehicle_data(data: &VehicleData) -> Option<Self> {
        Some(Self {
            identifier: data.identifier.clone(),
            max_speed_kmh: data.max_speed_kmh?,
            max_positive_g: data.max_positive_g?,
            max_negative_g: data.max_negative_g?,
        })
    }
}

// ===== КЭШИРОВАННЫЕ ДАННЫЕ ПОПУЛЯРНЫХ САМОЛЕТОВ =====
// (для работы без интернета)

lazy_static::lazy_static! {
    pub static ref DEFAULT_LIMITS: HashMap<&'static str, VehicleLimits> = {
        let mut m = HashMap::new();
        
        // Bf 109 F-4
        m.insert("bf-109f-4", VehicleLimits {
            identifier: "bf-109f-4".to_string(),
            max_speed_kmh: 635.0,
            max_positive_g: 12.5,
            max_negative_g: -6.0,
        });
        
        // Spitfire Mk Vb
        m.insert("spitfire_mk5b", VehicleLimits {
            identifier: "spitfire_mk5b".to_string(),
            max_speed_kmh: 605.0,
            max_positive_g: 12.0,
            max_negative_g: -5.5,
        });
        
        // P-51D-5
        m.insert("p-51d-5", VehicleLimits {
            identifier: "p-51d-5".to_string(),
            max_speed_kmh: 710.0,
            max_positive_g: 11.5,
            max_negative_g: -5.0,
        });
        
        // Yak-3
        m.insert("yak-3", VehicleLimits {
            identifier: "yak-3".to_string(),
            max_speed_kmh: 655.0,
            max_positive_g: 13.0,
            max_negative_g: -6.5,
        });
        
        // La-5FN
        m.insert("la-5fn", VehicleLimits {
            identifier: "la-5fn".to_string(),
            max_speed_kmh: 635.0,
            max_positive_g: 12.0,
            max_negative_g: -6.0,
        });
        
        // Fw 190 A-5
        m.insert("fw-190a-5", VehicleLimits {
            identifier: "fw-190a-5".to_string(),
            max_speed_kmh: 670.0,
            max_positive_g: 11.0,
            max_negative_g: -4.5,
        });
        
        // Дефолтные значения для неизвестной техники
        m.insert("default", VehicleLimits {
            identifier: "default".to_string(),
            max_speed_kmh: 700.0,
            max_positive_g: 12.0,
            max_negative_g: -6.0,
        });
        
        m
    };
}

