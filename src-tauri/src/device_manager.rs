/// Device Manager
/// Управление вибро-устройствами через Buttplug.io и Lovense API

use anyhow::{Result, Context};
use buttplug::client::ButtplugClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: u32,
    pub name: String,
    pub device_type: DeviceType,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Buttplug,
    Lovense,
}

pub struct DeviceManager {
    buttplug_client: Arc<RwLock<Option<ButtplugClient>>>,
    devices: Arc<RwLock<Vec<DeviceInfo>>>,
    lovense_enabled: bool,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            buttplug_client: Arc::new(RwLock::new(None)),
            devices: Arc::new(RwLock::new(Vec::new())),
            lovense_enabled: false,
        }
    }

    /// Инициализация Buttplug клиента
    pub async fn init_buttplug(&self) -> Result<()> {
        use buttplug::core::connector::ButtplugInProcessClientConnector;
        
        // Проверяем, не подключен ли уже клиент
        if self.buttplug_client.read().await.is_some() {
            log::info!("Buttplug client already initialized");
            return Ok(());
        }
        
        let client = ButtplugClient::new("Haptic Feedback System");
        let connector = ButtplugInProcessClientConnector::default();
        
        match client.connect(connector).await {
            Ok(_) => {
                log::info!("✅ Buttplug client connected successfully");
                *self.buttplug_client.write().await = Some(client);
                Ok(())
            }
            Err(e) => {
                log::warn!("⚠️ Buttplug connection failed: {}", e);
                log::info!("💡 Запустите Intiface Central для подключения устройств");
                Err(anyhow::anyhow!("Buttplug connection failed: {}", e))
            }
        }
    }

    /// Сканирование устройств
    pub async fn scan_devices(&self) -> Result<()> {
        if let Some(client) = self.buttplug_client.read().await.as_ref() {
            client.start_scanning().await
                .context("Failed to start device scanning")?;
            
            log::info!("Scanning for devices...");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Buttplug client not initialized"))
        }
    }

    /// Остановка сканирования
    pub async fn stop_scanning(&self) -> Result<()> {
        if let Some(client) = self.buttplug_client.read().await.as_ref() {
            client.stop_scanning().await
                .context("Failed to stop scanning")?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Buttplug client not initialized"))
        }
    }

    /// Получение списка подключенных устройств
    pub async fn get_devices(&self) -> Vec<DeviceInfo> {
        let mut devices = Vec::new();
        
        if let Some(client) = self.buttplug_client.read().await.as_ref() {
            for device in client.devices() {
                devices.push(DeviceInfo {
                    id: device.index(),
                    name: device.name().to_string(),
                    device_type: DeviceType::Buttplug,
                    connected: true,
                });
            }
        }

        *self.devices.write().await = devices.clone();
        devices
    }

    /// Отправка команды вибрации на все устройства
    pub async fn send_vibration(&self, intensity: f32) -> Result<()> {
        let intensity = intensity.clamp(0.0, 1.0);

        let guard = self.buttplug_client.read().await;
        if let Some(client) = guard.as_ref() {
            for device in client.devices() {
                if let Err(e) = device.vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(intensity.into())).await {
                    log::warn!("Failed to send vibration to {}: {}", device.name(), e);
                }
            }
        }

        Ok(())
    }

    /// Остановка всех вибраций (Fail-Safe)
    pub async fn stop_all(&self) -> Result<()> {
        self.send_vibration(0.0).await
    }

    /// Проверка подключения
    pub async fn is_connected(&self) -> bool {
        self.buttplug_client.read().await.is_some()
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DeviceManager {
    fn drop(&mut self) {
        // Fail-safe: останавливаем все вибрации при выходе
        let client_lock = Arc::clone(&self.buttplug_client);
        tokio::spawn(async move {
            if let Some(client) = client_lock.write().await.take() {
                for device in client.devices() {
                    let _ = device.vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(0.0.into())).await;
                }
                let _ = client.disconnect().await;
            }
        });
    }
}

