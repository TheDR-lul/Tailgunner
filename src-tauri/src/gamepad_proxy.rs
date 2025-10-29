use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumbleState {
    pub left_motor: f32,   // 0.0 - 1.0 (low frequency)
    pub right_motor: f32,  // 0.0 - 1.0 (high frequency)
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadProxyConfig {
    pub enabled: bool,
    pub proxy_to_devices: bool,
    pub sensitivity: f32,
    pub deadzone: f32,
    pub left_motor_weight: f32,
    pub right_motor_weight: f32,
}

impl Default for GamepadProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_to_devices: true,
            sensitivity: 1.0,
            deadzone: 0.05,
            left_motor_weight: 0.7,   // Low frequency = 70% intensity
            right_motor_weight: 1.0,  // High frequency = 100% intensity
        }
    }
}

pub struct GamepadProxy {
    config: Arc<Mutex<GamepadProxyConfig>>,
    last_rumble: Arc<Mutex<RumbleState>>,
    running: Arc<Mutex<bool>>,
}

impl GamepadProxy {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(GamepadProxyConfig::default())),
            last_rumble: Arc::new(Mutex::new(RumbleState {
                left_motor: 0.0,
                right_motor: 0.0,
                timestamp: 0,
            })),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn get_config(&self) -> GamepadProxyConfig {
        self.config.lock().await.clone()
    }

    pub async fn set_config(&self, config: GamepadProxyConfig) {
        *self.config.lock().await = config;
    }

    pub async fn set_enabled(&self, enabled: bool) {
        self.config.lock().await.enabled = enabled;
    }

    pub async fn is_enabled(&self) -> bool {
        self.config.lock().await.enabled
    }

    pub async fn get_rumble_state(&self) -> RumbleState {
        self.last_rumble.lock().await.clone()
    }

    /// Calculate combined intensity from both motors
    pub async fn get_combined_intensity(&self) -> f32 {
        let rumble = self.last_rumble.lock().await;
        let config = self.config.lock().await;
        
        let left = rumble.left_motor * config.left_motor_weight;
        let right = rumble.right_motor * config.right_motor_weight;
        
        // Combine with weighted average
        let combined = (left + right) / (config.left_motor_weight + config.right_motor_weight);
        
        // Apply sensitivity
        let scaled = combined * config.sensitivity;
        
        // Apply deadzone
        if scaled < config.deadzone {
            0.0
        } else {
            // Re-map to 0.0-1.0 range after deadzone
            (scaled - config.deadzone) / (1.0 - config.deadzone)
        }
    }

    /// Start monitoring gamepad rumble
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Ok(());
        }
        
        *running = true;
        
        log::info!("[GamepadProxy] Started gamepad rumble monitoring");
        
        Ok(())
    }

    /// Stop monitoring
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        
        log::info!("[GamepadProxy] Stopped gamepad rumble monitoring");
        
        Ok(())
    }

    /// Manually set rumble state (for testing or external input)
    pub async fn set_rumble(&self, left_motor: f32, right_motor: f32) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        *self.last_rumble.lock().await = RumbleState {
            left_motor: left_motor.clamp(0.0, 1.0),
            right_motor: right_motor.clamp(0.0, 1.0),
            timestamp,
        };
    }

    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }
}

impl Default for GamepadProxy {
    fn default() -> Self {
        Self::new()
    }
}

// Platform-specific gamepad reading implementations
#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    
    // We'll use XInput on Windows
    // For now, this is a placeholder for the actual implementation
    pub async fn read_gamepad_state() -> Option<RumbleState> {
        // TODO: Implement XInput reading
        // This would require:
        // 1. XInput library binding (xinput1_4.dll)
        // 2. Poll XInput state
        // 3. Extract rumble motor values
        // 4. Return normalized 0.0-1.0 values
        
        None
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use super::*;
    
    // For Linux/macOS, we'd use evdev or other platform-specific APIs
    pub async fn read_gamepad_state() -> Option<RumbleState> {
        // TODO: Implement platform-specific reading
        None
    }
}


