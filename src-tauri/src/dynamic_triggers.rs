/// Dynamic triggers
/// Automatic adaptation for specific vehicle based on WT Vehicles API

use crate::wt_telemetry::GameState;
use crate::wt_vehicles_api::{WTVehiclesAPI, VehicleLimits, DEFAULT_LIMITS};
use crate::event_triggers::TriggerCondition;
use crate::pattern_engine::GameEvent;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Dynamic trigger (simplified version for internal use)
#[derive(Debug, Clone)]
struct DynamicTrigger {
    id: String,
    name: String,
    condition: TriggerCondition,
    event: GameEvent,
    last_fired: Option<std::time::Instant>,
    cooldown_ms: u64,
}

impl DynamicTrigger {
    fn new(id: &str, name: &str, condition: TriggerCondition, event: GameEvent, cooldown_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            condition,
            event,
            last_fired: None,
            cooldown_ms,
        }
    }
    
    /// Check trigger
    fn check(&mut self, state: &GameState) -> Option<GameEvent> {
        // Check cooldown
        if let Some(last) = self.last_fired {
            if last.elapsed().as_millis() < self.cooldown_ms as u128 {
                return None;
            }
        }
        
        // Check condition
        if self.condition.evaluate(state) {
            self.last_fired = Some(std::time::Instant::now());
            log::debug!("Dynamic trigger '{}' fired", self.name);
            return Some(self.event.clone());
        }
        
        None
    }
}

/// Dynamic Trigger Manager
pub struct DynamicTriggerManager {
    api: WTVehiclesAPI,
    current_vehicle: Arc<RwLock<Option<String>>>,
    current_limits: Arc<RwLock<Option<VehicleLimits>>>,
    dynamic_triggers: Arc<RwLock<Vec<DynamicTrigger>>>,
}

impl DynamicTriggerManager {
    pub fn new() -> Self {
        Self {
            api: WTVehiclesAPI::new(),
            current_vehicle: Arc::new(RwLock::new(None)),
            current_limits: Arc::new(RwLock::new(None)),
            dynamic_triggers: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Update current vehicle and rebuild triggers
    pub async fn update_vehicle(&self, vehicle_identifier: &str) -> anyhow::Result<()> {
        log::info!("Updating vehicle: {}", vehicle_identifier);
        
        // Get vehicle data (from defaults first, then API)
        let limits = self.get_vehicle_limits(vehicle_identifier).await?;
        
        // Save
        {
            let mut current = self.current_vehicle.write().await;
            *current = Some(vehicle_identifier.to_string());
        }
        
        {
            let mut current_limits = self.current_limits.write().await;
            *current_limits = Some(limits.clone());
        }
        
        // Rebuild dynamic triggers
        self.rebuild_dynamic_triggers(&limits).await;
        
        Ok(())
    }
    
    /// Get vehicle limits
    async fn get_vehicle_limits(&self, identifier: &str) -> anyhow::Result<VehicleLimits> {
        // 1. Check defaults (offline)
        if let Some(default) = DEFAULT_LIMITS.get(identifier) {
            log::info!("Using cached limits for {}", identifier);
            return Ok(default.clone());
        }
        
        // 2. Request from API
        match self.api.get_vehicle(identifier).await {
            Ok(vehicle_data) => {
                if let Some(limits) = VehicleLimits::from_vehicle_data(&vehicle_data) {
                    log::info!("Fetched limits from API for {}", identifier);
                    return Ok(limits);
                }
            }
            Err(e) => {
                log::warn!("Failed to fetch vehicle from API: {}", e);
            }
        }
        
        // 3. Use defaults for unknown vehicle
        log::warn!("Using default limits for unknown vehicle: {}", identifier);
        Ok(DEFAULT_LIMITS.get("default").unwrap().clone())
    }
    
    /// Rebuild dynamic triggers based on limits
    async fn rebuild_dynamic_triggers(&self, limits: &VehicleLimits) {
        let mut triggers = Vec::new();
        
        // 🔥 Overspeed: 95% of max speed
        let overspeed_threshold = limits.max_speed_kmh * 0.95;
        triggers.push(DynamicTrigger::new(
            "dynamic_overspeed",
            &format!("Overspeed ({}+ km/h)", overspeed_threshold as i32),
            TriggerCondition::IASAbove(overspeed_threshold),
            GameEvent::Overspeed,
            5000,
        ));
        
        // ⚡ Critical Speed: 99% of max
        let critical_speed = limits.max_speed_kmh * 0.99;
        triggers.push(DynamicTrigger::new(
            "dynamic_critical_speed",
            &format!("Critical Speed ({}+ km/h)", critical_speed as i32),
            TriggerCondition::IASAbove(critical_speed),
            GameEvent::Overspeed,
            3000,
        ));
        
        // 💥 OverG: 90% of max positive G
        let overg_threshold = limits.max_positive_g * 0.90;
        triggers.push(DynamicTrigger::new(
            "dynamic_overg",
            &format!("OverG ({}+ G)", overg_threshold as i32),
            TriggerCondition::GLoadAbove(overg_threshold),
            GameEvent::OverG,
            2000,
        ));
        
        // 🔻 Critical OverG: 100% of max
        triggers.push(DynamicTrigger::new(
            "dynamic_critical_overg",
            &format!("Critical OverG ({}+ G)", limits.max_positive_g as i32),
            TriggerCondition::GLoadAbove(limits.max_positive_g),
            GameEvent::OverG,
            1000,
        ));
        
        // 🔼 Negative G Warning
        let neg_g_threshold = limits.max_negative_g.abs() * 0.90;
        triggers.push(DynamicTrigger::new(
            "dynamic_negative_g",
            &format!("Negative G (-{}+ G)", neg_g_threshold as i32),
            TriggerCondition::GLoadBelow(-neg_g_threshold),
            GameEvent::OverG,
            2000,
        ));
        
        // Save triggers
        {
            let mut dynamic = self.dynamic_triggers.write().await;
            *dynamic = triggers;
        }
        
        log::info!("Rebuilt {} dynamic triggers for {}", 
            self.dynamic_triggers.read().await.len(), 
            limits.identifier
        );
    }
    
    /// Check dynamic triggers
    pub async fn check_dynamic_triggers(&self, state: &GameState) -> Vec<GameEvent> {
        let mut events = Vec::new();
        
        let mut triggers = self.dynamic_triggers.write().await;
        for trigger in triggers.iter_mut() {
            if let Some(event) = trigger.check(state) {
                events.push(event);
            }
        }
        
        events
    }
    
    /// Получить текущие лимиты
    pub async fn get_current_limits(&self) -> Option<VehicleLimits> {
        self.current_limits.read().await.clone()
    }
    
    /// Получить количество активных динамических триггеров
    pub async fn get_trigger_count(&self) -> usize {
        self.dynamic_triggers.read().await.len()
    }
}

impl Default for DynamicTriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dynamic_triggers() {
        let manager = DynamicTriggerManager::new();
        
        // Устанавливаем Bf 109 F-4
        manager.update_vehicle("bf-109f-4").await.unwrap();
        
        // Проверяем, что триггеры созданы
        assert!(manager.get_trigger_count().await > 0);
        
        // Проверяем лимиты
        let limits = manager.get_current_limits().await.unwrap();
        assert_eq!(limits.max_speed_kmh, 635.0);
    }
}

