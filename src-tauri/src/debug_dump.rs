/// Debug tool to dump all data from War Thunder API
/// This helps understand what endpoints exist and what data they provide

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

const WT_API_BASE: &str = "http://127.0.0.1:8111";

pub struct ApiDumper {
    client: reqwest::Client,
}

impl ApiDumper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Known War Thunder API endpoints
    /// Based on official WT localhost:8111 API and community research
    fn get_known_endpoints(&self) -> Vec<&'static str> {
        vec![
            // Main page (HTML interface)
            "/",
            
            // Map data
            "/map_info.json",        // Map boundaries, grid info
            "/map_obj.json",         // Objects on map (players, capture zones, etc)
            
            // Vehicle telemetry
            "/indicators",           // All cockpit indicators (speed, RPM, fuel, etc)
            "/state",                // Detailed flight/vehicle state
            
            // HUD messages & chat
            "/hudmsg?lastEvt=0&lastDmg=0",  // HUD messages (kills, damage, warnings)
            "/gamechat?lastId=0",           // Game chat messages
            
            // Mission info
            "/mission.json",         // Objectives, mission status
            
            // Other endpoints
            "/status",               // Server/game status
            "/info",                 // Additional info
            "/gunner_view",          // Gunner camera view data
            "/damage",               // Damage model/hitpoints
            
            // Alternative endpoints (may not always work)
            "/hudmsg",               // HUD messages without params
            "/gamechat",             // Chat without params
        ]
    }

    /// Dump all data from all known endpoints
    pub async fn dump_all(&self) -> HashMap<String, Value> {
        let mut results = HashMap::new();
        let endpoints = self.get_known_endpoints();
        
        log::info!("[API Dump] Checking {} endpoints...", endpoints.len());
        
        for (idx, endpoint) in endpoints.iter().enumerate() {
            log::info!("[API Dump] [{}/{}] Fetching {}", idx + 1, endpoints.len(), endpoint);
            
            match self.fetch_endpoint(endpoint).await {
                Ok(data) => {
                    log::info!("[API Dump] ✅ {}", endpoint);
                    results.insert(endpoint.to_string(), data);
                }
                Err(e) => {
                    log::warn!("[API Dump] ❌ {}: {}", endpoint, e);
                    results.insert(
                        endpoint.to_string(), 
                        serde_json::json!({"error": e.to_string()})
                    );
                }
            }
        }
        
        log::info!("[API Dump] Completed! {} endpoints processed", endpoints.len());
        results
    }

    /// Fetch a single endpoint
    async fn fetch_endpoint(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", WT_API_BASE, endpoint);
        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        // Try to parse as JSON
        match serde_json::from_str::<Value>(&text) {
            Ok(json) => Ok(json),
            Err(_) => {
                // If not JSON, return as string
                Ok(serde_json::json!({"raw_text": text}))
            }
        }
    }
}

impl Default for ApiDumper {
    fn default() -> Self {
        Self::new()
    }
}

