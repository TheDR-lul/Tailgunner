/// Aircraft data extractor from flight model files

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::datamine::{parser, types::*};

/// Parse all aircraft from game files
pub fn parse_aircraft(game_path: &PathBuf) -> Result<Vec<AircraftLimits>> {
    // Try multiple possible paths (depending on how data was prepared)
    let possible_paths = vec![
        game_path.join("gamedata").join("flightmodels").join("fm"),  // If game_path already points to unpacked folder
        game_path.join("aces.vromfs.bin_u").join("gamedata").join("flightmodels").join("fm"),  // If game_path is game root
    ];
    
    let fm_path = possible_paths.into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow::anyhow!("Flight models directory not found in: {:?}", game_path))?;
    
    log::debug!("[Aircraft] Using path: {:?}", fm_path);
    
    let mut aircraft = Vec::new();
    
    for entry in fs::read_dir(fm_path)? {
        let entry = entry?;
        let path = entry.path();
        
        // wt_blk unpacks as .blk (not .blkx!)
        if path.extension().and_then(|s| s.to_str()) != Some("blk") {
            continue;
        }
        
        match parse_aircraft_file(&path) {
            Ok(limits) => aircraft.push(limits),
            Err(e) => {
                log::info!("[Aircraft] ⚠️ Failed to parse {:?}: {}", path.file_name(), e);
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
    // VNE: Try WingPlaneSweep0 (F-14), then WingPlane (JAS39), then VneControl, then root Vne
    let vne = json.get("Aerodynamics")
        .and_then(|a| a.get("WingPlaneSweep0"))
        .and_then(|w| w.get("Strength"))
        .and_then(|s| s.get("VNE"))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("Aerodynamics")
                .and_then(|a| a.get("WingPlane"))
                .and_then(|w| w.get("Strength"))
                .and_then(|s| s.get("VNE"))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| json.get("VneControl").and_then(|v| v.as_f64()))
        .or_else(|| json.get("Vne").and_then(|v| v.as_f64()))
        .unwrap_or(800.0) as f32;
    
    let vne_mach = json.get("Aerodynamics")
        .and_then(|a| a.get("WingPlaneSweep0"))
        .and_then(|w| w.get("Strength"))
        .and_then(|s| s.get("MNE"))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("Aerodynamics")
                .and_then(|a| a.get("WingPlane"))
                .and_then(|w| w.get("Strength"))
                .and_then(|s| s.get("MNE"))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| json.get("VneMach").and_then(|v| v.as_f64()))
        .map(|v| v as f32);
    
    let max_speed_ground = json.get("MaxSpeedNearGround")
        .and_then(|v| v.as_f64())
        .unwrap_or(600.0) as f32;
    
    // Stall speed - try multiple locations:
    // 1. Passport.Alt.stallSpeed[1] (modern format)
    // 2. MinimalSpeed (older format)
    let stall_speed = json.get("Passport")
        .and_then(|p| p.get("Alt"))
        .and_then(|alt| alt.get("stallSpeed"))
        .and_then(|arr| arr.as_array())
        .and_then(|arr| arr.get(1))  // [altitude, speed] -> take speed
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("MinimalSpeed")
                .and_then(|v| v.as_f64())
        })
        .unwrap_or(150.0) as f32;
    
    // Mass data - prefer EmptyMass for G-load calculation, fallback to Takeoff or EmptyMass + fuel
    let mass_kg = json.get("Mass")
        .and_then(|m| m.get("EmptyMass"))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("Mass")
                .and_then(|m| m.get("Takeoff"))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| {
            // Calculate from EmptyMass + MaxFuelMass
            let empty = json.get("Mass")?.get("EmptyMass")?.as_f64()?;
            let fuel = json.get("Mass")?.get("MaxFuelMass0")?.as_f64()?;
            Some(empty + fuel)
        })
        .unwrap_or(3000.0) as f32;
    
    // Wing overload (Newtons) - try four locations:
    // 1. Aerodynamics.WingPlaneSweep0.Strength.CritOverload (swept-wing jets like F-14)
    // 2. Aerodynamics.WingPlane.Strength.CritOverload (modern jets like JAS39)
    // 3. Strength.CritOverload (older jets)
    // 4. WingCritOverload (older aircraft like BF-109)
    let wing_neg_n = json.get("Aerodynamics")
        .and_then(|a| a.get("WingPlaneSweep0"))
        .and_then(|w| w.get("Strength"))
        .and_then(|s| s.get("CritOverload"))
        .and_then(|arr| arr.get(0))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("Aerodynamics")
                .and_then(|a| a.get("WingPlane"))
                .and_then(|w| w.get("Strength"))
                .and_then(|s| s.get("CritOverload"))
                .and_then(|arr| arr.get(0))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| {
            json.get("Strength")
                .and_then(|s| s.get("CritOverload"))
                .and_then(|arr| arr.get(0))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| {
            json.get("WingCritOverload")
                .and_then(|arr| arr.get(0))
                .and_then(|v| v.as_f64())
        })
        .unwrap_or(-100000.0) as f32;
    
    let wing_pos_n = json.get("Aerodynamics")
        .and_then(|a| a.get("WingPlaneSweep0"))
        .and_then(|w| w.get("Strength"))
        .and_then(|s| s.get("CritOverload"))
        .and_then(|arr| arr.get(1))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            json.get("Aerodynamics")
                .and_then(|a| a.get("WingPlane"))
                .and_then(|w| w.get("Strength"))
                .and_then(|s| s.get("CritOverload"))
                .and_then(|arr| arr.get(1))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| {
            json.get("Strength")
                .and_then(|s| s.get("CritOverload"))
                .and_then(|arr| arr.get(1))
                .and_then(|v| v.as_f64())
        })
        .or_else(|| {
            json.get("WingCritOverload")
                .and_then(|arr| arr.get(1))
                .and_then(|v| v.as_f64())
        })
        .unwrap_or(200000.0) as f32;
    
    let max_positive_g = AircraftLimits::calculate_g_limits(wing_pos_n, mass_kg);
    let max_negative_g = AircraftLimits::calculate_g_limits(wing_neg_n, mass_kg); // Keep negative!
    
    // DEBUG: Log G-load calculation for buccaneer_s1
    if identifier == "buccaneer_s1" {
        log::warn!("[Aircraft] 🔍 BUCCANEER S1 - G-load calculation:");
        log::warn!("[Aircraft]   Mass (EmptyMass): {} kg", mass_kg);
        log::warn!("[Aircraft]   CritOverload: [{}, {}] N", wing_neg_n, wing_pos_n);
        log::warn!("[Aircraft]   Calculated G-Load: +{:.1}G / {:.1}G", max_positive_g, max_negative_g);
        log::warn!("[Aircraft]   Aerodynamics exists: {}", json.get("Aerodynamics").is_some());
        
        // Dump ALL Aerodynamics keys for old aircraft
        if let Some(aero) = json.get("Aerodynamics") {
            if let Some(obj) = aero.as_object() {
                let keys: Vec<&String> = obj.keys().collect();
                log::warn!("[Aircraft]   Aerodynamics keys ({} total): {:?}", keys.len(), keys);
                
                // Check for Wing0, Wing1, Wing2 (old format)
                for i in 0..3 {
                    let key = format!("Wing{}", i);
                    if let Some(wing) = aero.get(&key) {
                        log::warn!("[Aircraft]   {} exists!", key);
                        if let Some(wing_obj) = wing.as_object() {
                            log::warn!("[Aircraft]     {}.keys: {:?}", key, wing_obj.keys().collect::<Vec<_>>());
                            if let Some(strength) = wing.get("Strength") {
                                log::warn!("[Aircraft]     {}.Strength: {:?}", key, strength);
                            }
                        }
                    }
                }
                
                // Check Fuselage, Stab, Fin for Strength/CritOverload
                for part in ["Fuselage", "Stab", "Fin"] {
                    if let Some(part_obj) = aero.get(part) {
                        log::warn!("[Aircraft]   {} exists!", part);
                        if let Some(obj) = part_obj.as_object() {
                            log::warn!("[Aircraft]     {}.keys: {:?}", part, obj.keys().collect::<Vec<_>>());
                            if let Some(strength) = part_obj.get("Strength") {
                                log::warn!("[Aircraft]     {}.Strength: {:?}", part, strength);
                            }
                            if let Some(crit) = part_obj.get("CritOverload") {
                                log::warn!("[Aircraft]     {}.CritOverload: {:?}", part, crit);
                            }
                        }
                    }
                }
            }
        }
        
        // Check Passport.IAS for stall speed
        if let Some(passport) = json.get("Passport") {
            if let Some(ias) = passport.get("IAS") {
                log::warn!("[Aircraft]   Passport.IAS: {:?}", ias);
            }
            if let Some(alt) = passport.get("Alt") {
                log::warn!("[Aircraft]   Passport.Alt: {:?}", alt);
            }
        }
    }
    
    // Flutter speed (estimate if not provided)
    let flutter_speed = Some(AircraftLimits::estimate_flutter_speed(vne));
    
    // Gear and flaps speed limits (from Mass section)
    let gear_max_speed_kmh = json.get("Mass")
        .and_then(|m| m.get("GearDestructionIndSpeed"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    let flaps_max_speed_kmh = json.get("Mass")
        .and_then(|m| m.get("FlapsDestructionIndSpeedP"))
        .and_then(|arr| arr.as_array())
        .and_then(|arr| arr.first())  // Take first flap position limit
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Engine data (optional)
    let max_rpm = json.get("Engine0")
        .and_then(|e| e.get("RPM"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    let horse_power = json.get("Engine0")
        .and_then(|e| e.get("horsePowers"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
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
        gear_max_speed_kmh,
        flaps_max_speed_kmh,
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

