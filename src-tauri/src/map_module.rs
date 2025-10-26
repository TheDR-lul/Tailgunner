/// Map data module for War Thunder API
/// Provides map objects, boundaries, and player positions
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub color: String,
    #[serde(rename = "color[]")]
    pub color_rgb: Vec<u8>,
    pub blink: u8,
    pub icon: String,
    pub icon_bg: String,
    
    // Position fields
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    
    // Direction (for player/respawn points)
    #[serde(default)]
    pub dx: f32,
    #[serde(default)]
    pub dy: f32,
    
    // Line objects (airfields)
    #[serde(default)]
    pub sx: f32,
    #[serde(default)]
    pub sy: f32,
    #[serde(default)]
    pub ex: f32,
    #[serde(default)]
    pub ey: f32,
}

impl MapObject {
    /// Check if this is the player
    pub fn is_player(&self) -> bool {
        self.icon == "Player"
    }
    
    /// Check if this is friendly unit
    pub fn is_friendly(&self) -> bool {
        // Blue color = friendly
        self.color.contains("#174DFF") || self.color.contains("#174dff")
    }
    
    /// Check if this is enemy unit
    pub fn is_enemy(&self) -> bool {
        // Red color = enemy
        self.color.contains("#fa0C00") || self.color.contains("#fa0c00")
    }
    
    /// Check if this is a ship
    pub fn is_ship(&self) -> bool {
        self.icon == "Ship" || self.obj_type.contains("ship")
    }
    
    /// Check if this is an aircraft
    pub fn is_aircraft(&self) -> bool {
        self.icon == "Aircraft" || self.obj_type.contains("aircraft")
    }
    
    /// Check if this is a tank
    pub fn is_tank(&self) -> bool {
        self.icon == "Tank" || self.obj_type.contains("tank")
    }
    
    /// Get heading angle in degrees
    pub fn get_heading(&self) -> f32 {
        if self.dx == 0.0 && self.dy == 0.0 {
            return 0.0;
        }
        let angle = self.dy.atan2(self.dx).to_degrees();
        (angle + 360.0) % 360.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapInfo {
    pub grid_size: Vec<f32>,
    pub grid_steps: Vec<f32>,
    pub grid_zero: Vec<f32>,
    pub map_min: Vec<f32>,
    pub map_max: Vec<f32>,
    pub map_generation: u32,
    pub hud_type: u32,
    pub valid: bool,
}

impl MapInfo {
    /// Convert world coordinates to normalized map coordinates (0..1)
    pub fn world_to_map(&self, x: f32, y: f32) -> (f32, f32) {
        let map_x = (x - self.map_min[0]) / (self.map_max[0] - self.map_min[0]);
        let map_y = (y - self.map_min[1]) / (self.map_max[1] - self.map_min[1]);
        (map_x, map_y)
    }
    
    /// Get grid cell for world coordinates
    pub fn get_grid_cell(&self, x: f32, y: f32) -> (i32, i32) {
        let cell_x = ((x - self.grid_zero[0]) / self.grid_steps[0]).floor() as i32;
        let cell_y = ((y - self.grid_zero[1]) / self.grid_steps[1]).floor() as i32;
        (cell_x, cell_y)
    }
    
    /// Convert grid cell to letter-number format (e.g., "A5")
    pub fn grid_cell_to_string(&self, cell_x: i32, cell_y: i32) -> String {
        if cell_y >= 0 && cell_y < 26 {
            let letter = (b'A' + cell_y as u8) as char;
            format!("{}{}", letter, cell_x + 1)
        } else {
            format!("?{}", cell_x + 1)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub objects: Vec<MapObject>,
    pub info: MapInfo,
    pub player_position: Option<(f32, f32)>,
    pub player_heading: Option<f32>,
}

impl MapData {
    pub fn new(objects: Vec<MapObject>, info: MapInfo) -> Self {
        // Find player
        let player = objects.iter().find(|obj| obj.is_player());
        let player_position = player.map(|p| (p.x, p.y));
        let player_heading = player.map(|p| p.get_heading());
        
        Self {
            objects,
            info,
            player_position,
            player_heading,
        }
    }
    
    /// Get all friendly ships
    pub fn get_friendly_ships(&self) -> Vec<&MapObject> {
        self.objects.iter()
            .filter(|obj| obj.is_friendly() && obj.is_ship())
            .collect()
    }
    
    /// Get all enemy ships
    pub fn get_enemy_ships(&self) -> Vec<&MapObject> {
        self.objects.iter()
            .filter(|obj| obj.is_enemy() && obj.is_ship())
            .collect()
    }
    
    /// Get all capture zones
    pub fn get_capture_zones(&self) -> Vec<&MapObject> {
        self.objects.iter()
            .filter(|obj| obj.obj_type == "capture_zone")
            .collect()
    }
    
    /// Get player grid position
    pub fn get_player_grid(&self) -> Option<String> {
        if let Some((x, y)) = self.player_position {
            // Convert normalized to world coordinates
            let world_x = x * (self.info.map_max[0] - self.info.map_min[0]) + self.info.map_min[0];
            let world_y = y * (self.info.map_max[1] - self.info.map_min[1]) + self.info.map_min[1];
            let (cell_x, cell_y) = self.info.get_grid_cell(world_x, world_y);
            Some(self.info.grid_cell_to_string(cell_x, cell_y))
        } else {
            None
        }
    }
    
    /// Count nearby enemies (within radius in normalized coordinates)
    pub fn count_nearby_enemies(&self, radius: f32) -> usize {
        if let Some((px, py)) = self.player_position {
            self.objects.iter()
                .filter(|obj| {
                    if !obj.is_enemy() || obj.x == 0.0 {
                        return false;
                    }
                    let dx = obj.x - px;
                    let dy = obj.y - py;
                    (dx * dx + dy * dy).sqrt() <= radius
                })
                .count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_player() {
        let obj = MapObject {
            obj_type: "ground_model".to_string(),
            color: "#faC81E".to_string(),
            color_rgb: vec![250, 200, 30],
            blink: 0,
            icon: "Player".to_string(),
            icon_bg: "none".to_string(),
            x: 0.5,
            y: 0.5,
            dx: 1.0,
            dy: 0.0,
            sx: 0.0,
            sy: 0.0,
            ex: 0.0,
            ey: 0.0,
        };
        assert!(obj.is_player());
    }
    
    #[test]
    fn test_is_friendly() {
        let obj = MapObject {
            obj_type: "ground_model".to_string(),
            color: "#174DFF".to_string(),
            color_rgb: vec![23, 77, 255],
            blink: 0,
            icon: "Ship".to_string(),
            icon_bg: "none".to_string(),
            x: 0.5,
            y: 0.5,
            dx: 0.0,
            dy: 0.0,
            sx: 0.0,
            sy: 0.0,
            ex: 0.0,
            ey: 0.0,
        };
        assert!(obj.is_friendly());
        assert!(obj.is_ship());
    }
}


