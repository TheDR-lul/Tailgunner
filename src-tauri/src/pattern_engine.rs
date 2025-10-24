/// Pattern Engine - "Sensation Synthesizer"
/// ADSR patterns for creating complex tactile textures

use serde::{Deserialize, Serialize};
use std::time::Duration;

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

/// Событие из игры (расширенный набор из всех WT API эндпоинтов)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GameEvent {
    // === ПОПАДАНИЯ И УРОН ===
    Hit,
    CriticalHit,
    PenetrationHit,
    Ricochet,
    HitCamera,
    
    // === ПОВРЕЖДЕНИЯ ТЕХНИКИ ===
    // Двигатель
    EngineDestroyed,
    EngineDamaged,
    EngineOverheat,
    EngineFire,
    OilLeak,
    WaterLeak,
    
    // Экипаж
    CrewKnocked,
    PilotKnockedOut,
    GunnerKnockedOut,
    DriverKnockedOut,
    
    // Танк
    TrackBroken,
    TurretJammed,
    GunBreach,
    TransmissionDamaged,
    AmmunitionExploded,
    FuelTankHit,
    
    // Самолет
    WingDamaged,
    TailDamaged,
    ElevatorDamaged,
    RudderDamaged,
    AileronDamaged,
    GearDamaged,
    FlapsDamaged,
    
    // === СОСТОЯНИЯ САМОЛЕТА ===
    Stall,
    Spin,
    FlatSpin,
    Overspeed,            // Превышение макс скорости
    OverG,                // Превышение макс G-перегрузки
    CompressorStall,      // Срыв компрессора
    EngineCompressorDamage,
    
    // Управление
    GearUp,
    GearDown,
    GearStuck,
    FlapsExtended,
    FlapsRetracted,
    AirbrakeDeployed,
    ParachuteDeployed,
    
    // === БОЕВЫЕ ДЕЙСТВИЯ ===
    Shooting,
    CannonFiring,
    MachineGunFiring,
    RocketLaunched,
    BombDropped,
    TorpedoDropped,
    
    // === НЕПРЕРЫВНЫЕ СОСТОЯНИЯ ===
    EngineRunning,
    
    // Попадания игрока
    TargetHit,
    TargetDestroyed,
    TargetCritical,
    AircraftDestroyed,
    TankDestroyed,
    
    // === ТОПЛИВО И БОЕЗАПАС ===
    LowFuel,              // <10%
    CriticalFuel,         // <5%
    OutOfFuel,
    LowAmmo,              // <20%
    OutOfAmmo,
    
    // === АЭРОДИНАМИКА ===
    HighAOA,              // Высокий угол атаки >15°
    CriticalAOA,          // Критический угол >20°
    HighSlip,             // Скольжение
    Mach1,                // Преодоление звукового барьера
    
    // === СИСТЕМА УПРАВЛЕНИЯ ===
    AutopilotEngaged,
    AutopilotDisengaged,
    TrimAdjusted,
    
    // === ОКРУЖЕНИЕ ===
    LowAltitude,          // <100м
    CriticalAltitude,     // <50м
    HighAltitude,         // >5000м
    Touchdown,            // Касание земли
    Landed,
    Takeoff,
    
    // === ЭКИПАЖ И СИСТЕМЫ ===
    FireExtinguished,
    RepairCompleted,
    CrewReplenished,
    
    // === МИССИЯ ===
    MissionStarted,
    MissionObjectiveCompleted,
    MissionFailed,
    MissionSuccess,
    Respawn,
    
    // === MULTIPLAYER ===
    TeamKill,
    Assist,
    BaseCapture,
    
    // === CUSTOM TRIGGERS ===
    CustomTrigger(String),
    UserTriggered,  // Universal event for UI patterns
}

impl VibrationPattern {
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
        
        for burst_idx in 0..=self.burst.repeat_count {
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

