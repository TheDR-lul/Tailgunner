/// Event Triggers - Система триггеров для сложных условий
/// Позволяет создавать кастомные события на основе значений индикаторов

use crate::wt_telemetry::GameState;
use crate::pattern_engine::{GameEvent, VibrationPattern};
use serde::{Deserialize, Serialize};

/// Условие триггера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    // Сравнение значений
    SpeedAbove(f32),
    SpeedBelow(f32),
    AltitudeAbove(f32),
    AltitudeBelow(f32),
    RPMAbove(f32),
    TempAbove(f32),
    
    // G-перегрузки
    GLoadAbove(f32),
    GLoadBelow(f32),
    
    // Угол атаки
    AOAAbove(f32),
    AOABelow(f32),
    
    // Скорость (разные типы)
    IASAbove(f32),
    TASAbove(f32),
    MachAbove(f32),
    
    // Топливо
    FuelBelow(f32),        // процент
    FuelTimeBelow(f32),    // минуты
    
    // Боезапас
    AmmoBelow(f32),        // процент
    
    // Повреждения
    EngineDamageAbove(f32),
    ControlsDamageAbove(f32),
    
    // Логические
    And(Box<TriggerCondition>, Box<TriggerCondition>),
    Or(Box<TriggerCondition>, Box<TriggerCondition>),
    Not(Box<TriggerCondition>),
}

impl TriggerCondition {
    /// Проверяет, выполняется ли условие на основе текущего GameState
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
            
            TriggerCondition::And(a, b) => a.evaluate(state) && b.evaluate(state),
            TriggerCondition::Or(a, b) => a.evaluate(state) || b.evaluate(state),
            TriggerCondition::Not(cond) => !cond.evaluate(state),
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
}

impl TriggerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            triggers: Vec::new(),
            last_triggered: std::collections::HashMap::new(),
        };
        
        manager.load_default_triggers();
        manager
    }
    
    /// Load built-in triggers
    fn load_default_triggers(&mut self) {
        // Overspeed (for aircraft)
        self.triggers.push(EventTrigger {
            id: "overspeed_800".to_string(),
            name: "Overspeed 800 km/h".to_string(),
            description: "Triggers when indicated airspeed exceeds 800 km/h. Used for critical speed warning.".to_string(),
            condition: TriggerCondition::IASAbove(800.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 5000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Critical G-overload
        self.triggers.push(EventTrigger {
            id: "over_g_10".to_string(),
            name: "G-Overload >10G".to_string(),
            description: "Triggers at extreme G-load (>10G positive or >5G negative). Warning of stall risk or structural damage.".to_string(),
            condition: TriggerCondition::Or(
                Box::new(TriggerCondition::GLoadAbove(10.0)),
                Box::new(TriggerCondition::GLoadBelow(-5.0)),
            ),
            event: GameEvent::OverG,
            cooldown_ms: 2000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // High angle of attack
        self.triggers.push(EventTrigger {
            id: "high_aoa_15".to_string(),
            name: "Angle of Attack >15°".to_string(),
            description: "Triggers at high angle of attack (>15°). Warning of approaching stall.".to_string(),
            condition: TriggerCondition::AOAAbove(15.0),
            event: GameEvent::HighAOA,
            cooldown_ms: 3000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Critical angle of attack
        self.triggers.push(EventTrigger {
            id: "critical_aoa_20".to_string(),
            name: "Critical Angle of Attack >20°".to_string(),
            description: "Triggers at critical angle of attack (>20°). High risk of stall!".to_string(),
            condition: TriggerCondition::AOAAbove(20.0),
            event: GameEvent::CriticalAOA,
            cooldown_ms: 2000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Breaking the sound barrier
        self.triggers.push(EventTrigger {
            id: "mach_1".to_string(),
            name: "Breaking Mach 1.0".to_string(),
            description: "Triggers when reaching sound speed (Mach >0.98). Feel the sonic boom!".to_string(),
            condition: TriggerCondition::MachAbove(0.98),
            event: GameEvent::Mach1,
            cooldown_ms: 10000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Low fuel
        self.triggers.push(EventTrigger {
            id: "low_fuel_10".to_string(),
            name: "Fuel <10%".to_string(),
            description: "Triggers when less than 10% fuel remains. Time to return to base!".to_string(),
            condition: TriggerCondition::FuelBelow(10.0),
            event: GameEvent::LowFuel,
            cooldown_ms: 30000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Critical fuel
        self.triggers.push(EventTrigger {
            id: "critical_fuel_5".to_string(),
            name: "Fuel <5%".to_string(),
            description: "Triggers when less than 5% fuel remains. CRITICALLY LOW FUEL!".to_string(),
            condition: TriggerCondition::FuelBelow(5.0),
            event: GameEvent::CriticalFuel,
            cooldown_ms: 15000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Low altitude
        self.triggers.push(EventTrigger {
            id: "low_altitude_100".to_string(),
            name: "Low Altitude <100m".to_string(),
            description: "Triggers at dangerously low altitude (<100m) at speed >200 km/h. Watch out, ground is close!".to_string(),
            condition: TriggerCondition::And(
                Box::new(TriggerCondition::AltitudeBelow(100.0)),
                Box::new(TriggerCondition::SpeedAbove(200.0)),
            ),
            event: GameEvent::LowAltitude,
            cooldown_ms: 5000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Engine overheat
        self.triggers.push(EventTrigger {
            id: "engine_overheat_250".to_string(),
            name: "Engine Overheat >250°".to_string(),
            description: "Triggers when engine temperature exceeds 250°C. Risk of engine damage!".to_string(),
            condition: TriggerCondition::TempAbove(250.0),
            event: GameEvent::EngineOverheat,
            cooldown_ms: 10000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
        
        // Low ammo
        self.triggers.push(EventTrigger {
            id: "low_ammo_20".to_string(),
            name: "Ammo <20%".to_string(),
            description: "Triggers when less than 20% ammo remains. Conserve ammunition!".to_string(),
            condition: TriggerCondition::AmmoBelow(20.0),
            event: GameEvent::LowAmmo,
            cooldown_ms: 30000,
            enabled: false,  // Disabled by default
            is_builtin: true,
            pattern: None,   // Built-in triggers use patterns from ProfileManager
        });
    }
    
    /// Check all triggers
    pub fn check_triggers(&mut self, state: &GameState) -> Vec<(GameEvent, Option<VibrationPattern>)> {
        let mut events = Vec::new();
        let now = std::time::Instant::now();
        
        log::trace!("[Triggers] Checking {} triggers", self.triggers.len());
        
        for trigger in &self.triggers {
            if !trigger.enabled {
                log::trace!("[Triggers] Skipping disabled trigger: {}", trigger.name);
                continue;
            }
            
            // Проверка cooldown
            if let Some(last_time) = self.last_triggered.get(&trigger.id) {
                let elapsed = now.duration_since(*last_time).as_millis() as u64;
                if elapsed < trigger.cooldown_ms {
                    log::trace!("[Triggers] {} on cooldown ({}/{}ms)", trigger.name, elapsed, trigger.cooldown_ms);
                    continue;
                }
            }
            
            // Проверка условия
            let result = self.evaluate_condition(&trigger.condition, state);
            log::debug!("[Triggers] {} - Condition: {:?}, Result: {}", trigger.name, trigger.condition, result);
            
            if result {
                log::info!("[Triggers] ✅ TRIGGERED: {} -> {:?}", trigger.name, trigger.event);
                // Возвращаем событие И паттерн (если есть)
                events.push((trigger.event.clone(), trigger.pattern.clone()));
                self.last_triggered.insert(trigger.id.clone(), now);
            }
        }
        
        if !events.is_empty() {
            log::info!("[Triggers] Total events triggered: {}", events.len());
        }
        
        events
    }
    
    /// Оценка условия триггера
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
                // Упрощенная проверка, нужно знать максимум
                (ind.ammo_count as f32) < (*percent / 100.0 * 1000.0)
            },
            
            TriggerCondition::EngineDamageAbove(val) => ind.engine_damage > *val,
            TriggerCondition::ControlsDamageAbove(val) => ind.controls_damage > *val,
            
            TriggerCondition::And(left, right) => {
                self.evaluate_condition(left, state) && self.evaluate_condition(right, state)
            },
            
            TriggerCondition::Or(left, right) => {
                self.evaluate_condition(left, state) || self.evaluate_condition(right, state)
            },
            
            TriggerCondition::Not(inner) => {
                !self.evaluate_condition(inner, state)
            },
        }
    }
    
    /// Добавление кастомного триггера
    pub fn add_trigger(&mut self, trigger: EventTrigger) {
        self.triggers.push(trigger);
    }
    
    /// Получение всех триггеров
    pub fn get_triggers(&self) -> &[EventTrigger] {
        &self.triggers
    }
    
    /// Включение/выключение триггера
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

