/// Player Identity Database
/// Stores player names, clan tags, and enemy lists

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::PathBuf;

pub struct PlayerIdentityDB {
    conn: Connection,
}

impl PlayerIdentityDB {
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
        
        log::info!("[Player Identity DB] Opened database: {:?}", db_path);
        Ok(db)
    }
    
    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("Tailgunner");
        path.push("player_identity.db");
        path
    }
    
    fn init_tables(&mut self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS player_identity (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                player_names TEXT NOT NULL,
                clan_tags TEXT NOT NULL,
                enemy_names TEXT NOT NULL,
                enemy_clans TEXT NOT NULL,
                last_updated TEXT NOT NULL
            )",
            [],
        )?;
        
        // Insert default row if not exists
        self.conn.execute(
            "INSERT OR IGNORE INTO player_identity (id, player_names, clan_tags, enemy_names, enemy_clans, last_updated)
             VALUES (1, '[]', '[]', '[]', '[]', datetime('now'))",
            [],
        )?;
        
        log::info!("[Player Identity DB] ✅ Tables initialized");
        Ok(())
    }
    
    /// Save all player identity data
    pub fn save(&mut self, player_names: &[String], clan_tags: &[String], enemy_names: &[String], enemy_clans: &[String]) -> Result<()> {
        let player_names_json = serde_json::to_string(player_names)?;
        let clan_tags_json = serde_json::to_string(clan_tags)?;
        let enemy_names_json = serde_json::to_string(enemy_names)?;
        let enemy_clans_json = serde_json::to_string(enemy_clans)?;
        
        self.conn.execute(
            "UPDATE player_identity SET 
                player_names = ?1,
                clan_tags = ?2,
                enemy_names = ?3,
                enemy_clans = ?4,
                last_updated = datetime('now')
             WHERE id = 1",
            params![player_names_json, clan_tags_json, enemy_names_json, enemy_clans_json],
        )?;
        
        log::info!("[Player Identity DB] ✅ Saved: {} players, {} clans, {} enemies, {} enemy clans", 
            player_names.len(), clan_tags.len(), enemy_names.len(), enemy_clans.len());
        
        Ok(())
    }
    
    /// Load all player identity data
    pub fn load(&self) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>)> {
        let mut stmt = self.conn.prepare(
            "SELECT player_names, clan_tags, enemy_names, enemy_clans FROM player_identity WHERE id = 1"
        )?;
        
        let result = stmt.query_row([], |row| {
            let player_names_json: String = row.get(0)?;
            let clan_tags_json: String = row.get(1)?;
            let enemy_names_json: String = row.get(2)?;
            let enemy_clans_json: String = row.get(3)?;
            
            Ok((player_names_json, clan_tags_json, enemy_names_json, enemy_clans_json))
        })?;
        
        let player_names: Vec<String> = serde_json::from_str(&result.0).unwrap_or_default();
        let clan_tags: Vec<String> = serde_json::from_str(&result.1).unwrap_or_default();
        let enemy_names: Vec<String> = serde_json::from_str(&result.2).unwrap_or_default();
        let enemy_clans: Vec<String> = serde_json::from_str(&result.3).unwrap_or_default();
        
        log::info!("[Player Identity DB] ✅ Loaded: {} players, {} clans, {} enemies, {} enemy clans", 
            player_names.len(), clan_tags.len(), enemy_names.len(), enemy_clans.len());
        
        Ok((player_names, clan_tags, enemy_names, enemy_clans))
    }
}

