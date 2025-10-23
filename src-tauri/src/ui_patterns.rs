/// Интеграция паттернов созданных в UI (React Flow) с Rust движком
use serde::{Deserialize, Serialize};
use crate::event_triggers::{TriggerCondition, EventTrigger};
use crate::pattern_engine::{GameEvent, VibrationPattern, EnvelopeStage, Curve, BurstConfig};

/// Паттерн созданный в UI редакторе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPattern {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub nodes: Vec<UINode>,
    pub edges: Vec<UIEdge>,
}

/// Нода из React Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UINode {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String, // "input" | "vibration"
    pub data: serde_json::Value,
}

/// Связь между нодами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEdge {
    pub source: String,
    pub target: String,
}

impl UIPattern {
    /// Конвертировать UI паттерн в EventTrigger для движка
    pub fn to_trigger(&self) -> Option<EventTrigger> {
        log::info!("[UI Pattern] Converting '{}' to trigger", self.name);
        
        // 1. Найти InputNode (условие триггера)
        let input_node = self.nodes.iter()
            .find(|n| n.type_ == "input")?;
        
        log::debug!("[UI Pattern] Found InputNode: {:?}", input_node.data);
        
        // 2. Parse condition
        let indicator = input_node.data.get("indicator")?.as_str()?;
        let operator = input_node.data.get("operator")?.as_str()?;
        let value = input_node.data.get("value")?.as_f64()? as f32;
        
        let condition = Self::parse_condition(indicator, operator, value)?;
        log::debug!("[UI Pattern] Parsed condition: {:?}", condition);
        
        // 3. Find VibrationNode (vibration pattern)
        let vibration_node = self.nodes.iter()
            .find(|n| n.type_ == "vibration")?;
        
        log::debug!("[UI Pattern] Found VibrationNode: {:?}", vibration_node.data);
        
        // 4. Parse vibration pattern
        let pattern = Self::parse_vibration_pattern(&vibration_node.data)?;
        log::debug!("[UI Pattern] Parsed pattern: {:?}", pattern);
        
        // 5. Create EventTrigger
        Some(EventTrigger {
            id: self.id.clone(),
            name: self.name.clone(),
            description: format!("User pattern: {}", self.name),
            condition,
            event: GameEvent::UserTriggered,  // Universal event for UI triggers
            cooldown_ms: 1000, // Default 1 second
            enabled: self.enabled,
            is_builtin: false,
            pattern: Some(pattern),  // Custom pattern directly in trigger
        })
    }
    
    /// Parse trigger condition from UI
    fn parse_condition(indicator: &str, operator: &str, value: f32) -> Option<TriggerCondition> {
        match (indicator, operator) {
            // Speed
            ("speed", ">") => Some(TriggerCondition::SpeedAbove(value)),
            ("speed", "<") => Some(TriggerCondition::SpeedBelow(value)),
            
            // Altitude
            ("altitude", ">") => Some(TriggerCondition::AltitudeAbove(value)),
            ("altitude", "<") => Some(TriggerCondition::AltitudeBelow(value)),
            
            // Engine RPM
            ("rpm", ">") => Some(TriggerCondition::RPMAbove(value)),
            
            // Temperature
            ("temperature", ">") => Some(TriggerCondition::TempAbove(value)),
            
            // G-load
            ("g_load", ">") => Some(TriggerCondition::GLoadAbove(value)),
            ("g_load", "<") => Some(TriggerCondition::GLoadBelow(value)),
            
            // Angle of attack
            ("aoa", ">") => Some(TriggerCondition::AOAAbove(value)),
            ("aoa", "<") => Some(TriggerCondition::AOABelow(value)),
            
            // IAS (indicated airspeed)
            ("ias", ">") => Some(TriggerCondition::IASAbove(value)),
            
            // TAS (true airspeed)
            ("tas", ">") => Some(TriggerCondition::TASAbove(value)),
            
            // Mach
            ("mach", ">") => Some(TriggerCondition::MachAbove(value)),
            
            // Fuel (percentage)
            ("fuel", "<") => Some(TriggerCondition::FuelBelow(value)),
            
            // Ammo (percentage)
            ("ammo", "<") => Some(TriggerCondition::AmmoBelow(value)),
            
            _ => {
                log::warn!("[UI Pattern] Unknown indicator/operator: {} {}", indicator, operator);
                None
            }
        }
    }
    
    /// Parse vibration pattern from UI
    fn parse_vibration_pattern(data: &serde_json::Value) -> Option<VibrationPattern> {
        // Duration in seconds
        let duration = data.get("duration")?.as_f64()? as u64;
        let duration_ms = duration * 1000;
        
        // Intensity curve
        let curve = data.get("curve")?.as_array()?;
        
        // Get average intensity from all points
        let intensity = if curve.is_empty() {
            0.5 // Default average
        } else {
            let sum: f64 = curve.iter()
                .filter_map(|p| p.get("y")?.as_f64())
                .sum();
            (sum / curve.len() as f64) as f32
        };
        
        // Vibration mode
        let mode = data.get("mode")?.as_str().unwrap_or("once");
        let repeat_count = data.get("repeatCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;
        
        // Создаем простой ADSR паттерн
        // TODO: В будущем можно конвертировать кривую в более сложный паттерн
        let attack_duration = duration_ms / 4;
        let hold_duration = duration_ms / 2;
        let decay_duration = duration_ms / 4;
        
        Some(VibrationPattern {
            name: "UI Custom Pattern".to_string(),  // Имя паттерна
            attack: EnvelopeStage {
                duration_ms: attack_duration,
                start_intensity: 0.0,
                end_intensity: intensity,
                curve: Curve::EaseIn,
            },
            hold: EnvelopeStage {
                duration_ms: hold_duration,
                start_intensity: intensity,
                end_intensity: intensity,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: decay_duration,
                start_intensity: intensity,
                end_intensity: 0.0,
                curve: Curve::EaseOut,
            },
            burst: BurstConfig {
                repeat_count: if mode == "repeat" { repeat_count } else { 1 },
                pause_between_ms: if mode == "continuous" { 0 } else { 100 },
            },
        })
    }
}


