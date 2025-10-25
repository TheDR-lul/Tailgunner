/// Ground vehicle data extractor

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all ground vehicles from game files (without Wiki)
pub async fn parse_ground(game_path: &PathBuf) -> Result<Vec<GroundLimits>> {
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
        
        match parse_ground_file(&path).await {
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
async fn parse_ground_file(path: &PathBuf) -> Result<GroundLimits> {
    let json = parser::read_json_file(path)?;
    let identifier = parser::extract_identifier(path)
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    
    // Speed - NOT stored in game files (AI pathfinding values)
    // Will be fetched from Wiki on-demand when vehicle is selected
    let max_speed_kmh = None;
    let max_reverse_speed_kmh = None;
    
    // Mass - base weight (empty, without fuel/ammo)
    let mass_kg = json.get("mass")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Engine data
    let horse_power = json.get("VehiclePhys")
        .and_then(|v| v.get("engine"))
        .and_then(|e| e.get("horsePowers"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    let max_rpm = json.get("VehiclePhys")
        .and_then(|v| v.get("engine"))
        .and_then(|e| e.get("maxRPM"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    let min_rpm = json.get("VehiclePhys")
        .and_then(|v| v.get("engine"))
        .and_then(|e| e.get("minRPM"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Crew HP
    let crew_hp = json.get("DamageParts")
        .and_then(|d| d.get("crew"))
        .and_then(|c| c.get("hp"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Crew count
    let crew_count = json.get("tank_crew")
        .and_then(|tc| tc.as_object())
        .map(|obj| obj.keys().filter(|k| *k != "changeTimeMult").count() as u8);
    
    // Weapon data
    let (main_gun_caliber_mm, main_gun_fire_rate, ammo_count) = extract_weapon_data(&json);
    
    // Transmission - parse gears from VehiclePhys.mechanics.gearRatios
    let (forward_gears, reverse_gears) = extract_gears(&json);
    
    // Vehicle type
    let vehicle_type = json.get("subclass")
        .and_then(|v| v.as_str())
        .unwrap_or("mediumVehicle")
        .to_string();
    
    // Wiki data will be fetched on-demand when vehicle is selected
    let data_source = "datamine".to_string();
    
    Ok(GroundLimits {
        identifier: identifier.clone(),
        display_name: identifier.replace('_', " "),
        max_speed_kmh,
        max_reverse_speed_kmh,
        mass_kg,
        horse_power,
        max_rpm,
        min_rpm,
        crew_hp,
        crew_count,
        main_gun_caliber_mm,
        main_gun_fire_rate,
        ammo_count,
        forward_gears,
        reverse_gears,
        vehicle_type,
        data_source,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

/// Extract gear counts from VehiclePhys.mechanics.gearRatios
fn extract_gears(json: &serde_json::Value) -> (Option<u8>, Option<u8>) {
    let ratios = match json.get("VehiclePhys")
        .and_then(|v| v.get("mechanics"))
        .and_then(|m| m.get("gearRatios"))
        .and_then(|g| g.get("ratio"))
        .and_then(|r| r.as_array())
    {
        Some(r) => r,
        None => return (None, None),
    };
    
    let mut reverse = 0u8;
    let mut forward = 0u8;
    
    for ratio in ratios {
        if let Some(val) = ratio.as_f64() {
            if val < 0.0 {
                reverse += 1;
            } else if val > 0.0 {
                forward += 1;
            }
            // Skip neutral (0.0)
        }
    }
    
    if forward > 0 || reverse > 0 {
        (Some(forward), Some(reverse))
    } else {
        (None, None)
    }
}

/// Extract weapon data (caliber, fire rate, ammo) from commonWeapons
fn extract_weapon_data(json: &serde_json::Value) -> (Option<f32>, Option<f32>, Option<u32>) {
    let weapons = match json.get("commonWeapons").and_then(|w| w.get("Weapon")).and_then(|w| w.as_array()) {
        Some(w) => w,
        None => return (None, None, None),
    };
    
    // Find first cannon (main gun)
    for weapon in weapons {
        if let Some(blk) = weapon.get("blk").and_then(|v| v.as_str()) {
            // Extract caliber from blk path (e.g., "120mm_kan_Strv_122_user_cannon.blk" -> 120)
            let caliber = blk.split('/').last()
                .and_then(|filename| filename.split('_').next())
                .and_then(|cal_str| cal_str.replace("mm", "").parse::<f32>().ok());
            
            // Extract fire rate (shotFreq in Hz -> shots/sec)
            let fire_rate = weapon.get("shotFreq").and_then(|v| v.as_f64()).map(|v| v as f32);
            
            // Extract ammo count
            let ammo = weapon.get("bullets").and_then(|v| v.as_f64()).map(|v| v as u32);
            
            // If we found a cannon, return its data
            if caliber.is_some() && caliber.unwrap() > 20.0 {
                return (caliber, fire_rate, ammo);
            }
        }
    }
    
    (None, None, None)
}

