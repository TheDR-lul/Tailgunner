/// HUD Messages parser for War Thunder API
/// Used primarily for ships since they don't have indicators/state telemetry
use serde::{Deserialize, Serialize};
use crate::pattern_engine::GameEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudMessage {
    pub id: u32,
    pub msg: String,
    #[serde(default)]
    pub sender: String,
    #[serde(default)]
    pub enemy: bool,
    #[serde(default)]
    pub mode: String,
    pub time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudMessages {
    #[serde(default)]
    pub events: Vec<HudMessage>,
    #[serde(default)]
    pub damage: Vec<HudMessage>,
}

impl HudMessages {
    /// Parse HUD message text to detect game events
    pub fn parse_event(msg: &str) -> Option<GameEvent> {
        let msg_lower = msg.to_lowercase();
        
        // Ship-specific events from HUD messages
        // Examples:
        // "=DEST= _WingsOfPrey_ (IJN Yamashiro) destroyed PT-103"
        // "gouliguojia386 (HMS Nelson) set afire Emden"
        // "Dogbolter (HMS Rodney) has delivered the first strike!"
        // "⋇Henry_Morrison12 (RN Duilio) has achieved \"Ship Rescuer\""
        
        // === DAMAGE DEALT ===
        if msg_lower.contains("destroyed") && !msg_lower.contains("has been") {
            return Some(GameEvent::TargetDestroyed);
        }
        
        if msg_lower.contains("set afire") || msg_lower.contains("set on fire") {
            return Some(GameEvent::TargetSetOnFire);
        }
        
        if msg_lower.contains("critically damaged") {
            return Some(GameEvent::CriticalHit);
        }
        
        if msg_lower.contains("severely damaged") {
            return Some(GameEvent::TargetSeverelyDamaged);
        }
        
        if msg_lower.contains("shot down") {
            return Some(GameEvent::AircraftDestroyed);
        }
        
        // === DAMAGE RECEIVED ===
        if msg_lower.contains("has been wrecked") || msg_lower.contains("has crashed") {
            return Some(GameEvent::VehicleDestroyed);
        }
        
        // === ACHIEVEMENTS ===
        if msg_lower.contains("first strike") || msg_lower.contains("delivered the first strike") {
            return Some(GameEvent::FirstStrike);
        }
        
        if msg_lower.contains("ship rescuer") {
            return Some(GameEvent::ShipRescuer);
        }
        
        if msg_lower.contains("assist") {
            return Some(GameEvent::Assist);
        }
        
        if msg_lower.contains("base capturer") || msg_lower.contains("base defender") {
            return Some(GameEvent::BaseCapture);
        }
        
        if msg_lower.contains("team kill") || msg_lower.contains("teamkill") {
            return Some(GameEvent::TeamKill);
        }
        
        if msg_lower.contains("has achieved") {
            return Some(GameEvent::Achievement);
        }
        
        // === PLAYER STATUS ===
        if msg_lower.contains("disconnected from the game") {
            return Some(GameEvent::PlayerDisconnected);
        }
        
        // === ENGINE ===
        if msg_lower.contains("engine overheated") || msg_lower.contains("water overheated") {
            return Some(GameEvent::EngineOverheat);
        }
        
        if msg_lower.contains("oil overheated") {
            return Some(GameEvent::OilOverheated);
        }
        
        None
    }
    
    /// Get new messages since last ID
    pub fn get_new_messages(&self, last_event_id: u32, last_damage_id: u32) -> (Vec<HudMessage>, Vec<HudMessage>) {
        let new_events: Vec<HudMessage> = self.events.iter()
            .filter(|msg| msg.id > last_event_id)
            .cloned()
            .collect();
        
        let new_damage: Vec<HudMessage> = self.damage.iter()
            .filter(|msg| msg.id > last_damage_id)
            .cloned()
            .collect();
        
        (new_events, new_damage)
    }
    
    /// Extract all game events from messages
    pub fn extract_events(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        
        for msg in &self.events {
            if let Some(event) = Self::parse_event(&msg.msg) {
                events.push(event);
            }
        }
        
        for msg in &self.damage {
            if let Some(event) = Self::parse_event(&msg.msg) {
                events.push(event);
            }
        }
        
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_destroyed() {
        let msg = "=DEST= _WingsOfPrey_ (IJN Yamashiro) destroyed PT-103";
        assert_eq!(HudMessages::parse_event(msg), Some(GameEvent::TargetDestroyed));
    }
    
    #[test]
    fn test_parse_set_afire() {
        let msg = "gouliguojia386 (HMS Nelson) set afire Emden";
        assert_eq!(HudMessages::parse_event(msg), Some(GameEvent::TargetSetOnFire));
    }
    
    #[test]
    fn test_parse_first_strike() {
        let msg = "Dogbolter (HMS Rodney) has delivered the first strike!";
        assert_eq!(HudMessages::parse_event(msg), Some(GameEvent::FirstStrike));
    }
    
    #[test]
    fn test_parse_critically_damaged() {
        let msg = "=TVS4N= The_Fent_Fairy (␗F-16A) critically damaged [UNSKL] FriskQaQ (F-15E)";
        assert_eq!(HudMessages::parse_event(msg), Some(GameEvent::TargetCritical));
    }
}


