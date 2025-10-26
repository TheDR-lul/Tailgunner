/// Rate Limiter
/// Limit command rate to devices (QoS)

use std::sync::Mutex;
use std::time::{Duration, Instant};

const MAX_COMMANDS_PER_SECOND: u32 = 8;
const MIN_INTERVAL_MS: u64 = 1000 / MAX_COMMANDS_PER_SECOND as u64;

pub struct RateLimiter {
    last_send: Mutex<Instant>,
    min_interval: Duration,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            last_send: Mutex::new(Instant::now() - Duration::from_secs(1)),
            min_interval: Duration::from_millis(MIN_INTERVAL_MS),
        }
    }

    /// Check if command can be sent
    pub fn should_send(&self) -> bool {
        let now = Instant::now();
        let last = *self.last_send.lock()
            .expect("RateLimiter mutex poisoned");
        
        now.duration_since(last) >= self.min_interval
    }

    /// Mark that command was sent
    pub fn mark_sent(&self) {
        *self.last_send.lock()
            .expect("RateLimiter mutex poisoned") = Instant::now();
    }

    /// Attempt to send (returns true if allowed)
    pub fn try_send(&self) -> bool {
        if self.should_send() {
            self.mark_sent();
            true
        } else {
            false
        }
    }

    /// Time until next send is possible
    #[allow(dead_code)]
    pub fn time_until_next(&self) -> Duration {
        let now = Instant::now();
        let last = *self.last_send.lock()
            .expect("RateLimiter mutex poisoned");
        let elapsed = now.duration_since(last);
        
        if elapsed >= self.min_interval {
            Duration::ZERO
        } else {
            self.min_interval - elapsed
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_rate_limiting() {
        let limiter = RateLimiter::new();
        
        // First send should succeed
        assert!(limiter.try_send());
        
        // Immediate retry should be blocked
        assert!(!limiter.try_send());
        
        // Wait and try again
        thread::sleep(Duration::from_millis(MIN_INTERVAL_MS + 10));
        assert!(limiter.try_send());
    }
}

