/// Wiki scraper for War Thunder vehicle data
/// 
/// Scrapes https://wiki.warthunder.com to fetch missing data not found in game files
/// (e.g., max speed for tanks, which is calculated by game engine and not stored in .blk)

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const WIKI_BASE_URL: &str = "https://wiki.warthunder.com/unit/";

/// Data scraped from Wiki
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiGroundData {
    pub max_speed_kmh: Option<f32>,
    pub max_reverse_speed_kmh: Option<f32>,
    pub forward_gears: Option<u8>,
    pub reverse_gears: Option<u8>,
}

/// Scrape ground vehicle data from War Thunder Wiki
pub async fn scrape_ground_vehicle(identifier: &str) -> Result<WikiGroundData, anyhow::Error> {
    let url = format!("{}{}", WIKI_BASE_URL, identifier);
    
    log::info!("[Wiki] 🌐 Scraping: {}", url);
    
    let response = reqwest::get(&url).await?;
    
    log::info!("[Wiki] 📄 HTTP Status: {}", response.status());
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("HTTP {}: {}", response.status(), url));
    }
    
    let html = response.text().await?;
    
    // Debug: log first 500 chars of HTML
    log::debug!("[Wiki] 📝 HTML preview (first 500 chars):");
    log::debug!("{}", &html.chars().take(500).collect::<String>());
    let document = Html::parse_document(&html);
    
    // Parse specs table
    let mut data = WikiGroundData {
        max_speed_kmh: None,
        max_reverse_speed_kmh: None,
        forward_gears: None,
        reverse_gears: None,
    };
    
    // Look for characteristic blocks (new Wiki structure)
    let block_selector = Selector::parse("div.game-unit_chars-block").unwrap();
    let header_selector = Selector::parse(".game-unit_chars-header").unwrap();
    let subline_selector = Selector::parse(".game-unit_chars-subline").unwrap();
    let value_selector = Selector::parse(".game-unit_chars-value").unwrap();
    let rb_selector = Selector::parse(".show-char-rb").unwrap();
    
    let blocks_count = document.select(&block_selector).count();
    log::debug!("[Wiki] 🔍 Found {} characteristic blocks", blocks_count);
    
    for (block_idx, block) in document.select(&block_selector).enumerate() {
        // Get block header
        let header = match block.select(&header_selector).next() {
            Some(h) => h.text().collect::<String>().trim().to_string(),
            None => continue,
        };
        
        log::debug!("[Wiki] 📊 Block #{}: '{}'", block_idx + 1, header);
        
        // Parse "Max speed" block
        if header == "Max speed" {
            for subline in block.select(&subline_selector) {
                let subline_text = subline.text().collect::<String>();
                let label = subline_text.split_whitespace().next().unwrap_or("").to_lowercase();
                
                // Get RB (Realistic Battle) value from <span class="show-char-rb">
                if let Some(value_elem) = subline.select(&value_selector).next() {
                    if let Some(rb_span) = value_elem.select(&rb_selector).next() {
                        let speed_str = rb_span.text().collect::<String>().trim().to_string();
                        if let Ok(speed) = speed_str.parse::<f32>() {
                            if label == "forward" {
                                data.max_speed_kmh = Some(speed);
                                log::debug!("[Wiki]   ✅ Forward Speed: {} km/h (RB)", speed);
                            } else if label == "backward" {
                                data.max_reverse_speed_kmh = Some(speed);
                                log::debug!("[Wiki]   ✅ Reverse Speed: {} km/h (RB)", speed);
                            }
                        }
                    }
                }
            }
        }
    }
    
    if data.max_speed_kmh.is_none() && data.forward_gears.is_none() {
        log::warn!("[Wiki] ⚠️ No data extracted from: {}", url);
    }
    
    Ok(data)
}

/// Extract speed value from strings like:
/// "68 km/h", "68 km/h (AB / RB / SB)", "68 / 72 / 68 km/h"
fn extract_speed(text: &str) -> Option<f32> {
    // Try to find first number before "km/h"
    let parts: Vec<&str> = text.split("km/h").collect();
    if parts.is_empty() {
        return None;
    }
    
    // Get the part before "km/h"
    let speed_part = parts[0].trim();
    
    // Extract last number (handles "68 / 72 / 68" -> take first)
    let numbers: Vec<&str> = speed_part.split('/').collect();
    for num_str in numbers {
        let num_str = num_str.trim().replace(',', "");
        if let Ok(speed) = num_str.parse::<f32>() {
            return Some(speed);
        }
    }
    
    None
}

/// Extract forward and reverse gears from strings like:
/// "8 / 4", "Forward / Reverse: 8 / 4", "8 forward, 4 reverse"
fn extract_gears(text: &str) -> Option<(u8, u8)> {
    // Look for pattern "X / Y" or "X forward Y reverse"
    let text_lower = text.to_lowercase();
    
    // Try "X / Y" pattern
    if text.contains('/') {
        let parts: Vec<&str> = text.split('/').collect();
        if parts.len() >= 2 {
            // Extract numbers
            let fwd_str = parts[0].trim().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let rev_str = parts[1].trim().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            
            if let (Ok(fwd), Ok(rev)) = (fwd_str.parse::<u8>(), rev_str.parse::<u8>()) {
                return Some((fwd, rev));
            }
        }
    }
    
    // Try "X forward Y reverse" pattern
    if text_lower.contains("forward") && text_lower.contains("reverse") {
        let numbers: Vec<u8> = text
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        
        if numbers.len() >= 2 {
            return Some((numbers[0], numbers[1]));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_speed() {
        assert_eq!(extract_speed("68 km/h"), Some(68.0));
        assert_eq!(extract_speed("68 km/h (AB / RB / SB)"), Some(68.0));
        assert_eq!(extract_speed("68 / 72 / 68 km/h"), Some(68.0));
        assert_eq!(extract_speed("10 km/h"), Some(10.0));
        assert_eq!(extract_speed("invalid"), None);
    }
    
    #[test]
    fn test_extract_gears() {
        assert_eq!(extract_gears("8 / 4"), Some((8, 4)));
        assert_eq!(extract_gears("Forward / Reverse: 8 / 4"), Some((8, 4)));
        assert_eq!(extract_gears("8 forward, 4 reverse"), Some((8, 4)));
        assert_eq!(extract_gears("invalid"), None);
    }
}

