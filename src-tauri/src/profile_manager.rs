/// Profile Manager
/// Automatic vehicle type detection and profile switching

use crate::pattern_engine::{GameEvent, VibrationPattern};
use crate::wt_telemetry::VehicleType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub vehicle_type: VehicleType,
    pub game_mode: GameMode,
    pub event_mappings: HashMap<GameEvent, VibrationPattern>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameMode {
    Arcade,
    Realistic,
    Simulator,
    Any,
}

pub struct ProfileManager {
    profiles: Vec<Profile>,
    active_profile: Option<String>,
}

impl ProfileManager {
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: Vec::new(),
            active_profile: None,
        };
        
        // Load default profiles
        manager.load_default_profiles();
        manager
    }

    /// Load ready-made presets
    fn load_default_profiles(&mut self) {
        // Tank Profile (Universal)
        let mut tank_mappings = HashMap::new();
        
        // Common events (controlled by built-in triggers)
        tank_mappings.insert(GameEvent::Hit, VibrationPattern::preset_simple_hit());
        tank_mappings.insert(GameEvent::CriticalHit, VibrationPattern::preset_critical_hit());
        tank_mappings.insert(GameEvent::LowFuel, VibrationPattern::preset_simple_hit());
        tank_mappings.insert(GameEvent::CriticalFuel, VibrationPattern::preset_fire());
        tank_mappings.insert(GameEvent::LowAmmo, VibrationPattern::preset_simple_hit());
        tank_mappings.insert(GameEvent::EngineDamaged, VibrationPattern::preset_simple_hit());
        tank_mappings.insert(GameEvent::EngineFire, VibrationPattern::preset_fire());
        
        // Tank-specific events
        tank_mappings.insert(GameEvent::EngineRunning, VibrationPattern::preset_engine_rumble());
        tank_mappings.insert(GameEvent::TrackBroken, VibrationPattern::preset_simple_hit());
        tank_mappings.insert(GameEvent::AmmunitionExploded, VibrationPattern::preset_critical_hit());
        tank_mappings.insert(GameEvent::PenetrationHit, VibrationPattern::preset_critical_hit());
        
        // Combat events from HUD
        tank_mappings.insert(GameEvent::TargetDestroyed, VibrationPattern::preset_simple_hit());

        self.profiles.push(Profile {
            id: "tank_universal".to_string(),
            name: "Tank - Universal".to_string(),
            vehicle_type: VehicleType::Tank,
            game_mode: GameMode::Any,
            event_mappings: tank_mappings,
            enabled: true,
        });

        // Aircraft Profile (Universal)
        let mut aircraft_mappings = HashMap::new();
        
        // Common events (controlled by built-in triggers)
        aircraft_mappings.insert(GameEvent::Hit, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::CriticalHit, VibrationPattern::preset_critical_hit());
        aircraft_mappings.insert(GameEvent::LowFuel, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::CriticalFuel, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::LowAmmo, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::EngineDamaged, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::EngineFire, VibrationPattern::preset_fire());
        
        // Aircraft-specific flight dynamics
        aircraft_mappings.insert(GameEvent::Stall, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::Spin, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::Overspeed, VibrationPattern::preset_critical_hit());
        aircraft_mappings.insert(GameEvent::OverG, VibrationPattern::preset_critical_hit());
        aircraft_mappings.insert(GameEvent::HighAOA, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::CriticalAOA, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::Mach1, VibrationPattern::preset_critical_hit());
        aircraft_mappings.insert(GameEvent::LowAltitude, VibrationPattern::preset_simple_hit());
        aircraft_mappings.insert(GameEvent::EngineOverheat, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::OilOverheated, VibrationPattern::preset_fire());
        aircraft_mappings.insert(GameEvent::Crashed, VibrationPattern::preset_critical_hit());
        
        // Combat events from HUD
        aircraft_mappings.insert(GameEvent::TargetDestroyed, VibrationPattern::preset_simple_hit());

        self.profiles.push(Profile {
            id: "aircraft_universal".to_string(),
            name: "Aircraft - Universal".to_string(),
            vehicle_type: VehicleType::Aircraft,
            game_mode: GameMode::Any,
            event_mappings: aircraft_mappings,
            enabled: true,
        });

        // Light Background Profile (Minimal, all vehicles)
        // NOTE: This profile overrides built-in patterns with lighter versions
        let mut light_mappings = HashMap::new();
        let light_hit = VibrationPattern {
            name: "Light Touch".to_string(),
            attack: crate::pattern_engine::EnvelopeStage {
                duration_ms: 20,
                start_intensity: 0.0,
                end_intensity: 0.3,
                curve: crate::pattern_engine::Curve::Linear,
            },
            hold: crate::pattern_engine::EnvelopeStage {
                duration_ms: 30,
                start_intensity: 0.3,
                end_intensity: 0.3,
                curve: crate::pattern_engine::Curve::Linear,
            },
            decay: crate::pattern_engine::EnvelopeStage {
                duration_ms: 80,
                start_intensity: 0.3,
                end_intensity: 0.0,
                curve: crate::pattern_engine::Curve::EaseOut,
            },
            burst: crate::pattern_engine::BurstConfig {
                repeat_count: 1,
                pause_between_ms: 0,
            },
        };
        
        // Only override most common events with light version
        light_mappings.insert(GameEvent::Hit, light_hit.clone());
        light_mappings.insert(GameEvent::CriticalHit, light_hit.clone());
        light_mappings.insert(GameEvent::OverG, light_hit.clone());
        light_mappings.insert(GameEvent::Overspeed, light_hit);

        self.profiles.push(Profile {
            id: "light_background".to_string(),
            name: "Light Background".to_string(),
            vehicle_type: VehicleType::Unknown,
            game_mode: GameMode::Any,
            event_mappings: light_mappings,
            enabled: false,  // Disabled by default
        });
    }

    /// Automatic profile selection based on vehicle type
    pub fn auto_select_profile(&mut self, vehicle_type: &VehicleType) -> Option<&Profile> {
        // Find the best matching profile
        let best_match = self.profiles
            .iter()
            .filter(|p| p.enabled)
            .filter(|p| p.vehicle_type == *vehicle_type || p.vehicle_type == VehicleType::Unknown)
            .max_by_key(|p| {
                // Priority: exact match > Any
                if p.vehicle_type == *vehicle_type { 2 } else { 1 }
            });

        if let Some(profile) = best_match {
            self.active_profile = Some(profile.id.clone());
            log::info!("Auto-selected profile: {}", profile.name);
        }

        best_match
    }

    /// Get active profile
    pub fn get_active_profile(&self) -> Option<&Profile> {
        self.active_profile.as_ref()
            .and_then(|id| self.profiles.iter().find(|p| &p.id == id))
    }

    /// Get pattern for event
    pub fn get_pattern_for_event(&self, event: &GameEvent) -> Option<&VibrationPattern> {
        self.get_active_profile()
            .and_then(|profile| profile.event_mappings.get(event))
    }

    /// List all profiles
    pub fn get_all_profiles(&self) -> &[Profile] {
        &self.profiles
    }

    /// Add custom profile
    #[allow(dead_code)]
    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    /// Remove profile
    #[allow(dead_code)]
    pub fn remove_profile(&mut self, id: &str) {
        self.profiles.retain(|p| p.id != id);
        if self.active_profile.as_deref() == Some(id) {
            self.active_profile = None;
        }
    }

    /// Toggle profile on/off
    #[allow(dead_code)]
    pub fn toggle_profile(&mut self, id: &str, enabled: bool) {
        if let Some(profile) = self.profiles.iter_mut().find(|p| p.id == id) {
            profile.enabled = enabled;
        }
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

