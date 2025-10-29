# Changelog

All notable changes to Tailgunner will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.9.0] - 2025-10-29

### Added
- **Test Mode Emulator** (v0.8.2-0.8.8)
  - Full War Thunder API emulator on port 8112 with 12 HTTP endpoints
  - 27 vehicle models with realistic characteristics (Aircraft, Tanks, Ships)
  - 70+ emulated parameters (speed, altitude, fuel, G-load, etc.)
  - Automatic computed parameters (IAS, TAS, Mach, RPM, oil temp, etc.)
  - Chat emulation with 6 player presets and team/enemy identification
  - Test Mode toggle in header with automatic telemetry port switching

- **Gamepad Rumble Proxy**
  - Translate War Thunder gamepad vibration to haptic devices
  - Real-time rumble intensity monitoring with visual indicators
  - Configurable sensitivity, motor weights, and smoothing
  - Enable/disable toggle with persistent configuration

- **Pattern Templates**
  - 9 pre-built patterns (Quick Burst, Hit Marker, Engine Rumble, Explosion, etc.)
  - Categories: Combat, Movement, Ambient
  - One-click import with search and filtering

- **Keyboard Shortcuts**
  - `Ctrl+E`: Toggle engine, `Ctrl+N`: New pattern, `Ctrl+T`: Templates, `Ctrl+K`: Shortcuts help

- **Analytics Dashboard**
  - Track vibrations sent, duration, top patterns, session time

- **Interactive Curve Editors**
  - Canvas-based motion/speed curve graphs for Linear and Rotate nodes
  - Drag points with LMB/RMB, double-click to remove
  - Time-synchronized axis labels and grid overlay

- **Interactive MiniMap**
  - Real-time tactical map with live object tracking
  - Dynamic zoom & pan (1.0x-4.0x), mouse wheel + drag controls
  - Follow Player mode with auto-centering
  - View modes: Current (cropped, matches in-game) / Full Map
  - Map tools: Distance measure, place markers, nearest units tracker
  - Grid reference system (A-Z, 1-9) matching War Thunder
  - Compass bearings (N, NE, E, SE, S, SW, W, NW)
  - Configurable update rate (200-2000ms)
  - Accurate coordinate conversion and map crop matching in-game view
  - Real-time distance calculations in meters

### Changed
- **UI Responsive Design**
  - Complete interface scaling audit with flexible CSS Grid layouts
  - Media queries for mobile/tablet/desktop breakpoints
  - Converted fixed pixels to responsive units (`rem`, `flex`, `%`)
  - Minimap and Player Info properly positioned

- **Pattern Editor Improvements**
  - Categorized sidebar (Inputs, Logic, Actions, Output, Tools)
  - Real-time stats in header (nodes, connections, validation)
  - Snap to grid, auto-layout, clear all functions
  - `Ctrl+S` save shortcut
  - Removed React Flow watermark and right-click menu

- **Node UX Improvements**
  - Duration changed from slider to number input
  - Removed redundant position/speed sliders (replaced by curves)
  - Fixed all interactive controls with `nodrag` class

### Fixed
- **Event Filtering** (v0.8.9)
  - Fixed Game Feed not clearing on battle change
  - Fixed triggers firing on old events after app restart
  - Two-level filtering (backend + frontend) with time reset detection
  - Bulk load detection to ignore 100+ accumulated HUD messages

- **Pattern System**
  - Fixed sliders not responding in Linear/Rotate nodes
  - Fixed "Cannot read properties of undefined" error in pattern loading
  - Fixed templates creating invalid node types
  - Improved localStorage validation with default values

- **UI Consistency**
  - Fixed modal close button inconsistency
  - Fixed Pattern Editor going off-screen on window resize
  - Reduced HUD fetch log verbosity (debug → trace level)

---

## [0.8.1] - 2025-01-26

### Added

#### 🎮 HUD Events System
Real-time event tracking from War Thunder's kill feed:
- **Kill Feed Integration** - Monitors `/hudmsg` endpoint for real-time events
- **Event Types** - Kills, Deaths, Crashes, Engine Overheats, Fire Events, Achievements
- **Smart Initialization** - Processes recent 15 seconds of events on startup (no missed kills!)
- **Intelligent Caching** - 10-second TTL to prevent duplicate event processing
- **Event ID Tracking** - Prevents processing old events after app restart

#### 👤 Player Identity System
Track specific players and create personalized feedback:
- **SQLite Database** - Persistent storage for player/clan tracking
- **Your Nickname** - Set your in-game name for "I killed someone" triggers
- **Clan Tags** - Monitor your clan's activities
- **Enemy Lists** - Track specific opponents for targeted feedback
- **Enemy Clans** - Create faction-based triggers

**Smart Filters:**
- ✅ "I killed someone" - Vibrates when YOU get a kill
- 🎯 "I killed tracked enemy" - Special feedback for specific targets
- 💥 "Enemy damaged me" - Feel when you're taking damage
- ☠️ "Enemy clan hit me" - Know when enemy clan attacks

#### 🛩️ Vehicle Datamine Integration
Auto-generated triggers based on your aircraft's real limits:
- **VROMFS Unpacker** - Safely extracts War Thunder files (EAC-safe, unpacks to TEMP)
- **Aircraft Performance Database** - G-load, Vne, flutter speed for 1000+ aircraft
- **Auto-Generated Triggers:**
  - High G Warning (80% of max positive G)
  - Negative G Warning (80% of max negative G)
  - Flutter Speed Warning (at flutter speed threshold)
  - Critical Overspeed (95% of Vne wing rip speed)
- **Per-Vehicle Limits** - Triggers automatically update when changing aircraft

#### 🔄 Continuous Vibration Mode
Sustained feedback while conditions are active:
- **Real Sustained Feedback** - Vibrates continuously while condition is true
- **Smooth Patterns** - 200ms loops blend seamlessly
- **Auto-Stop** - Vibration ends when condition becomes false
- **Perfect For:** G-load warnings, overspeed alerts, flutter warnings, low fuel

#### 🐛 Debug Console
Monitor everything in real-time:
- **Live Trigger Logs** - See exactly when triggers fire
- **Event Tracking** - "🎯 Kill Pattern: TargetDestroyed (T-62)"
- **Timestamps** - Track event timing
- **Color-Coded** - Success (green), Info (blue), Warn (yellow), Error (red)
- **Duplicate Prevention** - Clean, spam-free logs
- **Game State Display** - Real-time telemetry monitoring

### Changed

#### 🔧 Event System Refactoring
- **Separated Trigger Types:**
  - Event-based: Kills, damage (fire only on HUD events)
  - State-based: Speed, altitude (check every 200ms)
- **Improved Performance** - Reduced unnecessary trigger checks
- **No Spam** - Event triggers fire once per event, not every tick

#### Kill Event Filtering
- Fixed incorrect "my_players" filter logic
- "I killed someone" now correctly identifies player as attacker
- "Enemy damaged me" properly checks victim identity
- Added explanatory UI hints in EventNode

#### Pattern Execution
- Fixed vibration duration calculation bug (patterns were too long)
- Corrected burst repeat count (was off by 1)
- Changed sleep logic to use time differences
- Custom curve points from UI editor now properly applied

#### UI/UX Improvements
- User-created patterns no longer clutter "All Triggers" section
- "All Triggers" section hidden when empty
- Better separation: Pattern Manager vs Game Events
- Context-aware filter descriptions in EventNode
- Full localization support (EN/RU)

### Fixed

#### Critical Bugs
- Fixed crash when HUD message time wraps around (overflow protection with `saturating_sub`)
- Fixed API parsing: `events` (plural) vs `event` (singular) - HUD system now works!
- Fixed HUD event system not being initialized in main loop
- Fixed `get_vehicle_limits_manager()` creating new instance instead of returning existing
- Fixed vibration patterns ignoring custom curve points from UI

#### Linter Warnings
- Removed unused `active_continuous_triggers` HashMap field
- Added `#[allow(dead_code)]` to future-use API methods
- Removed unused imports (`Curve`, `EnvelopeStage`)
- Fixed unused variable `_id` in HUD parser

#### Logging
- Reduced log spam (commented out high-frequency debug messages)
- Changed HUD logs from ERROR to INFO/DEBUG levels
- Only log damage messages when count > 0

### Performance
- **Polling Rate:** 10 Hz (100ms intervals)
- **HUD Events:** Real-time processing as they arrive
- **Continuous Triggers:** 200ms pattern loops
- **Memory Usage:** ~50-80 MB
- **CPU Usage:** <5% during normal gameplay
- **Optimizations:**
  - HUD event caching with 10-second TTL
  - Separated event/state trigger evaluation paths
  - Improved cooldown tracking with HashMap

### Known Issues
- `usePercentage` feature in ConditionNode not yet implemented (use raw values)
- Tank datamine not yet fully integrated (only aircraft limits available)
- Naval datamine not yet integrated

---

## [0.7.10] - 2025-01-20

### Added
- VROMFS unpacker with fallback mechanisms
- Vehicle limits database
- Basic datamine integration

### Fixed
- Multiple VROMFS unpacking issues
- Vehicle info display bugs

---

## [0.7.0] - 2025-01-15

### Added
- Visual node-based pattern editor
- 50+ game parameters from War Thunder API
- Multi-condition nodes with AND/OR logic
- Custom vibration curve editor
- Per-trigger cooldown and duration settings
- Profile system (Tank RB, Aircraft, Light Background)
- Linear motion and rotation device support
- Full English/Russian localization
- Persistent trigger configuration

### Changed
- Complete UI redesign with React Flow
- Moved from simple trigger list to node-based system

---

## [0.6.0] - 2025-01-10

### Added
- Basic trigger system
- Profile switching
- Buttplug.io device integration

---

## Earlier Versions

See [reports/](reports/) directory for detailed changelogs of earlier versions.

