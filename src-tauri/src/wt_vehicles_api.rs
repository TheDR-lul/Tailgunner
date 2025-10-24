/// WT Vehicles API Integration
/// Integration via REST API (NO GPL dependencies!)
/// Source: https://github.com/Sgambe33/WarThunder-Vehicles-API
/// Public API: www.wtvehiclesapi.sgambe.serv00.net

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const WT_VEHICLES_API_BASE: &str = "https://www.wtvehiclesapi.sgambe.serv00.net";

/// Vehicle information from WT Vehicles API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleData {
    pub identifier: String,
    #[serde(default)]
    pub wikiname: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub vehicle_type: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub rank: i32,
    #[serde(default)]
    pub battle_rating: HashMap<String, f32>,
    
    // Engine characteristics
    #[serde(default)]
    pub max_speed_kmh: Option<f32>,
    #[serde(default)]
    pub max_altitude_meters: Option<f32>,
    #[serde(default)]
    pub engine_power_hp: Option<f32>,
    
    // Flight limits (not in API - calculated)
    #[serde(skip)]
    pub wing_rip_speed_kmh: Option<f32>,
    #[serde(skip)]
    pub flutter_speed_kmh: Option<f32>,
    
    // G-load limits
    #[serde(default)]
    pub max_positive_g: Option<f32>,
    #[serde(default)]
    pub max_negative_g: Option<f32>,
    
    // Fuel
    #[serde(default)]
    pub fuel_capacity_kg: Option<f32>,
    
    // Weapons
    #[serde(default)]
    pub weapons: Option<Vec<String>>,
}

/// Client for WT Vehicles API
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
    
    /// Get vehicle data by identifier
    pub async fn get_vehicle(&self, identifier: &str) -> anyhow::Result<VehicleData> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(identifier) {
                return Ok(data.clone());
            }
        }
        
        // API request
        let url = format!("{}/api/vehicles/{}", WT_VEHICLES_API_BASE, identifier);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "ButtThunder/0.2.0")
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());
            log::error!("[WT Vehicles API] Error {}: {}", status, body);
            return Err(anyhow::anyhow!("API returned status {}: {}", status, body));
        }
        
        // Get response text first for debugging
        let body_text = response.text().await?;
        log::debug!("[WT Vehicles API] Response for '{}': {}", identifier, &body_text[..body_text.len().min(500)]);
        
        // Try to parse
        let data: VehicleData = serde_json::from_str(&body_text)
            .map_err(|e| {
                log::error!("[WT Vehicles API] Failed to parse response for '{}': {}", identifier, e);
                log::error!("[WT Vehicles API] Response body: {}", body_text);
                anyhow::anyhow!("Failed to parse vehicle data: {}", e)
            })?;
        
        // Cache it
        {
            let mut cache = self.cache.write().await;
            cache.insert(identifier.to_string(), data.clone());
        }
        
        Ok(data)
    }
    
    /// Search for vehicle by name (returns first match)
    pub async fn search_vehicle(&self, name: &str) -> anyhow::Result<VehicleData> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(name) {
                log::debug!("[WT Vehicles API] Cache hit for: '{}'", name);
                return Ok(data.clone());
            }
        }
        
        // Step 1: Search for vehicle identifier
        let search_url = format!("{}/api/vehicles/search/{}", WT_VEHICLES_API_BASE, name);
        log::info!("[WT Vehicles API] 🔍 Searching for: '{}'", name);
        
        let search_response = self.client
            .get(&search_url)
            .header("User-Agent", "Tailgunner/0.2.0")
            .send()
            .await?;
        
        if !search_response.status().is_success() {
            let status = search_response.status();
            let body = search_response.text().await.unwrap_or_else(|_| "".to_string());
            log::error!("[WT Vehicles API] Search error {}: {}", status, body);
            return Err(anyhow::anyhow!("Search failed: {}", status));
        }
        
        // Parse search results (array of identifier strings)
        let body_text = search_response.text().await?;
        log::debug!("[WT Vehicles API] Search response: {}", &body_text[..body_text.len().min(300)]);
        
        let identifiers: Vec<String> = serde_json::from_str(&body_text)
            .map_err(|e| {
                log::error!("[WT Vehicles API] Failed to parse identifiers for '{}': {}", name, e);
                log::error!("[WT Vehicles API] Response: {}", body_text);
                anyhow::anyhow!("Failed to parse search results: {}", e)
            })?;
        
        // Get first identifier
        let identifier = identifiers.into_iter().next()
            .ok_or_else(|| {
                log::warn!("[WT Vehicles API] No results for '{}'", name);
                anyhow::anyhow!("No vehicles found matching '{}'", name)
            })?;
        
        log::info!("[WT Vehicles API] 📋 Found identifier: '{}'", identifier);
        
        // Step 2: Get full vehicle data by identifier
        let mut data = self.get_vehicle(&identifier).await?;
        
        // Calculate wing rip speed (flutter speed)
        // War Thunder mechanics: wing rip typically occurs at 110-130% of max speed
        // Modern jets: ~115%, Props/Early jets: ~120%, Heavy bombers: ~110%
        if let Some(max_speed) = data.max_speed_kmh {
            let multiplier = match data.vehicle_type.as_str() {
                "fighter" | "jet_fighter" => 1.15,     // Modern fighters
                "bomber" | "heavy_bomber" => 1.10,     // Heavy aircraft
                "attacker" | "strike_aircraft" => 1.12,
                _ => 1.15,  // Default
            };
            
            data.wing_rip_speed_kmh = Some(max_speed * multiplier);
            data.flutter_speed_kmh = Some(max_speed * (multiplier - 0.05)); // Flutter warning ~5% earlier
            
            log::info!("[WT Vehicles API] 💨 Calculated wing rip: {:.0} km/h (flutter: {:.0} km/h)", 
                data.wing_rip_speed_kmh.unwrap(), 
                data.flutter_speed_kmh.unwrap()
            );
        }
        
        log::info!("[WT Vehicles API] ✅ Loaded: {} ({})", data.display_name, data.identifier);
        
        // Cache with original name as key
        {
            let mut cache = self.cache.write().await;
            cache.insert(name.to_string(), data.clone());
        }
        
        Ok(data)
    }
    
    /// Get list of all vehicles (with limit)
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
    
    /// Get max speed for specific vehicle
    pub async fn get_max_speed(&self, identifier: &str) -> Option<f32> {
        self.get_vehicle(identifier).await.ok()?.max_speed_kmh
    }
    
    /// Get max G-load limits
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

/// Simplified version for quick data access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLimits {
    pub identifier: String,
    pub max_speed_kmh: f32,
    pub wing_rip_speed_kmh: f32,
    pub flutter_speed_kmh: f32,
    pub max_positive_g: f32,
    pub max_negative_g: f32,
}

impl VehicleLimits {
    /// Create from full vehicle data
    pub fn from_vehicle_data(data: &VehicleData) -> Option<Self> {
        Some(Self {
            identifier: data.identifier.clone(),
            max_speed_kmh: data.max_speed_kmh?,
            wing_rip_speed_kmh: data.wing_rip_speed_kmh?,
            flutter_speed_kmh: data.flutter_speed_kmh?,
            max_positive_g: data.max_positive_g?,
            max_negative_g: data.max_negative_g?,
        })
    }
}

// ===== CACHED DATA FOR POPULAR AIRCRAFT =====
// (for offline operation)

lazy_static::lazy_static! {
    pub static ref DEFAULT_LIMITS: HashMap<&'static str, VehicleLimits> = {
        let mut m = HashMap::new();
        
        // Bf 109 F-4 (Prop fighter)
        m.insert("bf-109f-4", VehicleLimits {
            identifier: "bf-109f-4".to_string(),
            max_speed_kmh: 635.0,
            wing_rip_speed_kmh: 762.0,  // ~120% for props
            flutter_speed_kmh: 730.0,   // ~115%
            max_positive_g: 12.5,
            max_negative_g: -6.0,
        });
        
        // Spitfire Mk Vb (Prop fighter)
        m.insert("spitfire_mk5b", VehicleLimits {
            identifier: "spitfire_mk5b".to_string(),
            max_speed_kmh: 605.0,
            wing_rip_speed_kmh: 726.0,  // ~120%
            flutter_speed_kmh: 695.0,   // ~115%
            max_positive_g: 12.0,
            max_negative_g: -5.5,
        });
        
        // P-51D-5 (Prop fighter)
        m.insert("p-51d-5", VehicleLimits {
            identifier: "p-51d-5".to_string(),
            max_speed_kmh: 710.0,
            wing_rip_speed_kmh: 852.0,  // ~120%
            flutter_speed_kmh: 817.0,   // ~115%
            max_positive_g: 11.5,
            max_negative_g: -5.0,
        });
        
        // Yak-3 (Prop fighter)
        m.insert("yak-3", VehicleLimits {
            identifier: "yak-3".to_string(),
            max_speed_kmh: 655.0,
            wing_rip_speed_kmh: 786.0,  // ~120%
            flutter_speed_kmh: 753.0,   // ~115%
            max_positive_g: 13.0,
            max_negative_g: -6.5,
        });
        
        // La-5FN (Prop fighter)
        m.insert("la-5fn", VehicleLimits {
            identifier: "la-5fn".to_string(),
            max_speed_kmh: 635.0,
            wing_rip_speed_kmh: 762.0,  // ~120%
            flutter_speed_kmh: 730.0,   // ~115%
            max_positive_g: 12.0,
            max_negative_g: -6.0,
        });
        
        // Fw 190 A-5 (Prop fighter)
        m.insert("fw-190a-5", VehicleLimits {
            identifier: "fw-190a-5".to_string(),
            max_speed_kmh: 670.0,
            wing_rip_speed_kmh: 804.0,  // ~120%
            flutter_speed_kmh: 770.0,   // ~115%
            max_positive_g: 11.0,
            max_negative_g: -4.5,
        });
        
        // Default values for unknown vehicles
        m.insert("default", VehicleLimits {
            identifier: "default".to_string(),
            max_speed_kmh: 700.0,
            wing_rip_speed_kmh: 805.0,  // ~115% (conservative for unknown aircraft)
            flutter_speed_kmh: 770.0,   // ~110%
            max_positive_g: 12.0,
            max_negative_g: -6.0,
        });
        
        m
    };
}

