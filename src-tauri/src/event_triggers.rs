/// Event Triggers - Система триггеров для сложных условий
/// Позволяет создавать кастомные события на основе значений индикаторов

use crate::wt_telemetry::GameState;
use crate::pattern_engine::GameEvent;
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
    pub condition: TriggerCondition,
    pub event: GameEvent,
    pub cooldown_ms: u64,       // Минимальное время между срабатываниями
    pub enabled: bool,
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
    
    /// Загрузка встроенных триггеров
    fn load_default_triggers(&mut self) {
        // Превышение макс скорости (для самолетов)
        self.triggers.push(EventTrigger {
            id: "overspeed_800".to_string(),
            name: "Превышение 800 км/ч".to_string(),
            condition: TriggerCondition::IASAbove(800.0),
            event: GameEvent::Overspeed,
            cooldown_ms: 5000,
            enabled: true,
        });
        
        // Критическая G-перегрузка
        self.triggers.push(EventTrigger {
            id: "over_g_10".to_string(),
            name: "G-перегрузка >10G".to_string(),
            condition: TriggerCondition::Or(
                Box::new(TriggerCondition::GLoadAbove(10.0)),
                Box::new(TriggerCondition::GLoadBelow(-5.0)),
            ),
            event: GameEvent::OverG,
            cooldown_ms: 2000,
            enabled: true,
        });
        
        // Высокий угол атаки
        self.triggers.push(EventTrigger {
            id: "high_aoa_15".to_string(),
            name: "Угол атаки >15°".to_string(),
            condition: TriggerCondition::AOAAbove(15.0),
            event: GameEvent::HighAOA,
            cooldown_ms: 3000,
            enabled: true,
        });
        
        // Критический угол атаки
        self.triggers.push(EventTrigger {
            id: "critical_aoa_20".to_string(),
            name: "Критический угол атаки >20°".to_string(),
            condition: TriggerCondition::AOAAbove(20.0),
            event: GameEvent::CriticalAOA,
            cooldown_ms: 2000,
            enabled: true,
        });
        
        // Преодоление звукового барьера
        self.triggers.push(EventTrigger {
            id: "mach_1".to_string(),
            name: "Преодоление Mach 1.0".to_string(),
            condition: TriggerCondition::MachAbove(0.98),
            event: GameEvent::Mach1,
            cooldown_ms: 10000,
            enabled: true,
        });
        
        // Низкое топливо
        self.triggers.push(EventTrigger {
            id: "low_fuel_10".to_string(),
            name: "Топливо <10%".to_string(),
            condition: TriggerCondition::FuelBelow(10.0),
            event: GameEvent::LowFuel,
            cooldown_ms: 30000,
            enabled: true,
        });
        
        // Критическое топливо
        self.triggers.push(EventTrigger {
            id: "critical_fuel_5".to_string(),
            name: "Топливо <5%".to_string(),
            condition: TriggerCondition::FuelBelow(5.0),
            event: GameEvent::CriticalFuel,
            cooldown_ms: 15000,
            enabled: true,
        });
        
        // Низкая высота
        self.triggers.push(EventTrigger {
            id: "low_altitude_100".to_string(),
            name: "Низкая высота <100м".to_string(),
            condition: TriggerCondition::And(
                Box::new(TriggerCondition::AltitudeBelow(100.0)),
                Box::new(TriggerCondition::SpeedAbove(200.0)),
            ),
            event: GameEvent::LowAltitude,
            cooldown_ms: 5000,
            enabled: true,
        });
        
        // Перегрев двигателя
        self.triggers.push(EventTrigger {
            id: "engine_overheat_250".to_string(),
            name: "Перегрев двигателя >250°".to_string(),
            condition: TriggerCondition::TempAbove(250.0),
            event: GameEvent::EngineOverheat,
            cooldown_ms: 10000,
            enabled: true,
        });
        
        // Низкий боезапас
        self.triggers.push(EventTrigger {
            id: "low_ammo_20".to_string(),
            name: "Боезапас <20%".to_string(),
            condition: TriggerCondition::AmmoBelow(20.0),
            event: GameEvent::LowAmmo,
            cooldown_ms: 30000,
            enabled: true,
        });
    }
    
    /// Проверка всех триггеров
    pub fn check_triggers(&mut self, state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();
        let now = std::time::Instant::now();
        
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
            
            // Проверка условия
            if self.evaluate_condition(&trigger.condition, state) {
                events.push(trigger.event.clone());
                self.last_triggered.insert(trigger.id.clone(), now);
            }
        }
        
        events
    }
    
    /// Оценка условия триггера
    fn evaluate_condition(&self, condition: &TriggerCondition, state: &GameState) -> bool {
        let ind = &state.indicators;
        
        match condition {
            TriggerCondition::SpeedAbove(val) => ind.speed > *val,
            TriggerCondition::SpeedBelow(val) => ind.speed < *val,
            TriggerCondition::AltitudeAbove(val) => ind.altitude > *val,
            TriggerCondition::AltitudeBelow(val) => ind.altitude < *val,
            TriggerCondition::RPMAbove(val) => ind.engine_rpm > *val,
            TriggerCondition::TempAbove(val) => ind.engine_temp > *val,
            
            TriggerCondition::GLoadAbove(val) => ind.g_load > *val,
            TriggerCondition::GLoadBelow(val) => ind.g_load < *val,
            
            TriggerCondition::AOAAbove(val) => ind.aoa > *val,
            TriggerCondition::AOABelow(val) => ind.aoa < *val,
            
            TriggerCondition::IASAbove(val) => ind.ias > *val,
            TriggerCondition::TASAbove(val) => ind.tas > *val,
            TriggerCondition::MachAbove(val) => ind.mach > *val,
            
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
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

