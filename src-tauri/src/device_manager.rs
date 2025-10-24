/// Device Manager
/// Manage haptic devices via Buttplug.io and Lovense API

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

    /// Initialize Buttplug client
    pub async fn init_buttplug(&self) -> Result<()> {
        use buttplug::core::connector::ButtplugWebsocketClientTransport;
        use buttplug::core::connector::ButtplugRemoteClientConnector;
        
        // Check if client is already connected
        if self.buttplug_client.read().await.is_some() {
            log::info!("Buttplug client already initialized");
            return Ok(());
        }
        
        let client = ButtplugClient::new("Butt Thunder");
        
        // Try to connect to Intiface Central via WebSocket
        let ws_url = "ws://127.0.0.1:12345";
        log::info!("🔌 Connecting to Intiface Central: {}", ws_url);
        
        let transport = ButtplugWebsocketClientTransport::new_insecure_connector(ws_url);
        let connector = ButtplugRemoteClientConnector::<ButtplugWebsocketClientTransport>::new(transport);
        
        match client.connect(connector).await {
            Ok(_) => {
                log::info!("✅ Connected to Intiface Central");
                
                // Automatically start device scanning
                match client.start_scanning().await {
                    Ok(_) => {
                        log::info!("🔍 Started device scanning...");
                        
                        // Wait 2 seconds for devices to connect
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        
                        let device_count = client.devices().len();
                        log::info!("📱 Devices found: {}", device_count);
                        
                        for device in client.devices() {
                            log::info!("  ✓ {} (index: {})", device.name(), device.index());
                        }
                    }
                    Err(e) => log::warn!("⚠️ Failed to start device scanning: {}", e),
                }
                
                *self.buttplug_client.write().await = Some(client);
                Ok(())
            }
            Err(e) => {
                log::error!("❌ Failed to connect to Intiface Central: {}", e);
                log::info!("💡 Make sure that:");
                log::info!("   1. Intiface Central is running");
                log::info!("   2. WebSocket server is active on ws://127.0.0.1:12345");
                log::info!("   3. 'Start Server Automatically' is enabled in Intiface settings");
                Err(anyhow::anyhow!("Buttplug connection failed: {}", e))
            }
        }
    }

    /// Scan for devices
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

    /// Stop device scanning
    pub async fn stop_scanning(&self) -> Result<()> {
        if let Some(client) = self.buttplug_client.read().await.as_ref() {
            client.stop_scanning().await
                .context("Failed to stop scanning")?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Buttplug client not initialized"))
        }
    }

    /// Get list of connected devices
    pub async fn get_devices(&self) -> Vec<DeviceInfo> {
        let mut devices = Vec::new();
        
        if let Some(client) = self.buttplug_client.read().await.as_ref() {
            let client_devices = client.devices();
            log::info!("📱 Devices found: {}", client_devices.len());
            
            for device in client_devices {
                log::info!("  → {} (index: {}, type: {})", 
                    device.name(), 
                    device.index(),
                    if device.vibrate_attributes().is_empty() { "no vibrate" } else { "vibrate OK" }
                );
                
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

    /// Send vibration command to all devices
    pub async fn send_vibration(&self, intensity: f32) -> Result<()> {
        let intensity = intensity.clamp(0.0, 1.0);

        let guard = self.buttplug_client.read().await;
        if let Some(client) = guard.as_ref() {
            let devices = client.devices();
            
            if devices.is_empty() {
                log::warn!("⚠️ No connected devices! Start scanning.");
                return Ok(());
            }
            
            log::info!("🎮 Sending vibration {} to {} devices", intensity, devices.len());
            
            for device in devices {
                log::info!("  → {} (index: {})", device.name(), device.index());
                
                match device.vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(intensity.into())).await {
                    Ok(_) => log::info!("    ✅ Success"),
                    Err(e) => log::error!("    ❌ Error: {}", e),
                }
            }
        } else {
            log::error!("❌ Buttplug client not initialized!");
            return Err(anyhow::anyhow!("Buttplug client not initialized"));
        }

        Ok(())
    }

    /// Stop all vibrations (Fail-Safe)
    pub async fn stop_all(&self) -> Result<()> {
        self.send_vibration(0.0).await
    }

    /// Check connection status
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
        // Fail-safe: stop all vibrations on exit
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

