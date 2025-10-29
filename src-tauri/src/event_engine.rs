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

    /// Map WT state strings to GameEvent (cleaned up - only possible events)
    fn map_wt_state_to_event(&self, state: &str) -> Option<GameEvent> {
        let state_lower = state.to_lowercase();
        
        match state_lower.as_str() {
            // === COMBAT ===
            s if s.contains("critical") && s.contains("hit") => Some(GameEvent::CriticalHit),
            s if s.contains("hit") => Some(GameEvent::Hit),
            s if s.contains("target") && s.contains("destroyed") => Some(GameEvent::TargetDestroyed),
            s if s.contains("target") && s.contains("hit") => Some(GameEvent::TargetHit),
            s if s.contains("aircraft") && s.contains("destroyed") => Some(GameEvent::AircraftDestroyed),
            s if s.contains("tank") && s.contains("destroyed") => Some(GameEvent::TankDestroyed),
            s if s.contains("ship") && s.contains("destroyed") => Some(GameEvent::ShipDestroyed),
            s if s.contains("destroyed") => Some(GameEvent::VehicleDestroyed),
            
            // === CREW ===
            s if s.contains("crew") && s.contains("knocked") => Some(GameEvent::CrewKnocked),
            
            // === MISSION ===
            s if s.contains("mission") && s.contains("success") => Some(GameEvent::MissionSuccess),
            s if s.contains("mission") && s.contains("failed") => Some(GameEvent::MissionFailed),
            s if s.contains("objective") && s.contains("completed") => Some(GameEvent::MissionObjectiveCompleted),
            
            // === MULTIPLAYER ===
            s if s.contains("team") && s.contains("kill") => Some(GameEvent::TeamKill),
            s if s.contains("assist") => Some(GameEvent::Assist),
            s if s.contains("base") && s.contains("capture") => Some(GameEvent::BaseCapture),
            
            // === ENGINE ===
            s if s.contains("engine") && s.contains("overheat") => Some(GameEvent::EngineOverheat),
            s if s.contains("oil") && s.contains("overheat") => Some(GameEvent::OilOverheated),
            
            // === GENERAL ===
            s if s.contains("shoot") || s.contains("firing") => Some(GameEvent::Shooting),
            
            _ => {
                // Custom event from state string
                if !state.is_empty() {
                    log::debug!("[Event Engine] Unknown state: '{}' - treating as custom trigger", state);
                    Some(GameEvent::CustomTrigger(state.to_string()))
                } else {
                    None
                }
            }
        }
    }

    /// Map HUD events to game events (used for ships and all vehicles)
    fn map_hud_event_to_game_event(&self, hud_event: &HudEvent) -> Option<GameEvent> {
        match hud_event {
            HudEvent::Kill(msg) => {
                // Determine kill type from message
                if msg.to_lowercase().contains("aircraft") || msg.to_lowercase().contains("shot down") {
                    Some(GameEvent::AircraftDestroyed)
                } else if msg.to_lowercase().contains("ship") {
                    Some(GameEvent::ShipDestroyed)
                } else if msg.to_lowercase().contains("tank") {
                    Some(GameEvent::TankDestroyed)
                } else {
                    Some(GameEvent::TargetDestroyed)
                }
            },
            HudEvent::SetAfire(_) => Some(GameEvent::TargetSetOnFire),
            HudEvent::TakeDamage(_) => Some(GameEvent::Hit),
            HudEvent::SeverelyDamaged(_) => Some(GameEvent::TargetSeverelyDamaged),
            HudEvent::CriticallyDamaged(_) => Some(GameEvent::CriticalHit),
            HudEvent::ShotDown(_) => Some(GameEvent::AircraftDestroyed),
            HudEvent::FirstStrike => Some(GameEvent::FirstStrike),
            HudEvent::Achievement(msg) => {
                // Detect special achievements
                let msg_lower = msg.to_lowercase();
                if msg_lower.contains("first strike") {
                    Some(GameEvent::FirstStrike)
                } else if msg_lower.contains("rescuer") {
                    Some(GameEvent::ShipRescuer)
                } else if msg_lower.contains("assist") {
                    Some(GameEvent::Assist)
                } else {
                    Some(GameEvent::Achievement)
                }
            },
            HudEvent::Crashed => Some(GameEvent::Crashed),
            HudEvent::EngineOverheated => Some(GameEvent::EngineOverheat),
            HudEvent::OilOverheated => Some(GameEvent::OilOverheated),
            HudEvent::ChatMessage(details) => {
                // Map chat message to specific type based on mode and enemy flag
                if details.is_enemy {
                    Some(GameEvent::EnemyChatMessage)
                } else if let Some(mode) = &details.mode {
                    match mode.to_lowercase().as_str() {
                        "team" => Some(GameEvent::TeamChatMessage),
                        "all" => Some(GameEvent::AllChatMessage),
                        "squad" => Some(GameEvent::SquadChatMessage),
                        _ => Some(GameEvent::ChatMessage)
                    }
                } else {
                    Some(GameEvent::ChatMessage)
                }
            },
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

