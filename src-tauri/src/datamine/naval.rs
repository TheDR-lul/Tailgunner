/// Naval vessel data extractor

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all ships from game files
pub fn parse_naval(game_path: &PathBuf) -> Result<Vec<ShipLimits>> {
    // Try multiple possible paths
    let possible_paths = vec![
        game_path.join("gamedata").join("units").join("ships"),
        game_path.join("aces.vromfs.bin_u").join("gamedata").join("units").join("ships"),
    ];
    
    let ship_path = possible_paths.into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("Ships directory not found in: {:?}", game_path))?;
    
    log::debug!("[Naval] Using path: {:?}", ship_path);
    
    let mut ships = Vec::new();
    let mut total_files = 0;
    let mut skipped_files = 0;
    let mut failed_files = 0;
    
    for entry in fs::read_dir(ship_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // wt_blk unpacks as .blk (not .blkx!)
        if path.extension().and_then(|s| s.to_str()) != Some("blk") {
            continue;
        }
        
        total_files += 1;
        
        // Skip non-ship files
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if filename.starts_with("fishboat") || 
           filename.starts_with("cargo") || 
           filename.starts_with("debris") ||
           filename.starts_with("float_") ||
           filename.starts_with("buoy_") {
            skipped_files += 1;
            continue;
        }
        
        match parse_ship_file(&path) {
            Ok(limits) => {
                ships.push(limits);
                if ships.len() % 100 == 0 {
                    log::debug!("[Naval] Progress: {} ships parsed", ships.len());
                }
            }
            Err(e) => {
                failed_files += 1;
                // Log first 5 failures in detail
                if failed_files <= 5 {
                    log::info!("[Naval] ⚠️ Failed to parse {:?}: {}", path.file_name(), e);
                }
            }
        }
    }
    
    log::info!("[Naval] Parsed {} ships (total: {}, skipped: {}, failed: {})", 
        ships.len(), total_files, skipped_files, failed_files);
    Ok(ships)
}

/// Parse single ship file
fn parse_ship_file(path: &PathBuf) -> Result<ShipLimits> {
    let json = parser::read_json_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to read JSON: {}", e))?;
    
    let identifier = parser::extract_identifier(path)
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    
    // Speed (in knots)
    let max_speed_knots = json.get("maxFwdSpeed")
        .and_then(|v| v.as_f64())
        .unwrap_or(30.0) as f32;
    let max_reverse_speed_knots = json.get("maxRevSpeed")
        .and_then(|v| v.as_f64())
        .unwrap_or(5.0) as f32;
    
    // Extract compartments from DamageParts
    let compartments = extract_compartments(&json);
    
    // Ship class
    let ship_class = json.get("type")
        .and_then(|v| v.as_str())
        .or_else(|| json.get("subclass").and_then(|v| v.as_str()))
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
    
    if let Some(damage_parts) = json.get("DamageParts").and_then(|v| v.as_object()) {
        // Crew compartments
        if let Some(crew) = damage_parts.get("crew_compartments").and_then(|v| v.as_object()) {
            for (key, value) in crew {
                if key.ends_with("_dm") {
                    if let Some(hp) = value.get("hp").and_then(|v| v.as_f64()) {
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
                        if let Some(hp) = value.get("hp").and_then(|v| v.as_f64()) {
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
                        if let Some(hp) = value.get("hp").and_then(|v| v.as_f64()) {
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

