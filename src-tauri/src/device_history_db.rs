/// Device History Database
/// Stores previously connected devices for UI autocomplete/suggestions

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub last_seen: String,  // ISO 8601 timestamp
}

pub struct DeviceHistoryDB {
    conn: Connection,
}

impl DeviceHistoryDB {
    /// Create or open database
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        
        // Create parent directory if needed
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(&db_path)?;
        
        let mut db = Self { conn };
        db.init_tables()?;
        
        log::info!("[Device History DB] Opened database: {:?}", db_path);
        Ok(db)
    }
    
    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("Tailgunner");
        path.push("device_history.db");
        path
    }
    
    fn init_tables(&mut self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS device_history (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                device_type TEXT NOT NULL,
                last_seen TEXT NOT NULL
            )",
            [],
        )?;
        
        log::info!("[Device History DB] ✅ Tables initialized");
        Ok(())
    }
    
    /// Add or update device in history
    pub fn upsert_device(&mut self, id: &str, name: &str, device_type: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO device_history (id, name, device_type, last_seen)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, name, device_type, now],
        )?;
        
        log::debug!("[Device History DB] 💾 Saved: {} ({})", name, device_type);
        
        Ok(())
    }
    
    /// Get all devices from history, sorted by last seen
    pub fn get_all_devices(&self) -> Result<Vec<DeviceRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, device_type, last_seen FROM device_history ORDER BY last_seen DESC"
        )?;
        
        let devices = stmt.query_map([], |row| {
            Ok(DeviceRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                device_type: row.get(2)?,
                last_seen: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(devices)
    }
    
    /// Get device by ID
    #[allow(dead_code)]
    pub fn get_device(&self, id: &str) -> Result<Option<DeviceRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, device_type, last_seen FROM device_history WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(DeviceRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                device_type: row.get(2)?,
                last_seen: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Delete device from history
    #[allow(dead_code)]
    pub fn delete_device(&mut self, id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM device_history WHERE id = ?1",
            params![id],
        )?;
        
        log::info!("[Device History DB] 🗑️  Deleted: {}", id);
        
        Ok(())
    }
}

