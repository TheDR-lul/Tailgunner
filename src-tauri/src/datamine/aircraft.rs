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
    // VNE: Try WingPlaneSweep* (F-14 has 0/1/2), then WingPlane (JAS39), then VneControl
    // For swept wing aircraft, collect all VNE values and use min/max as range
    let mut vne_values: Vec<f32> = Vec::new();
    
    // Check all WingPlaneSweep positions (0, 1, 2, ...)
    if let Some(aero) = json.get("Aerodynamics").and_then(|a| a.as_object()) {
        for i in 0..10 {  // Check up to 10 positions
            let key = format!("WingPlaneSweep{}", i);
            if let Some(vne) = aero.get(&key)
                .and_then(|w| w.get("Strength"))
                .and_then(|s| s.get("VNE"))
                .and_then(|v| v.as_f64()) {
                vne_values.push(vne as f32);
            }
        }
    }
    
    let (vne, vne_kmh_max) = if !vne_values.is_empty() {
        // Found swept wing VNE values
        let min_vne = vne_values.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_vne = vne_values.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        
        if (max_vne - min_vne).abs() > 10.0 {
            log::info!("[Aircraft] {} - VNE range (swept wing): {} - {} km/h", identifier, min_vne, max_vne);
            (min_vne, Some(max_vne))
        } else {
            log::info!("[Aircraft] {} - VNE single value: {} km/h", identifier, min_vne);
            (min_vne, None)
        }
    } else {
        // Fallback to standard paths
        let vne_val = json.get("Aerodynamics")
            .and_then(|a| a.get("WingPlane"))
            .and_then(|w| w.get("Strength"))
            .and_then(|s| s.get("VNE"))
            .or_else(|| json.get("VneControl"))
            .or_else(|| json.get("Vne"));
        
        if let Some(val) = vne_val.and_then(|v| v.as_f64()) {
            log::info!("[Aircraft] {} - VNE single value: {} km/h", identifier, val);
            (val as f32, None)
        } else {
            log::warn!("[Aircraft] {} - VNE not found, using fallback 800 km/h", identifier);
            (800.0, None)
        }
    };
    
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
    // If not found anywhere - return None (will show "N/A" in UI)
    let wing_neg_n_opt = json.get("Aerodynamics")
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
        .map(|v| v as f32);
    
    let wing_pos_n_opt = json.get("Aerodynamics")
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
        .map(|v| v as f32);
    
    // Calculate G-limits only if CritOverload data exists
    let (max_positive_g, max_negative_g) = match (wing_pos_n_opt, wing_neg_n_opt) {
        (Some(wing_pos_n), Some(wing_neg_n)) => (
            Some(AircraftLimits::calculate_g_limits(wing_pos_n, mass_kg)),
            Some(AircraftLimits::calculate_g_limits(wing_neg_n, mass_kg)),
        ),
        _ => (None, None), // Data not available - will show "N/A"
    };
    
    // Flutter speed (estimate if not provided)
    let flutter_speed = Some(AircraftLimits::estimate_flutter_speed(vne));
    
    // Gear and flaps speed limits (from Mass section)
    let gear_max_speed_kmh = json.get("Mass")
        .and_then(|m| m.get("GearDestructionIndSpeed"))
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    
    // Flap speed limits - ALL positions (L/T/C/...)
    // Multiple values for different flap positions
    // Can check current position via telemetry (localhost:8111)
    let flaps_speeds_kmh: Vec<f32> = json.get("Mass")
        .and_then(|m| m.get("FlapsDestructionIndSpeedP"))
        .and_then(|arr| arr.as_array())
        .map(|arr| {
            let mut speeds: Vec<f32> = arr.iter()
                .filter_map(|v| v.as_f64())
                .map(|v| v as f32)
                .filter(|&v| v >= 50.0)  // Filter out junk values (0, 1, etc)
                .collect();
            
            // Remove duplicates and sort
            speeds.sort_by(|a, b| a.partial_cmp(b).unwrap());
            speeds.dedup();
            speeds
        })
        .unwrap_or_default();
    
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
        vne_kmh_max,  // Range for swept wing aircraft
        max_speed_ground,
        stall_speed,
        flutter_speed,
        gear_max_speed_kmh,
        flaps_speeds_kmh,  // All flap positions
        mass_kg,
        wing_overload_pos_n: wing_pos_n_opt,  // Option<f32> - None if not found
        wing_overload_neg_n: wing_neg_n_opt,  // Option<f32> - None if not found
        max_positive_g,  // Option<f32> - None if CritOverload not available
        max_negative_g,  // Option<f32> - None if CritOverload not available
        max_rpm,
        horse_power,
        vehicle_type,
        data_source: "datamine".to_string(),  // Wiki data will be fetched on-demand
        last_updated: chrono::Utc::now().to_rfc3339(),
    })
}

