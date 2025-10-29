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
    // Core state
    pub enabled: bool,
    pub vehicle_type: VehicleType,
    pub vehicle_name: String,        // Actual vehicle name (e.g. "f_16a")
    pub vehicle_display_name: String, // Display name (e.g. "F-16A")
    pub in_battle: bool,
    
    // Movement
    pub speed: f32,           // km/h (IAS equivalent)
    pub altitude: f32,        // meters
    pub heading: f32,         // degrees 0-360
    pub position: [f32; 2],   // map coordinates [0..1, 0..1]
    
    // Combat
    pub ammo: i32,            // rounds
    pub engine_running: bool,
    
    // Aircraft specific (for /state)
    pub tas: f32,             // True Air Speed km/h
    pub ias: f32,             // Indicated Air Speed km/h
    pub mach: f32,            // Mach number
    pub aoa: f32,             // Angle of Attack deg
    pub aos: f32,             // Angle of Sideslip deg
    pub g_load: f32,          // Ny (G-load)
    pub vertical_speed: f32,  // Vy m/s
    pub roll_rate: f32,       // Wx deg/s
    
    // Fuel
    pub fuel_kg: f32,         // Current fuel kg
    pub fuel_max_kg: f32,     // Max fuel kg
    
    // Engine (for /indicators)
    pub rpm: f32,             // Engine RPM
    pub throttle: f32,        // 0-100%
    pub manifold_pressure: f32, // atm
    pub oil_temp: f32,        // Celsius
    pub water_temp: f32,      // Celsius
    pub thrust: f32,          // kgs
    
    // Controls
    pub stick_elevator: f32,  // -1 to 1
    pub stick_ailerons: f32,  // -1 to 1
    pub pedals: f32,          // -1 to 1
    pub flaps: f32,           // 0-1
    pub gear: f32,            // 0-1 (0=retracted, 1=extended)
    
    // Orientation
    pub pitch: f32,           // aviahorizon_pitch deg
    pub roll: f32,            // aviahorizon_roll deg
    pub compass: f32,         // heading deg
}

impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            enabled: false,
            vehicle_type: VehicleType::Aircraft,
            vehicle_name: "f_16a".to_string(),
            vehicle_display_name: "F-16A".to_string(),
            in_battle: false,
            
            speed: 0.0,
            altitude: 1000.0,
            heading: 0.0,
            position: [0.5, 0.5],
            
            ammo: 300,
            engine_running: true,
            
            tas: 0.0,
            ias: 0.0,
            mach: 0.0,
            aoa: 0.0,
            aos: 0.0,
            g_load: 1.0,
            vertical_speed: 0.0,
            roll_rate: 0.0,
            
            fuel_kg: 3000.0,
            fuel_max_kg: 5000.0,
            
            rpm: 0.0,
            throttle: 0.0,
            manifold_pressure: 1.0,
            oil_temp: 15.0,
            water_temp: 15.0,
            thrust: 0.0,
            
            stick_elevator: 0.0,
            stick_ailerons: 0.0,
            pedals: 0.0,
            flaps: 0.0,
            gear: 1.0, // Down by default
            
            pitch: 0.0,
            roll: 0.0,
            compass: 0.0,
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

    pub fn get_enabled(&self) -> bool {
        self.state.lock().unwrap().enabled
    }
    
    pub fn set_enabled(&self, enabled: bool) {
        self.state.lock().unwrap().enabled = enabled;
    }

    pub fn set_vehicle_type(&self, vehicle_type: VehicleType) {
        self.state.lock().unwrap().vehicle_type = vehicle_type;
    }
    
    pub fn set_vehicle_name(&self, name: String, display_name: String) {
        let mut state = self.state.lock().unwrap();
        state.vehicle_name = name;
        state.vehicle_display_name = display_name;
    }

    pub fn set_speed(&self, speed: f32) {
        let mut state = self.state.lock().unwrap();
        state.speed = speed;
        state.ias = speed;
        
        // Calculate TAS based on altitude (TAS increases with altitude)
        let altitude_factor = 1.0 + (state.altitude / 10000.0) * 0.15;
        state.tas = speed * altitude_factor;
        
        // Calculate Mach number (speed of sound ~1225 km/h at sea level)
        state.mach = state.tas / 1225.0;
        
        // Calculate RPM based on speed (realistic for jet engines)
        state.rpm = if state.engine_running && speed > 0.0 {
            (speed * 8.0).max(1000.0).min(10000.0)
        } else {
            0.0
        };
        
        // Calculate throttle % from speed
        state.throttle = (speed / 1000.0 * 100.0).min(100.0).max(0.0);
        
        // Calculate thrust (realistic for jets)
        state.thrust = if state.engine_running {
            state.throttle * 100.0 // ~10,000 kgs max thrust
        } else {
            0.0
        };
        
        // Update temperatures based on throttle
        if state.engine_running {
            state.oil_temp = 50.0 + state.throttle * 0.5;
            state.water_temp = 50.0 + state.throttle * 0.7;
        }
        
        // Calculate manifold pressure
        state.manifold_pressure = 1.0 + (state.throttle / 100.0) * 0.5;
    }

    pub fn set_altitude(&self, altitude: f32) {
        let mut state = self.state.lock().unwrap();
        state.altitude = altitude;
        
        // Recalculate TAS when altitude changes
        let altitude_factor = 1.0 + (altitude / 10000.0) * 0.15;
        state.tas = state.ias * altitude_factor;
        state.mach = state.tas / 1225.0;
    }

    pub fn set_heading(&self, heading: f32) {
        let mut state = self.state.lock().unwrap();
        state.heading = heading;
        state.compass = heading; // Sync compass with heading
    }

    pub fn set_position(&self, x: f32, y: f32) {
        self.state.lock().unwrap().position = [x, y];
    }

    pub fn set_ammo(&self, ammo: i32) {
        self.state.lock().unwrap().ammo = ammo;
    }

    #[allow(dead_code)]
    pub fn set_engine_running(&self, running: bool) {
        self.state.lock().unwrap().engine_running = running;
    }

    pub fn set_in_battle(&self, in_battle: bool) {
        self.state.lock().unwrap().in_battle = in_battle;
    }

    pub fn set_g_load(&self, g_load: f32) {
        self.state.lock().unwrap().g_load = g_load;
    }

    pub fn set_fuel(&self, fuel_kg: f32) {
        self.state.lock().unwrap().fuel_kg = fuel_kg;
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get_events(&self, since_timestamp: u64) -> Vec<EmulatedEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.timestamp > since_timestamp)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
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
            VehicleType::Tank => {
                // Replace spaces with underscores for WT API format
                let vehicle_name_formatted = state.vehicle_name.replace(' ', "_");
                
                serde_json::json!({
                    "valid": true,
                    "army": "ground",
                    "type": format!("tankModels/{}", &vehicle_name_formatted),
                "speed": state.speed,
                "rpm": state.rpm,
                "gear": if state.gear > 0.5 { 1 } else { 0 },
                "throttle": state.throttle,
                "oil_temperature": state.oil_temp,
                "water_temperature": state.water_temp,
                "ammo_count": state.ammo,
                })
            },
            VehicleType::Aircraft => {
                // Calculate realistic altimeter digits
                let alt_m = state.altitude;
                let alt_hour = (alt_m / 1000.0).floor();
                let alt_min = ((alt_m % 1000.0) / 100.0).floor();
                
                // Replace spaces with underscores for WT API format
                let vehicle_name_formatted = state.vehicle_name.replace(' ', "_");
                
                serde_json::json!({
                    "valid": true,
                    "army": "air",
                    "type": &vehicle_name_formatted,
                    "speed": state.ias,
                    "pedals": state.pedals,
                    "pedals1": state.pedals,
                    "pedals2": state.pedals,
                    "stick_elevator": state.stick_elevator,
                    "stick_ailerons": state.stick_ailerons,
                    "altitude_hour": alt_hour,
                    "altitude_min": alt_min,
                    "aviahorizon_roll": state.roll,
                    "aviahorizon_pitch": state.pitch,
                    "compass": state.compass,
                    "compass1": state.compass,
                    "rpm": state.rpm,
                    "throttle": state.throttle,
                    "water_temperature": state.water_temp,
                    "gears": state.gear,
                    "gear_lamp_down": if state.gear > 0.9 { 1.0 } else { 0.0 },
                    "gear_lamp_up": if state.gear < 0.1 { 1.0 } else { 0.0 },
                    "gear_lamp_off": if state.gear > 0.1 && state.gear < 0.9 { 1.0 } else { 0.0 },
                    "weapon2": if state.ammo > 0 { 1.0 } else { 0.0 },
                    "weapon4": if state.ammo > 0 { 1.0 } else { 0.0 },
                    "blister1": 0.0,
                    "blister2": 0.0,
                    "blister3": 0.0,
                    "blister4": 0.0,
                    "blister5": 0.0,
                    "blister6": 0.0,
                    "blister11": 0.0,
                })
            },
            VehicleType::Ship => {
                // Replace spaces with underscores for WT API format
                let vehicle_name_formatted = state.vehicle_name.replace(' ', "_");
                
                serde_json::json!({
                    "valid": true,
                    "army": "sea",
                    "type": &vehicle_name_formatted,
                "speed": state.speed,
                "rpm": state.rpm,
                "throttle": state.throttle,
                "compass": state.compass,
                "ammo_count": state.ammo,
                })
            },
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
                // Altitude & Speed
                "H, m": state.altitude,
                "TAS, km/h": state.tas,
                "IAS, km/h": state.ias,
                "M": state.mach,
                
                // Angles
                "AoA, deg": state.aoa,
                "AoS, deg": state.aos,
                
                // G-load & vertical speed
                "Ny": state.g_load,
                "Vy, m/s": state.vertical_speed,
                "Wx, deg/s": state.roll_rate,
                
                // Fuel
                "Mfuel, kg": state.fuel_kg,
                "Mfuel0, kg": state.fuel_max_kg,
                
                // Engine 1
                "throttle 1, %": state.throttle,
                "RPM throttle 1, %": state.throttle,
                "power 1, hp": if state.engine_running { state.throttle * 15.0 } else { 0.0 },
                "RPM 1": state.rpm,
                "manifold pressure 1, atm": state.manifold_pressure,
                "oil temp 1, C": state.oil_temp,
                "water temp 1, C": state.water_temp,
                "thrust 1, kgs": state.thrust,
                "efficiency 1, %": if state.throttle > 0.0 { 85.0 } else { 0.0 },
                
                // Controls
                "aileron, %": state.stick_ailerons * 100.0,
                "elevator, %": state.stick_elevator * 100.0,
                "rudder, %": state.pedals * 100.0,
                "flaps, %": state.flaps * 100.0,
                "gear, %": state.gear * 100.0,
                "airbrake, %": 0.0,
            }),
            VehicleType::Tank => serde_json::json!({
                "valid": true,
                // Speed
                "TAS, km/h": state.speed,
                "IAS, km/h": state.speed,
                
                // G-load
                "Ny": state.g_load,
                
                // Fuel
                "Mfuel, kg": state.fuel_kg,
                "Mfuel0, kg": state.fuel_max_kg,
                
                // Engine
                "throttle 1, %": state.throttle,
                "RPM 1": state.rpm,
                "oil temp 1, C": state.oil_temp,
                "water temp 1, C": state.water_temp,
                
                // Controls
                "gear, %": state.gear * 100.0,
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

        // Add airfields (for aircraft)
        if matches!(state.vehicle_type, VehicleType::Aircraft) {
            // Friendly airfield
            objects.push(serde_json::json!({
                "type": "airfield",
                "color": "#174DFF",
                "color[]": [23, 77, 255],
                "blink": 0,
                "icon": "none",
                "icon_bg": "none",
                "sx": 0.85,
                "sy": 0.45,
                "ex": 0.79,
                "ey": 0.44
            }));
            
            // Enemy airfield
            objects.push(serde_json::json!({
                "type": "airfield",
                "color": "#fa0C00",
                "color[]": [250, 12, 0],
                "blink": 0,
                "icon": "none",
                "icon_bg": "none",
                "sx": 0.14,
                "sy": 0.48,
                "ex": 0.19,
                "ey": 0.48
            }));
        }

        // Add capture zones (for ground battles)
        if matches!(state.vehicle_type, VehicleType::Tank) {
            // Zone A (enemy)
            objects.push(serde_json::json!({
                "type": "capture_zone",
                "color": "#fa0C00",
                "color[]": [250, 12, 0],
                "blink": 0,
                "icon": "capture_zone",
                "icon_bg": "none",
                "x": 0.30,
                "y": 0.50
            }));
            
            // Zone B (neutral)
            objects.push(serde_json::json!({
                "type": "capture_zone",
                "color": "#FFFFFF",
                "color[]": [255, 255, 255],
                "blink": 0,
                "icon": "capture_zone",
                "icon_bg": "none",
                "x": 0.50,
                "y": 0.50
            }));
            
            // Zone C (friendly)
            objects.push(serde_json::json!({
                "type": "capture_zone",
                "color": "#174DFF",
                "color[]": [23, 77, 255],
                "blink": 0,
                "icon": "capture_zone",
                "icon_bg": "none",
                "x": 0.70,
                "y": 0.50
            }));
        }

        // Add respawn points (for aircraft)
        if matches!(state.vehicle_type, VehicleType::Aircraft) {
            // Friendly fighter spawn
            objects.push(serde_json::json!({
                "type": "respawn_base_fighter",
                "color": "#174DFF",
                "color[]": [23, 77, 255],
                "blink": 0,
                "icon": "respawn_base_fighter",
                "icon_bg": "none",
                "x": 0.85,
                "y": 0.45,
                "dx": -399.0,
                "dy": 17.0
            }));
            
            // Friendly bomber spawn
            objects.push(serde_json::json!({
                "type": "respawn_base_bomber",
                "color": "#174DFF",
                "color[]": [23, 77, 255],
                "blink": 0,
                "icon": "respawn_base_bomber",
                "icon_bg": "none",
                "x": 0.85,
                "y": 0.48,
                "dx": -399.0,
                "dy": 17.0
            }));
            
            // Enemy fighter spawn
            objects.push(serde_json::json!({
                "type": "respawn_base_fighter",
                "color": "#fa0C00",
                "color[]": [250, 12, 0],
                "blink": 0,
                "icon": "respawn_base_fighter",
                "icon_bg": "none",
                "x": 0.15,
                "y": 0.48,
                "dx": 399.0,
                "dy": -11.0
            }));
        }

        objects
    }
}


