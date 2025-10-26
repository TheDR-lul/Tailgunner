/// Event Triggers - System for complex trigger conditions
/// Allows creating custom events based on indicator values

use crate::wt_telemetry::GameState;
use crate::pattern_engine::{GameEvent, VibrationPattern};
use crate::state_history::{StateHistory, speed_extractor, altitude_extractor, g_load_extractor, fuel_extractor};
use serde::{Deserialize, Serialize};

/// Trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    // Value comparison
    SpeedAbove(f32),
    SpeedBelow(f32),
    AltitudeAbove(f32),
    AltitudeBelow(f32),
    RPMAbove(f32),
    TempAbove(f32),
    
    // G-load
    GLoadAbove(f32),
    GLoadBelow(f32),
    
    // Angle of Attack
    AOAAbove(f32),
    AOABelow(f32),
    
    // Speed (different types)
    IASAbove(f32),
    TASAbove(f32),
    MachAbove(f32),
    
    // Fuel/Ammo
    FuelBelow(f32),        // percentage
    FuelTimeBelow(f32),    // minutes
    
    AmmoBelow(f32),        // percentage
    
    // Damage
    EngineDamageAbove(f32),
    ControlsDamageAbove(f32),
    
    // Tank-specific
    StabilizerActive,
    StabilizerInactive,
    CrewLost,                  // crew_current < crew_total
    CrewMemberDead(String),    // "gunner" or "driver"
    GearAbove(f32),
    GearBelow(f32),
    GearEquals(f32),
    CruiseControlAbove(f32),
    CruiseControlBelow(f32),
    DrivingForward,
    DrivingBackward,
    
    // Logical
    And(Box<TriggerCondition>, Box<TriggerCondition>),
    Or(Box<TriggerCondition>, Box<TriggerCondition>),
    Not(Box<TriggerCondition>),
    AlwaysTrue,  // Always triggers (for event-based patterns)
    
    // Temporal conditions (require state history)
    // Speed changes
    SpeedDroppedBy { threshold: f32, window_seconds: f32 },     // Speed dropped by X km/h in Y seconds
    SpeedIncreasedBy { threshold: f32, window_seconds: f32 },   // Speed increased by X km/h in Y seconds
    AccelerationAbove { threshold: f32, window_seconds: f32 },  // Acceleration > X km/h/s over Y seconds
    AccelerationBelow { threshold: f32, window_seconds: f32 },  // Deceleration > X km/h/s over Y seconds
    
    // Altitude changes
    AltitudeDroppedBy { threshold: f32, window_seconds: f32 },  // Altitude dropped by X meters in Y seconds
    AltitudeGainedBy { threshold: f32, window_seconds: f32 },   // Altitude gained by X meters in Y seconds
    ClimbRateAbove { threshold: f32, window_seconds: f32 },     // Climb rate > X m/s over Y seconds
    
    // G-load changes
    GLoadSpiked { threshold: f32, window_seconds: f32 },        // G-load increased by X G in Y seconds
    SuddenGChange { threshold: f32, window_seconds: f32 },      // Sudden G change (any direction)
    
    // Fuel depletion rate
    FuelDepletingFast { threshold: f32, window_seconds: f32 },  // Fuel dropping faster than X kg/s
    
    // Averages over time
    AverageSpeedAbove { threshold: f32, window_seconds: f32 },  // Average speed > X over Y seconds
    AverageGLoadAbove { threshold: f32, window_seconds: f32 },  // Average G-load > X over Y seconds
}

impl TriggerCondition {
    /// Check if condition is met based on current GameState
    pub fn evaluate(&self, state: &GameState) -> bool {
        match self {
            TriggerCondition::AlwaysTrue => true,
            TriggerCondition::SpeedAbove(threshold) => state.indicators.speed > *threshold,
            TriggerCondition::SpeedBelow(threshold) => state.indicators.speed < *threshold,
            TriggerCondition::AltitudeAbove(threshold) => state.indicators.altitude > *threshold,
            TriggerCondition::AltitudeBelow(threshold) => state.indicators.altitude < *threshold,
            TriggerCondition::RPMAbove(threshold) => state.indicators.engine_rpm > *threshold,
            TriggerCondition::TempAbove(threshold) => state.indicators.engine_temp > *threshold,
            
            TriggerCondition::GLoadAbove(threshold) => state.indicators.g_load > *threshold,
            TriggerCondition::GLoadBelow(threshold) => state.indicators.g_load < *threshold,
            
            TriggerCondition::AOAAbove(threshold) => state.indicators.aoa > *threshold,
            TriggerCondition::AOABelow(threshold) => state.indicators.aoa < *threshold,
            
            TriggerCondition::IASAbove(threshold) => state.indicators.ias > *threshold,
            TriggerCondition::TASAbove(threshold) => state.indicators.tas > *threshold,
            TriggerCondition::MachAbove(threshold) => state.indicators.mach > *threshold,
            
            // Fuel percentage (threshold is in %, e.g. 10.0 for 10%)
            TriggerCondition::FuelBelow(threshold) => {
                if state.indicators.fuel_max > 0.0 {
                    (state.indicators.fuel / state.indicators.fuel_max * 100.0) < *threshold
                } else {
                    false
                }
            },
            TriggerCondition::FuelTimeBelow(threshold) => state.indicators.fuel_time < *threshold,
            
            // Ammo percentage (threshold is in %, e.g. 20.0 for 20%)
            // Note: We don't know max ammo from telemetry, so this is approximate
            TriggerCondition::AmmoBelow(threshold) => {
                // Simple fallback: assume 1000 rounds max for aircraft, 50 for tanks
                let estimated_max = if state.indicators.ammo_count > 100 { 1000.0 } else { 50.0 };
                (state.indicators.ammo_count as f32 / estimated_max * 100.0) < *threshold
            },
            
            TriggerCondition::EngineDamageAbove(threshold) => state.indicators.engine_damage > *threshold,
            TriggerCondition::ControlsDamageAbove(threshold) => state.indicators.controls_damage > *threshold,
            
            // Tank-specific
            TriggerCondition::StabilizerActive => state.indicators.stabilizer > 0.5,
            TriggerCondition::StabilizerInactive => state.indicators.stabilizer < 0.5,
            TriggerCondition::CrewLost => state.indicators.crew_current < state.indicators.crew_total,
            TriggerCondition::CrewMemberDead(member) => {
                match member.as_str() {
                    "gunner" => state.indicators.gunner_state > 0,
                    "driver" => state.indicators.driver_state > 0,
                    _ => false,
                }
            },
            TriggerCondition::GearAbove(threshold) => state.indicators.gear > *threshold,
            TriggerCondition::GearBelow(threshold) => state.indicators.gear < *threshold,
            TriggerCondition::GearEquals(value) => (state.indicators.gear - *value).abs() < 0.1,
            TriggerCondition::CruiseControlAbove(threshold) => state.indicators.cruise_control > *threshold,
            TriggerCondition::CruiseControlBelow(threshold) => state.indicators.cruise_control < *threshold,
            TriggerCondition::DrivingForward => state.indicators.driving_direction == 0,
            TriggerCondition::DrivingBackward => state.indicators.driving_direction != 0,
            
            TriggerCondition::And(a, b) => a.evaluate(state) && b.evaluate(state),
            TriggerCondition::Or(a, b) => a.evaluate(state) || b.evaluate(state),
            TriggerCondition::Not(cond) => !cond.evaluate(state),
            
            // Temporal conditions require state history (return false if not available)
            TriggerCondition::SpeedDroppedBy { .. } => false,
            TriggerCondition::SpeedIncreasedBy { .. } => false,
            TriggerCondition::AccelerationAbove { .. } => false,
            TriggerCondition::AccelerationBelow { .. } => false,
            TriggerCondition::AltitudeDroppedBy { .. } => false,
            TriggerCondition::AltitudeGainedBy { .. } => false,
            TriggerCondition::ClimbRateAbove { .. } => false,
            TriggerCondition::GLoadSpiked { .. } => false,
            TriggerCondition::SuddenGChange { .. } => false,
            TriggerCondition::FuelDepletingFast { .. } => false,
            TriggerCondition::AverageSpeedAbove { .. } => false,
            TriggerCondition::AverageGLoadAbove { .. } => false,
        }
    }
    
    /// Check if condition is met with state history support
    pub fn evaluate_with_history(&self, state: &GameState, history: Option<&StateHistory>) -> bool {
        match self {
            // Always true
            TriggerCondition::AlwaysTrue => true,
            
            // Regular conditions use standard evaluate
            TriggerCondition::SpeedAbove(_) | TriggerCondition::SpeedBelow(_) |
            TriggerCondition::AltitudeAbove(_) | TriggerCondition::AltitudeBelow(_) |
            TriggerCondition::RPMAbove(_) | TriggerCondition::TempAbove(_) |
            TriggerCondition::GLoadAbove(_) | TriggerCondition::GLoadBelow(_) |
            TriggerCondition::AOAAbove(_) | TriggerCondition::AOABelow(_) |
            TriggerCondition::IASAbove(_) | TriggerCondition::TASAbove(_) | TriggerCondition::MachAbove(_) |
            TriggerCondition::FuelBelow(_) | TriggerCondition::FuelTimeBelow(_) |
            TriggerCondition::AmmoBelow(_) |
            TriggerCondition::EngineDamageAbove(_) | TriggerCondition::ControlsDamageAbove(_) |
            TriggerCondition::StabilizerActive | TriggerCondition::StabilizerInactive |
            TriggerCondition::CrewLost | TriggerCondition::CrewMemberDead(_) |
            TriggerCondition::GearAbove(_) | TriggerCondition::GearBelow(_) | TriggerCondition::GearEquals(_) |
            TriggerCondition::CruiseControlAbove(_) | TriggerCondition::CruiseControlBelow(_) |
            TriggerCondition::DrivingForward | TriggerCondition::DrivingBackward => {
                self.evaluate(state)
            },
            
            // Logical conditions need recursive evaluation with history
            TriggerCondition::And(a, b) => {
                a.evaluate_with_history(state, history) && b.evaluate_with_history(state, history)
            },
            TriggerCondition::Or(a, b) => {
                a.evaluate_with_history(state, history) || b.evaluate_with_history(state, history)
            },
            TriggerCondition::Not(cond) => {
                !cond.evaluate_with_history(state, history)
            },
            
            // Temporal conditions require state history
            TriggerCondition::SpeedDroppedBy { threshold, window_seconds } => {
                history.map(|h| h.dropped_by(*threshold, *window_seconds, speed_extractor)).unwrap_or(false)
            },
            TriggerCondition::SpeedIncreasedBy { threshold, window_seconds } => {
                history.map(|h| h.increased_by(*threshold, *window_seconds, speed_extractor)).unwrap_or(false)
            },
            TriggerCondition::AccelerationAbove { threshold, window_seconds } => {
                history.map(|h| {
                    h.rate_of_change(*window_seconds, speed_extractor)
                        .map(|rate| rate > *threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
            TriggerCondition::AccelerationBelow { threshold, window_seconds } => {
                history.map(|h| {
                    h.rate_of_change(*window_seconds, speed_extractor)
                        .map(|rate| rate < -*threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
            TriggerCondition::AltitudeDroppedBy { threshold, window_seconds } => {
                history.map(|h| h.dropped_by(*threshold, *window_seconds, altitude_extractor)).unwrap_or(false)
            },
            TriggerCondition::AltitudeGainedBy { threshold, window_seconds } => {
                history.map(|h| h.increased_by(*threshold, *window_seconds, altitude_extractor)).unwrap_or(false)
            },
            TriggerCondition::ClimbRateAbove { threshold, window_seconds } => {
                history.map(|h| {
                    h.rate_of_change(*window_seconds, altitude_extractor)
                        .map(|rate| rate > *threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
            TriggerCondition::GLoadSpiked { threshold, window_seconds } => {
                history.map(|h| h.increased_by(*threshold, *window_seconds, g_load_extractor)).unwrap_or(false)
            },
            TriggerCondition::SuddenGChange { threshold, window_seconds } => {
                history.map(|h| {
                    let rate = h.rate_of_change(*window_seconds, g_load_extractor).unwrap_or(0.0);
                    rate.abs() > *threshold
                }).unwrap_or(false)
            },
            TriggerCondition::FuelDepletingFast { threshold, window_seconds } => {
                history.map(|h| {
                    h.rate_of_change(*window_seconds, fuel_extractor)
                        .map(|rate| rate < -*threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
            TriggerCondition::AverageSpeedAbove { threshold, window_seconds } => {
                history.map(|h| {
                    h.average(*window_seconds, speed_extractor)
                        .map(|avg| avg > *threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
            TriggerCondition::AverageGLoadAbove { threshold, window_seconds } => {
                history.map(|h| {
                    h.average(*window_seconds, g_load_extractor)
                        .map(|avg| avg > *threshold)
                        .unwrap_or(false)
                }).unwrap_or(false)
            },
        }
    }
}

/// Event Trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTrigger {
    pub id: String,
    pub name: String,
    pub description: String,
    pub condition: TriggerCondition,
    pub event: GameEvent,
    pub cooldown_ms: u64,
    pub enabled: bool,
    pub is_builtin: bool,
    pub pattern: Option<crate::pattern_engine::VibrationPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve_points: Option<Vec<crate::pattern_engine::CurvePoint>>,
    #[serde(default)]
    pub continuous: bool, // If true, vibrates continuously while condition is true
    #[serde(default)]
    pub is_event_based: bool, // If true, only fires on HUD events, not on check_triggers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>, // Filter type: "any", "my_players", "my_clans", "text_contains", "enemy_players", "enemy_clans"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>, // Filter text for "text_contains" mode
}

/// Trigger Manager
pub struct TriggerManager {
    triggers: Vec<EventTrigger>,
    last_triggered: std::collections::HashMap<String, std::time::Instant>,
    pub state_history: StateHistory,  // pub for access from HapticEngine
}

impl TriggerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            triggers: Vec::new(),
            last_triggered: std::collections::HashMap::new(),
            state_history: StateHistory::default(),
        };
        
        manager.load_default_triggers();
        manager
    }
    
    /// Load built-in triggers (common events for all vehicle types)
    fn load_default_triggers(&mut self) {
        // === FUEL WARNINGS (COMMON FOR ALL) ===
        
        // Low fuel (<10%)
        self.triggers.push(EventTrigger {
            id: "low_fuel_10".to_string(),
            name: "Low Fuel <10%".to_string(),
            description: "Triggers when less than 10% fuel remains. Time to return to base!".to_string(),
            condition: TriggerCondition::FuelBelow(10.0),
            event: GameEvent::LowFuel,
            cooldown_ms: 30000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // Critical fuel (<5%)
        self.triggers.push(EventTrigger {
            id: "critical_fuel_5".to_string(),
            name: "Critical Fuel <5%".to_string(),
            description: "Triggers when less than 5% fuel remains. CRITICALLY LOW!".to_string(),
            condition: TriggerCondition::FuelBelow(5.0),
            event: GameEvent::CriticalFuel,
            cooldown_ms: 15000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // === AMMO WARNINGS (COMMON FOR ALL) ===
        
        // Low ammo (<20%)
        self.triggers.push(EventTrigger {
            id: "low_ammo_20".to_string(),
            name: "Low Ammo <20%".to_string(),
            description: "Triggers when less than 20% ammo remains. Conserve ammunition!".to_string(),
            condition: TriggerCondition::AmmoBelow(20.0),
            event: GameEvent::LowAmmo,
            cooldown_ms: 30000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // === ENGINE WARNINGS (COMMON FOR ALL) ===
        
        // Engine damaged
        self.triggers.push(EventTrigger {
            id: "engine_damaged".to_string(),
            name: "Engine Damaged".to_string(),
            description: "Triggers when engine is damaged. Reduced performance.".to_string(),
            condition: TriggerCondition::EngineDamageAbove(0.5),
            event: GameEvent::EngineDamaged,
            cooldown_ms: 10000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // Engine fire
        self.triggers.push(EventTrigger {
            id: "engine_fire".to_string(),
            name: "Engine Fire".to_string(),
            description: "Triggers when engine is on fire. CRITICAL EMERGENCY!".to_string(),
            condition: TriggerCondition::EngineDamageAbove(0.9),
            event: GameEvent::EngineFire,
            cooldown_ms: 5000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // === AIRCRAFT G-LOAD WARNINGS (VEHICLE-SPECIFIC, UPDATED ON VEHICLE CHANGE) ===
        
        // High +G Warning (default 8G, updated from datamine to 80% of vehicle max)
        self.triggers.push(EventTrigger {
            id: "dynamic_high_g".to_string(),
            name: "High G Warning (8.0+ G)".to_string(),
            description: "Approaching positive G-load limit (default 8G, updates per vehicle)".to_string(),
            condition: TriggerCondition::GLoadAbove(8.0),
            event: GameEvent::OverG,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // Negative G Warning (default -4G, updated from datamine to 80% of vehicle max)
        self.triggers.push(EventTrigger {
            id: "dynamic_negative_g".to_string(),
            name: "Negative G Warning (-4.0 G)".to_string(),
            description: "Approaching negative G-load limit (default -4G, updates per vehicle)".to_string(),
            condition: TriggerCondition::GLoadBelow(-4.0),
            event: GameEvent::OverG,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // === AIRCRAFT SPEED WARNINGS (VEHICLE-SPECIFIC, UPDATED ON VEHICLE CHANGE) ===
        
        // Flutter Speed Warning (default 1400 km/h, updated from datamine to 95% of flutter)
        self.triggers.push(EventTrigger {
            id: "dynamic_flutter".to_string(),
            name: "Flutter Warning (1400+ km/h)".to_string(),
            description: "Approaching flutter speed (default 1400 km/h, updates per vehicle)".to_string(),
            condition: TriggerCondition::SpeedAbove(1400.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 3000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // Overspeed Warning (default 1550 km/h, updated from datamine to Vne)
        self.triggers.push(EventTrigger {
            id: "dynamic_overspeed".to_string(),
            name: "Critical Overspeed (1550+ km/h)".to_string(),
            description: "Exceeding maximum speed (default 1550 km/h, updates per vehicle)".to_string(),
            condition: TriggerCondition::SpeedAbove(1550.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 2000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
        
        // === GROUND SPEED WARNING (VEHICLE-SPECIFIC, UPDATED ON VEHICLE CHANGE) ===
        
        // Max Speed Warning for ground vehicles (default 60 km/h, updated from Wiki)
        self.triggers.push(EventTrigger {
            id: "dynamic_ground_maxspeed".to_string(),
            name: "Max Speed (60+ km/h)".to_string(),
            description: "Approaching maximum speed for ground vehicle (default 60 km/h, updates per vehicle)".to_string(),
            condition: TriggerCondition::SpeedAbove(60.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 5000,
            enabled: false,
            is_builtin: true,
            pattern: None,
            curve_points: None,
            continuous: false,
            is_event_based: false,
            filter_type: None,
            filter_text: None,
        });
    }
    
    /// Check all triggers
    pub fn check_triggers(&mut self, state: &GameState) -> Vec<(GameEvent, Option<VibrationPattern>)> {
        let mut events = Vec::new();
        let now = std::time::Instant::now();
        
        // Add current state to history for temporal conditions
        use crate::state_history::StateSnapshot;
        self.state_history.push(StateSnapshot::from_game_state(state));
        
        // Log current game state for debugging
        static mut LAST_LOG_TIME: Option<std::time::Instant> = None;
        let should_log = unsafe {
            if let Some(last) = LAST_LOG_TIME {
                now.duration_since(last).as_secs() >= 2 // Log every 2 seconds
            } else {
                true
            }
        };
        
        if should_log {
            log::error!("[Triggers DEBUG] 🎮 Current state: Speed={:.0}, G-load={:.1}, Alt={:.0}", 
                state.indicators.speed, state.indicators.g_load, state.indicators.altitude);
            log::error!("[Triggers DEBUG] 📋 Active triggers: {}", 
                self.triggers.iter().filter(|t| t.enabled).count());
            unsafe { LAST_LOG_TIME = Some(now); }
        }
        
        for trigger in &self.triggers {
            if !trigger.enabled {
                continue;
            }
            
            // ❌ SKIP event-based triggers - they should ONLY fire on HUD events!
            // Event-based triggers are handled separately in haptic_engine when HUD events occur
            if trigger.is_event_based {
                continue;
            }
            
            // For continuous triggers, skip cooldown check (they fire continuously while condition is true)
            if !trigger.continuous {
                // Check cooldown for one-shot triggers
                if let Some(last_time) = self.last_triggered.get(&trigger.id) {
                    let elapsed = now.duration_since(*last_time).as_millis() as u64;
                    if elapsed < trigger.cooldown_ms {
                        continue;
                    }
                }
            }
            
            // Check condition (with temporal support)
            let result = trigger.condition.evaluate_with_history(state, Some(&self.state_history));
            
            // Debug G-load triggers specifically
            if matches!(trigger.condition, TriggerCondition::GLoadAbove(_) | TriggerCondition::GLoadBelow(_)) {
                log::error!("[Triggers DEBUG] 🎯 G-load trigger '{}': {:?} => {} (current G: {:.2})", 
                    trigger.name, trigger.condition, result, state.indicators.g_load);
            }
            
            if result {
                if trigger.continuous {
                    log::debug!("[Triggers] 🔄 CONTINUOUS: '{}' -> {:?}", trigger.name, trigger.event);
                } else {
                    log::error!("[Triggers DEBUG] ✅ TRIGGERED: '{}' (ID: {}) -> {:?} (condition: {:?})", 
                        trigger.name, trigger.id, trigger.event, trigger.condition);
                }
                
                // Return event and pattern (if exists)
                events.push((trigger.event.clone(), trigger.pattern.clone()));
                
                // For one-shot triggers, update last_triggered
                // For continuous triggers, don't update (will fire every frame while condition is true)
                if !trigger.continuous {
                    self.last_triggered.insert(trigger.id.clone(), now);
                }
            } else if trigger.continuous {
                // Condition is now false for continuous trigger - reset cooldown
                // This allows it to restart when condition becomes true again
                if self.last_triggered.contains_key(&trigger.id) {
                    log::debug!("[Triggers] ⏹️ CONTINUOUS STOPPED: '{}'", trigger.name);
                    self.last_triggered.insert(trigger.id.clone(), now);
                }
            }
        }
        
        if !events.is_empty() {
            log::error!("[Triggers DEBUG] 🎊 Total events triggered: {}", events.len());
        }
        
        events
    }
    
    /// Evaluate trigger condition
    #[allow(dead_code)]
    fn evaluate_condition(&self, condition: &TriggerCondition, state: &GameState) -> bool {
        let ind = &state.indicators;
        
        match condition {
            TriggerCondition::AlwaysTrue => true,
            TriggerCondition::SpeedAbove(val) => {
                let result = ind.speed > *val;
                log::trace!("  SpeedAbove: {} > {} = {}", ind.speed, val, result);
                result
            },
            TriggerCondition::SpeedBelow(val) => ind.speed < *val,
            TriggerCondition::AltitudeAbove(val) => ind.altitude > *val,
            TriggerCondition::AltitudeBelow(val) => ind.altitude < *val,
            TriggerCondition::RPMAbove(val) => ind.engine_rpm > *val,
            TriggerCondition::TempAbove(val) => ind.engine_temp > *val,
            
            TriggerCondition::GLoadAbove(val) => {
                let result = ind.g_load > *val;
                log::trace!("  GLoadAbove: {} > {} = {}", ind.g_load, val, result);
                result
            },
            TriggerCondition::GLoadBelow(val) => ind.g_load < *val,
            
            TriggerCondition::AOAAbove(val) => {
                let result = ind.aoa > *val;
                log::trace!("  AOAAbove: {} > {} = {}", ind.aoa, val, result);
                result
            },
            TriggerCondition::AOABelow(val) => ind.aoa < *val,
            
            TriggerCondition::IASAbove(val) => {
                let result = ind.ias > *val;
                log::trace!("  IASAbove: {} > {} = {}", ind.ias, val, result);
                result
            },
            TriggerCondition::TASAbove(val) => ind.tas > *val,
            TriggerCondition::MachAbove(val) => {
                let result = ind.mach > *val;
                log::trace!("  MachAbove: {} > {} = {}", ind.mach, val, result);
                result
            },
            
            TriggerCondition::FuelBelow(percent) => {
                if ind.fuel_max > 0.0 {
                    (ind.fuel / ind.fuel_max * 100.0) < *percent
                } else {
                    false
                }
            },
            
            TriggerCondition::FuelTimeBelow(minutes) => ind.fuel_time < *minutes,
            
            TriggerCondition::AmmoBelow(percent) => {
                // Simplified check, max ammo should be known
                (ind.ammo_count as f32) < (*percent / 100.0 * 1000.0)
            },
            
            TriggerCondition::EngineDamageAbove(val) => ind.engine_damage > *val,
            TriggerCondition::ControlsDamageAbove(val) => ind.controls_damage > *val,
            
            // Tank-specific
            TriggerCondition::StabilizerActive => ind.stabilizer > 0.5,
            TriggerCondition::StabilizerInactive => ind.stabilizer < 0.5,
            TriggerCondition::CrewLost => ind.crew_current < ind.crew_total,
            TriggerCondition::CrewMemberDead(member) => {
                match member.as_str() {
                    "gunner" => ind.gunner_state > 0,
                    "driver" => ind.driver_state > 0,
                    _ => false,
                }
            },
            TriggerCondition::GearAbove(val) => ind.gear > *val,
            TriggerCondition::GearBelow(val) => ind.gear < *val,
            TriggerCondition::GearEquals(val) => (ind.gear - *val).abs() < 0.1,
            TriggerCondition::CruiseControlAbove(val) => ind.cruise_control > *val,
            TriggerCondition::CruiseControlBelow(val) => ind.cruise_control < *val,
            TriggerCondition::DrivingForward => ind.driving_direction == 0,
            TriggerCondition::DrivingBackward => ind.driving_direction != 0,
            
            TriggerCondition::And(left, right) => {
                self.evaluate_condition(left, state) && self.evaluate_condition(right, state)
            },
            
            TriggerCondition::Or(left, right) => {
                self.evaluate_condition(left, state) || self.evaluate_condition(right, state)
            },
            
            TriggerCondition::Not(inner) => {
                !self.evaluate_condition(inner, state)
            },
            
            // Temporal conditions (not supported in this legacy method)
            TriggerCondition::SpeedDroppedBy { .. } => false,
            TriggerCondition::SpeedIncreasedBy { .. } => false,
            TriggerCondition::AccelerationAbove { .. } => false,
            TriggerCondition::AccelerationBelow { .. } => false,
            TriggerCondition::AltitudeDroppedBy { .. } => false,
            TriggerCondition::AltitudeGainedBy { .. } => false,
            TriggerCondition::ClimbRateAbove { .. } => false,
            TriggerCondition::GLoadSpiked { .. } => false,
            TriggerCondition::SuddenGChange { .. } => false,
            TriggerCondition::FuelDepletingFast { .. } => false,
            TriggerCondition::AverageSpeedAbove { .. } => false,
            TriggerCondition::AverageGLoadAbove { .. } => false,
        }
    }
    
    /// Add custom trigger
    pub fn add_trigger(&mut self, trigger: EventTrigger) {
        // Check if trigger with this ID already exists
        if let Some(existing) = self.triggers.iter_mut().find(|t| t.id == trigger.id) {
            log::error!("[Triggers] 🔄 Updating existing trigger: '{}' -> '{}' (enabled: {})", 
                existing.name, trigger.name, trigger.enabled);
            *existing = trigger;
        } else {
            log::error!("[Triggers] ➕ Adding new trigger: '{}' (enabled: {})", 
                trigger.name, trigger.enabled);
            self.triggers.push(trigger);
            log::error!("[Triggers] 📊 Total triggers now: {}", self.triggers.len());
        }
    }
    
    /// Get all triggers
    pub fn get_triggers(&self) -> &[EventTrigger] {
        &self.triggers
    }
    
    pub fn get_triggers_mut(&mut self) -> &mut Vec<EventTrigger> {
        &mut self.triggers
    }
    
    /// Toggle trigger on/off
    pub fn toggle_trigger(&mut self, id: &str, enabled: bool) {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == id) {
            trigger.enabled = enabled;
        }
    }
    
    /// Clear all cooldowns (for emergency stop)
    pub fn clear_cooldowns(&mut self) {
        self.last_triggered.clear();
        log::info!("[Triggers] ⏱️ All cooldowns cleared");
    }
    
    /// Disable trigger by ID and clear its cooldown
    pub fn disable_trigger(&mut self, trigger_id: &str) -> bool {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == trigger_id) {
            trigger.enabled = false;
            self.last_triggered.remove(trigger_id);
            log::info!("[Triggers] 🚫 Disabled trigger: '{}' ({})", trigger.name, trigger_id);
            true
        } else {
            log::warn!("[Triggers] ⚠️ Trigger not found: {}", trigger_id);
            false
        }
    }
    
    /// Enable trigger by ID
    pub fn enable_trigger(&mut self, trigger_id: &str) -> bool {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == trigger_id) {
            trigger.enabled = true;
            log::info!("[Triggers] ✅ Enabled trigger: '{}' ({})", trigger.name, trigger_id);
            true
        } else {
            log::warn!("[Triggers] ⚠️ Trigger not found: {}", trigger_id);
            false
        }
    }
    
    /// Update trigger settings (cooldown, pattern, etc.)
    #[allow(dead_code)]
    pub fn update_trigger(&mut self, id: &str, cooldown_ms: Option<u64>, pattern: Option<Option<VibrationPattern>>) -> Result<(), String> {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == id) {
            if let Some(cooldown) = cooldown_ms {
                trigger.cooldown_ms = cooldown;
                log::info!("[Triggers] Updated trigger '{}' cooldown to {}ms", id, cooldown);
            }
            
            if let Some(pattern_opt) = pattern {
                trigger.pattern = pattern_opt;
                log::info!("[Triggers] Updated trigger '{}' pattern", id);
            }
            
            Ok(())
        } else {
            Err(format!("Trigger not found: {}", id))
        }
    }
    
    /// Update trigger condition and metadata (for vehicle-specific limits)
    pub fn update_trigger_condition(
        &mut self, 
        id: &str, 
        name: String,
        description: String,
        condition: TriggerCondition
    ) -> Result<(), String> {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == id) {
            trigger.name = name.clone();
            trigger.description = description;
            trigger.condition = condition;
            log::info!("[Triggers] ✅ Updated built-in trigger '{}' with vehicle-specific values", id);
            Ok(())
        } else {
            Err(format!("Trigger not found: {}", id))
        }
    }
    
    /// Update trigger with curve points
    pub fn update_trigger_with_curve(
        &mut self, 
        id: &str, 
        cooldown_ms: Option<u64>, 
        pattern: Option<Option<VibrationPattern>>,
        curve_points: Option<Vec<crate::pattern_engine::CurvePoint>>
    ) -> Result<(), String> {
        if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == id) {
            if let Some(cooldown) = cooldown_ms {
                trigger.cooldown_ms = cooldown;
                log::info!("[Triggers] Updated trigger '{}' cooldown to {}ms", id, cooldown);
            }
            
            if let Some(pattern_opt) = pattern {
                trigger.pattern = pattern_opt;
                trigger.curve_points = curve_points;
                log::info!("[Triggers] Updated trigger '{}' pattern with curve", id);
            } else if curve_points.is_some() {
                // If only curve_points provided without pattern, update them
                trigger.curve_points = curve_points;
                log::info!("[Triggers] Updated trigger '{}' curve points only", id);
            }
            
            Ok(())
        } else {
            Err(format!("Trigger not found: {}", id))
        }
    }
    
    /// Save trigger settings to file
    pub fn save_settings(&self, path: &std::path::Path) -> Result<(), String> {
        // Save only customizable settings (enabled, cooldown, pattern, curve_points)
        let settings: Vec<TriggerSettings> = self.triggers.iter()
            .map(|t| TriggerSettings {
                id: t.id.clone(),
                enabled: t.enabled,
                cooldown_ms: t.cooldown_ms,
                pattern: t.pattern.clone(),
                curve_points: t.curve_points.clone(),
            })
            .collect();
        
        let json = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        
        std::fs::write(path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;
        
        log::info!("[Triggers] Saved {} trigger settings to {:?}", settings.len(), path);
        Ok(())
    }
    
    /// Load trigger settings from file
    pub fn load_settings(&mut self, path: &std::path::Path) -> Result<(), String> {
        if !path.exists() {
            log::info!("[Triggers] Settings file not found, using defaults");
            return Ok(());
        }
        
        let json = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;
        
        let settings: Vec<TriggerSettings> = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse settings: {}", e))?;
        
        // Apply settings to triggers
        for setting in settings {
            if let Some(trigger) = self.triggers.iter_mut().find(|t| t.id == setting.id) {
                trigger.enabled = setting.enabled;
                trigger.cooldown_ms = setting.cooldown_ms;
                trigger.pattern = setting.pattern;
                trigger.curve_points = setting.curve_points;
            }
        }
        
        log::info!("[Triggers] Loaded trigger settings from {:?}", path);
        Ok(())
    }
    
    /// Remove trigger
    pub fn remove_trigger(&mut self, id: &str) -> bool {
        if let Some(pos) = self.triggers.iter().position(|t| t.id == id) {
            self.triggers.remove(pos);
            log::info!("[Triggers] Removed trigger: {}", id);
            true
        } else {
            log::warn!("[Triggers] Trigger not found: {}", id);
            false
        }
    }
    
    /// Check if event-based trigger should fire based on filter
    pub fn should_fire_event_trigger(
        trigger: &EventTrigger,
        entity_name: &str,
        player_names: &[String],
        clan_tags: &[String],
        enemy_names: &[String],
        enemy_clans: &[String],
    ) -> bool {
        match trigger.filter_type.as_deref() {
            None | Some("any") => true,
            
            // For TargetDestroyed (kills), "my_players" means YOU are the attacker
            // wt_telemetry.rs already filtered out other players' kills
            // So if we got this event, it's OUR kill - always pass
            Some("my_players") => {
                if matches!(trigger.event, GameEvent::TargetDestroyed | GameEvent::EnemySetAfire) {
                    // For kill/afire events, entity_name is the VICTIM
                    // If we got this event, WE are the attacker (wt_telemetry filtered it)
                    // So always return true for "my_players" filter
                    log::info!("[HUD] ✅ KILL event - 'my_players' filter passed (you are attacker)");
                    true
                } else {
                    // For other events (TakingDamage, ShotDown), entity_name is the attacker
                    // Check if attacker is us
                    player_names.iter().any(|name| entity_name.contains(name))
                }
            },
            
            Some("my_clans") => {
                if matches!(trigger.event, GameEvent::TargetDestroyed | GameEvent::EnemySetAfire) {
                    // Same logic for clan filter
                    log::info!("[HUD] ✅ KILL event - 'my_clans' filter passed (you are attacker)");
                    true
                } else {
                    clan_tags.iter().any(|tag| entity_name.contains(tag))
                }
            },
            
            Some("enemy_players") => {
                if matches!(trigger.event, GameEvent::TargetDestroyed | GameEvent::EnemySetAfire) {
                    // For kills, check if VICTIM is in enemy list
                    enemy_names.iter().any(|name| entity_name.contains(name))
                } else {
                    // For damage events, check if ATTACKER is in enemy list
                    enemy_names.iter().any(|name| entity_name.contains(name))
                }
            },
            
            Some("enemy_clans") => {
                if matches!(trigger.event, GameEvent::TargetDestroyed | GameEvent::EnemySetAfire) {
                    // For kills, check if VICTIM is in enemy clan list
                    enemy_clans.iter().any(|tag| entity_name.contains(tag))
                } else {
                    // For damage events, check if ATTACKER is in enemy clan list
                    enemy_clans.iter().any(|tag| entity_name.contains(tag))
                }
            },
            
            Some("text_contains") => {
                if let Some(filter_text) = &trigger.filter_text {
                    entity_name.to_lowercase().contains(&filter_text.to_lowercase())
                } else {
                    false
                }
            },
            _ => false,
        }
    }
}

/// Settings for a single trigger (for save/load)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TriggerSettings {
    id: String,
    enabled: bool,
    cooldown_ms: u64,
    pattern: Option<VibrationPattern>,
    #[serde(skip_serializing_if = "Option::is_none")]
    curve_points: Option<Vec<crate::pattern_engine::CurvePoint>>,
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

