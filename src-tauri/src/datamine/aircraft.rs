/// Aircraft data extractor from flight model files

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all aircraft from game files
pub fn parse_aircraft(game_path: &PathBuf) -> Result<Vec<AircraftLimits>> {
    let fm_path = game_path
        .join("aces.vromfs.bin_u")
        .join("gamedata")
        .join("flightmodels")
        .join("fm");
    
    if !fm_path.exists() {
        return Err(anyhow::anyhow!("Flight models directory not found: {:?}", fm_path));
    }
    
    let mut aircraft = Vec::new();
    
    for entry in fs::read_dir(fm_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("blkx") {
            continue;
        }
        
        match parse_aircraft_file(&path) {
            Ok(limits) => aircraft.push(limits),
            Err(e) => {
                log::warn!("[Aircraft] Failed to parse {:?}: {}", path.file_name(), e);
            }
        }
    }
    
    log::info!("[Aircraft] Parsed {} aircraft", aircraft.len());
    Ok(aircraft)
}

/// Parse single aircraft file
fn parse_aircraft_file(path: &PathBuf) -> Result<AircraftLimits> {
    let json = parser::read_json_file(path)?;
    let identifier = parser::extract_identifier(path)
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    
    // Extract values with fallbacks
    let vne = json["Vne"].as_f64().unwrap_or(800.0) as f32;
    let vne_mach = json["VneMach"].as_f64().map(|v| v as f32);
    let max_speed_ground = json["MaxSpeedNearGround"].as_f64().unwrap_or(600.0) as f32;
    let stall_speed = json["MinimalSpeed"].as_f64().unwrap_or(150.0) as f32;
    
    // Mass data
    let mass_kg = json["Mass"]["Takeoff"].as_f64().unwrap_or(3000.0) as f32;
    
    // Wing overload (Newtons)
    let wing_overload = &json["Mass"]["WingCritOverload"];
    let wing_neg_n = wing_overload[0].as_f64().unwrap_or(-100000.0) as f32;
    let wing_pos_n = wing_overload[1].as_f64().unwrap_or(200000.0) as f32;
    
    // Calculate G-limits
    let max_positive_g = AircraftLimits::calculate_g_limits(wing_pos_n, mass_kg);
    let max_negative_g = AircraftLimits::calculate_g_limits(wing_neg_n.abs(), mass_kg);
    
    // Flutter speed (estimate if not provided)
    let flutter_speed = Some(AircraftLimits::estimate_flutter_speed(vne));
    
    // Engine data (optional)
    let max_rpm = json["Engine0"]["RPM"].as_f64().map(|v| v as f32);
    let horse_power = json["Engine0"]["horsePowers"].as_f64().map(|v| v as f32);
    
    // Vehicle type
    let vehicle_type = identifier.split('_').next()
        .unwrap_or("unknown")
        .to_string();
    
    Ok(AircraftLimits {
        identifier: identifier.clone(),
        display_name: identifier.replace('_', " ").replace('-', " "),
        vne_kmh: vne,
        vne_mach,
        max_speed_ground,
        stall_speed,
        flutter_speed,
        mass_kg,
        wing_overload_pos_n: wing_pos_n,
        wing_overload_neg_n: wing_neg_n,
        max_positive_g,
        max_negative_g,
        max_rpm,
        horse_power,
        vehicle_type,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

