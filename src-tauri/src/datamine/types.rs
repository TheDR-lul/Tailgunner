/// Common data structures for datamine module

use serde::{Deserialize, Serialize};

/// Vehicle limits for haptic feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VehicleLimits {
    Aircraft(AircraftLimits),
    Ground(GroundLimits),
    Ship(ShipLimits),
}

/// Aircraft flight limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AircraftLimits {
    pub identifier: String,
    pub display_name: String,
    
    // Speed limits (km/h)
    pub vne_kmh: f32,              // Never Exceed Speed
    pub vne_mach: Option<f32>,     // Max Mach number
    pub max_speed_ground: f32,     // Max speed at ground level
    pub stall_speed: f32,          // Stall speed
    pub flutter_speed: Option<f32>, // Flutter warning speed
    
    // G-load limits
    pub mass_kg: f32,              // Takeoff mass
    pub wing_overload_pos_n: f32,  // Positive G limit (Newtons)
    pub wing_overload_neg_n: f32,  // Negative G limit (Newtons)
    pub max_positive_g: f32,       // Calculated +G
    pub max_negative_g: f32,       // Calculated -G
    
    // Engine
    pub max_rpm: Option<f32>,
    pub horse_power: Option<f32>,
    
    // Metadata
    pub vehicle_type: String,      // fighter, bomber, attacker, etc
    pub last_updated: String,
}

impl AircraftLimits {
    /// Calculate G-limits from wing overload and mass
    pub fn calculate_g_limits(wing_overload_n: f32, mass_kg: f32) -> f32 {
        const GRAVITY: f32 = 9.81;
        wing_overload_n / (mass_kg * GRAVITY)
    }
    
    /// Estimate flutter speed if not provided (typically ~95% of Vne)
    pub fn estimate_flutter_speed(vne: f32) -> f32 {
        vne * 0.95
    }
}

/// Ground vehicle limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundLimits {
    pub identifier: String,
    pub display_name: String,
    
    // Speed and mass
    pub max_speed_kmh: f32,
    pub max_reverse_speed_kmh: f32,
    pub mass_kg: f32,
    
    // Engine
    pub horse_power: f32,
    pub max_rpm: f32,
    pub min_rpm: f32,
    
    // Durability
    pub hull_hp: f32,
    pub armor_thickness_mm: Option<f32>,
    
    // Metadata
    pub vehicle_type: String,      // heavyVehicle, mediumVehicle, etc
    pub last_updated: String,
}

/// Ship limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipLimits {
    pub identifier: String,
    pub display_name: String,
    
    // Speed
    pub max_speed_knots: f32,
    pub max_reverse_speed_knots: f32,
    
    // Compartments (for damage tracking)
    pub compartments: Vec<Compartment>,
    
    // Metadata
    pub ship_class: String,        // destroyer, cruiser, battleship, etc
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compartment {
    pub name: String,
    pub hp: f32,
    pub critical: bool,  // Engine room, ammo storage, etc
}

/// Raw BLK file data structures
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AircraftFM {
    #[serde(rename = "Vne")]
    pub vne: f32,
    
    #[serde(rename = "VneMach")]
    pub vne_mach: Option<f32>,
    
    #[serde(rename = "MaxSpeedNearGround")]
    pub max_speed_near_ground: f32,
    
    #[serde(rename = "MinimalSpeed")]
    pub minimal_speed: f32,
    
    #[serde(rename = "Mass")]
    pub mass: Option<MassData>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct MassData {
    #[serde(rename = "Takeoff")]
    pub takeoff: f32,
    
    #[serde(rename = "WingCritOverload")]
    pub wing_crit_overload: [f32; 2], // [negative, positive] in Newtons
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TankData {
    #[serde(rename = "maxFwdSpeed")]
    pub max_fwd_speed: f32,
    
    #[serde(rename = "maxRevSpeed")]
    pub max_rev_speed: f32,
    
    #[serde(rename = "mass")]
    pub mass: f32,
    
    pub engine: Option<EngineData>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct EngineData {
    #[serde(rename = "horsePowers")]
    pub horse_powers: f32,
    
    #[serde(rename = "maxRPM")]
    pub max_rpm: f32,
    
    #[serde(rename = "minRPM")]
    pub min_rpm: Option<f32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ShipData {
    #[serde(rename = "maxFwdSpeed")]
    pub max_fwd_speed: f32,
    
    #[serde(rename = "maxRevSpeed")]
    pub max_rev_speed: f32,
    
    #[serde(rename = "DamageParts")]
    pub damage_parts: Option<serde_json::Value>,
}

