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
    
    /// Get heading angle in degrees (0° = North, clockwise)
    /// Synchronized with War Thunder compass
    pub fn get_heading(&self) -> f32 {
        if self.dx == 0.0 && self.dy == 0.0 {
            return 0.0;
        }
        // atan2 gives angle from X axis
        // Convert to compass bearing (0° = North/up, 90° = East/right)
        let angle_from_x = self.dy.atan2(self.dx).to_degrees();
        // Convert: X-axis angle → North-based compass bearing
        let compass_bearing = (90.0 - angle_from_x + 360.0) % 360.0;
        compass_bearing
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub objects: Vec<MapObject>,
    pub info: MapInfo,
    pub player_position: Option<(f32, f32)>,
    pub player_heading: Option<f32>,
    pub map_name: Option<String>,  // Detected map name
    pub player_grid: Option<String>,  // Grid reference (e.g. "P-10")
}

impl MapData {
    pub fn new(objects: Vec<MapObject>, info: MapInfo) -> Self {
        // Find player
        let player = objects.iter().find(|obj| obj.is_player());
        let player_position = player.map(|p| (p.x, p.y));
        let player_heading = player.map(|p| p.get_heading());
        
        // TODO: Map name detection (requires datamining or database)
        let map_name = None;
        
        let mut data = Self {
            objects,
            info,
            player_position,
            player_heading,
            map_name,
            player_grid: None,
        };
        
        // Calculate grid reference
        data.player_grid = data.get_player_grid_reference();
        
        data
    }
    
    /// Get player position as grid reference (e.g. "P-10")
    /// War Thunder grid: Letters A-Z (vertical/Y axis), Numbers 1-20 (horizontal/X axis)
    pub fn get_player_grid_reference(&self) -> Option<String> {
        let (x, y) = self.player_position?;
        
        // API coords are ALREADY relative to visible area (0..1)
        // Convert to grid cells using grid_steps
        let grid_step_x = self.info.grid_steps[0];
        let grid_step_y = self.info.grid_steps[1];
        let grid_size_x = self.info.grid_size[0];
        let grid_size_y = self.info.grid_size[1];
        
        // Calculate position in meters from top-left of visible area
        let pos_x = x * grid_size_x;
        let pos_y = y * grid_size_y;
        
        // Calculate grid cell
        let grid_x = (pos_x / grid_step_x).floor() as i32;
        let grid_y = (pos_y / grid_step_y).floor() as i32;
        
        // Convert to letter (A=0, B=1, ..., Z=25) for Y axis
        let letter = if grid_y >= 0 && grid_y < 26 {
            ((b'A' + grid_y as u8) as char).to_string()
        } else {
            format!("?{}", grid_y)
        };
        
        // X axis is 1-based numbering
        let number = grid_x + 1;
        
        Some(format!("{}-{}", letter, number))
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


