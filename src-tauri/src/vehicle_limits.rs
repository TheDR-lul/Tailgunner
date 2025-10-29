/// Vehicle Limits System
/// Automatically creates triggers based on real vehicle limits from datamine

use crate::datamine::{self, VehicleLimits, AircraftLimits};
use crate::event_triggers::{TriggerCondition, EventTrigger};
use crate::pattern_engine::{GameEvent, VibrationPattern};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VehicleLimitsManager {
    current_vehicle: Arc<RwLock<Option<String>>>,
    current_limits: Arc<RwLock<Option<VehicleLimits>>>,
}

impl VehicleLimitsManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            current_vehicle: Arc::new(RwLock::new(None)),
            current_limits: Arc::new(RwLock::new(None)),
        })
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
        
        log::info!("[Vehicle Limits] Fetching limits for: {}", vehicle_name);
        
        // Convert vehicle name to identifier format
        let identifier = vehicle_name
            .to_lowercase()
            .replace(" ", "_")
            .replace("-", "_");
        
        // Get limits from database
        let db = datamine::database::VehicleDatabase::new()?;
        if let Some(limits) = db.get_limits(&identifier) {
            log::info!("[Vehicle Limits] Found limits for: {}", vehicle_name);
            
            // Log details based on type
            match &limits {
                VehicleLimits::Aircraft(aircraft) => {
                    log::info!("[Vehicle Limits]   Vne: {} km/h", aircraft.vne_kmh);
                    if let Some(g) = aircraft.max_positive_g {
                        log::info!("[Vehicle Limits]   Max +G: {:.1}G", g);
                    } else {
                        log::info!("[Vehicle Limits]   Max +G: N/A");
                    }
                    if let Some(g) = aircraft.max_negative_g {
                        log::info!("[Vehicle Limits]   Max -G: {:.1}G", g);
                    } else {
                        log::info!("[Vehicle Limits]   Max -G: N/A");
                    }
                    if let Some(flutter) = aircraft.flutter_speed {
                        log::info!("[Vehicle Limits]   Flutter: {} km/h", flutter);
                    }
                }
                VehicleLimits::Ground(ground) => {
                    if let Some(speed) = ground.max_speed_kmh {
                        log::info!("[Vehicle Limits]   Max Speed: {} km/h", speed);
                    }
                    if let Some(hp) = ground.horse_power {
                        log::info!("[Vehicle Limits]   Power: {} HP", hp);
                    }
                    if let Some(mass) = ground.mass_kg {
                        log::info!("[Vehicle Limits]   Mass: {:.1} t", mass / 1000.0);
                    }
                }
                VehicleLimits::Ship(ship) => {
                    log::info!("[Vehicle Limits]   Max Speed: {} knots", ship.max_speed_knots);
                    log::info!("[Vehicle Limits]   Compartments: {}", ship.compartments.len());
                }
            }
            
            *self.current_limits.write().await = Some(limits);
        } else {
            log::warn!("[Vehicle Limits] No limits found for '{}' (tried identifier: '{}')", vehicle_name, identifier);
            log::info!("[Vehicle Limits] Using default behavior");
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
            match limits {
                VehicleLimits::Aircraft(aircraft) => {
                    triggers.extend(generate_aircraft_triggers(&aircraft));
                }
                VehicleLimits::Ground(ground) => {
                    triggers.extend(generate_ground_triggers(&ground));
                }
                VehicleLimits::Ship(_ship) => {
                    // Ships don't have speed/G-load triggers yet
                }
            }
            
            log::info!("[Vehicle Limits] Generated {} dynamic triggers", triggers.len());
        }
        
        triggers
    }
}

/// Generate triggers for aircraft
/// NOTE: These triggers cannot work as War Thunder API does not provide speed/G-load data
fn generate_aircraft_triggers(_aircraft: &AircraftLimits) -> Vec<EventTrigger> {
    // Disabled: War Thunder API does not provide speed or G-load data
    // Even with datamined limits, we have no real-time telemetry to compare against
    Vec::new()
}

/// Generate triggers for ground vehicles
/// NOTE: These triggers cannot work as War Thunder API does not provide accurate speed data for ground vehicles
fn generate_ground_triggers(_ground: &crate::datamine::types::GroundLimits) -> Vec<EventTrigger> {
    // Disabled: War Thunder API does not provide reliable speed data for ground vehicles
    Vec::new()
}

impl Default for VehicleLimitsManager {
    fn default() -> Self {
        Self::new()
            .expect("Failed to initialize VehicleLimitsManager: database connection error")
    }
}
