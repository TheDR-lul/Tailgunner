/// Vehicle Limits System
/// Automatically creates triggers based on real vehicle limits from datamine

use crate::datamine::{self, VehicleLimits, AircraftLimits};
use crate::event_triggers::{TriggerCondition, EventTrigger};
use crate::pattern_engine::GameEvent;
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
fn generate_aircraft_triggers(aircraft: &AircraftLimits) -> Vec<EventTrigger> {
    let mut triggers = Vec::new();
    
    // Flutter Warning (if available)
    if let Some(flutter_speed) = aircraft.flutter_speed {
        triggers.push(EventTrigger {
            id: "dynamic_flutter".to_string(),
            name: format!("Flutter Warning ({}+ km/h)", flutter_speed as i32),
            description: format!("Wings starting to flutter! Rip at {} km/h", aircraft.vne_kmh as i32),
            condition: TriggerCondition::SpeedAbove(flutter_speed),
            event: GameEvent::Overspeed,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: false,
            pattern: None,
            curve_points: None,
            continuous: true,  // ✅ Вибрация пока скорость выше порога
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
    }
    
    // Critical Speed Warning (95% of Vne)
    let critical_speed = aircraft.vne_kmh * 0.95;
    triggers.push(EventTrigger {
        id: "dynamic_overspeed".to_string(),
        name: format!("CRITICAL SPEED ({}+ km/h)", critical_speed as i32),
        description: format!("DANGER! Wings will rip at {} km/h!", aircraft.vne_kmh as i32),
        condition: TriggerCondition::SpeedAbove(critical_speed),
        event: GameEvent::Overspeed,
        cooldown_ms: 2000,
        enabled: false,
        is_builtin: false,
        pattern: None,
        curve_points: None,
        continuous: true,  // ✅ Вибрация пока скорость критическая
        is_event_based: false,
        filter_type: None,
        filter_text: None,
    });
    
    // Max +G Warning (80% of max) - only if data available
    if let Some(max_g) = aircraft.max_positive_g {
        let warning_g = max_g * 0.8;
        triggers.push(EventTrigger {
            id: "dynamic_high_g".to_string(),
            name: format!("High G Warning ({:.1}+ G)", warning_g),
            description: format!("Approaching max +G of {:.1}G", max_g),
            condition: TriggerCondition::GLoadAbove(warning_g),
            event: GameEvent::OverG,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: false,
            pattern: None,
            curve_points: None,
            continuous: true,  // ✅ Вибрация пока G-нагрузка высокая
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
    }
    
    // Max -G Warning (80% of max) - only if data available
    if let Some(max_g_neg) = aircraft.max_negative_g {
        let warning_g_neg = max_g_neg * 0.8;
        triggers.push(EventTrigger {
            id: "dynamic_negative_g".to_string(),
            name: format!("Negative G Warning ({:.1} G)", warning_g_neg),
            description: format!("Approaching max -G of {:.1}G", max_g_neg),
            condition: TriggerCondition::GLoadBelow(warning_g_neg),
            event: GameEvent::OverG,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: false,
            pattern: None,
            curve_points: None,
            continuous: true,  // ✅ Вибрация пока отрицательная G-нагрузка
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
    }
    
    triggers
}

/// Generate triggers for ground vehicles
fn generate_ground_triggers(ground: &crate::datamine::types::GroundLimits) -> Vec<EventTrigger> {
    let mut triggers = Vec::new();
    
    // Max speed warning (98% of max) - only if data available
    if let Some(max_speed) = ground.max_speed_kmh {
        let warning_speed = max_speed * 0.98;
        triggers.push(EventTrigger {
            id: "dynamic_ground_maxspeed".to_string(),
            name: format!("Max Speed ({:.0}+ km/h)", warning_speed),
            description: format!("Approaching maximum speed of {:.0} km/h", max_speed),
            condition: TriggerCondition::SpeedAbove(warning_speed),
            event: GameEvent::Overspeed,
            cooldown_ms: 5000,
            enabled: false,
            is_builtin: false,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
    }
    
    triggers
}

impl Default for VehicleLimitsManager {
    fn default() -> Self {
        Self::new()
            .expect("Failed to initialize VehicleLimitsManager: database connection error")
    }
}
