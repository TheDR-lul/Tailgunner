/// War Thunder Datamine Module
/// Extracts vehicle limits and characteristics from game files
/// 
/// Safe for EAC: Only reads files on disk, no process interaction

pub mod parser;
pub mod aircraft;
pub mod ground;
pub mod naval;
pub mod database;
pub mod types;
pub mod vromfs;
pub mod wiki_scraper;

use anyhow::Result;
use std::path::PathBuf;
pub use types::*;

/// Main datamine interface
pub struct Datamine {
    game_path: PathBuf,
    db: database::VehicleDatabase,
}

impl Datamine {
    /// Create new datamine instance
    pub fn new(game_path: PathBuf) -> Result<Self> {
        Ok(Self {
            game_path,
            db: database::VehicleDatabase::new()?,
        })
    }
    
    /// Auto-detect War Thunder installation
    pub fn auto_detect() -> Result<PathBuf> {
        Self::find_game_path()
            .ok_or_else(|| anyhow::anyhow!("War Thunder installation not found"))
    }
    
    /// Parse all vehicle data from game files (without Wiki)
    pub async fn parse_all(&mut self) -> Result<ParseStats> {
        log::info!("[Datamine] Starting full parse from: {:?}", self.game_path);
        
        let start = std::time::Instant::now();
        
        // Prepare game files (check for unpacked data or archives)
        let parse_path = self.prepare_game_files()?;
        log::info!("[Datamine] Using data from: {:?}", parse_path);
        
        // Parse aircraft
        let aircraft = aircraft::parse_aircraft(&parse_path)?;
        self.db.save_aircraft(&aircraft)?;
        
        // Parse ground vehicles (Wiki data will be fetched on-demand)
        let ground = ground::parse_ground(&parse_path).await?;
        self.db.save_ground(&ground)?;
        
        // Parse ships
        let ships = naval::parse_naval(&parse_path)?;
        self.db.save_ships(&ships)?;
        
        let duration = start.elapsed();
        
        let stats = ParseStats {
            aircraft_count: aircraft.len(),
            ground_count: ground.len(),
            ships_count: ships.len(),
            duration_ms: duration.as_millis() as u64,
        };
        
        log::info!("[Datamine] Parse complete: {} aircraft, {} ground, {} ships in {}ms",
            stats.aircraft_count, stats.ground_count, stats.ships_count, stats.duration_ms);
        
        Ok(stats)
    }
    
    /// Get vehicle limits by identifier
    #[allow(dead_code)]
    pub fn get_limits(&self, identifier: &str) -> Option<VehicleLimits> {
        self.db.get_limits(identifier)
    }
    
    /// Prepare game files for parsing (unpack if needed)
    /// ✅ EAC-SAFE: распаковывает в TEMP, не трогает файлы игры!
    fn prepare_game_files(&self) -> Result<PathBuf> {
        log::info!("[Datamine] Preparing game files...");
        
        // 1. Check if already unpacked IN GAME FOLDER (юзер запустил игру с -unpack)
        let game_unpacked_dir = self.game_path.join("aces.vromfs.bin_u");
        if game_unpacked_dir.exists() && game_unpacked_dir.join("gamedata").exists() {
            log::info!("[Datamine] ✅ Using pre-unpacked files from game: {:?}", game_unpacked_dir);
            return Ok(self.game_path.clone());
        }
        
        // 2. Check loose files in game folder
        let gamedata_dir = self.game_path.join("gamedata");
        if gamedata_dir.exists() {
            log::info!("[Datamine] ✅ Using loose gamedata files: {:?}", gamedata_dir);
            return Ok(self.game_path.clone());
        }
        
        // 3. Check TEMP directory (кешированная распаковка)
        let temp_base = std::env::temp_dir().join("tailgunner_datamine");
        let temp_aces = temp_base.join("aces.vromfs.bin_u");
        
        // Check if aces.vromfs.bin is already unpacked (contains all needed data in Steam version)
        if temp_aces.exists() && temp_aces.join("gamedata").exists() {
            log::info!("[Datamine] ✅ Using cached unpacked files from TEMP: {:?}", temp_aces);
            return Ok(temp_aces);
        }
        
        // 4. Try to find and unpack BOTH VROMFS archives → TEMP
        log::info!("[Datamine] 📦 Looking for VROMFS archives to unpack...");
        vromfs::unpack_all_required_archives(&self.game_path)
    }
    
    /// Search for game installation paths
    fn find_game_path() -> Option<PathBuf> {
        log::info!("[Datamine] 🔍 Searching for War Thunder installation...");
        
        // 1. Check registry FIRST (most reliable)
        #[cfg(target_os = "windows")]
        {
            log::debug!("[Datamine] Checking Windows Registry...");
            if let Some(path) = Self::find_from_registry() {
                log::info!("[Datamine] ✅ Found in registry: {:?}", path);
                return Some(path);
            }
        }
        
        // 2. Common Steam paths (multiple drives + folder variants)
        let steam_paths = vec![
            // Standard Program Files location
            r"C:\Program Files (x86)\Steam\steamapps\common\War Thunder",
            r"D:\Program Files (x86)\Steam\steamapps\common\War Thunder",
            r"E:\Program Files (x86)\Steam\steamapps\common\War Thunder",
            r"F:\Program Files (x86)\Steam\steamapps\common\War Thunder",
            r"G:\Program Files (x86)\Steam\steamapps\common\War Thunder",
            
            // Steam folder (root)
            r"C:\Steam\steamapps\common\War Thunder",
            r"D:\Steam\steamapps\common\War Thunder",
            r"E:\Steam\steamapps\common\War Thunder",
            r"F:\Steam\steamapps\common\War Thunder",
            r"G:\Steam\steamapps\common\War Thunder",
            
            // SteamLibrary folder (additional libraries)
            r"C:\SteamLibrary\steamapps\common\War Thunder",
            r"D:\SteamLibrary\steamapps\common\War Thunder",
            r"E:\SteamLibrary\steamapps\common\War Thunder",
            r"F:\SteamLibrary\steamapps\common\War Thunder",
            r"G:\SteamLibrary\steamapps\common\War Thunder",
            
            // Steam Library folder (with space)
            r"C:\Steam Library\steamapps\common\War Thunder",
            r"D:\Steam Library\steamapps\common\War Thunder",
            r"E:\Steam Library\steamapps\common\War Thunder",
            r"F:\Steam Library\steamapps\common\War Thunder",
            r"G:\Steam Library\steamapps\common\War Thunder",
        ];
        
        log::debug!("[Datamine] Checking {} Steam paths...", steam_paths.len());
        for path_str in &steam_paths {
            let path = PathBuf::from(path_str);
            log::trace!("[Datamine]   Trying: {}", path_str);
            if path.exists() {
                log::info!("[Datamine] ✅ Found Steam install: {:?}", path);
                return Some(path);
            }
        }
        
        // 3. Standalone installations
        let standalone_paths = vec![
            r"C:\Games\War Thunder",
            r"D:\Games\War Thunder",
            r"E:\Games\War Thunder",
            r"F:\Games\War Thunder",
            r"G:\Games\War Thunder",
            r"C:\War Thunder",
            r"D:\War Thunder",
            r"E:\War Thunder",
            r"F:\War Thunder",
            r"G:\War Thunder",
        ];
        
        log::debug!("[Datamine] Checking {} standalone paths...", standalone_paths.len());
        for path_str in &standalone_paths {
            let path = PathBuf::from(path_str);
            log::trace!("[Datamine]   Trying: {}", path_str);
            if path.exists() {
                log::info!("[Datamine] ✅ Found standalone: {:?}", path);
                return Some(path);
            }
        }
        
        log::error!("[Datamine] ❌ War Thunder not found in any known location");
        log::error!("[Datamine] Checked:");
        log::error!("[Datamine]   - Windows Registry (4 keys)");
        log::error!("[Datamine]   - {} Steam paths", steam_paths.len());
        log::error!("[Datamine]   - {} Standalone paths", standalone_paths.len());
        None
    }
    
    #[cfg(target_os = "windows")]
    fn find_from_registry() -> Option<PathBuf> {
        use winreg::enums::*;
        use winreg::RegKey;
        
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        
        // Try different registry paths
        let reg_paths = vec![
            r"SOFTWARE\Gaijin\War Thunder",
            r"SOFTWARE\WOW6432Node\Gaijin\War Thunder",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\War Thunder",
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\War Thunder",
        ];
        
        for (idx, reg_path) in reg_paths.iter().enumerate() {
            log::trace!("[Datamine]   Registry {}/{}: {}", idx + 1, reg_paths.len(), reg_path);
            
            if let Ok(key) = hklm.open_subkey(reg_path) {
                // Try "InstallLocation" key
                if let Ok(install_path) = key.get_value::<String, _>("InstallLocation") {
                    let path_buf = PathBuf::from(&install_path);
                    if path_buf.exists() {
                        log::info!("[Datamine] ✅ Found in registry (InstallLocation): {:?}", path_buf);
                        return Some(path_buf);
                    } else {
                        log::trace!("[Datamine]   Path exists in registry but not on disk: {}", install_path);
                    }
                }
                
                // Try "Path" key
                if let Ok(install_path) = key.get_value::<String, _>("Path") {
                    let path_buf = PathBuf::from(&install_path);
                    if path_buf.exists() {
                        log::info!("[Datamine] ✅ Found in registry (Path): {:?}", path_buf);
                        return Some(path_buf);
                    } else {
                        log::trace!("[Datamine]   Path exists in registry but not on disk: {}", install_path);
                    }
                }
                
                // Try "DisplayIcon" key (may contain exe path)
                if let Ok(icon_path) = key.get_value::<String, _>("DisplayIcon") {
                    // Extract directory from "C:\...\aces.exe"
                    if let Some(parent) = PathBuf::from(&icon_path).parent() {
                        if parent.exists() {
                            log::info!("[Datamine] ✅ Found in registry (DisplayIcon): {:?}", parent);
                            return Some(parent.to_path_buf());
                        }
                    }
                }
            }
        }
        
        log::trace!("[Datamine]   No valid paths found in registry");
        None
    }
}

/// Statistics from parsing operation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ParseStats {
    pub aircraft_count: usize,
    pub ground_count: usize,
    pub ships_count: usize,
    pub duration_ms: u64,
}

