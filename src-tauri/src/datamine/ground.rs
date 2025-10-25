/// Ground vehicle data extractor

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all ground vehicles from game files
pub fn parse_ground(game_path: &PathBuf) -> Result<Vec<GroundLimits>> {
    // Try multiple possible paths
    let possible_paths = vec![
        game_path.join("gamedata").join("units").join("tankmodels"),
        game_path.join("aces.vromfs.bin_u").join("gamedata").join("units").join("tankmodels"),
    ];
    
    let tank_path = possible_paths.into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("Tank models directory not found in: {:?}", game_path))?;
    
    log::debug!("[Ground] Using path: {:?}", tank_path);
    
    let mut vehicles = Vec::new();
    
    for entry in fs::read_dir(tank_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // wt_blk unpacks as .blk (not .blkx!)
        if path.extension().and_then(|s| s.to_str()) != Some("blk") {
            continue;
        }
        
        match parse_ground_file(&path) {
            Ok(limits) => vehicles.push(limits),
            Err(e) => {
                log::info!("[Ground] ⚠️ Failed to parse {:?}: {}", path.file_name(), e);
            }
        }
    }
    
    log::info!("[Ground] Parsed {} ground vehicles", vehicles.len());
    Ok(vehicles)
}

/// Parse single ground vehicle file
fn parse_ground_file(path: &PathBuf) -> Result<GroundLimits> {
    let json = parser::read_json_file(path)?;
    let identifier = parser::extract_identifier(path)
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    
    // Speed
    let max_speed_kmh = json.get("maxFwdSpeed").and_then(|v| v.as_f64()).unwrap_or(50.0) as f32;
    let max_reverse_speed_kmh = json.get("maxRevSpeed").and_then(|v| v.as_f64()).unwrap_or(10.0) as f32;
    
    // Mass
    let mass_kg = json.get("mass").and_then(|v| v.as_f64()).unwrap_or(30000.0) as f32;
    
    // Engine data
    let horse_power = json.get("engine")
        .and_then(|e| e.get("horsePowers"))
        .and_then(|v| v.as_f64())
        .unwrap_or(500.0) as f32;
    let max_rpm = json.get("engine")
        .and_then(|e| e.get("maxRPM"))
        .and_then(|v| v.as_f64())
        .unwrap_or(2500.0) as f32;
    let min_rpm = json.get("engine")
        .and_then(|e| e.get("minRPM"))
        .and_then(|v| v.as_f64())
        .unwrap_or(600.0) as f32;
    
    // HP (hull points)
    let hull_hp = json.get("DamageParts")
        .and_then(|d| d.get("hp"))
        .and_then(|v| v.as_f64())
        .unwrap_or(5000.0) as f32;
    
    // Armor (try to get from first body part)
    let armor_thickness_mm = json.get("DamageParts")
        .and_then(|d| d.get("hull"))
        .and_then(|h| h.get("body_front_dm"))
        .and_then(|b| b.get("armorThickness"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Vehicle type
    let vehicle_type = json.get("subclass")
        .and_then(|v| v.as_str())
        .unwrap_or("mediumVehicle")
        .to_string();
    
    Ok(GroundLimits {
        identifier: identifier.clone(),
        display_name: identifier.replace('_', " "),
        max_speed_kmh,
        max_reverse_speed_kmh,
        mass_kg,
        horse_power,
        max_rpm,
        min_rpm,
        hull_hp,
        armor_thickness_mm,
        vehicle_type,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

