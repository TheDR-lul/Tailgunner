/// Event Engine
/// Detect events from game state

use crate::pattern_engine::GameEvent;
use crate::wt_telemetry::{GameState, HudEvent, VehicleType};
use std::collections::HashSet;

pub struct EventEngine {
    previous_state: Option<GameState>,
    #[allow(dead_code)]
    active_events: HashSet<String>,
}

impl EventEngine {
    pub fn new() -> Self {
        Self {
            previous_state: None,
            active_events: HashSet::new(),
        }
    }

    /// Detect new events by comparing states
    pub fn detect_events(&mut self, current_state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();

        // Check if game state is valid
        if !current_state.valid {
            return events;
        }

        // FOR SHIPS: Use HUD events as primary source (indicators/state not available)
        if matches!(current_state.type_, VehicleType::Ship) {
            // Process HUD events
            for hud_event in &current_state.hud_events {
                if let Some(game_event) = self.map_hud_event_to_game_event(hud_event) {
                    events.push(game_event);
                }
            }
        }

        // FOR TANKS/AIRCRAFT: Detect new events in state array (standard method)
        if !matches!(current_state.type_, VehicleType::Ship) {
            let current_states: HashSet<_> = current_state.state.iter().cloned().collect();
            
            // Find new states (not present in previous state)
            let new_states: Vec<_> = if let Some(prev) = &self.previous_state {
                let prev_states: HashSet<_> = prev.state.iter().cloned().collect();
                current_states.difference(&prev_states).cloned().collect()
            } else {
                current_states.iter().cloned().collect()
            };

            // Map WT string events to our GameEvents
            for state_str in new_states {
                if let Some(event) = self.map_wt_state_to_event(&state_str) {
                    events.push(event);
                }
            }
        }

        // Save current state
        self.previous_state = Some(current_state.clone());

        events
    }

    /// Map WT state strings to GameEvent (extended)
    fn map_wt_state_to_event(&self, state: &str) -> Option<GameEvent> {
        let state_lower = state.to_lowercase();
        
        match state_lower.as_str() {
            // === HITS ===
            s if s.contains("critical") && s.contains("hit") => Some(GameEvent::CriticalHit),
            s if s.contains("penetration") => Some(GameEvent::PenetrationHit),
            s if s.contains("ricochet") => Some(GameEvent::Ricochet),
            s if s.contains("hit") => Some(GameEvent::Hit),
            
            // === ENGINE DAMAGE ===
            s if s.contains("engine") && s.contains("destroyed") => Some(GameEvent::EngineDestroyed),
            s if s.contains("engine") && s.contains("fire") => Some(GameEvent::EngineFire),
            s if s.contains("engine") && s.contains("damaged") => Some(GameEvent::EngineDamaged),
            s if s.contains("engine") && s.contains("overheat") => Some(GameEvent::EngineOverheat),
            s if s.contains("oil") && s.contains("leak") => Some(GameEvent::OilLeak),
            s if s.contains("water") && s.contains("leak") => Some(GameEvent::WaterLeak),
            
            // === CREW ===
            s if s.contains("pilot") && s.contains("knocked") => Some(GameEvent::PilotKnockedOut),
            s if s.contains("gunner") && s.contains("knocked") => Some(GameEvent::GunnerKnockedOut),
            s if s.contains("driver") && s.contains("knocked") => Some(GameEvent::DriverKnockedOut),
            s if s.contains("crew") && s.contains("knocked") => Some(GameEvent::CrewKnocked),
            
            // === TANK ===
            s if s.contains("track") && s.contains("broken") => Some(GameEvent::TrackBroken),
            s if s.contains("turret") && s.contains("jammed") => Some(GameEvent::TurretJammed),
            s if s.contains("gun") && s.contains("breach") => Some(GameEvent::GunBreach),
            s if s.contains("transmission") => Some(GameEvent::TransmissionDamaged),
            s if s.contains("ammunition") && s.contains("exploded") => Some(GameEvent::AmmunitionExploded),
            s if s.contains("fuel") && s.contains("tank") => Some(GameEvent::FuelTankHit),
            
            // === AIRCRAFT DAMAGE ===
            s if s.contains("wing") && s.contains("damaged") => Some(GameEvent::WingDamaged),
            s if s.contains("tail") && s.contains("damaged") => Some(GameEvent::TailDamaged),
            s if s.contains("elevator") && s.contains("damaged") => Some(GameEvent::ElevatorDamaged),
            s if s.contains("rudder") && s.contains("damaged") => Some(GameEvent::RudderDamaged),
            s if s.contains("aileron") && s.contains("damaged") => Some(GameEvent::AileronDamaged),
            s if s.contains("gear") && s.contains("damaged") => Some(GameEvent::GearDamaged),
            s if s.contains("flaps") && s.contains("damaged") => Some(GameEvent::FlapsDamaged),
            
            // === AERODYNAMICS ===
            s if s.contains("stall") && s.contains("compressor") => Some(GameEvent::CompressorStall),
            s if s.contains("flat") && s.contains("spin") => Some(GameEvent::FlatSpin),
            s if s.contains("stall") => Some(GameEvent::Stall),
            s if s.contains("spin") => Some(GameEvent::Spin),
            
            // === CONTROLS ===
            s if s.contains("gear") && s.contains("up") => Some(GameEvent::GearUp),
            s if s.contains("gear") && s.contains("down") => Some(GameEvent::GearDown),
            s if s.contains("gear") && s.contains("stuck") => Some(GameEvent::GearStuck),
            s if s.contains("flaps") && s.contains("extended") => Some(GameEvent::FlapsExtended),
            s if s.contains("flaps") && s.contains("retracted") => Some(GameEvent::FlapsRetracted),
            s if s.contains("airbrake") => Some(GameEvent::AirbrakeDeployed),
            s if s.contains("parachute") => Some(GameEvent::ParachuteDeployed),
            
            // === WEAPONS ===
            s if s.contains("cannon") && s.contains("firing") => Some(GameEvent::CannonFiring),
            s if s.contains("machine") && s.contains("gun") => Some(GameEvent::MachineGunFiring),
            s if s.contains("rocket") && s.contains("launched") => Some(GameEvent::RocketLaunched),
            s if s.contains("bomb") && s.contains("dropped") => Some(GameEvent::BombDropped),
            s if s.contains("torpedo") => Some(GameEvent::TorpedoDropped),
            s if s.contains("shoot") || s.contains("firing") => Some(GameEvent::Shooting),
            
            // === PLAYER HITS ===
            s if s.contains("target") && s.contains("destroyed") => Some(GameEvent::TargetDestroyed),
            s if s.contains("target") && s.contains("critical") => Some(GameEvent::TargetCritical),
            s if s.contains("target") && s.contains("hit") => Some(GameEvent::TargetHit),
            s if s.contains("aircraft") && s.contains("destroyed") => Some(GameEvent::AircraftDestroyed),
            s if s.contains("tank") && s.contains("destroyed") => Some(GameEvent::TankDestroyed),
            
            // === FUEL ===
            s if s.contains("out") && s.contains("fuel") => Some(GameEvent::OutOfFuel),
            s if s.contains("fuel") && s.contains("empty") => Some(GameEvent::OutOfFuel),
            
            // === AMMO ===
            s if s.contains("out") && s.contains("ammo") => Some(GameEvent::OutOfAmmo),
            s if s.contains("ammo") && s.contains("empty") => Some(GameEvent::OutOfAmmo),
            
            // === LANDING/TAKEOFF ===
            s if s.contains("touchdown") => Some(GameEvent::Touchdown),
            s if s.contains("landed") => Some(GameEvent::Landed),
            s if s.contains("takeoff") => Some(GameEvent::Takeoff),
            
            // === SYSTEMS ===
            s if s.contains("fire") && s.contains("extinguished") => Some(GameEvent::FireExtinguished),
            s if s.contains("repair") && s.contains("completed") => Some(GameEvent::RepairCompleted),
            s if s.contains("autopilot") && s.contains("engaged") => Some(GameEvent::AutopilotEngaged),
            s if s.contains("autopilot") && s.contains("disengaged") => Some(GameEvent::AutopilotDisengaged),
            
            // === MISSION ===
            s if s.contains("mission") && s.contains("started") => Some(GameEvent::MissionStarted),
            s if s.contains("mission") && s.contains("success") => Some(GameEvent::MissionSuccess),
            s if s.contains("mission") && s.contains("failed") => Some(GameEvent::MissionFailed),
            s if s.contains("objective") && s.contains("completed") => Some(GameEvent::MissionObjectiveCompleted),
            s if s.contains("respawn") => Some(GameEvent::Respawn),
            
            // === MULTIPLAYER ===
            s if s.contains("team") && s.contains("kill") => Some(GameEvent::TeamKill),
            s if s.contains("assist") => Some(GameEvent::Assist),
            s if s.contains("base") && s.contains("capture") => Some(GameEvent::BaseCapture),
            
            // === FIRE ===
            s if s.contains("fire") => Some(GameEvent::EngineFire),
            
            _ => {
                // Custom event
                if !state.is_empty() {
                    Some(GameEvent::CustomTrigger(state.to_string()))
                } else {
                    None
                }
            }
        }
    }

    /// Map HUD events to game events (used for ships)
    fn map_hud_event_to_game_event(&self, hud_event: &HudEvent) -> Option<GameEvent> {
        match hud_event {
            HudEvent::Kill(_) => Some(GameEvent::TargetDestroyed),
            HudEvent::SetAfire(_) => Some(GameEvent::TargetSetOnFire),
            HudEvent::TakeDamage(_) => Some(GameEvent::Hit),
            HudEvent::SeverelyDamaged(_) => Some(GameEvent::SeverelyDamaged),
            HudEvent::ShotDown(_) => Some(GameEvent::VehicleDestroyed),
            HudEvent::Achievement(_) => Some(GameEvent::Achievement),
            HudEvent::Crashed => Some(GameEvent::Crashed),
            HudEvent::EngineOverheated => Some(GameEvent::EngineOverheat),
            HudEvent::OilOverheated => Some(GameEvent::OilOverheated),
            HudEvent::ChatMessage(_) => None, // Ignore chat messages
        }
    }

    /// Check continuous events (e.g. engine running)
    pub fn check_continuous_events(&self, current_state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();

        // If engine RPM > 0, engine is running
        if current_state.indicators.engine_rpm > 100.0 {
            events.push(GameEvent::EngineRunning);
        }

        events
    }

    /// Reset state (e.g. when exiting battle)
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.previous_state = None;
        self.active_events.clear();
    }
}

impl Default for EventEngine {
    fn default() -> Self {
        Self::new()
    }
}

