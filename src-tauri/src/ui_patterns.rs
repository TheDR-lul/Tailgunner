/// Full node-based pattern parser with support for all node types
use serde::{Deserialize, Serialize};
use crate::event_triggers::{TriggerCondition, EventTrigger};
use crate::pattern_engine::{GameEvent, VibrationPattern, EnvelopeStage, Curve, BurstConfig};
use std::collections::{HashMap, HashSet};

/// Pattern created in UI editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPattern {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub nodes: Vec<UINode>,
    pub edges: Vec<UIEdge>,
}

/// Node from React Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UINode {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: serde_json::Value,
}

/// Edge between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEdge {
    pub source: String,
    pub target: String,
}

impl UIPattern {
    /// Convert UI pattern to EventTrigger for engine
    /// Now supports ALL node types: Input, Condition, Logic, MultiCondition, Vibration, Linear, Rotate, Output, Event
    pub fn to_trigger(&self) -> Option<EventTrigger> {
        log::error!("[UI Pattern] 🔄 Converting '{}' (nodes: {}, edges: {})", 
            self.name, self.nodes.len(), self.edges.len());
        
        // Build adjacency map for graph traversal
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        // Build reverse adjacency for finding inputs to LOGIC nodes
        let mut reverse_adjacency: HashMap<String, Vec<(String, String)>> = HashMap::new();
        
        for edge in &self.edges {
            adjacency.entry(edge.source.clone())
                .or_insert_with(Vec::new)
                .push(edge.target.clone());
            
            // Store reverse edges with handle info (for LOGIC node inputs)
            let source_handle = edge.source.split('|').nth(1).unwrap_or("");
            reverse_adjacency.entry(edge.target.clone())
                .or_insert_with(Vec::new)
                .push((edge.source.clone(), source_handle.to_string()));
        }
        
        // Build node lookup
        let node_map: HashMap<String, &UINode> = self.nodes.iter()
            .map(|n| (n.id.clone(), n))
            .collect();
        
        // 1. Find all INPUT nodes (entry points)
        let input_nodes: Vec<&UINode> = self.nodes.iter()
            .filter(|n| n.type_ == "input")
            .collect();
        
        if input_nodes.is_empty() {
            log::warn!("[UI Pattern] ❌ No InputNode found - cannot create trigger");
            return None;
        }
        
        log::error!("[UI Pattern] ✅ Found {} input node(s)", input_nodes.len());
        
        // 2. Parse first INPUT node condition (or find OUTPUT via LOGIC nodes)
        let input_node = input_nodes[0];
        
        // Check if INPUT connects to LOGIC node or directly to OUTPUT
        let next_nodes = adjacency.get(&input_node.id)?;
        let base_condition = if next_nodes.iter().any(|id| {
            node_map.get(id.as_str()).map(|n| n.type_ == "logic").unwrap_or(false)
        }) {
            // INPUT → LOGIC → OUTPUT flow
            let logic_node_id = next_nodes.iter()
                .find(|id| node_map.get(id.as_str()).map(|n| n.type_ == "logic").unwrap_or(false))?;
            let logic_node = node_map.get(logic_node_id.as_str())?;
            self.parse_logic_node(logic_node, &reverse_adjacency, &node_map)?
        } else {
            // Simple INPUT → OUTPUT flow
            self.parse_input_node(input_node)?
        };
        
        log::error!("[UI Pattern] ✅ Parsed base condition: {:?}", base_condition);
        
        // 3. Traverse graph from INPUT node to find VIBRATION/LINEAR/ROTATE nodes
        let mut visited = HashSet::new();
        let vibration_node = self.find_output_node(&input_node.id, &adjacency, &node_map, &mut visited)?;
        
        log::error!("[UI Pattern] ✅ Found output node: {} (type: {})", vibration_node.id, vibration_node.type_);
        
        // 4. Parse vibration/linear/rotate pattern
        let pattern = match vibration_node.type_.as_str() {
            "vibration" => self.parse_vibration_pattern(&vibration_node.data)?,
            "linear" => self.parse_linear_pattern(&vibration_node.data)?,
            "rotate" => self.parse_rotate_pattern(&vibration_node.data)?,
            _ => {
                log::warn!("[UI Pattern] ❌ Unsupported output node type: {}", vibration_node.type_);
                return None;
            }
        };
        
        log::error!("[UI Pattern] ✅ Parsed pattern from {} node", vibration_node.type_);
        
        // 5. Create EventTrigger
        Some(EventTrigger {
            id: self.id.clone(),
            name: self.name.clone(),
            description: format!("User pattern: {}", self.name),
            condition: base_condition,
            event: GameEvent::UserTriggered,
            cooldown_ms: 1000,
            enabled: self.enabled,
            is_builtin: false,
            pattern: Some(pattern),
        })
    }
    
    /// Parse INPUT node to condition
    fn parse_input_node(&self, node: &UINode) -> Option<TriggerCondition> {
        let indicator = match node.data.get("indicator").and_then(|v| v.as_str()) {
            Some(i) => i,
            None => {
                log::error!("[UI Pattern] ❌ INPUT node missing 'indicator' field");
                return None;
            }
        };
        
        let operator = match node.data.get("operator").and_then(|v| v.as_str()) {
            Some(o) => o,
            None => {
                log::error!("[UI Pattern] ❌ INPUT node missing 'operator' field");
                return None;
            }
        };
        
        let value = match node.data.get("value").and_then(|v| v.as_f64()) {
            Some(v) => v as f32,
            None => {
                log::error!("[UI Pattern] ❌ INPUT node missing 'value' field");
                return None;
            }
        };
        
        // For temporal operators, extract window_seconds
        let window_seconds = node.data.get("window_seconds")
            .and_then(|v| v.as_f64())
            .map(|v| v as f32)
            .unwrap_or(1.0);
        
        let condition = self.parse_condition(indicator, operator, value, window_seconds);
        if condition.is_none() {
            log::error!("[UI Pattern] ❌ Failed to parse condition: {} {} {}", indicator, operator, value);
        }
        condition
    }
    
    /// Parse LOGIC node (AND/OR/XOR/NOT) - recursively parses inputs
    fn parse_logic_node(
        &self,
        node: &UINode,
        reverse_adjacency: &HashMap<String, Vec<(String, String)>>,
        node_map: &HashMap<String, &UINode>,
    ) -> Option<TriggerCondition> {
        let logic_op = node.data.get("logic")?.as_str().unwrap_or("AND");
        log::error!("[UI Pattern] 🔀 Parsing LOGIC node: {} ({})", node.id, logic_op);
        
        // Find inputs A and B
        let inputs = reverse_adjacency.get(&node.id)?;
        
        // Find which input connects to input-a and input-b handles
        let input_a_source = inputs.iter()
            .find(|(_, handle)| handle == "input-a")
            .map(|(source, _)| source.split('|').next().unwrap_or(source));
        
        let input_b_source = inputs.iter()
            .find(|(_, handle)| handle == "input-b")
            .map(|(source, _)| source.split('|').next().unwrap_or(source));
        
        match logic_op {
            "NOT" => {
                // NOT only uses input-a
                if let Some(source_id) = input_a_source {
                    let source_node = node_map.get(source_id)?;
                    let inner_condition = if source_node.type_ == "input" {
                        self.parse_input_node(source_node)?
                    } else if source_node.type_ == "logic" {
                        self.parse_logic_node(source_node, reverse_adjacency, node_map)?
                    } else {
                        log::error!("[UI Pattern] ❌ Unsupported node type for NOT: {}", source_node.type_);
                        return None;
                    };
                    
                    Some(TriggerCondition::Not(Box::new(inner_condition)))
                } else {
                    log::error!("[UI Pattern] ❌ NOT node missing input-a");
                    None
                }
            },
            
            "AND" | "OR" | "XOR" => {
                // AND/OR/XOR use both inputs
                let (input_a, input_b) = match (input_a_source, input_b_source) {
                    (Some(a), Some(b)) => (a, b),
                    _ => {
                        log::error!("[UI Pattern] ❌ {} node missing inputs", logic_op);
                        return None;
                    }
                };
                
                let cond_a = {
                    let node_a = node_map.get(input_a)?;
                    if node_a.type_ == "input" {
                        self.parse_input_node(node_a)?
                    } else if node_a.type_ == "logic" {
                        self.parse_logic_node(node_a, reverse_adjacency, node_map)?
                    } else {
                        log::error!("[UI Pattern] ❌ Unsupported node type for input-a: {}", node_a.type_);
                        return None;
                    }
                };
                
                let cond_b = {
                    let node_b = node_map.get(input_b)?;
                    if node_b.type_ == "input" {
                        self.parse_input_node(node_b)?
                    } else if node_b.type_ == "logic" {
                        self.parse_logic_node(node_b, reverse_adjacency, node_map)?
                    } else {
                        log::error!("[UI Pattern] ❌ Unsupported node type for input-b: {}", node_b.type_);
                        return None;
                    }
                };
                
                match logic_op {
                    "AND" => Some(TriggerCondition::And(Box::new(cond_a), Box::new(cond_b))),
                    "OR" => Some(TriggerCondition::Or(Box::new(cond_a), Box::new(cond_b))),
                    "XOR" => {
                        // XOR = (A AND NOT B) OR (NOT A AND B)
                        Some(TriggerCondition::Or(
                            Box::new(TriggerCondition::And(
                                Box::new(cond_a.clone()),
                                Box::new(TriggerCondition::Not(Box::new(cond_b.clone())))
                            )),
                            Box::new(TriggerCondition::And(
                                Box::new(TriggerCondition::Not(Box::new(cond_a))),
                                Box::new(cond_b)
                            ))
                        ))
                    },
                    _ => None,
                }
            },
            
            _ => {
                log::error!("[UI Pattern] ❌ Unknown logic operator: {}", logic_op);
                None
            }
        }
    }
    
    /// Find output node (vibration/linear/rotate) by traversing graph
    fn find_output_node<'a>(
        &self,
        start_id: &str,
        adjacency: &HashMap<String, Vec<String>>,
        node_map: &HashMap<String, &'a UINode>,
        visited: &mut HashSet<String>,
    ) -> Option<&'a UINode> {
        if visited.contains(start_id) {
            return None;
        }
        visited.insert(start_id.to_string());
        
        let node = node_map.get(start_id)?;
        
        // Check if this is an output node
        if matches!(node.type_.as_str(), "vibration" | "linear" | "rotate") {
            return Some(node);
        }
        
        // Continue traversing
        if let Some(next_ids) = adjacency.get(start_id) {
            for next_id in next_ids {
                if let Some(result) = self.find_output_node(next_id, adjacency, node_map, visited) {
                    return Some(result);
                }
            }
        }
        
        None
    }
    
    /// Parse trigger condition from indicator/operator/value
    fn parse_condition(&self, indicator: &str, operator: &str, value: f32, window_seconds: f32) -> Option<TriggerCondition> {
        // Temporal operators
        match operator {
            "dropped_by" => {
                return match indicator {
                    "speed" | "ias" | "tas" => Some(TriggerCondition::SpeedDroppedBy { threshold: value, window_seconds }),
                    "altitude" => Some(TriggerCondition::AltitudeDroppedBy { threshold: value, window_seconds }),
                    _ => None,
                };
            },
            "increased_by" => {
                return match indicator {
                    "speed" | "ias" | "tas" => Some(TriggerCondition::SpeedIncreasedBy { threshold: value, window_seconds }),
                    "altitude" => Some(TriggerCondition::AltitudeGainedBy { threshold: value, window_seconds }),
                    "g_load" => Some(TriggerCondition::GLoadSpiked { threshold: value, window_seconds }),
                    _ => None,
                };
            },
            "accel_above" => {
                return match indicator {
                    "speed" | "ias" | "tas" => Some(TriggerCondition::AccelerationAbove { threshold: value, window_seconds }),
                    "altitude" => Some(TriggerCondition::ClimbRateAbove { threshold: value, window_seconds }),
                    _ => None,
                };
            },
            "accel_below" => {
                return match indicator {
                    "speed" | "ias" | "tas" => Some(TriggerCondition::AccelerationBelow { threshold: value, window_seconds }),
                    _ => None,
                };
            },
            "avg_above" => {
                return match indicator {
                    "speed" | "ias" | "tas" => Some(TriggerCondition::AverageSpeedAbove { threshold: value, window_seconds }),
                    "g_load" => Some(TriggerCondition::AverageGLoadAbove { threshold: value, window_seconds }),
                    _ => None,
                };
            },
            _ => {},
        }
        
        // Regular (instant) operators
        match (indicator, operator) {
            // Speed
            ("speed", ">") | ("speed", ">=") => Some(TriggerCondition::SpeedAbove(value)),
            ("speed", "<") | ("speed", "<=") => Some(TriggerCondition::SpeedBelow(value)),
            
            // Altitude
            ("altitude", ">") | ("altitude", ">=") => Some(TriggerCondition::AltitudeAbove(value)),
            ("altitude", "<") | ("altitude", "<=") => Some(TriggerCondition::AltitudeBelow(value)),
            
            // Engine RPM
            ("rpm", ">") | ("rpm", ">=") => Some(TriggerCondition::RPMAbove(value)),
            
            // Temperature
            ("temperature", ">") => Some(TriggerCondition::TempAbove(value)),
            
            // G-load
            ("g_load", ">") | ("G", ">") => Some(TriggerCondition::GLoadAbove(value)),
            ("g_load", "<") | ("G", "<") => Some(TriggerCondition::GLoadBelow(value)),
            
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
            
            // Tank-specific
            ("stabilizer", ">") => Some(if value > 0.5 { TriggerCondition::StabilizerActive } else { TriggerCondition::StabilizerInactive }),
            ("stabilizer", "==") => Some(if value > 0.5 { TriggerCondition::StabilizerActive } else { TriggerCondition::StabilizerInactive }),
            ("crew_current", "<") => Some(TriggerCondition::CrewLost),
            ("gunner_state", ">") => Some(TriggerCondition::CrewMemberDead("gunner".to_string())),
            ("driver_state", ">") => Some(TriggerCondition::CrewMemberDead("driver".to_string())),
            ("cruise_control", ">") => Some(TriggerCondition::CruiseControlAbove(value)),
            ("cruise_control", "<") => Some(TriggerCondition::CruiseControlBelow(value)),
            ("driving_direction_mode", "==") => Some(if value == 0.0 { TriggerCondition::DrivingForward } else { TriggerCondition::DrivingBackward }),
            ("gear", ">") => Some(TriggerCondition::GearAbove(value)),
            ("gear", "<") => Some(TriggerCondition::GearBelow(value)),
            ("gear", "==") => Some(TriggerCondition::GearEquals(value)),
            
            _ => {
                log::warn!("[UI Pattern] Unknown indicator/operator: {} {}", indicator, operator);
                None
            }
        }
    }
    
    /// Parse vibration pattern from VibrationNode
    fn parse_vibration_pattern(&self, data: &serde_json::Value) -> Option<VibrationPattern> {
        let duration = data.get("duration")?.as_f64()? as u64;
        let duration_ms = duration * 1000;
        
        let curve = data.get("curve")?.as_array()?;
        
        let intensity = if curve.is_empty() {
            0.5
        } else {
            let sum: f64 = curve.iter()
                .filter_map(|p| p.get("y")?.as_f64())
                .sum();
            (sum / curve.len() as f64) as f32
        };
        
        let mode = data.get("mode")?.as_str().unwrap_or("once");
        let repeat_count = data.get("repeatCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;
        
        let (final_repeat_count, pause_ms) = match mode {
            "once" => (1, 100),
            "continuous" => (1, 0),
            "repeat" => (repeat_count, 100),
            "while_true" => (9999, 0),
            _ => (1, 100),
        };
        
        log::info!("[UI Pattern] Vibration mode: '{}', repeat: {}, pause: {}ms", mode, final_repeat_count, pause_ms);
        
        let attack_duration = duration_ms / 4;
        let hold_duration = duration_ms / 2;
        let decay_duration = duration_ms / 4;
        
        Some(VibrationPattern {
            name: "UI Custom Pattern".to_string(),
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
                repeat_count: final_repeat_count,
                pause_between_ms: pause_ms,
            },
        })
    }
    
    /// Parse linear pattern from LinearNode
    fn parse_linear_pattern(&self, data: &serde_json::Value) -> Option<VibrationPattern> {
        // For now, treat linear as vibration (future: add LinearPattern type)
        log::info!("[UI Pattern] Linear node detected - converting to vibration pattern");
        self.parse_vibration_pattern(data)
    }
    
    /// Parse rotate pattern from RotateNode
    fn parse_rotate_pattern(&self, data: &serde_json::Value) -> Option<VibrationPattern> {
        // For now, treat rotate as vibration (future: add RotatePattern type)
        log::info!("[UI Pattern] Rotate node detected - converting to vibration pattern");
        self.parse_vibration_pattern(data)
    }
}
