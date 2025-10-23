/// Haptic Engine - Главный координатор системы
/// Связывает все модули и управляет потоком данных

use crate::{
    device_manager::DeviceManager,
    event_engine::EventEngine,
    event_triggers::TriggerManager,
    dynamic_triggers::DynamicTriggerManager,
    pattern_engine::VibrationPattern,
    profile_manager::ProfileManager,
    rate_limiter::RateLimiter,
    wt_telemetry::WTTelemetryReader,
};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatusInfo {
    pub connected: bool,
    pub vehicle_name: String,
    pub speed_kmh: i32,
    pub altitude_m: i32,
    pub g_load: f32,
    pub engine_rpm: i32,
    pub fuel_percent: i32,
}

impl GameStatusInfo {
    pub fn disconnected() -> Self {
        Self {
            connected: false,
            vehicle_name: "N/A".to_string(),
            speed_kmh: 0,
            altitude_m: 0,
            g_load: 0.0,
            engine_rpm: 0,
            fuel_percent: 0,
        }
    }
}

pub struct HapticEngine {
    telemetry: Arc<RwLock<WTTelemetryReader>>,
    device_manager: Arc<DeviceManager>,
    event_engine: Arc<RwLock<EventEngine>>,
    profile_manager: Arc<RwLock<ProfileManager>>,
    pub trigger_manager: Arc<RwLock<TriggerManager>>,  // pub для доступа из lib.rs
    dynamic_trigger_manager: Arc<DynamicTriggerManager>,
    rate_limiter: Arc<RateLimiter>,
    running: Arc<RwLock<bool>>,
    current_intensity: Arc<RwLock<f32>>,
}

impl HapticEngine {
    pub fn new() -> Self {
        Self {
            telemetry: Arc::new(RwLock::new(WTTelemetryReader::new())),
            device_manager: Arc::new(DeviceManager::new()),
            event_engine: Arc::new(RwLock::new(EventEngine::new())),
            profile_manager: Arc::new(RwLock::new(ProfileManager::new())),
            trigger_manager: Arc::new(RwLock::new(TriggerManager::new())),
            dynamic_trigger_manager: Arc::new(DynamicTriggerManager::new()),
            rate_limiter: Arc::new(RateLimiter::new()),
            running: Arc::new(RwLock::new(false)),
            current_intensity: Arc::new(RwLock::new(0.0)),
        }
    }

    /// Инициализация устройств
    pub async fn init_devices(&self) -> Result<()> {
        self.device_manager.init_buttplug().await?;
        self.device_manager.scan_devices().await?;
        
        // Даем время на обнаружение устройств
        tokio::time::sleep(Duration::from_secs(3)).await;
        self.device_manager.stop_scanning().await?;
        
        log::info!("Devices initialized");
        Ok(())
    }

    /// Запуск главного цикла
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;
        
        let telemetry = Arc::clone(&self.telemetry);
        let device_manager = Arc::clone(&self.device_manager);
        let event_engine = Arc::clone(&self.event_engine);
        let profile_manager = Arc::clone(&self.profile_manager);
        let trigger_manager = Arc::clone(&self.trigger_manager);
        let dynamic_trigger_manager = Arc::clone(&self.dynamic_trigger_manager);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let running = Arc::clone(&self.running);
        let current_intensity = Arc::clone(&self.current_intensity);

        tokio::spawn(async move {
            let mut tick_interval = interval(WTTelemetryReader::get_poll_interval());
            
            while *running.read().await {
                tick_interval.tick().await;

                // Проверяем подключение к игре
                let game_state = {
                    let mut telem = telemetry.write().await;
                    match telem.get_state().await {
                        Ok(state) => {
                            log::debug!("[WT] Vehicle: {:?}, Speed: {:.0} km/h, Alt: {:.0}m, Fuel: {:.0}/{:.0} kg", 
                                state.type_, 
                                state.indicators.speed,  // Уже в км/ч из API
                                state.indicators.altitude,
                                state.indicators.fuel,
                                state.indicators.fuel_max
                            );
                            state
                        },
                        Err(e) => {
                            // Игра не запущена или не доступна
                            log::warn!("[WT] Game not connected: {}", e);
                            drop(telem);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    }
                };

                // Автоматический выбор профиля
                {
                    let mut pm = profile_manager.write().await;
                    pm.auto_select_profile(&game_state.type_);
                }

                // Детектируем события из state
                let mut events = {
                    let mut ee = event_engine.write().await;
                    ee.detect_events(&game_state)
                };
                
                // Проверяем кастомные триггеры (превышение скорости, G, и т.п.)
                let trigger_events = {
                    let mut tm = trigger_manager.write().await;
                    tm.check_triggers(&game_state)
                };
                
                // Проверяем динамические триггеры (на основе данных о технике)
                let dynamic_events = dynamic_trigger_manager.check_dynamic_triggers(&game_state).await;
                
                events.extend(trigger_events);
                events.extend(dynamic_events);

                // Обрабатываем каждое событие
                if !events.is_empty() {
                    log::info!("[Events] Detected {} events: {:?}", events.len(), events);
                }
                
                for event in events {
                    let pattern = {
                        let pm = profile_manager.read().await;
                        pm.get_pattern_for_event(&event).cloned()
                    };
                    
                    if let Some(pattern) = pattern {
                        log::info!("[Pattern] Executing pattern for event {:?}: Attack={:.0}ms, Hold={:.0}ms", 
                            event, pattern.attack.duration_ms, pattern.hold.duration_ms);
                        Self::execute_pattern_async(
                            Arc::clone(&device_manager),
                            Arc::clone(&rate_limiter),
                            Arc::clone(&current_intensity),
                            pattern,
                        );
                    } else {
                        log::warn!("[Pattern] No pattern found for event {:?}", event);
                    }
                }

                // Проверяем непрерывные события (двигатель и т.д.)
                let continuous = {
                    let ee = event_engine.read().await;
                    ee.check_continuous_events(&game_state)
                };
                
                for _event in continuous {
                    // Для непрерывных событий не спамим, а обновляем плавно
                    if rate_limiter.should_send() {
                        let intensity = *current_intensity.read().await;
                        if let Err(e) = device_manager.send_vibration(intensity * 0.3).await {
                            log::warn!("Failed to send continuous vibration: {}", e);
                        }
                        rate_limiter.mark_sent();
                    }
                }
            }

            // Fail-safe: останавливаем все вибрации при выходе
            let _ = device_manager.stop_all().await;
        });

        log::info!("Haptic engine started");
        Ok(())
    }

    /// Остановка главного цикла
    pub async fn stop(&self) -> Result<()> {
        *self.running.write().await = false;
        self.device_manager.stop_all().await?;
        log::info!("Haptic engine stopped");
        Ok(())
    }

    /// Выполнение паттерна асинхронно
    fn execute_pattern_async(
        device_manager: Arc<DeviceManager>,
        rate_limiter: Arc<RateLimiter>,
        current_intensity: Arc<RwLock<f32>>,
        pattern: VibrationPattern,
    ) {
        tokio::spawn(async move {
            let points = pattern.generate_points(20); // 20 Hz sampling
            
            for (delay, intensity) in points {
                tokio::time::sleep(delay).await;
                
                *current_intensity.write().await = intensity;
                
                if rate_limiter.try_send() {
                    if let Err(e) = device_manager.send_vibration(intensity).await {
                        log::warn!("Failed to execute pattern: {}", e);
                        break;
                    }
                }
            }
            
            // Плавное затухание в конце
            *current_intensity.write().await = 0.0;
            if rate_limiter.try_send() {
                let _ = device_manager.send_vibration(0.0).await;
            }
        });
    }

    /// Получить текущее состояние игры
    pub async fn get_game_status(&self) -> Result<GameStatusInfo> {
        let mut telem = self.telemetry.write().await;
        match telem.get_state().await {
            Ok(state) => {
                // Рассчитываем топливо в процентах (fuel в кг, fuel_max в кг)
                let fuel_percent = if state.indicators.fuel_max > 0.0 {
                    ((state.indicators.fuel / state.indicators.fuel_max * 100.0) as i32).min(100)
                } else {
                    0
                };
                
                Ok(GameStatusInfo {
                    connected: true,
                    vehicle_name: format!("{:?}", state.type_),
                    speed_kmh: state.indicators.speed as i32, // Уже в км/ч
                    altitude_m: state.indicators.altitude as i32,
                    g_load: state.indicators.g_load,
                    engine_rpm: state.indicators.engine_rpm as i32,
                    fuel_percent,
                })
            },
            Err(_) => Ok(GameStatusInfo::disconnected()),
        }
    }

    /// Проверка статуса
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Получение менеджеров (для UI)
    pub fn get_profile_manager(&self) -> Arc<RwLock<ProfileManager>> {
        Arc::clone(&self.profile_manager)
    }

    pub fn get_device_manager(&self) -> Arc<DeviceManager> {
        Arc::clone(&self.device_manager)
    }

    /// Тестовая вибрация
    pub async fn test_vibration(&self, intensity: f32, duration_ms: u64) -> Result<()> {
        self.device_manager.send_vibration(intensity).await?;
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;
        self.device_manager.send_vibration(0.0).await?;
        Ok(())
    }
}

impl Default for HapticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HapticEngine {
    fn drop(&mut self) {
        let running = Arc::clone(&self.running);
        tokio::spawn(async move {
            *running.write().await = false;
        });
    }
}

