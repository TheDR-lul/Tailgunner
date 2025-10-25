/// State History System
/// Tracks historical game state for temporal condition analysis
/// Examples: "speed dropped by 200 km/h over last 2 seconds"

use crate::wt_telemetry::GameState;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Historical snapshot of game state
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    pub timestamp: Instant,
    pub speed: f32,
    pub altitude: f32,
    pub g_load: f32,
    pub aoa: f32,
    pub rpm: f32,
    pub fuel: f32,
}

impl StateSnapshot {
    pub fn from_game_state(state: &GameState) -> Self {
        Self {
            timestamp: Instant::now(),
            speed: state.indicators.speed,
            altitude: state.indicators.altitude,
            g_load: state.indicators.g_load,
            aoa: state.indicators.aoa,
            rpm: state.indicators.engine_rpm,
            fuel: state.indicators.fuel,
        }
    }
}

/// Circular buffer for state history
pub struct StateHistory {
    buffer: VecDeque<StateSnapshot>,
    max_size: usize,
    max_age: Duration,
}

impl StateHistory {
    /// Create new history tracker
    /// max_size: maximum number of snapshots to keep
    /// max_age: maximum age of snapshots (older ones are pruned)
    pub fn new(max_size: usize, max_age: Duration) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
            max_age,
        }
    }
    
    /// Add new state snapshot
    pub fn push(&mut self, snapshot: StateSnapshot) {
        // Remove snapshots older than max_age
        let now = Instant::now();
        while let Some(front) = self.buffer.front() {
            if now.duration_since(front.timestamp) > self.max_age {
                self.buffer.pop_front();
            } else {
                break;
            }
        }
        
        // Add new snapshot
        self.buffer.push_back(snapshot);
        
        // Keep buffer size limited
        if self.buffer.len() > self.max_size {
            self.buffer.pop_front();
        }
    }
    
    /// Get most recent snapshot
    pub fn latest(&self) -> Option<&StateSnapshot> {
        self.buffer.back()
    }
    
    /// Get snapshot from N seconds ago (approximate)
    pub fn get_seconds_ago(&self, seconds: f32) -> Option<&StateSnapshot> {
        let now = Instant::now();
        let target_age = Duration::from_secs_f32(seconds);
        
        // Find snapshot closest to target age
        self.buffer.iter().rev().find(|snapshot| {
            let age = now.duration_since(snapshot.timestamp);
            age >= target_age
        })
    }
    
    /// Calculate rate of change for a metric
    /// Returns delta per second
    pub fn rate_of_change<F>(&self, seconds_ago: f32, extractor: F) -> Option<f32>
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        let current = self.latest()?;
        let past = self.get_seconds_ago(seconds_ago)?;
        
        let current_value = extractor(current);
        let past_value = extractor(past);
        
        let actual_duration = current.timestamp.duration_since(past.timestamp).as_secs_f32();
        
        if actual_duration > 0.0 {
            Some((current_value - past_value) / actual_duration)
        } else {
            None
        }
    }
    
    /// Check if value dropped by threshold over time window
    pub fn dropped_by<F>(&self, threshold: f32, window_seconds: f32, extractor: F) -> bool
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        if let (Some(current), Some(past)) = (self.latest(), self.get_seconds_ago(window_seconds)) {
            let drop = extractor(past) - extractor(current);
            drop >= threshold
        } else {
            false
        }
    }
    
    /// Check if value increased by threshold over time window
    pub fn increased_by<F>(&self, threshold: f32, window_seconds: f32, extractor: F) -> bool
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        if let (Some(current), Some(past)) = (self.latest(), self.get_seconds_ago(window_seconds)) {
            let increase = extractor(current) - extractor(past);
            increase >= threshold
        } else {
            false
        }
    }
    
    /// Get average value over time window
    pub fn average<F>(&self, window_seconds: f32, extractor: F) -> Option<f32>
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        let cutoff = Instant::now() - Duration::from_secs_f32(window_seconds);
        
        let samples: Vec<f32> = self.buffer.iter()
            .filter(|s| s.timestamp >= cutoff)
            .map(&extractor)
            .collect();
        
        if samples.is_empty() {
            None
        } else {
            Some(samples.iter().sum::<f32>() / samples.len() as f32)
        }
    }
    
    /// Get minimum value over time window
    #[allow(dead_code)]
    pub fn min<F>(&self, window_seconds: f32, extractor: F) -> Option<f32>
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        let cutoff = Instant::now() - Duration::from_secs_f32(window_seconds);
        
        self.buffer.iter()
            .filter(|s| s.timestamp >= cutoff)
            .map(&extractor)
            .min_by(|a, b| a.partial_cmp(b).expect("NaN in state history"))
    }
    
    /// Get maximum value over time window
    #[allow(dead_code)]
    pub fn max<F>(&self, window_seconds: f32, extractor: F) -> Option<f32>
    where
        F: Fn(&StateSnapshot) -> f32,
    {
        let cutoff = Instant::now() - Duration::from_secs_f32(window_seconds);
        
        self.buffer.iter()
            .filter(|s| s.timestamp >= cutoff)
            .map(&extractor)
            .max_by(|a, b| a.partial_cmp(b).expect("NaN in state history"))
    }
    
    /// Clear all history
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for StateHistory {
    fn default() -> Self {
        // Default: keep last 100 snapshots, max 10 seconds
        Self::new(100, Duration::from_secs(10))
    }
}

// ===== CONVENIENCE EXTRACTORS =====

pub fn speed_extractor(s: &StateSnapshot) -> f32 { s.speed }
pub fn altitude_extractor(s: &StateSnapshot) -> f32 { s.altitude }
pub fn g_load_extractor(s: &StateSnapshot) -> f32 { s.g_load }
#[allow(dead_code)]
pub fn aoa_extractor(s: &StateSnapshot) -> f32 { s.aoa }
#[allow(dead_code)]
pub fn rpm_extractor(s: &StateSnapshot) -> f32 { s.rpm }
pub fn fuel_extractor(s: &StateSnapshot) -> f32 { s.fuel }


