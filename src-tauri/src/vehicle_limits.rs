/// Vehicle Limits System
/// Automatically creates triggers based on real vehicle limits from WT Vehicles API

use crate::wt_vehicles_api::{WTVehiclesAPI, VehicleLimits};
use crate::event_triggers::{TriggerCondition, EventTrigger};
use crate::pattern_engine::GameEvent;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VehicleLimitsManager {
    api: WTVehiclesAPI,
    current_vehicle: Arc<RwLock<Option<String>>>,
    current_limits: Arc<RwLock<Option<VehicleLimits>>>,
}

impl VehicleLimitsManager {
    pub fn new() -> Self {
        Self {
            api: WTVehiclesAPI::new(),
            current_vehicle: Arc::new(RwLock::new(None)),
            current_limits: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Update vehicle and fetch its limits
    pub async fn update_vehicle(&self, vehicle_name: &str) -> anyhow::Result<()> {
        // Check if vehicle changed
        {
            let current = self.current_vehicle.read().await;
            if current.as_ref().map(|v| v == vehicle_name).unwrap_or(false) {
                return Ok(()); // Same vehicle, no update needed
            }
        }
        
        log::info!("[Vehicle Limits] 🔄 Fetching limits for: {}", vehicle_name);
        
        // Try to get vehicle data from API
        match self.api.get_vehicle(vehicle_name).await {
            Ok(data) => {
                log::info!("[Vehicle Limits] ✅ Found vehicle: {} ({})", 
                    data.display_name, data.vehicle_type);
                
                if let Some(max_speed) = data.max_speed_kmh {
                    log::info!("[Vehicle Limits]    Max Speed: {} km/h", max_speed);
                }
                if let Some(max_g) = data.max_positive_g {
                    log::info!("[Vehicle Limits]    Max +G: {:.1}G", max_g);
                }
                if let Some(max_alt) = data.max_altitude_meters {
                    log::info!("[Vehicle Limits]    Max Altitude: {} m", max_alt);
                }
                
                // Extract limits
                if let Some(limits) = VehicleLimits::from_vehicle_data(&data) {
                    *self.current_limits.write().await = Some(limits);
                }
            }
            Err(e) => {
                log::warn!("[Vehicle Limits] ⚠️ Failed to fetch data for '{}': {}", vehicle_name, e);
                log::info!("[Vehicle Limits] 💡 Using default limits");
            }
        }
        
        // Update current vehicle
        *self.current_vehicle.write().await = Some(vehicle_name.to_string());
        
        Ok(())
    }
    
    /// Get current vehicle limits
    pub async fn get_limits(&self) -> Option<VehicleLimits> {
        self.current_limits.read().await.clone()
    }
    
    /// Generate dynamic triggers based on current vehicle limits
    pub async fn generate_limit_triggers(&self) -> Vec<EventTrigger> {
        let mut triggers = Vec::new();
        
        if let Some(limits) = self.get_limits().await {
            log::info!("[Vehicle Limits] 🎯 Generating dynamic triggers for {}", limits.identifier);
            
            // Max Speed Warning (90% of max)
            if limits.max_speed_kmh > 0.0 {
                let warning_speed = limits.max_speed_kmh * 0.9;
                triggers.push(EventTrigger {
                    id: "dynamic_overspeed".to_string(),
                    name: format!("Overspeed Warning ({}+ km/h)", warning_speed as i32),
                    description: format!("Approaching max speed of {} km/h", limits.max_speed_kmh as i32),
                    condition: TriggerCondition::SpeedAbove(warning_speed),
                    event: GameEvent::Overspeed,
                    cooldown_ms: 5000,
                    enabled: false,  // OFF by default - user must enable manually
                    is_builtin: false,
                    pattern: None,
                });
            }
            
            // Max +G Warning (95% of max)
            if limits.max_positive_g > 0.0 {
                let warning_g = limits.max_positive_g * 0.95;
                triggers.push(EventTrigger {
                    id: "dynamic_high_g".to_string(),
                    name: format!("High G Warning ({:.1}+ G)", warning_g),
                    description: format!("Approaching max +G of {:.1}G", limits.max_positive_g),
                    condition: TriggerCondition::GLoadAbove(warning_g),
                    event: GameEvent::OverG,
                    cooldown_ms: 3000,
                    enabled: false,  // OFF by default - user must enable manually
                    is_builtin: false,
                    pattern: None,
                });
            }
            
            // Max -G Warning (95% of max)
            if limits.max_negative_g < 0.0 {
                let warning_g = limits.max_negative_g * 0.95;
                triggers.push(EventTrigger {
                    id: "dynamic_negative_g".to_string(),
                    name: format!("Negative G Warning ({:.1} G)", warning_g),
                    description: format!("Approaching max -G of {:.1}G", limits.max_negative_g),
                    condition: TriggerCondition::GLoadBelow(warning_g),
                    event: GameEvent::OverG,
                    cooldown_ms: 3000,
                    enabled: false,  // OFF by default - user must enable manually
                    is_builtin: false,
                    pattern: None,
                });
            }
            
            log::info!("[Vehicle Limits] ✅ Generated {} dynamic triggers", triggers.len());
        }
        
        triggers
    }
}

impl Default for VehicleLimitsManager {
    fn default() -> Self {
        Self::new()
    }
}

