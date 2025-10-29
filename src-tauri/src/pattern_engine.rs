/// Pattern Engine - "Sensation Synthesizer"
/// ADSR patterns for creating complex tactile textures

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Curve point for custom vibration curves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvePoint {
    pub x: f32,
    pub y: f32,
}

/// Main vibration pattern (ADSR + burst)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationPattern {
    pub name: String,
    pub attack: EnvelopeStage,   // Instant hit
    pub hold: EnvelopeStage,     // Hold
    pub decay: EnvelopeStage,    // Decay
    pub burst: BurstConfig,      // Repeat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeStage {
    pub duration_ms: u64,
    pub start_intensity: f32,  // 0.0 - 1.0
    pub end_intensity: f32,    // 0.0 - 1.0
    pub curve: Curve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Curve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstConfig {
    pub repeat_count: u32,
    pub pause_between_ms: u64,
}

/// Game Event - Only events that can be detected from War Thunder API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GameEvent {
    // === COMBAT (HUD Events) ===
    Hit,                      // Player hit by enemy (HUD)
    CriticalHit,              // Critical hit (HUD "critically damaged")
    TargetHit,                // Player hit enemy
    TargetDestroyed,          // Player destroyed enemy
    TargetSetOnFire,          // Player set enemy on fire (HUD)
    TargetSeverelyDamaged,    // Player severely damaged enemy (HUD)
    AircraftDestroyed,        // Player shot down aircraft (HUD)
    ShipDestroyed,            // Player destroyed ship (HUD)
    TankDestroyed,            // Player destroyed tank (HUD)
    VehicleDestroyed,         // Player destroyed vehicle (HUD)
    
    // === ENGINE & MOVEMENT (indicators.rpm) ===
    EngineRunning,            // indicators.rpm > 100
    EngineOverheat,           // HUD event
    OilOverheated,            // HUD event
    Shooting,                 // RPM spike pattern detection
    
    // === CREW (indicators.crew_current) ===
    CrewKnocked,              // indicators.crew_current decreased
    
    // === MISSION (/mission.json) ===
    MissionObjectiveCompleted,
    MissionFailed,
    MissionSuccess,
    
    // === MULTIPLAYER (HUD + /gamechat) ===
    Achievement,              // HUD "has achieved"
    ChatMessage,              // /gamechat - any chat message
    TeamChatMessage,          // /gamechat - [Team] messages
    AllChatMessage,           // /gamechat - [All] messages  
    SquadChatMessage,         // /gamechat - [Squad] messages
    EnemyChatMessage,         // /gamechat - enemy messages
    FirstStrike,              // HUD "first strike"
    ShipRescuer,              // HUD "rescuer"
    Assist,                   // HUD "assist"
    BaseCapture,              // /map_obj capture zone tracking
    TeamKill,                 // HUD "team kill"
    PlayerDisconnected,       // /gamechat disconnect message
    
    // === SYSTEM (HUD) ===
    Crashed,                  // HUD "has crashed"
    
    // === CUSTOM TRIGGERS ===
    CustomTrigger(String),    // User-defined event from HUD messages
    UserTriggered,            // Universal event for UI testing
}

impl VibrationPattern {
    /// Create vibration pattern from curve points
    pub fn from_curve_points(curve_points: Vec<CurvePoint>, total_duration_ms: u64) -> Self {
        if curve_points.is_empty() {
            return Self::simple(1.0, total_duration_ms);
        }
        
        // Sort points by x (time)
        let mut points = curve_points;
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        
        // Get start and end points
        let start_point = points.first()
            .expect("Curve must have at least one point");
        let end_point = points.last()
            .expect("Curve must have at least one point");
        
        // Find peak intensity point
        let peak_point = points.iter()
            .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
            .expect("Curve must have at least one point for peak calculation");
        
        // Calculate durations based on point positions
        let attack_duration = (peak_point.x * total_duration_ms as f32) as u64;
        let decay_duration = ((1.0 - peak_point.x) * total_duration_ms as f32) as u64;
        let hold_duration = total_duration_ms.saturating_sub(attack_duration + decay_duration);
        
        Self {
            name: "Custom Curve".to_string(),
            attack: EnvelopeStage {
                duration_ms: attack_duration.max(10),
                start_intensity: start_point.y.clamp(0.0, 1.0),
                end_intensity: peak_point.y.clamp(0.0, 1.0),
                curve: Curve::EaseInOut,
            },
            hold: EnvelopeStage {
                duration_ms: hold_duration,
                start_intensity: peak_point.y.clamp(0.0, 1.0),
                end_intensity: peak_point.y.clamp(0.0, 1.0),
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: decay_duration.max(10),
                start_intensity: peak_point.y.clamp(0.0, 1.0),
                end_intensity: end_point.y.clamp(0.0, 1.0),
                curve: Curve::EaseInOut,
            },
            burst: BurstConfig {
                repeat_count: 1,
                pause_between_ms: 0,
            },
        }
    }

    /// Create simple vibration pattern with intensity and duration
    pub fn simple(intensity: f32, duration_ms: u64) -> Self {
        let intensity = intensity.clamp(0.0, 1.0);
        Self {
            name: "Simple".to_string(),
            attack: EnvelopeStage {
                duration_ms: 50,
                start_intensity: 0.0,
                end_intensity: intensity,
                curve: Curve::Linear,
            },
            hold: EnvelopeStage {
                duration_ms: duration_ms.saturating_sub(100),  // Total - attack - decay
                start_intensity: intensity,
                end_intensity: intensity,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: 50,
                start_intensity: intensity,
                end_intensity: 0.0,
                curve: Curve::Linear,
            },
            burst: BurstConfig {
                repeat_count: 1,
                pause_between_ms: 0,
            },
        }
    }

    /// Create standard presets
    pub fn preset_critical_hit() -> Self {
        Self {
            name: "Critical Hit".to_string(),
            attack: EnvelopeStage {
                duration_ms: 80,
                start_intensity: 0.0,
                end_intensity: 1.0,
                curve: Curve::Linear,
            },
            hold: EnvelopeStage {
                duration_ms: 250,
                start_intensity: 1.0,
                end_intensity: 1.0,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: 400,
                start_intensity: 1.0,
                end_intensity: 0.0,
                curve: Curve::EaseOut,
            },
            burst: BurstConfig {
                repeat_count: 2,
                pause_between_ms: 200,
            },
        }
    }

    pub fn preset_engine_rumble() -> Self {
        Self {
            name: "Engine Rumble".to_string(),
            attack: EnvelopeStage {
                duration_ms: 100,
                start_intensity: 0.0,
                end_intensity: 0.3,
                curve: Curve::EaseIn,
            },
            hold: EnvelopeStage {
                duration_ms: 2000,
                start_intensity: 0.3,
                end_intensity: 0.35,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: 150,
                start_intensity: 0.35,
                end_intensity: 0.3,
                curve: Curve::EaseOut,
            },
            burst: BurstConfig {
                repeat_count: 0,
                pause_between_ms: 0,
            },
        }
    }

    pub fn preset_simple_hit() -> Self {
        Self {
            name: "Simple Hit".to_string(),
            attack: EnvelopeStage {
                duration_ms: 50,
                start_intensity: 0.0,
                end_intensity: 0.7,
                curve: Curve::Linear,
            },
            hold: EnvelopeStage {
                duration_ms: 100,
                start_intensity: 0.7,
                end_intensity: 0.7,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: 200,
                start_intensity: 0.7,
                end_intensity: 0.0,
                curve: Curve::EaseOut,
            },
            burst: BurstConfig {
                repeat_count: 1,
                pause_between_ms: 0,
            },
        }
    }

    pub fn preset_fire() -> Self {
        Self {
            name: "Fire".to_string(),
            attack: EnvelopeStage {
                duration_ms: 150,
                start_intensity: 0.0,
                end_intensity: 0.8,
                curve: Curve::EaseIn,
            },
            hold: EnvelopeStage {
                duration_ms: 300,
                start_intensity: 0.8,
                end_intensity: 0.75,
                curve: Curve::Linear,
            },
            decay: EnvelopeStage {
                duration_ms: 500,
                start_intensity: 0.75,
                end_intensity: 0.4,
                curve: Curve::Linear,
            },
            burst: BurstConfig {
                repeat_count: 3,
                pause_between_ms: 150,
            },
        }
    }

    /// Generate pattern points for smooth interpolation
    pub fn generate_points(&self, sample_rate_hz: u32) -> Vec<(Duration, f32)> {
        let mut points = Vec::new();
        let dt = Duration::from_millis(1000 / sample_rate_hz as u64);
        
        for burst_idx in 0..self.burst.repeat_count {
            let burst_offset = Duration::from_millis(
                (self.attack.duration_ms + self.hold.duration_ms + self.decay.duration_ms + self.burst.pause_between_ms) * burst_idx as u64
            );

            // Attack
            let attack_samples = (self.attack.duration_ms * sample_rate_hz as u64) / 1000;
            for i in 0..attack_samples {
                let t = i as f32 / attack_samples as f32;
                let intensity = interpolate(
                    self.attack.start_intensity,
                    self.attack.end_intensity,
                    t,
                    &self.attack.curve,
                );
                points.push((burst_offset + dt * i as u32, intensity));
            }

            // Hold
            let hold_start = burst_offset + Duration::from_millis(self.attack.duration_ms);
            let hold_samples = (self.hold.duration_ms * sample_rate_hz as u64) / 1000;
            for i in 0..hold_samples {
                let t = i as f32 / hold_samples.max(1) as f32;
                let intensity = interpolate(
                    self.hold.start_intensity,
                    self.hold.end_intensity,
                    t,
                    &self.hold.curve,
                );
                points.push((hold_start + dt * i as u32, intensity));
            }

            // Decay
            let decay_start = hold_start + Duration::from_millis(self.hold.duration_ms);
            let decay_samples = (self.decay.duration_ms * sample_rate_hz as u64) / 1000;
            for i in 0..decay_samples {
                let t = i as f32 / decay_samples as f32;
                let intensity = interpolate(
                    self.decay.start_intensity,
                    self.decay.end_intensity,
                    t,
                    &self.decay.curve,
                );
                points.push((decay_start + dt * i as u32, intensity));
            }
        }

        points
    }
}

fn interpolate(start: f32, end: f32, t: f32, curve: &Curve) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let adjusted_t = match curve {
        Curve::Linear => t,
        Curve::EaseIn => t * t,
        Curve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        Curve::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - 2.0 * (1.0 - t) * (1.0 - t)
            }
        }
    };
    
    start + (end - start) * adjusted_t
}

