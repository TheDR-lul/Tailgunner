/// Device Manager
/// Manage haptic devices via Buttplug.io and Lovense API

use anyhow::{Result, Context};
use buttplug::client::ButtplugClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct LovenseDevice {
    id: String,
    name: String,
    ip_address: String,
    port: u16,
}

pub struct DeviceManager {
    buttplug_client: Arc<RwLock<Option<ButtplugClient>>>,
    devices: Arc<RwLock<Vec<DeviceInfo>>>,
    lovense_devices: Arc<RwLock<HashMap<String, LovenseDevice>>>,
    lovense_enabled: bool,
    http_client: reqwest::Client,
}

impl DeviceManager {
    pub fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            buttplug_client: Arc::new(RwLock::new(None)),
            devices: Arc::new(RwLock::new(Vec::new())),
            lovense_devices: Arc::new(RwLock::new(HashMap::new())),
            lovense_enabled: false,
            http_client,
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
            
            // Try to open Device History DB for saving
            let mut db_opt = match crate::device_history_db::DeviceHistoryDB::new() {
                Ok(db) => Some(db),
                Err(e) => {
                    log::warn!("[Device Manager] Failed to open Device History DB: {}", e);
                    None
                }
            };
            
            for device in client_devices {
                log::info!("  → {} (index: {}, type: {})", 
                    device.name(), 
                    device.index(),
                    if device.vibrate_attributes().is_empty() { "no vibrate" } else { "vibrate OK" }
                );
                
                let device_id = device.index().to_string();
                let device_name = device.name().to_string();
                
                devices.push(DeviceInfo {
                    id: device.index(),
                    name: device_name.clone(),
                    device_type: DeviceType::Buttplug,
                    connected: true,
                });
                
                // Save to Device History
                if let Some(ref mut db) = db_opt {
                    if let Err(e) = db.upsert_device(&device_id, &device_name, "Buttplug") {
                        log::warn!("[Device Manager] Failed to save device to history: {}", e);
                    }
                }
            }
        }

        *self.devices.write().await = devices.clone();
        devices
    }

    /// Send vibration command to all devices (Buttplug + Lovense)
    pub async fn send_vibration(&self, intensity: f32) -> Result<()> {
        let intensity = intensity.clamp(0.0, 1.0);
        
        let mut buttplug_count = 0;
        let mut lovense_count = 0;

        // Send to Buttplug devices
        let guard = self.buttplug_client.read().await;
        if let Some(client) = guard.as_ref() {
            let devices = client.devices();
            buttplug_count = devices.len();
            
            if buttplug_count > 0 {
                // Reduced logging spam - only log errors
                // log::debug!("🎮 Sending vibration {} to {} Buttplug devices", intensity, buttplug_count);
                
                for device in devices {
                    match device.vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(intensity.into())).await {
                        Ok(_) => {}, // log::debug!("  ✅ {} vibrated", device.name()),
                        Err(e) => log::error!("  ❌ {} error: {}", device.name(), e),
                    }
                }
            }
        }
        
        // Send to Lovense devices (if enabled)
        if self.lovense_enabled {
            lovense_count = self.lovense_devices.read().await.len();
            if lovense_count > 0 {
                self.send_lovense_vibration(intensity).await?;
            }
        }
        
        let total_devices = buttplug_count + lovense_count;
        if total_devices == 0 {
            log::warn!("⚠️ No connected devices! Add Buttplug or Lovense devices.");
        }

        Ok(())
    }

    /// Stop all vibrations (Fail-Safe)
    pub async fn stop_all(&self) -> Result<()> {
        self.send_vibration(0.0).await
    }

    /// Check connection status
    #[allow(dead_code)]
    pub async fn is_connected(&self) -> bool {
        self.buttplug_client.read().await.is_some()
    }
    
    // ============ LOVENSE API METHODS ============
    
    /// Add Lovense device manually by IP address
    /// Lovense LAN API requires knowing the device IP (can be found in Lovense Remote app)
    pub async fn add_lovense_device(&self, ip: String, port: Option<u16>) -> Result<()> {
        let port = port.unwrap_or(20010); // Default Lovense LAN API port
        let url = format!("http://{}:{}/GetToyName", ip, port);
        
        log::info!("🔍 Discovering Lovense device at {}:{}...", ip, port);
        
        // Try to get device name
        match self.http_client.get(&url).send().await {
            Ok(response) => {
                if let Ok(name) = response.text().await {
                    let device_id = format!("lovense_{}", ip.replace(".", "_"));
                    let device = LovenseDevice {
                        id: device_id.clone(),
                        name: name.trim().to_string(),
                        ip_address: ip.clone(),
                        port,
                    };
                    
                    self.lovense_devices.write().await.insert(device_id.clone(), device.clone());
                    
                    log::info!("✅ Lovense device added: {} ({}:{})", device.name, ip, port);
                    
                    // Add to devices list
                    let mut devices = self.devices.write().await;
                    let device_index = devices.len() as u32;
                    devices.push(DeviceInfo {
                        id: device_index,
                        name: device.name.clone(),
                        device_type: DeviceType::Lovense,
                        connected: true,
                    });
                    
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Failed to parse device name from {}", url))
                }
            },
            Err(e) => {
                log::error!("❌ Failed to connect to Lovense device at {}:{}: {}", ip, port, e);
                Err(anyhow::anyhow!("Lovense device not found at {}:{}", ip, port))
            }
        }
    }
    
    /// Send vibration to Lovense devices
    /// Lovense API: POST http://ip:port/Vibrate?v=<0-20>
    /// Note: Lovense uses 0-20 scale, we convert from 0.0-1.0
    async fn send_lovense_vibration(&self, intensity: f32) -> Result<()> {
        let lovense_devices = self.lovense_devices.read().await;
        
        if lovense_devices.is_empty() {
            return Ok(()); // No Lovense devices, skip
        }
        
        // Convert 0.0-1.0 to 0-20 scale
        let lovense_intensity = (intensity * 20.0).round() as u8;
        
        log::debug!("🎮 Sending Lovense vibration: {} (intensity: {})", lovense_intensity, intensity);
        
        for (_id, device) in lovense_devices.iter() {
            let url = format!("http://{}:{}/Vibrate?v={}", device.ip_address, device.port, lovense_intensity);
            
            match self.http_client.post(&url).send().await {
                Ok(_) => log::debug!("  ✅ Lovense {} vibrated", device.name),
                Err(e) => log::error!("  ❌ Lovense {} failed: {}", device.name, e),
            }
        }
        
        Ok(())
    }
    
    /// Remove Lovense device
    pub async fn remove_lovense_device(&self, device_id: &str) -> Result<()> {
        let mut lovense_devices = self.lovense_devices.write().await;
        
        if let Some(device) = lovense_devices.remove(device_id) {
            log::info!("✅ Removed Lovense device: {}", device.name);
            
            // Remove from devices list
            let mut devices = self.devices.write().await;
            devices.retain(|d| d.name != device.name);
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Lovense device not found: {}", device_id))
        }
    }
    
    /// Enable/disable Lovense integration
    #[allow(dead_code)]
    pub fn set_lovense_enabled(&mut self, enabled: bool) {
        self.lovense_enabled = enabled;
        log::info!("🔧 Lovense integration: {}", if enabled { "ENABLED" } else { "DISABLED" });
    }
    
    /// Get Lovense connection status
    #[allow(dead_code)]
    pub async fn is_lovense_connected(&self) -> bool {
        !self.lovense_devices.read().await.is_empty()
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

