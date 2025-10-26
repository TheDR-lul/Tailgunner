# 🚢 Ship Support & MiniMap Module

## Overview

War Thunder API does NOT provide `indicators` or `state` telemetry for ships, even during active battles. This module implements a **hybrid approach**:

- **Ships**: Use HUD events from `/hudmsg` endpoint
- **Tanks/Aircraft**: Use standard `indicators` and `state` telemetry

## Architecture

### 1. HUD Messages Parser (`hud_messages.rs`)

Parses HUD messages from `/hudmsg?lastEvt=&lastDmg=` endpoint.

**Supported Events:**
- `TargetDestroyed` - Enemy vehicle destroyed
- `TargetSetOnFire` - Enemy set on fire
- `TargetCritical` - Enemy critically damaged
- `TargetSeverelyDamaged` - Enemy severely damaged
- `VehicleDestroyed` - Player's vehicle destroyed
- `FirstStrike` - First strike achievement
- `ShipRescuer` - Ship rescuer achievement
- `Achievement` - Generic achievement

**Example HUD Message:**
```json
{
  "id": 7,
  "msg": "gouliguojia386 (HMS Nelson) set afire Emden",
  "sender": "",
  "enemy": false,
  "mode": "",
  "time": 114
}
```

### 2. Vehicle Type Detection (`wt_telemetry.rs`)

Auto-detects vehicle type from `army` field:
- `"air"` → `Aircraft`
- `"ground"` / `"tank"` → `Tank`
- `"ship"` → `Ship`
- `"helicopter"` → `Helicopter`

**Special handling for ships:**
```rust
// Mark as valid if we detected it's a ship, even if indicators say valid=false
let final_valid = if matches!(type_, VehicleType::Ship) && !valid {
    log::info!("[WT Parser] 🚢 Ship detected but indicators invalid - using HUD-only mode");
    !vehicle_name.is_empty() && vehicle_name != "unknown"
} else {
    valid
};
```

### 3. Event Engine Integration (`event_engine.rs`)

**Hybrid event detection:**
```rust
// FOR SHIPS: Use HUD events as primary source (indicators/state not available)
if matches!(current_state.type_, VehicleType::Ship) {
    for hud_event in &current_state.hud_events {
        if let Some(game_event) = self.map_hud_event_to_game_event(hud_event) {
            events.push(game_event);
        }
    }
}

// FOR TANKS/AIRCRAFT: Detect new events in state array (standard method)
if !matches!(current_state.type_, VehicleType::Ship) {
    // ... standard state-based detection
}
```

## 🗺️ MiniMap Module

### Backend (`map_module.rs`)

**Data Structures:**
```rust
pub struct MapObject {
    pub obj_type: String,
    pub icon: String,      // "Ship", "Aircraft", "Tank", "Player", "capture_zone"
    pub x: f32, y: f32,    // Position (0..1)
    pub dx: f32, dy: f32,  // Direction vector
    pub color: String,     // "#174DFF" (blue=friendly), "#fa0C00" (red=enemy)
}

pub struct MapInfo {
    pub grid_size: Vec<f32>,
    pub grid_steps: Vec<f32>,
    pub map_min: Vec<f32>,
    pub map_max: Vec<f32>,
    pub valid: bool,
}

pub struct MapData {
    pub objects: Vec<MapObject>,
    pub info: MapInfo,
    pub player_position: Option<(f32, f32)>,
    pub player_heading: Option<f32>,
}
```

**Helper Methods:**
- `is_player()` - Check if object is player
- `is_friendly()` - Blue color check
- `is_enemy()` - Red color check
- `is_ship()`, `is_aircraft()`, `is_tank()` - Vehicle type checks
- `get_heading()` - Calculate heading angle from dx/dy
- `get_friendly_ships()`, `get_enemy_ships()` - Filter by type
- `get_player_grid()` - Convert position to grid cell (e.g., "A5")
- `count_nearby_enemies()` - Count enemies within radius

### Tauri Commands (`lib.rs`)

```rust
#[tauri::command]
async fn get_map_objects() -> Result<Vec<MapObject>, String>

#[tauri::command]
async fn get_map_info() -> Result<MapInfo, String>

#[tauri::command]
async fn get_map_data() -> Result<MapData, String>
```

### Frontend (`MiniMap.tsx`)

**Features:**
- ✅ Real-time map updates (500ms interval)
- ✅ Grid overlay
- ✅ Player position with heading arrow (yellow)
- ✅ Friendly units (blue triangles)
- ✅ Enemy units (red triangles)
- ✅ Capture zones (circles)
- ✅ Stats overlay (position, heading, object count)
- ✅ Enable/Disable toggle
- ✅ Error handling (War Thunder not running, map unavailable)

**Rendering:**
```typescript
// Player: Yellow arrow with rotation based on heading
drawPlayer(ctx, canvas, playerObj, mapInfo);

// Ships: Triangles
// Capture zones: Large circles
// Other objects: Small squares
drawMapObject(ctx, canvas, obj, mapInfo);
```

## API Endpoints Used

### Ships (HUD-only mode):
- `/hudmsg?lastEvt=<id>&lastDmg=<id>` - HUD messages with combat events
- `/map_obj.json` - Map objects (ships, capture zones)
- `/map_info.json` - Map boundaries and grid

### Tanks/Aircraft (Full telemetry):
- `/indicators` - Cockpit indicators (speed, RPM, fuel, etc.)
- `/state` - Detailed flight/vehicle state
- `/hudmsg` - Supplementary events

## Event Mapping

### HudEvent → GameEvent
```rust
HudEvent::Kill(_) => GameEvent::TargetDestroyed
HudEvent::SetAfire(_) => GameEvent::TargetSetOnFire
HudEvent::TakeDamage(_) => GameEvent::Hit
HudEvent::SeverelyDamaged(_) => GameEvent::SeverelyDamaged
HudEvent::ShotDown(_) => GameEvent::VehicleDestroyed
HudEvent::Achievement(_) => GameEvent::Achievement
HudEvent::Crashed => GameEvent::Crashed
HudEvent::EngineOverheated => GameEvent::EngineOverheat
```

## Usage

### Creating Ship Patterns

1. **Enable system**: Click "Start" in Dashboard
2. **Join naval battle**: Enter any ship battle in War Thunder
3. **Open MiniMap**: Enable map in sidebar
4. **Create pattern**: Go to Pattern Manager → Create New Pattern
5. **Add triggers**:
   - Event Node: `TargetDestroyed` (when you destroy enemy ship)
   - Event Node: `TargetSetOnFire` (when you set enemy on fire)
   - Event Node: `Hit` (when you take damage)
6. **Add outputs**: Vibration nodes with intensity/duration

### Pattern Example (Ship Combat)
```
[TargetDestroyed] → [Linear Ramp] → [Vibration 100% for 1000ms]
                 ↘
[TargetSetOnFire] → [Vibration 70% for 500ms]
                 ↘
[Hit] → [Pulse] → [Vibration 50% for 200ms]
```

## Testing

### Test HUD Message Parsing
```bash
cargo test -p butt_thunder_lib test_parse
```

### Test Map Module
```bash
cargo test -p butt_thunder_lib --lib map_module
```

### Manual Testing
1. Start War Thunder
2. Join naval battle
3. Open application
4. Enable MiniMap
5. Check console for:
   ```
   [WT Parser] 🚢 Ship detected but indicators invalid - using HUD-only mode
   [HUD] 📥 Received N damage messages
   [HUD Event] 💬 CHAT at Xs: <message>
   ```

## Limitations

1. **No speed/RPM for ships**: War Thunder API doesn't provide these values
2. **HUD-only triggers**: Ships can only use event-based triggers (no speed/altitude/etc.)
3. **Update rate**: HUD messages update every ~500ms (vs 100ms for indicators)
4. **Map precision**: Map coordinates are normalized (0..1), not exact world positions

## Future Enhancements

- [ ] Parse ship-specific damage zones (fire, flooding, ammunition)
- [ ] Detect torpedo/bomb drops from HUD messages
- [ ] Add map zoom/pan controls
- [ ] Ship-specific trigger nodes (water speed, torpedo reload, etc.)
- [ ] Historical battle detection (show map context)

## References

- [War Thunder API Documentation](http://localhost:8111/)
- [WT API Endpoints Dump](reports/wt-api-dump-*.json)
- Ships use `army: "ship"` in indicators JSON (but `valid: false`)
- HUD messages work for ALL vehicle types (universal fallback)


