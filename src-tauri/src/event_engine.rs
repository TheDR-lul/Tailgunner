/// Event Engine
/// Detect events from game state

use crate::pattern_engine::GameEvent;
use crate::wt_telemetry::GameState;
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

    /// Обнаружение новых событий путем сравнения состояний
    pub fn detect_events(&mut self, current_state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();

        // Проверяем состояние игры
        if !current_state.valid {
            return events;
        }

        // Детектируем новые события в state массиве
        let current_states: HashSet<_> = current_state.state.iter().cloned().collect();
        
        // Новые события (которых не было в прошлом состоянии)
        let new_states: Vec<_> = if let Some(prev) = &self.previous_state {
            let prev_states: HashSet<_> = prev.state.iter().cloned().collect();
            current_states.difference(&prev_states).cloned().collect()
        } else {
            current_states.iter().cloned().collect()
        };

        // Маппинг строковых событий WT в наши события
        for state_str in new_states {
            if let Some(event) = self.map_wt_state_to_event(&state_str) {
                events.push(event);
            }
        }

        // Сохраняем текущее состояние
        self.previous_state = Some(current_state.clone());

        events
    }

    /// Маппинг строковых состояний WT в GameEvent (расширенный)
    fn map_wt_state_to_event(&self, state: &str) -> Option<GameEvent> {
        let state_lower = state.to_lowercase();
        
        match state_lower.as_str() {
            // === ПОПАДАНИЯ ===
            s if s.contains("critical") && s.contains("hit") => Some(GameEvent::CriticalHit),
            s if s.contains("penetration") => Some(GameEvent::PenetrationHit),
            s if s.contains("ricochet") => Some(GameEvent::Ricochet),
            s if s.contains("hit") => Some(GameEvent::Hit),
            
            // === ПОВРЕЖДЕНИЯ ДВИГАТЕЛЯ ===
            s if s.contains("engine") && s.contains("destroyed") => Some(GameEvent::EngineDestroyed),
            s if s.contains("engine") && s.contains("fire") => Some(GameEvent::EngineFire),
            s if s.contains("engine") && s.contains("damaged") => Some(GameEvent::EngineDamaged),
            s if s.contains("engine") && s.contains("overheat") => Some(GameEvent::EngineOverheat),
            s if s.contains("oil") && s.contains("leak") => Some(GameEvent::OilLeak),
            s if s.contains("water") && s.contains("leak") => Some(GameEvent::WaterLeak),
            
            // === ЭКИПАЖ ===
            s if s.contains("pilot") && s.contains("knocked") => Some(GameEvent::PilotKnockedOut),
            s if s.contains("gunner") && s.contains("knocked") => Some(GameEvent::GunnerKnockedOut),
            s if s.contains("driver") && s.contains("knocked") => Some(GameEvent::DriverKnockedOut),
            s if s.contains("crew") && s.contains("knocked") => Some(GameEvent::CrewKnocked),
            
            // === ТАНК ===
            s if s.contains("track") && s.contains("broken") => Some(GameEvent::TrackBroken),
            s if s.contains("turret") && s.contains("jammed") => Some(GameEvent::TurretJammed),
            s if s.contains("gun") && s.contains("breach") => Some(GameEvent::GunBreach),
            s if s.contains("transmission") => Some(GameEvent::TransmissionDamaged),
            s if s.contains("ammunition") && s.contains("exploded") => Some(GameEvent::AmmunitionExploded),
            s if s.contains("fuel") && s.contains("tank") => Some(GameEvent::FuelTankHit),
            
            // === САМОЛЕТ ПОВРЕЖДЕНИЯ ===
            s if s.contains("wing") && s.contains("damaged") => Some(GameEvent::WingDamaged),
            s if s.contains("tail") && s.contains("damaged") => Some(GameEvent::TailDamaged),
            s if s.contains("elevator") && s.contains("damaged") => Some(GameEvent::ElevatorDamaged),
            s if s.contains("rudder") && s.contains("damaged") => Some(GameEvent::RudderDamaged),
            s if s.contains("aileron") && s.contains("damaged") => Some(GameEvent::AileronDamaged),
            s if s.contains("gear") && s.contains("damaged") => Some(GameEvent::GearDamaged),
            s if s.contains("flaps") && s.contains("damaged") => Some(GameEvent::FlapsDamaged),
            
            // === АЭРОДИНАМИКА ===
            s if s.contains("stall") && s.contains("compressor") => Some(GameEvent::CompressorStall),
            s if s.contains("flat") && s.contains("spin") => Some(GameEvent::FlatSpin),
            s if s.contains("stall") => Some(GameEvent::Stall),
            s if s.contains("spin") => Some(GameEvent::Spin),
            
            // === УПРАВЛЕНИЕ ===
            s if s.contains("gear") && s.contains("up") => Some(GameEvent::GearUp),
            s if s.contains("gear") && s.contains("down") => Some(GameEvent::GearDown),
            s if s.contains("gear") && s.contains("stuck") => Some(GameEvent::GearStuck),
            s if s.contains("flaps") && s.contains("extended") => Some(GameEvent::FlapsExtended),
            s if s.contains("flaps") && s.contains("retracted") => Some(GameEvent::FlapsRetracted),
            s if s.contains("airbrake") => Some(GameEvent::AirbrakeDeployed),
            s if s.contains("parachute") => Some(GameEvent::ParachuteDeployed),
            
            // === ВООРУЖЕНИЕ ===
            s if s.contains("cannon") && s.contains("firing") => Some(GameEvent::CannonFiring),
            s if s.contains("machine") && s.contains("gun") => Some(GameEvent::MachineGunFiring),
            s if s.contains("rocket") && s.contains("launched") => Some(GameEvent::RocketLaunched),
            s if s.contains("bomb") && s.contains("dropped") => Some(GameEvent::BombDropped),
            s if s.contains("torpedo") => Some(GameEvent::TorpedoDropped),
            s if s.contains("shoot") || s.contains("firing") => Some(GameEvent::Shooting),
            
            // === ПОПАДАНИЯ ИГРОКА ===
            s if s.contains("target") && s.contains("destroyed") => Some(GameEvent::TargetDestroyed),
            s if s.contains("target") && s.contains("critical") => Some(GameEvent::TargetCritical),
            s if s.contains("target") && s.contains("hit") => Some(GameEvent::TargetHit),
            s if s.contains("aircraft") && s.contains("destroyed") => Some(GameEvent::AircraftDestroyed),
            s if s.contains("tank") && s.contains("destroyed") => Some(GameEvent::TankDestroyed),
            
            // === ТОПЛИВО ===
            s if s.contains("out") && s.contains("fuel") => Some(GameEvent::OutOfFuel),
            s if s.contains("fuel") && s.contains("empty") => Some(GameEvent::OutOfFuel),
            
            // === БОЕЗАПАС ===
            s if s.contains("out") && s.contains("ammo") => Some(GameEvent::OutOfAmmo),
            s if s.contains("ammo") && s.contains("empty") => Some(GameEvent::OutOfAmmo),
            
            // === ПОСАДКА/ВЗЛЕТ ===
            s if s.contains("touchdown") => Some(GameEvent::Touchdown),
            s if s.contains("landed") => Some(GameEvent::Landed),
            s if s.contains("takeoff") => Some(GameEvent::Takeoff),
            
            // === СИСТЕМЫ ===
            s if s.contains("fire") && s.contains("extinguished") => Some(GameEvent::FireExtinguished),
            s if s.contains("repair") && s.contains("completed") => Some(GameEvent::RepairCompleted),
            s if s.contains("autopilot") && s.contains("engaged") => Some(GameEvent::AutopilotEngaged),
            s if s.contains("autopilot") && s.contains("disengaged") => Some(GameEvent::AutopilotDisengaged),
            
            // === МИССИЯ ===
            s if s.contains("mission") && s.contains("started") => Some(GameEvent::MissionStarted),
            s if s.contains("mission") && s.contains("success") => Some(GameEvent::MissionSuccess),
            s if s.contains("mission") && s.contains("failed") => Some(GameEvent::MissionFailed),
            s if s.contains("objective") && s.contains("completed") => Some(GameEvent::MissionObjectiveCompleted),
            s if s.contains("respawn") => Some(GameEvent::Respawn),
            
            // === MULTIPLAYER ===
            s if s.contains("team") && s.contains("kill") => Some(GameEvent::TeamKill),
            s if s.contains("assist") => Some(GameEvent::Assist),
            s if s.contains("base") && s.contains("capture") => Some(GameEvent::BaseCapture),
            
            // === ПОЖАР ===
            s if s.contains("fire") => Some(GameEvent::EngineFire),
            
            _ => {
                // Кастомное событие
                if !state.is_empty() {
                    Some(GameEvent::CustomTrigger(state.to_string()))
                } else {
                    None
                }
            }
        }
    }

    /// Проверка непрерывных событий (например, работающий двигатель)
    pub fn check_continuous_events(&self, current_state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();

        // Если RPM двигателя > 0, двигатель работает
        if current_state.indicators.engine_rpm > 100.0 {
            events.push(GameEvent::EngineRunning);
        }

        events
    }

    /// Сброс состояния (например, при выходе из боя)
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

