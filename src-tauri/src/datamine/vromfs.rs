/// VROMFS (.vromfs.bin) unpacker using wt_blk library
/// War Thunder stores game data in VROMFS archives
/// 
/// Uses wt_blk crate (MIT licensed) - properly this time!

use anyhow::{Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use wt_blk::vromf::{VromfUnpacker, BlkOutputFormat};

/// Unpack VROMFS archive using wt_blk library
pub fn unpack_vromfs(vromfs_path: &Path) -> Result<PathBuf> {
    log::info!("[VROMFS] 🔍 Looking for game data...");
    
    if !vromfs_path.exists() {
        bail!("VROMFS file not found: {:?}", vromfs_path);
    }
    
    // 1. Check for pre-unpacked version (game ran with -unpack)
    let game_unpacked_dir = vromfs_path.parent()
        .ok_or_else(|| anyhow::anyhow!("No parent directory"))?
        .join(format!("{}_u", vromfs_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?));
    
    if game_unpacked_dir.exists() && game_unpacked_dir.join("gamedata").exists() {
        log::info!("[VROMFS] ✅ Found pre-unpacked folder: {:?}", game_unpacked_dir);
        return Ok(game_unpacked_dir.parent().unwrap().to_path_buf());
    }
    
    // 2. Check for loose gamedata folder
    let game_dir = vromfs_path.parent().unwrap();
    let loose_gamedata = game_dir.join("gamedata");
    if loose_gamedata.exists() {
        log::info!("[VROMFS] ✅ Found loose gamedata folder: {:?}", loose_gamedata);
        return Ok(game_dir.to_path_buf());
    }
    
    // 3. Check TEMP cache
    let temp_base = std::env::temp_dir().join("tailgunner_datamine");
    let archive_name = vromfs_path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
    let temp_unpacked = temp_base.join(format!("{}_u", archive_name));
    
    if temp_unpacked.exists() && temp_unpacked.join("gamedata").exists() {
        log::info!("[VROMFS] ✅ Found cached unpacked data: {:?}", temp_unpacked);
        return Ok(temp_unpacked);
    }
    
    // 4. Try to unpack with wt_blk
    log::info!("[VROMFS] 📦 No unpacked data found, attempting auto-unpack...");
    log::info!("[VROMFS] 🔓 Unpacking: {:?}", vromfs_path);
    
    match unpack_with_wt_blk(vromfs_path, &temp_unpacked) {
        Ok(_) => {
            log::info!("[VROMFS] ✅ Successfully unpacked to: {:?}", temp_unpacked);
            Ok(temp_unpacked)
        }
        Err(e) => {
            log::error!("[VROMFS] ❌ Auto-unpack failed: {}", e);
            log::error!("[VROMFS]");
            log::error!("[VROMFS] 💡 MANUAL SOLUTION:");
            log::error!("[VROMFS]   Option 1: Use wt-tools (Python)");
            log::error!("[VROMFS]     pip install wt-tools");
            log::error!("[VROMFS]     wt-tools unpack \"{}\"", vromfs_path.display());
            log::error!("[VROMFS]");
            log::error!("[VROMFS]   Option 2: Try game's -unpack parameter (may not work in new versions)");
            log::error!("[VROMFS]     Steam → Properties → Launch Options → Add: -unpack");
            Err(e)
        }
    }
}

/// Unpack VROMFS using wt_blk library - CORRECT API USAGE
fn unpack_with_wt_blk(vromfs_path: &Path, output_dir: &Path) -> Result<()> {
    log::info!("[VROMFS] 📖 Loading VROMFS file...");
    
    // Create wt_blk File from path
    let vromf_file = wt_blk::vromf::File::new(vromfs_path)
        .map_err(|e| anyhow::anyhow!("Failed to load VROMFS file: {}", e))?;
    
    log::info!("[VROMFS] 🔧 Creating unpacker...");
    
    // Create unpacker (validate=true)
    let unpacker = VromfUnpacker::from_file(&vromf_file, true)
        .map_err(|e| anyhow::anyhow!("Failed to create unpacker: {}", e))?;
    
    log::info!("[VROMFS] 📂 Unpacking files...");
    
    // Unpack all files with JSON format for BLK files
    let files = unpacker.unpack_all(Some(BlkOutputFormat::Json), false)
        .map_err(|e| anyhow::anyhow!("Failed to unpack: {}", e))?;
    
    log::info!("[VROMFS] ✅ Unpacked {} files, writing to disk...", files.len());
    
    // Create output directory
    fs::create_dir_all(output_dir)?;
    
    // Write files to disk
    for (idx, file) in files.iter().enumerate() {
        if idx % 100 == 0 {
            log::info!("[VROMFS] Writing: {}/{}", idx + 1, files.len());
        }
        
        // Get file path and content using correct API
        let file_path = file.path();
        let output_path = output_dir.join(file_path);
        
        // Create parent directories
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write file content using buf() method
        let content = file.buf();
        fs::write(&output_path, content)
            .map_err(|e| anyhow::anyhow!("Failed to write {}: {}", file_path.display(), e))?;
    }
    
    log::info!("[VROMFS] ✅ All files written successfully!");
    Ok(())
}


/// Find aces.vromfs.bin in War Thunder installation
pub fn find_vromfs_archive(game_path: &Path) -> Option<PathBuf> {
    let candidates = vec![
        game_path.join("aces.vromfs.bin"),
        game_path.join("content").join("aces.vromfs.bin"),
    ];
    
    for candidate in candidates {
        if candidate.exists() {
            log::info!("[VROMFS] 📦 Found archive: {:?}", candidate);
            return Some(candidate);
        }
    }
    
    log::warn!("[VROMFS] ⚠️ No aces.vromfs.bin found in: {:?}", game_path);
    None
}

/// Find gamedata.vromfs.bin in War Thunder installation
pub fn find_gamedata_vromfs(game_path: &Path) -> Option<PathBuf> {
    let candidates = vec![
        game_path.join("gamedata.vromfs.bin"),
        game_path.join("content").join("gamedata.vromfs.bin"),
    ];
    
    for candidate in candidates {
        if candidate.exists() {
            log::info!("[VROMFS] 📦 Found gamedata archive: {:?}", candidate);
            return Some(candidate);
        }
    }
    
    log::warn!("[VROMFS] ⚠️ No gamedata.vromfs.bin found in: {:?}", game_path);
    None
}

/// Unpack aces.vromfs.bin (and gamedata.vromfs.bin if exists) to TEMP
/// Returns path to unpacked aces.vromfs.bin_u directory
pub fn unpack_all_required_archives(game_path: &Path) -> Result<PathBuf> {
    log::info!("[VROMFS] 📦 Unpacking required War Thunder archives...");
    
    // 1. Unpack aces.vromfs.bin (FM data + damage_model in Steam version)
    let aces_unpacked = if let Some(aces_archive) = find_vromfs_archive(game_path) {
        log::info!("[VROMFS] 🔓 Unpacking aces.vromfs.bin (flight models + damage data)...");
        unpack_vromfs(&aces_archive)?
    } else {
        bail!("aces.vromfs.bin not found!");
    };
    
    // 2. Try to unpack gamedata.vromfs.bin (if exists in standalone version)
    if let Some(gamedata_archive) = find_gamedata_vromfs(game_path) {
        log::info!("[VROMFS] 🔓 Unpacking gamedata.vromfs.bin (additional models)...");
        let _ = unpack_vromfs(&gamedata_archive);
    } else {
        log::info!("[VROMFS] ℹ️ gamedata.vromfs.bin not found (normal for Steam version)");
    }
    
    log::info!("[VROMFS] ✅ Using data from: {:?}", aces_unpacked);
    Ok(aces_unpacked)
}
