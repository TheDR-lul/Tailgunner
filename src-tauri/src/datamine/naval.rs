/// Naval vessel data extractor

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all ships from game files
pub fn parse_naval(game_path: &PathBuf) -> Result<Vec<ShipLimits>> {
    let ship_path = game_path
        .join("aces.vromfs.bin_u")
        .join("gamedata")
        .join("units")
        .join("ships");
    
    if !ship_path.exists() {
        return Err(anyhow::anyhow!("Ships directory not found: {:?}", ship_path));
    }
    
    let mut ships = Vec::new();
    
    for entry in fs::read_dir(ship_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("blkx") {
            continue;
        }
        
        // Skip non-ship files
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if filename.starts_with("fishboat") || 
           filename.starts_with("cargo") || 
           filename.starts_with("debris") {
            continue;
        }
        
        match parse_ship_file(&path) {
            Ok(limits) => ships.push(limits),
            Err(e) => {
                log::warn!("[Naval] Failed to parse {:?}: {}", path.file_name(), e);
            }
        }
    }
    
    log::info!("[Naval] Parsed {} ships", ships.len());
    Ok(ships)
}

/// Parse single ship file
fn parse_ship_file(path: &PathBuf) -> Result<ShipLimits> {
    let json = parser::read_json_file(path)?;
    let identifier = parser::extract_identifier(path)
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    
    // Speed (in knots)
    let max_speed_knots = json["maxFwdSpeed"].as_f64().unwrap_or(30.0) as f32;
    let max_reverse_speed_knots = json["maxRevSpeed"].as_f64().unwrap_or(5.0) as f32;
    
    // Extract compartments from DamageParts
    let compartments = extract_compartments(&json);
    
    // Ship class
    let ship_class = json["type"].as_str()
        .or(json["subclass"].as_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(ShipLimits {
        identifier: identifier.clone(),
        display_name: identifier.replace('_', " "),
        max_speed_knots,
        max_reverse_speed_knots,
        compartments,
        ship_class,
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

/// Extract compartments from DamageParts section
fn extract_compartments(json: &serde_json::Value) -> Vec<Compartment> {
    let mut compartments = Vec::new();
    
    if let Some(damage_parts) = json["DamageParts"].as_object() {
        // Crew compartments
        if let Some(crew) = damage_parts["crew_compartments"].as_object() {
            for (key, value) in crew {
                if key.ends_with("_dm") {
                    if let Some(hp) = value["hp"].as_f64() {
                        compartments.push(Compartment {
                            name: key.clone(),
                            hp: hp as f32,
                            critical: false,
                        });
                    }
                }
            }
        }
        
        // Engine rooms (critical)
        if let Some(engines) = damage_parts.get("engines") {
            if let Some(obj) = engines.as_object() {
                for (key, value) in obj {
                    if key.ends_with("_dm") {
                        if let Some(hp) = value["hp"].as_f64() {
                            compartments.push(Compartment {
                                name: key.clone(),
                                hp: hp as f32,
                                critical: true,  // Engine critical!
                            });
                        }
                    }
                }
            }
        }
        
        // Ammo storage (critical)
        if let Some(ammo) = damage_parts.get("ammo_storages_shells") {
            if let Some(obj) = ammo.as_object() {
                for (key, value) in obj {
                    if key.contains("ammunition") {
                        if let Some(hp) = value["hp"].as_f64() {
                            compartments.push(Compartment {
                                name: key.clone(),
                                hp: hp as f32,
                                critical: true,  // Ammo explosion!
                            });
                        }
                    }
                }
            }
        }
    }
    
    compartments
}

