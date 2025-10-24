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
            
            TriggerCondition::FuelBelow(threshold) => state.indicators.fuel < *threshold,
            TriggerCondition::FuelTimeBelow(threshold) => state.indicators.fuel_time < *threshold,
            
            TriggerCondition::AmmoBelow(threshold) => (state.indicators.ammo_count as f32) < *threshold,
            
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

/// Триггер события
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTrigger {
    pub id: String,
    pub name: String,
    pub description: String,     // Описание триггера
    pub condition: TriggerCondition,
    pub event: GameEvent,
    pub cooldown_ms: u64,        // Минимальное время между срабатываниями
    pub enabled: bool,
    pub is_builtin: bool,        // Встроенный или пользовательский
    pub pattern: Option<crate::pattern_engine::VibrationPattern>, // Кастомный паттерн для UI триггеров
}

/// Менеджер триггеров
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
        // === COMMON COMBAT EVENTS ===
        
        // Hit (basic)
        self.triggers.push(EventTrigger {
            id: "hit_basic".to_string(),
            name: "Hit Received".to_string(),
            description: "Triggers on any hit received. Basic damage feedback.".to_string(),
            condition: TriggerCondition::SpeedAbove(0.0), // Always true when vehicle is active
            event: GameEvent::Hit,
            cooldown_ms: 500,
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
        });
        
        // Critical Hit (penetration or significant damage)
        self.triggers.push(EventTrigger {
            id: "critical_hit".to_string(),
            name: "Critical Hit".to_string(),
            description: "Triggers on penetration or critical damage. High intensity feedback.".to_string(),
            condition: TriggerCondition::SpeedAbove(0.0),
            event: GameEvent::CriticalHit,
            cooldown_ms: 1000,
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
        });
        
        // === FUEL WARNINGS (COMMON FOR ALL) ===
        
        // Low fuel (<10%)
        self.triggers.push(EventTrigger {
            id: "low_fuel_10".to_string(),
            name: "Low Fuel <10%".to_string(),
            description: "Triggers when less than 10% fuel remains. Time to return to base!".to_string(),
            condition: TriggerCondition::FuelBelow(10.0),
            event: GameEvent::LowFuel,
            cooldown_ms: 30000,
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
        });
        
        // Critical fuel (<5%)
        self.triggers.push(EventTrigger {
            id: "critical_fuel_5".to_string(),
            name: "Critical Fuel <5%".to_string(),
            description: "Triggers when less than 5% fuel remains. CRITICALLY LOW!".to_string(),
            condition: TriggerCondition::FuelBelow(5.0),
            event: GameEvent::CriticalFuel,
            cooldown_ms: 15000,
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
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
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
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
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
        });
        
        // Engine fire
        self.triggers.push(EventTrigger {
            id: "engine_fire".to_string(),
            name: "Engine Fire".to_string(),
            description: "Triggers when engine is on fire. CRITICAL EMERGENCY!".to_string(),
            condition: TriggerCondition::EngineDamageAbove(0.9),
            event: GameEvent::EngineFire,
            cooldown_ms: 5000,
            enabled: true,  // ENABLED - common event
            is_builtin: true,
            pattern: None,
        });
        
        // === AIRCRAFT-SPECIFIC (ADVANCED CONDITIONS) ===
        
        // Overspeed (for aircraft)
        self.triggers.push(EventTrigger {
            id: "overspeed_800".to_string(),
            name: "Overspeed 800 km/h".to_string(),
            description: "Triggers when indicated airspeed exceeds 800 km/h. Critical speed warning.".to_string(),
            condition: TriggerCondition::IASAbove(800.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 5000,
            enabled: true,  // ENABLED - common for aircraft
            is_builtin: true,
            pattern: None,
        });
        
        // Critical G-overload (COMBO: positive OR negative)
        self.triggers.push(EventTrigger {
            id: "over_g_10".to_string(),
            name: "G-Overload >10G".to_string(),
            description: "Triggers at extreme G-load (>10G positive OR >5G negative). Risk of blackout/structural damage!".to_string(),
            condition: TriggerCondition::Or(
                Box::new(TriggerCondition::GLoadAbove(10.0)),
                Box::new(TriggerCondition::GLoadBelow(-5.0)),
            ),
            event: GameEvent::OverG,
            cooldown_ms: 2000,
            enabled: true,  // ENABLED - critical safety
            is_builtin: true,
            pattern: None,
        });
        
        // High angle of attack
        self.triggers.push(EventTrigger {
            id: "high_aoa_15".to_string(),
            name: "High AoA >15°".to_string(),
            description: "Triggers at high angle of attack (>15°). Warning of approaching stall.".to_string(),
            condition: TriggerCondition::AOAAbove(15.0),
            event: GameEvent::HighAOA,
            cooldown_ms: 3000,
            enabled: true,  // ENABLED - safety warning
            is_builtin: true,
            pattern: None,
        });
        
        // Critical angle of attack (COMBO: high AoA AND low speed)
        self.triggers.push(EventTrigger {
            id: "critical_aoa_20".to_string(),
            name: "Critical AoA >20°".to_string(),
            description: "Triggers at critical angle of attack (>20° AND speed <350 km/h). STALL IMMINENT!".to_string(),
            condition: TriggerCondition::And(
                Box::new(TriggerCondition::AOAAbove(20.0)),
                Box::new(TriggerCondition::SpeedBelow(350.0)),
            ),
            event: GameEvent::CriticalAOA,
            cooldown_ms: 2000,
            enabled: true,  // ENABLED - critical warning
            is_builtin: true,
            pattern: None,
        });
        
        // Breaking the sound barrier
        self.triggers.push(EventTrigger {
            id: "mach_1".to_string(),
            name: "Mach 1.0 Breach".to_string(),
            description: "Triggers when reaching sound speed (Mach >0.98). Sonic boom!".to_string(),
            condition: TriggerCondition::MachAbove(0.98),
            event: GameEvent::Mach1,
            cooldown_ms: 10000,
            enabled: true,  // ENABLED - milestone event
            is_builtin: true,
            pattern: None,
        });
        
        // Low altitude warning (COMBO: low altitude AND high speed)
        self.triggers.push(EventTrigger {
            id: "low_altitude_100".to_string(),
            name: "Terrain Proximity <100m".to_string(),
            description: "Triggers at low altitude (<100m AND speed >200 km/h). PULL UP!".to_string(),
            condition: TriggerCondition::And(
                Box::new(TriggerCondition::AltitudeBelow(100.0)),
                Box::new(TriggerCondition::SpeedAbove(200.0)),
            ),
            event: GameEvent::LowAltitude,
            cooldown_ms: 5000,
            enabled: true,  // ENABLED - critical safety
            is_builtin: true,
            pattern: None,
        });
        
        // Engine overheat
        self.triggers.push(EventTrigger {
            id: "engine_overheat_250".to_string(),
            name: "Engine Overheat >250°C".to_string(),
            description: "Triggers when engine temperature exceeds 250°C. Risk of fire!".to_string(),
            condition: TriggerCondition::TempAbove(250.0),
            event: GameEvent::EngineOverheat,
            cooldown_ms: 10000,
            enabled: true,  // ENABLED - safety warning
            is_builtin: true,
            pattern: None,
        });
        
        // === TEMPORAL CONDITIONS (ADVANCED) ===
        
        // Hard braking (speed dropped rapidly)
        self.triggers.push(EventTrigger {
            id: "hard_brake".to_string(),
            name: "Hard Braking".to_string(),
            description: "Speed dropped by 150+ km/h in 1.5 seconds. Emergency stop!".to_string(),
            condition: TriggerCondition::SpeedDroppedBy { threshold: 150.0, window_seconds: 1.5 },
            event: GameEvent::Hit,  // Reuse Hit event for feedback
            cooldown_ms: 3000,
            enabled: true,  // ENABLED - dynamic feedback
            is_builtin: true,
            pattern: None,
        });
        
        // Aggressive maneuver (G-load spiked)
        self.triggers.push(EventTrigger {
            id: "aggressive_turn".to_string(),
            name: "Aggressive Maneuver".to_string(),
            description: "G-load increased by 5G+ in 0.5 seconds. Sharp turn detected!".to_string(),
            condition: TriggerCondition::GLoadSpiked { threshold: 5.0, window_seconds: 0.5 },
            event: GameEvent::OverG,
            cooldown_ms: 2000,
            enabled: true,  // ENABLED - dynamic feedback
            is_builtin: true,
            pattern: None,
        });
        
        // Sustained high speed
        self.triggers.push(EventTrigger {
            id: "sustained_speed".to_string(),
            name: "Sustained High Speed".to_string(),
            description: "Average speed >700 km/h for 5 seconds. Maintaining velocity!".to_string(),
            condition: TriggerCondition::AverageSpeedAbove { threshold: 700.0, window_seconds: 5.0 },
            event: GameEvent::Overspeed,
            cooldown_ms: 10000,
            enabled: false,  // DISABLED - optional feedback
            is_builtin: true,
            pattern: None,
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
            
            // Проверка cooldown
            if let Some(last_time) = self.last_triggered.get(&trigger.id) {
                let elapsed = now.duration_since(*last_time).as_millis() as u64;
                if elapsed < trigger.cooldown_ms {
                    continue;
                }
            }
            
            // Проверка условия (с поддержкой временных условий)
            let result = trigger.condition.evaluate_with_history(state, Some(&self.state_history));
            
            // Debug G-load triggers specifically
            if matches!(trigger.condition, TriggerCondition::GLoadAbove(_) | TriggerCondition::GLoadBelow(_)) {
                log::error!("[Triggers DEBUG] 🎯 G-load trigger '{}': {:?} => {} (current G: {:.2})", 
                    trigger.name, trigger.condition, result, state.indicators.g_load);
            }
            
            if result {
                log::error!("[Triggers DEBUG] ✅ TRIGGERED: '{}' -> {:?}", trigger.name, trigger.event);
                // Возвращаем событие И паттерн (если есть)
                events.push((trigger.event.clone(), trigger.pattern.clone()));
                self.last_triggered.insert(trigger.id.clone(), now);
            }
        }
        
        if !events.is_empty() {
            log::error!("[Triggers DEBUG] 🎊 Total events triggered: {}", events.len());
        }
        
        events
    }
    
    /// Evaluate trigger condition
    fn evaluate_condition(&self, condition: &TriggerCondition, state: &GameState) -> bool {
        let ind = &state.indicators;
        
        match condition {
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
        log::error!("[Triggers] ➕ Adding trigger: '{}' (enabled: {}, condition: {:?})", 
            trigger.name, trigger.enabled, trigger.condition);
        self.triggers.push(trigger);
        log::error!("[Triggers] 📊 Total triggers now: {}", self.triggers.len());
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
    
    /// Удаление триггера
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
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

