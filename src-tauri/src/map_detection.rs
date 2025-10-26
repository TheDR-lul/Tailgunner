/// Map Detection System
/// Identifies maps by their coordinates since WT API doesn't provide map names

use serde::{Serialize, Deserialize};
use crate::map_database::MapDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapIdentity {
    pub name: String,
    pub localized_name: String,  // For different languages
    pub game_mode: String,        // "air", "ground", "naval"
}

/// Detect map by its coordinate boundaries
/// First tries database, then falls back to heuristics
pub fn detect_map_by_coordinates(
    map_min: &[f32; 2],
    map_max: &[f32; 2],
    grid_zero: Option<&[f32; 2]>
) -> Option<MapIdentity> {
    // Try database first
    let db = MapDatabase::new();
    if let Some(map_info) = db.detect_map_by_coords(map_min, map_max, grid_zero) {
        return Some(MapIdentity {
            name: map_info.name.clone(),
            localized_name: map_info.localized_name.clone(),
            game_mode: map_info.game_modes.first()
                .unwrap_or(&"unknown".to_string())
                .clone(),
        });
    }
    
    // Fallback to heuristic detection
    // Maps are identified by their unique coordinate bounds
    // These values are extracted from multiple game sessions
    
    let map_width = map_max[0] - map_min[0];
    let map_height = map_max[1] - map_min[1];
    
    // Most WT maps are 4096x4096 or 8192x8192
    // Use grid_zero as unique identifier when available
    
    // === GROUND MAPS ===
    if let Some(zero) = grid_zero {
        // European Province (common tank map)
        if (zero[0] - 1282.0).abs() < 10.0 && (zero[1] - 3645.0).abs() < 10.0 {
            return Some(MapIdentity {
                name: "european_province".to_string(),
                localized_name: "European Province".to_string(),
                game_mode: "ground".to_string(),
            });
        }
        
        // Mozdok
        if (zero[0] - 2048.0).abs() < 10.0 && (zero[1] - 2048.0).abs() < 10.0 
            && (map_width - 4096.0).abs() < 10.0 {
            return Some(MapIdentity {
                name: "mozdok".to_string(),
                localized_name: "Mozdok".to_string(),
                game_mode: "ground".to_string(),
            });
        }
        
        // Fulda Gap
        if (zero[0] - 4096.0).abs() < 10.0 && (zero[1] - 4096.0).abs() < 10.0 {
            return Some(MapIdentity {
                name: "fulda".to_string(),
                localized_name: "Fulda Gap".to_string(),
                game_mode: "ground".to_string(),
            });
        }
    }
    
    // === AIR MAPS ===
    // Air maps usually have much larger boundaries
    if map_width > 50000.0 || map_height > 50000.0 {
        return Some(MapIdentity {
            name: "air_unknown".to_string(),
            localized_name: "Air Map".to_string(),
            game_mode: "air".to_string(),
        });
    }
    
    // === NAVAL MAPS ===
    if map_width > 20000.0 && map_width < 50000.0 {
        return Some(MapIdentity {
            name: "naval_unknown".to_string(),
            localized_name: "Naval Map".to_string(),
            game_mode: "naval".to_string(),
        });
    }
    
    // Unknown map
    None
}

/// Get map name display string
pub fn get_map_display_name(
    map_min: &[f32; 2],
    map_max: &[f32; 2],
    grid_zero: Option<&[f32; 2]>
) -> String {
    if let Some(identity) = detect_map_by_coordinates(map_min, map_max, grid_zero) {
        identity.localized_name
    } else {
        format!("Unknown Map ({}x{})", 
            (map_max[0] - map_min[0]) as i32,
            (map_max[1] - map_min[1]) as i32
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_european_province_detection() {
        let map_min = [0.0, 0.0];
        let map_max = [4096.0, 4096.0];
        let grid_zero = [1282.16, 3645.10];
        
        let identity = detect_map_by_coordinates(&map_min, &map_max, Some(&grid_zero));
        assert!(identity.is_some());
        assert_eq!(identity.unwrap().name, "european_province");
    }
}

