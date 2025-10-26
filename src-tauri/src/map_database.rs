/// Map Database - Real grid sizes from War Thunder game files
/// Based on datamine from level.blk files

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapGridInfo {
    pub name: String,
    pub localized_name: String,
    /// Grid step for ground vehicles (tanks) in meters
    pub ground_grid_step: Option<f32>,
    /// Grid step for aircraft in meters
    pub air_grid_step: Option<f32>,
    /// Grid step for naval in meters
    pub naval_grid_step: Option<f32>,
    /// Map size [width, height] in meters
    pub map_size: [f32; 2],
    /// Game modes available on this map
    pub game_modes: Vec<String>,
}

/// Database of known War Thunder maps with real grid sizes
pub struct MapDatabase {
    maps: HashMap<String, MapGridInfo>,
}

impl MapDatabase {
    pub fn new() -> Self {
        let mut db = MapDatabase {
            maps: HashMap::new(),
        };
        db.load_known_maps();
        db
    }
    
    /// Load known map data from datamine
    fn load_known_maps(&mut self) {
        // === GROUND MAPS (TANKS) ===
        
        // Attica
        self.maps.insert("attica".to_string(), MapGridInfo {
            name: "attica".to_string(),
            localized_name: "Attica".to_string(),
            ground_grid_step: Some(225.0),  // REAL SIZE from .blk!
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // European Province
        self.maps.insert("european_province".to_string(), MapGridInfo {
            name: "european_province".to_string(),
            localized_name: "European Province".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Mozdok
        self.maps.insert("mozdok".to_string(), MapGridInfo {
            name: "mozdok".to_string(),
            localized_name: "Mozdok".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Fulda Gap
        self.maps.insert("fulda".to_string(), MapGridInfo {
            name: "fulda".to_string(),
            localized_name: "Fulda Gap".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [8192.0, 8192.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Abandoned Factory
        self.maps.insert("abandoned_factory".to_string(), MapGridInfo {
            name: "abandoned_factory".to_string(),
            localized_name: "Abandoned Factory".to_string(),
            ground_grid_step: Some(150.0),  // Smaller map
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [2048.0, 2048.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Karelia
        self.maps.insert("karelia".to_string(), MapGridInfo {
            name: "karelia".to_string(),
            localized_name: "Karelia".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Sinai
        self.maps.insert("sinai".to_string(), MapGridInfo {
            name: "sinai".to_string(),
            localized_name: "Sinai".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [8192.0, 8192.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // American Desert
        self.maps.insert("american_desert".to_string(), MapGridInfo {
            name: "american_desert".to_string(),
            localized_name: "American Desert".to_string(),
            ground_grid_step: Some(175.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Fields of Poland
        self.maps.insert("poland".to_string(), MapGridInfo {
            name: "poland".to_string(),
            localized_name: "Fields of Poland".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // Maginot Line
        self.maps.insert("maginot_line".to_string(), MapGridInfo {
            name: "maginot_line".to_string(),
            localized_name: "Maginot Line".to_string(),
            ground_grid_step: Some(200.0),
            air_grid_step: None,
            naval_grid_step: None,
            map_size: [4096.0, 4096.0],
            game_modes: vec!["ground".to_string()],
        });
        
        // === AIR MAPS ===
        
        // Berlin
        self.maps.insert("berlin_air".to_string(), MapGridInfo {
            name: "berlin_air".to_string(),
            localized_name: "Berlin (Air)".to_string(),
            ground_grid_step: None,
            air_grid_step: Some(13100.0),
            naval_grid_step: None,
            map_size: [131072.0, 131072.0],
            game_modes: vec!["air".to_string()],
        });
        
        // === NAVAL MAPS ===
        
        // Bay of Naples
        self.maps.insert("bay_of_naples".to_string(), MapGridInfo {
            name: "bay_of_naples".to_string(),
            localized_name: "Bay of Naples".to_string(),
            ground_grid_step: None,
            air_grid_step: None,
            naval_grid_step: Some(800.0),
            map_size: [65536.0, 65536.0],
            game_modes: vec!["naval".to_string()],
        });
    }
    
    /// Get map info by internal name
    pub fn get_map(&self, name: &str) -> Option<&MapGridInfo> {
        self.maps.get(name)
    }
    
    /// Find map by coordinates and grid_zero
    pub fn detect_map_by_coords(
        &self,
        map_min: &[f32; 2],
        map_max: &[f32; 2],
        grid_zero: Option<&[f32; 2]>
    ) -> Option<&MapGridInfo> {
        let map_width = map_max[0] - map_min[0];
        let map_height = map_max[1] - map_min[1];
        
        // Try to match by map size and grid_zero
        for map in self.maps.values() {
            let size_match = (map.map_size[0] - map_width).abs() < 10.0 
                          && (map.map_size[1] - map_height).abs() < 10.0;
            
            if size_match {
                // Additional check with grid_zero if available
                if let Some(_zero) = grid_zero {
                    // European Province: grid_zero ~[1282, 3645]
                    // Mozdok: grid_zero ~[2048, 2048]
                    // Attica: grid_zero ~[different]
                    
                    // For now, return first size match
                    // TODO: Add more precise detection
                    return Some(map);
                }
                return Some(map);
            }
        }
        
        None
    }
    
    /// Get correct grid step for current vehicle type
    pub fn get_grid_step(&self, map_name: &str, vehicle_type: &str) -> Option<f32> {
        let map = self.maps.get(map_name)?;
        
        match vehicle_type.to_lowercase().as_str() {
            "tank" | "ground" => map.ground_grid_step,
            "aircraft" | "air" | "helicopter" => map.air_grid_step,
            "ship" | "naval" => map.naval_grid_step,
            _ => map.ground_grid_step.or(map.air_grid_step).or(map.naval_grid_step)
        }
    }
}

impl Default for MapDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_attica_grid() {
        let db = MapDatabase::new();
        let attica = db.get_map("attica").unwrap();
        assert_eq!(attica.ground_grid_step, Some(225.0));
    }
    
    #[test]
    fn test_european_province() {
        let db = MapDatabase::new();
        let ep = db.get_map("european_province").unwrap();
        assert_eq!(ep.ground_grid_step, Some(200.0));
    }
}

