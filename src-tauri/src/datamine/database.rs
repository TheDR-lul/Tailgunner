/// SQLite database for vehicle limits storage

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use crate::datamine::types::*;

pub struct VehicleDatabase {
    conn: Connection,
}

impl VehicleDatabase {
    /// Create new database (in app data directory)
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(&db_path)?;
        let mut db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }
    
    /// Get database file path (in app data)
    fn get_db_path() -> Result<PathBuf> {
        let app_data = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find app data directory"))?;
        
        let path = app_data
            .join("Tailgunner")
            .join("vehicle_limits.db");
        
        Ok(path)
    }
    
    /// Create database tables
    fn create_tables(&mut self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS aircraft (
                identifier TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                vne_kmh REAL NOT NULL,
                vne_mach REAL,
                max_speed_ground REAL NOT NULL,
                stall_speed REAL NOT NULL,
                flutter_speed REAL,
                gear_max_speed_kmh REAL,
                flaps_max_speed_kmh REAL,
                mass_kg REAL NOT NULL,
                wing_overload_pos_n REAL,
                wing_overload_neg_n REAL,
                max_positive_g REAL,
                max_negative_g REAL,
                max_rpm REAL,
                horse_power REAL,
                vehicle_type TEXT NOT NULL,
                last_updated TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS ground (
                identifier TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                max_speed_kmh REAL NOT NULL,
                max_reverse_speed_kmh REAL NOT NULL,
                mass_kg REAL NOT NULL,
                horse_power REAL NOT NULL,
                max_rpm REAL NOT NULL,
                min_rpm REAL NOT NULL,
                hull_hp REAL NOT NULL,
                armor_thickness_mm REAL,
                vehicle_type TEXT NOT NULL,
                last_updated TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS ships (
                identifier TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                max_speed_knots REAL NOT NULL,
                max_reverse_speed_knots REAL NOT NULL,
                compartments TEXT NOT NULL,  -- JSON
                ship_class TEXT NOT NULL,
                last_updated TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_aircraft_type ON aircraft(vehicle_type);
            CREATE INDEX IF NOT EXISTS idx_ground_type ON ground(vehicle_type);
            CREATE INDEX IF NOT EXISTS idx_ships_class ON ships(ship_class);
            "
        )?;
        Ok(())
    }
    
    /// Save aircraft to database
    pub fn save_aircraft(&mut self, aircraft: &[AircraftLimits]) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        for ac in aircraft {
            tx.execute(
                "INSERT OR REPLACE INTO aircraft VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18
                )",
                params![
                    ac.identifier,
                    ac.display_name,
                    ac.vne_kmh,
                    ac.vne_mach,
                    ac.max_speed_ground,
                    ac.stall_speed,
                    ac.flutter_speed,
                    ac.gear_max_speed_kmh,
                    ac.flaps_max_speed_kmh,
                    ac.mass_kg,
                    ac.wing_overload_pos_n,
                    ac.wing_overload_neg_n,
                    ac.max_positive_g,
                    ac.max_negative_g,
                    ac.max_rpm,
                    ac.horse_power,
                    ac.vehicle_type,
                    ac.last_updated,
                ],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
    
    /// Save ground vehicles to database
    pub fn save_ground(&mut self, vehicles: &[GroundLimits]) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        for v in vehicles {
            tx.execute(
                "INSERT OR REPLACE INTO ground VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12
                )",
                params![
                    v.identifier,
                    v.display_name,
                    v.max_speed_kmh,
                    v.max_reverse_speed_kmh,
                    v.mass_kg,
                    v.horse_power,
                    v.max_rpm,
                    v.min_rpm,
                    v.hull_hp,
                    v.armor_thickness_mm,
                    v.vehicle_type,
                    v.last_updated,
                ],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
    
    /// Save ships to database
    pub fn save_ships(&mut self, ships: &[ShipLimits]) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        for ship in ships {
            let compartments_json = serde_json::to_string(&ship.compartments)?;
            
            tx.execute(
                "INSERT OR REPLACE INTO ships VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    ship.identifier,
                    ship.display_name,
                    ship.max_speed_knots,
                    ship.max_reverse_speed_knots,
                    compartments_json,
                    ship.ship_class,
                    ship.last_updated,
                ],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
    
    /// Get vehicle limits by identifier with fuzzy matching
    pub fn get_limits(&self, identifier: &str) -> Option<VehicleLimits> {
        // Generate alternative identifier formats
        let alternatives = Self::generate_identifier_alternatives(identifier);
        
        log::info!("[Database] Searching for vehicle: '{}'", identifier);
        log::debug!("[Database] Generated {} alternatives: {:?}", alternatives.len(), alternatives);
        
        // Try each alternative
        for (idx, alt_id) in alternatives.iter().enumerate() {
            log::debug!("[Database] Try {}/{}: '{}'", idx + 1, alternatives.len(), alt_id);
            
            // Try aircraft
            if let Ok(aircraft) = self.get_aircraft(alt_id) {
                log::info!("[Database] ✅ Found AIRCRAFT: '{}' (matched: '{}')", aircraft.display_name, alt_id);
                return Some(VehicleLimits::Aircraft(aircraft));
            }
            
            // Try ground
            if let Ok(ground) = self.get_ground(alt_id) {
                log::info!("[Database] ✅ Found GROUND: '{}' (matched: '{}')", ground.display_name, alt_id);
                return Some(VehicleLimits::Ground(ground));
            }
            
            // Try ship
            if let Ok(ship) = self.get_ship(alt_id) {
                log::info!("[Database] ✅ Found SHIP: '{}' (matched: '{}')", ship.display_name, alt_id);
                return Some(VehicleLimits::Ship(ship));
            }
        }
        
        log::error!("[Database] ❌ NO MATCH for '{}' after trying {} alternatives", identifier, alternatives.len());
        log::error!("[Database] Alternatives tried: {:?}", alternatives);
        None
    }
    
    /// Generate alternative identifier formats for fuzzy matching
    fn generate_identifier_alternatives(input: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        let input_lower = input.to_lowercase();
        
        // Original
        alternatives.push(input.to_string());
        alternatives.push(input_lower.clone());
        
        // Replace spaces with underscores
        alternatives.push(input_lower.replace(" ", "_"));
        
        // Replace dashes with underscores
        alternatives.push(input_lower.replace("-", "_"));
        
        // Replace both spaces and dashes with underscores
        alternatives.push(input_lower.replace(" ", "_").replace("-", "_"));
        
        // Replace underscores with dashes
        alternatives.push(input_lower.replace("_", "-"));
        
        // Remove all separators
        alternatives.push(input_lower.replace(" ", "").replace("-", "").replace("_", ""));
        
        // Deduplicate
        alternatives.sort();
        alternatives.dedup();
        
        alternatives
    }
    
    /// Get aircraft by identifier
    fn get_aircraft(&self, identifier: &str) -> Result<AircraftLimits> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM aircraft WHERE identifier = ?1"
        )?;
        
        let aircraft = stmt.query_row(params![identifier], |row| {
            Ok(AircraftLimits {
                identifier: row.get(0)?,
                display_name: row.get(1)?,
                vne_kmh: row.get(2)?,
                vne_mach: row.get(3)?,
                max_speed_ground: row.get(4)?,
                stall_speed: row.get(5)?,
                flutter_speed: row.get(6)?,
                gear_max_speed_kmh: row.get(7)?,
                flaps_max_speed_kmh: row.get(8)?,
                mass_kg: row.get(9)?,
                wing_overload_pos_n: row.get(10)?,
                wing_overload_neg_n: row.get(11)?,
                max_positive_g: row.get(12)?,
                max_negative_g: row.get(13)?,
                max_rpm: row.get(14)?,
                horse_power: row.get(15)?,
                vehicle_type: row.get(16)?,
                last_updated: row.get(17)?,
            })
        })?;
        
        Ok(aircraft)
    }
    
    /// Get ground vehicle by identifier
    fn get_ground(&self, identifier: &str) -> Result<GroundLimits> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM ground WHERE identifier = ?1"
        )?;
        
        let ground = stmt.query_row(params![identifier], |row| {
            Ok(GroundLimits {
                identifier: row.get(0)?,
                display_name: row.get(1)?,
                max_speed_kmh: row.get(2)?,
                max_reverse_speed_kmh: row.get(3)?,
                mass_kg: row.get(4)?,
                horse_power: row.get(5)?,
                max_rpm: row.get(6)?,
                min_rpm: row.get(7)?,
                hull_hp: row.get(8)?,
                armor_thickness_mm: row.get(9)?,
                vehicle_type: row.get(10)?,
                last_updated: row.get(11)?,
            })
        })?;
        
        Ok(ground)
    }
    
    /// Get ship by identifier
    fn get_ship(&self, identifier: &str) -> Result<ShipLimits> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM ships WHERE identifier = ?1"
        )?;
        
        let ship = stmt.query_row(params![identifier], |row| {
            let compartments_json: String = row.get(4)?;
            let compartments: Vec<Compartment> = serde_json::from_str(&compartments_json)
                .unwrap_or_default();
            
            Ok(ShipLimits {
                identifier: row.get(0)?,
                display_name: row.get(1)?,
                max_speed_knots: row.get(2)?,
                max_reverse_speed_knots: row.get(3)?,
                compartments,
                ship_class: row.get(5)?,
                last_updated: row.get(6)?,
            })
        })?;
        
        Ok(ship)
    }
    
    /// Get total count of vehicles
    pub fn get_stats(&self) -> Result<(usize, usize, usize)> {
        let aircraft: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM aircraft",
            [],
            |row| row.get(0)
        )?;
        
        let ground: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM ground",
            [],
            |row| row.get(0)
        )?;
        
        let ships: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM ships",
            [],
            |row| row.get(0)
        )?;
        
        Ok((aircraft, ground, ships))
    }
}

