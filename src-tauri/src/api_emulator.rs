use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VehicleType {
    Tank,
    Aircraft,
    Ship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorState {
    pub enabled: bool,
    pub vehicle_type: VehicleType,
    pub speed: f32,
    pub altitude: f32,
    pub heading: f32,
    pub position: [f32; 2],
    pub ammo: i32,
    pub hp: f32,
    pub engine_running: bool,
    pub in_battle: bool,
}

impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            enabled: false,
            vehicle_type: VehicleType::Tank,
            speed: 0.0,
            altitude: 100.0,
            heading: 0.0,
            position: [0.5, 0.5],
            ammo: 50,
            hp: 100.0,
            engine_running: true,
            in_battle: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatedEvent {
    pub event_type: String,
    pub timestamp: u64,
    pub data: HashMap<String, String>,
}

pub struct APIEmulator {
    state: Arc<Mutex<EmulatorState>>,
    events: Arc<Mutex<Vec<EmulatedEvent>>>,
}

impl APIEmulator {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EmulatorState::default())),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_state(&self) -> EmulatorState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.state.lock().unwrap().enabled = enabled;
    }

    pub fn set_vehicle_type(&self, vehicle_type: VehicleType) {
        self.state.lock().unwrap().vehicle_type = vehicle_type;
    }

    pub fn set_speed(&self, speed: f32) {
        self.state.lock().unwrap().speed = speed;
    }

    pub fn set_altitude(&self, altitude: f32) {
        self.state.lock().unwrap().altitude = altitude;
    }

    pub fn set_heading(&self, heading: f32) {
        self.state.lock().unwrap().heading = heading;
    }

    pub fn set_position(&self, x: f32, y: f32) {
        self.state.lock().unwrap().position = [x, y];
    }

    pub fn set_ammo(&self, ammo: i32) {
        self.state.lock().unwrap().ammo = ammo;
    }

    pub fn set_hp(&self, hp: f32) {
        self.state.lock().unwrap().hp = hp;
    }

    pub fn set_engine_running(&self, running: bool) {
        self.state.lock().unwrap().engine_running = running;
    }

    pub fn set_in_battle(&self, in_battle: bool) {
        self.state.lock().unwrap().in_battle = in_battle;
    }

    pub fn trigger_event(&self, event_type: String, data: HashMap<String, String>) {
        let event = EmulatedEvent {
            event_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            data,
        };
        self.events.lock().unwrap().push(event);
    }

    pub fn get_events(&self, since_timestamp: u64) -> Vec<EmulatedEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.timestamp > since_timestamp)
            .cloned()
            .collect()
    }

    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }

    // Generate emulated API responses based on current state
    pub fn generate_indicators(&self) -> serde_json::Value {
        let state = self.state.lock().unwrap();
        if !state.enabled || !state.in_battle {
            return serde_json::json!({ "valid": false });
        }

        match state.vehicle_type {
            VehicleType::Tank => serde_json::json!({
                "valid": true,
                "type": "tank",
                "speed": state.speed,
                "rpm": (state.speed * 20.0).round(),
                "gear": if state.speed > 0.0 { 3 } else { 1 },
                "ammo_count": state.ammo,
                "throttle": (state.speed / 60.0 * 100.0).min(100.0),
            }),
            VehicleType::Aircraft => serde_json::json!({
                "valid": true,
                "type": "aircraft",
                "speed": state.speed,
                "altitude_hour": (state.altitude / 1000.0).floor(),
                "altitude_min": ((state.altitude % 1000.0) / 100.0).floor(),
                "rpm": (state.speed * 10.0).round(),
                "throttle": (state.speed / 500.0 * 100.0).min(100.0),
                "ammo_counter": state.ammo,
            }),
            VehicleType::Ship => serde_json::json!({
                "valid": false,  // Ships use HUD only
            }),
        }
    }

    pub fn generate_state(&self) -> serde_json::Value {
        let state = self.state.lock().unwrap();
        if !state.enabled || !state.in_battle {
            return serde_json::json!({ "valid": false });
        }

        match state.vehicle_type {
            VehicleType::Aircraft => serde_json::json!({
                "valid": true,
                "H, m": state.altitude,
                "TAS, km/h": state.speed,
                "IAS, km/h": state.speed * 0.95,
                "AoA, deg": 5.0,
                "Ny": 1.0,
                "throttle 1, %": (state.speed / 500.0 * 100.0).min(100.0),
                "RPM 1": (state.speed * 10.0).round(),
                "Mfuel, kg": 500.0,
            }),
            _ => serde_json::json!({ "valid": false }),
        }
    }

    pub fn generate_map_info(&self) -> serde_json::Value {
        let state = self.state.lock().unwrap();
        if !state.enabled || !state.in_battle {
            return serde_json::json!({ "valid": false });
        }

        serde_json::json!({
            "valid": true,
            "map_generation": 1,
            "map_min": [-16384.0, -16384.0],
            "map_max": [16384.0, 16384.0],
            "grid_zero": [-12216.0, 8120.0],
            "grid_size": [24432.0, 24432.0],
            "grid_steps": [2400.0, 2400.0],
            "hud_type": 1,
        })
    }

    pub fn generate_map_objects(&self) -> Vec<serde_json::Value> {
        let state = self.state.lock().unwrap();
        if !state.enabled || !state.in_battle {
            return vec![];
        }

        let mut objects = vec![];

        // Player
        objects.push(serde_json::json!({
            "type": "ground_model",
            "color": "#faC81E",
            "color[]": [250, 200, 30],
            "blink": 0,
            "icon": "Player",
            "icon_bg": "none",
            "x": state.position[0],
            "y": state.position[1],
            "dx": state.heading.to_radians().sin(),
            "dy": -state.heading.to_radians().cos(),
        }));

        // Add some enemy objects
        for i in 0..3 {
            let angle = (i as f32 * 120.0).to_radians();
            let distance = 0.1 + (i as f32 * 0.05);
            objects.push(serde_json::json!({
                "type": "ground_model",
                "color": "#fa0C00",
                "color[]": [250, 12, 0],
                "blink": 0,
                "icon": match state.vehicle_type {
                    VehicleType::Tank => "Tank",
                    VehicleType::Aircraft => "Aircraft",
                    VehicleType::Ship => "Ship",
                },
                "icon_bg": "none",
                "x": state.position[0] + angle.cos() * distance,
                "y": state.position[1] + angle.sin() * distance,
            }));
        }

        // Add some allies
        for i in 0..2 {
            let angle = (i as f32 * 180.0 + 60.0).to_radians();
            let distance = 0.08;
            objects.push(serde_json::json!({
                "type": "ground_model",
                "color": "#174DFF",
                "color[]": [23, 77, 255],
                "blink": 0,
                "icon": match state.vehicle_type {
                    VehicleType::Tank => "Tank",
                    VehicleType::Aircraft => "Aircraft",
                    VehicleType::Ship => "Ship",
                },
                "icon_bg": "none",
                "x": state.position[0] + angle.cos() * distance,
                "y": state.position[1] + angle.sin() * distance,
            }));
        }

        objects
    }
}


