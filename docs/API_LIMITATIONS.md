# War Thunder API Limitations

## Overview
The Tailgunner application uses the **War Thunder localhost API** (`localhost:8111`) to receive telemetry data from the game. This API has certain limitations that affect functionality.

---

## 🚫 **Known Limitations**

### 1. **Enemy Positions in Realistic Battles (RB)**
**Status:** ❌ Not Available

**Description:**  
In Realistic Battles (RB) and Simulator Battles (SB), the War Thunder API **does not provide enemy positions** on the map. This is an **intentional anti-cheat measure** by Gaijin Entertainment.

**What you'll see:**
- Only friendly units (blue markers) on the minimap
- No red enemy markers in RB/SB modes
- Full functionality in Arcade Battles (AB) where enemy positions are visible

**Affected features:**
- Enemy distance calculation
- Enemy markers on minimap
- "Nearest Enemies" display

---

### 2. **Ship Telemetry Data**
**Status:** ⚠️ Limited

**Description:**  
Ship telemetry (`/indicators` and `/state` endpoints) returns `valid: false` even when actively controlling a ship in battle.

**What works:**
- HUD events (kills, hits, fires, flooding)
- Combat achievements
- Map position
- Chat messages

**What doesn't work:**
- Speed indicators
- Engine RPM
- Detailed damage state
- Fuel consumption

**Workaround:**  
The application uses a **hybrid system**:
- Ships: HUD-only mode (combat events from `/hudmsg`)
- Tanks/Aircraft: Full telemetry mode (all data from `/indicators` and `/state`)

---

### 3. **Advanced Flight Data**
**Status:** ❌ Not Available

**Description:**  
The API does not provide real-time flight dynamics data:
- G-forces (current and historical)
- Angle of Attack (AoA)
- Stall/spin detection
- Overspeed warnings
- Flutter detection

**Impact:**  
Events like `OverG`, `Stall`, `Spin`, `HighAOA`, `Overspeed` have been **removed** from the application as they cannot be reliably detected.

---

### 4. **Detailed Damage Information**
**Status:** ⚠️ Limited

**Description:**  
Specific damage states are not reported:
- Individual module damage (engine, fuel tanks, ammo)
- Fire location and intensity
- Leak severity (fuel, water, oil)
- Track/wheel damage (ground vehicles)

**What works:**
- HUD-based damage notifications
- Crew knocked out events
- Critical hits (from HUD messages)
- General "taking damage" events

---

### 5. **Ammo and Fuel Counters**
**Status:** ❌ Not Reliable

**Description:**  
While `/indicators` provides `ammo` and `fuel` fields, they are:
- Not updated in real-time
- Often show incorrect values
- Not available for all vehicle types

**Impact:**  
Events like `LowAmmo`, `LowFuel`, `CriticalFuel` have been **removed**.

---

## ✅ **What Works Reliably**

### HUD Events (`/hudmsg`)
- ✅ Kills and assists
- ✅ Hits (giving and receiving)
- ✅ Critical hits
- ✅ Set enemy on fire
- ✅ Achievements
- ✅ Player join/disconnect
- ✅ Team kills
- ✅ Crashes

### Map Data (`/map_obj.json`, `/map_info.json`)
- ✅ Player position (all modes)
- ✅ Friendly unit positions (all modes)
- ✅ Enemy positions (Arcade only)
- ✅ Capture zones
- ✅ Spawn points
- ✅ Map boundaries and grid

### Chat (`/gamechat`)
- ✅ All chat messages (team, all, squad)
- ✅ Message timestamps
- ✅ Sender information
- ✅ Enemy/ally identification

### Mission Info (`/mission.json`)
- ✅ Primary and secondary objectives
- ✅ Objective status (in progress, completed, failed)
- ✅ Mission status

### Basic Indicators (`/indicators`)
- ✅ Engine RPM (tanks, aircraft - not ships)
- ✅ Crew status
- ✅ Gear position (aircraft)
- ✅ Flaps position (aircraft)

---

## 🔧 **Technical Implementation**

The application uses multiple strategies to work around API limitations:

1. **Hybrid Telemetry System**
   - Ships: HUD-only mode
   - Other vehicles: Full telemetry mode

2. **Event Detection Priorities**
   ```
   1. HUD messages (/hudmsg)        - Most reliable
   2. Indicators (/indicators)       - Moderate reliability
   3. State (/state)                 - Basic data only
   4. Chat (/gamechat)               - Supplementary
   5. Mission (/mission.json)        - Mission-specific
   ```

3. **Removed Non-Functional Events**
   - Over **60 events** that couldn't be reliably detected were removed
   - Only **24 actionable events** remain
   - See `src-tauri/src/pattern_engine.rs` for the current `GameEvent` enum

---

## 📚 **Further Reading**

- [War Thunder API Documentation](https://localhost:8111/) - Visit while game is running
- [Community API Reverse Engineering](https://wiki.warthunder.com/Localhost)

---

## ⚠️ **Disclaimer**

These limitations are **inherent to the War Thunder API** and cannot be bypassed without violating the game's Terms of Service. Tailgunner respects these limitations and only uses publicly available, officially sanctioned API endpoints.

**Last Updated:** October 2025  
**War Thunder Version:** Current Live Client

