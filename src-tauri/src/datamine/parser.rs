/// BLK/BLKX file parser
/// These files are JSON format with .blkx extension

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Parse .blkx file (JSON format)
#[allow(dead_code)]
pub fn parse_blkx<T>(path: &Path) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let content = fs::read_to_string(path)?;
    let data: T = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse {:?}: {}", path, e))?;
    Ok(data)
}

/// Read and parse JSON from .blkx file
pub fn read_json_file(path: &Path) -> Result<serde_json::Value> {
    let content = fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    Ok(json)
}

/// Extract identifier from filename
/// Example: "bf-109f-4.blkx" -> "bf-109f-4"
pub fn extract_identifier(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

/// Check if game path is valid War Thunder installation
#[allow(dead_code)]
pub fn validate_game_path(path: &Path) -> bool {
    // Check for key directories
    let aces_dir = path.join("aces.vromfs.bin_u");
    let gamedata = aces_dir.join("gamedata");
    
    aces_dir.exists() && gamedata.exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_identifier() {
        let path = Path::new("bf-109f-4.blkx");
        assert_eq!(extract_identifier(path), Some("bf-109f-4".to_string()));
    }
}

