# War Thunder Datamine Module

Extracts vehicle limits and characteristics from War Thunder game files for haptic feedback system.

## Safety

- ✅ **EAC-Safe**: Only reads files on disk, no process interaction
- ✅ **Read-only**: Never modifies game files
- ✅ **Offline**: Works without internet connection
- ⚠️ **ToS**: May violate Gaijin ToS (use at own risk)

## Features

- **Automatic initialization** on app startup
- Parses aircraft flight limits (Vne, G-limits, flutter speed)
- Parses ground vehicle characteristics (speed, engine, armor)
- Parses ship compartments and damage model
- SQLite database for fast access (cached in AppData)
- Auto-detection of game installation
- Background parsing (doesn't block UI)

## Automatic Initialization

The datamine **automatically initializes on app startup**:
1. Checks if database exists and has data
2. If empty, auto-detects War Thunder installation
3. Parses all vehicle files in background
4. Stores in SQLite cache for instant access

**No manual action required!**

## Manual Usage (for re-parsing)

```rust
use datamine::Datamine;

// Auto-detect game path
let game_path = Datamine::auto_detect()?;

// Create datamine instance
let mut dm = Datamine::new(game_path)?;

// Parse all vehicles
let stats = dm.parse_all()?;

// Get vehicle limits
let limits = dm.get_limits("bf-109f-4");
```

## Data Extracted

### Aircraft
- `vne_kmh`: Never Exceed Speed
- `max_positive_g`: Positive G-load limit
- `max_negative_g`: Negative G-load limit
- `flutter_speed`: Flutter warning speed
- `mass_kg`: Takeoff mass
- `max_rpm`: Maximum engine RPM

### Ground Vehicles
- `max_speed_kmh`: Maximum forward speed
- `horse_power`: Engine power
- `max_rpm`: Maximum RPM
- `hull_hp`: Hull hit points
- `armor_thickness_mm`: Frontal armor

### Ships
- `max_speed_knots`: Maximum speed
- `compartments`: Damage model compartments
- Critical components (engine room, ammo storage)

## Database Location

SQLite database is stored in:
- Windows: `%LOCALAPPDATA%\Tailgunner\vehicle_limits.db`
- Linux: `~/.local/share/Tailgunner/vehicle_limits.db`
- macOS: `~/Library/Application Support/Tailgunner/vehicle_limits.db`

## File Formats

Game files are JSON with `.blkx` extension:
- `aces.vromfs.bin_u/gamedata/flightmodels/fm/*.blkx` - Aircraft
- `aces.vromfs.bin_u/gamedata/units/tankmodels/*.blkx` - Tanks
- `aces.vromfs.bin_u/gamedata/units/ships/*.blkx` - Ships

## Performance

- ~1,200 aircraft: ~15 seconds
- ~800 tanks: ~10 seconds
- ~400 ships: ~5 seconds
- **Total**: ~30 seconds for full parse

## License

GPL-3.0-or-later (same as Tailgunner)

