# Changelog

All notable changes to Tailgunner will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.8.1] - 2025-01-26

### Added
- **HUD Events System**
  - Real-time kill feed integration from War Thunder's `/hudmsg` endpoint
  - Damage event tracking (player kills, deaths, crashes)
  - Smart initialization - processes recent 15 seconds of events on startup
  - Duplicate message prevention with time-based caching
  - Event ID tracking to prevent processing old events

- **Player Identity System**
  - SQLite-based persistent storage for player/clan tracking
  - Track your own nickname for "I killed someone" triggers
  - Clan tag support for team coordination
  - Enemy player lists for targeted feedback
  - Enemy clan lists for faction-based triggers
  - Context-aware filter descriptions in UI ("I killed tracked enemy" vs "Enemy damaged me")

- **Vehicle Datamine Integration**
  - VROMFS file unpacker (EAC-safe - unpacks to TEMP directory)
  - Aircraft performance limits parser (g-load, Vne, flutter speed)
  - Auto-generated vehicle-specific triggers:
    - High G Warning (80% of max positive G)
    - Negative G Warning (80% of max negative G)
    - Flutter Warning (at flutter speed threshold)
    - Critical Speed Warning (95% of Vne)
  - Automatic trigger updates when changing aircraft

- **Continuous Vibration Mode**
  - Triggers can now vibrate continuously while condition is active
  - Short 200ms patterns that repeat seamlessly every tick
  - Smooth sustained feedback for G-load, overspeed, etc.
  - Automatic stop when condition becomes false

- **Debug Console**
  - Real-time trigger activation logs with timestamps
  - Live trigger event display (name, event type, entity)
  - Duplicate filtering to prevent log spam
  - Color-coded log levels (success, info, warn, error)
  - Game state telemetry display

### Changed
- **Event System Refactoring**
  - Separated event-based triggers (kills, damage) from state-based triggers (speed, altitude)
  - Event triggers only fire on actual HUD events, not every tick
  - State triggers check conditions continuously (200ms polling)
  - Improved performance by reducing unnecessary trigger checks

- **Kill Event Filtering**
  - Fixed incorrect filtering logic for "my_players" on kill events
  - "I killed someone" now correctly identifies player as attacker
  - "Enemy damaged me" properly checks victim identity
  - Added explanatory hints in EventNode UI for clarity

- **Pattern Execution**
  - Fixed vibration duration calculation bug (patterns were too long)
  - Corrected burst repeat count (was off by 1)
  - Changed sleep logic to use time differences instead of absolute times
  - Custom curve points now properly applied from UI editor

- **UI Improvements**
  - User-created patterns no longer appear in "All Triggers" or "Active Profiles"
  - "All Triggers" section now hidden when empty
  - Better organization between Pattern Manager and Game Events
  - Localization improvements (all UI text now translatable)

### Fixed
- **Critical Bugs**
  - Fixed crash when HUD message time wraps around (overflow protection)
  - Fixed API parsing bug: `events` (plural) vs `event` (singular)
  - Fixed HUD event system not being called (was never initialized in main loop)
  - Fixed `get_vehicle_limits_manager` returning new instance instead of existing one
  - Fixed vibration patterns ignoring custom curve points from UI

- **Linter Warnings**
  - Removed unused `active_continuous_triggers` HashMap field
  - Added `#[allow(dead_code)]` to future-use API methods
  - Removed unused imports (`Curve`, `EnvelopeStage`)
  - Fixed unused variable `id` in HUD parser

- **Logging**
  - Reduced log spam by commenting out high-frequency debug messages
  - Changed HUD debug logs from ERROR to INFO/DEBUG levels
  - Only log damage messages when count > 0

### Performance
- **Optimizations**
  - HUD event caching with 10-second TTL to reduce redundant parsing
  - Separated event-based and state-based trigger evaluation paths
  - Continuous triggers use minimal 200ms patterns for smooth updates
  - Improved cooldown tracking with HashMap instead of linear search

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

