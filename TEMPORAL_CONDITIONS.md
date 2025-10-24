# Temporal Conditions Guide

## Overview

Temporal conditions allow you to create triggers based on how values **change over time**, not just their current values. This enables sophisticated patterns like:

- "Speed dropped by 200 km/h in 2 seconds" (hard brake)
- "G-load spiked by 5G in 0.5 seconds" (aggressive maneuver)
- "Average speed over 600 km/h for last 5 seconds" (sustained high speed)

---

## UI: Node Editor

### Input Node - Temporal Operators

When creating a pattern in the node editor, the **Input Node** now has two sections of operators:

#### **Instant Operators** (original)
- `>` Greater than
- `<` Less than
- `≥` Greater or equal
- `≤` Less or equal
- `=` Equal

#### **Over Time Operators** (new) ⏱️
- **▼ Dropped** - Value dropped by X over Y seconds
- **▲ Increased** - Value increased by X over Y seconds
- **⇧ Accel+** - Acceleration (positive change rate)
- **⇩ Accel-** - Deceleration (negative change rate)
- **~ Avg >** - Average value over time window

### Time Window Parameter

When you select a temporal operator, a new field appears:

```
over [1.0] sec
```

This controls the **time window** for analysis (0.1 to 10.0 seconds).

---

## Examples

### 1. Emergency Brake Detection

**Goal:** Detect when pilot/driver slams brakes (speed drops rapidly)

**Node Setup:**
```
[INPUT: Speed | ▼ Dropped | 150 | over 1.5 sec]
  ↓
[VIBRATION: Heavy Pulse]
  ↓
[OUTPUT]
```

**Explanation:** If speed drops by 150+ km/h within 1.5 seconds, trigger heavy vibration.

---

### 2. Aggressive Dogfight Maneuver

**Goal:** Detect sudden high-G turns

**Node Setup:**
```
[INPUT: G-Load | ▲ Increased | 5.0 | over 0.5 sec]
  ↓
[VIBRATION: Sharp Spike]
  ↓
[OUTPUT]
```

**Explanation:** If G-load increases by 5G+ within 0.5 seconds, trigger sharp spike.

---

### 3. Sustained High-Speed Flight

**Goal:** Reward maintaining high speed

**Node Setup:**
```
[INPUT: Speed | ~ Avg > | 600 | over 5.0 sec]
  ↓
[VIBRATION: Smooth Hum]
  ↓
[OUTPUT]
```

**Explanation:** If average speed stays above 600 km/h for 5 seconds, trigger smooth hum.

---

### 4. Stall Warning (Complex Logic)

**Goal:** Detect dive stall (altitude dropping + sudden G spike)

**Node Setup:**
```
[INPUT A: Altitude | ▼ Dropped | 1000 | over 3.0 sec]
  ↓
[LOGIC: AND]
  ↑
[INPUT B: G-Load | ▲ Increased | 5.0 | over 1.0 sec]
  ↓
[VIBRATION: Critical Warning]
  ↓
[OUTPUT]
```

**Explanation:** Combines two temporal conditions - altitude must drop 1000m+ in 3 seconds AND G-load spike 5G+ in 1 second.

---

## Available Temporal Conditions

### Speed/IAS/TAS
- `dropped_by` → `SpeedDroppedBy`
- `increased_by` → `SpeedIncreasedBy`
- `accel_above` → `AccelerationAbove` (km/h per second)
- `accel_below` → `AccelerationBelow` (deceleration)
- `avg_above` → `AverageSpeedAbove`

### Altitude
- `dropped_by` → `AltitudeDroppedBy`
- `increased_by` → `AltitudeGainedBy`
- `accel_above` → `ClimbRateAbove` (m/s)

### G-Load
- `increased_by` → `GLoadSpiked`
- `avg_above` → `AverageGLoadAbove`

---

## Backend: State History System

### Architecture

The system uses a **circular buffer** to store recent game state snapshots:

- **Capacity:** 100 snapshots (max 10 seconds at 10 Hz)
- **Auto-pruning:** Old snapshots are automatically removed
- **Tracked Metrics:** speed, altitude, g_load, aoa, rpm, fuel

### Rust API

#### StateHistory Methods

```rust
// Check if value dropped by threshold
history.dropped_by(150.0, 1.5, speed_extractor)  // true/false

// Check if value increased by threshold
history.increased_by(5.0, 0.5, g_load_extractor)

// Get rate of change (delta per second)
history.rate_of_change(1.0, speed_extractor)  // Some(50.0) or None

// Get average value over window
history.average(5.0, speed_extractor)  // Some(620.5) or None

// Get min/max over window
history.min(3.0, altitude_extractor)
history.max(3.0, g_load_extractor)
```

#### TriggerCondition Enum

```rust
pub enum TriggerCondition {
    // ... existing instant conditions ...
    
    // Temporal conditions
    SpeedDroppedBy { threshold: f32, window_seconds: f32 },
    SpeedIncreasedBy { threshold: f32, window_seconds: f32 },
    AccelerationAbove { threshold: f32, window_seconds: f32 },
    AccelerationBelow { threshold: f32, window_seconds: f32 },
    AltitudeDroppedBy { threshold: f32, window_seconds: f32 },
    AltitudeGainedBy { threshold: f32, window_seconds: f32 },
    ClimbRateAbove { threshold: f32, window_seconds: f32 },
    GLoadSpiked { threshold: f32, window_seconds: f32 },
    SuddenGChange { threshold: f32, window_seconds: f32 },
    FuelDepletingFast { threshold: f32, window_seconds: f32 },
    AverageSpeedAbove { threshold: f32, window_seconds: f32 },
    AverageGLoadAbove { threshold: f32, window_seconds: f32 },
}
```

#### Evaluation

Temporal conditions require history:

```rust
// Standard evaluation (instant conditions only)
condition.evaluate(state)  // temporal conditions always return false

// Evaluation with history (supports both)
condition.evaluate_with_history(state, Some(&history))  // full support
```

The `TriggerManager` automatically:
1. Records state snapshots on every tick
2. Evaluates conditions with history
3. Manages trigger cooldowns

---

## Dynamic Triggers

The system also creates **dynamic triggers** based on vehicle characteristics:

### From WT Vehicles API

When you enter a vehicle, the app fetches:
- `max_speed_kmh` → Overspeed warning at 90%
- `max_positive_g` → High G warning at 95%
- `max_negative_g` → Negative G warning at 95%

### Example

**Vehicle:** Rafale C F3  
**Max Speed:** 2650 km/h  
**Overspeed Warning:** 2385 km/h (90%)

**Auto-generated trigger:**
```rust
EventTrigger {
    id: "dynamic_overspeed",
    name: "Overspeed Warning (2385+ km/h)",
    description: "Approaching max speed of 2650 km/h",
    condition: TriggerCondition::SpeedAbove(2385.0),
    event: GameEvent::Overspeed,
    cooldown_ms: 5000,
    enabled: true,
    is_builtin: false,
}
```

---

## Performance Notes

- State history uses minimal memory (~50 KB)
- History pruning happens automatically
- Circular buffer prevents memory growth
- Rate of change calculations are O(1)

---

## Future Enhancements

Potential additions:
- **Peak Detection** - Detect local maxima/minima
- **Trend Analysis** - Detect rising/falling trends
- **Oscillation Detection** - Detect periodic patterns
- **Multi-metric Correlation** - "Speed up while altitude down"

---

## FAQ

**Q: Why are my temporal patterns not triggering?**  
A: State history needs at least `window_seconds` of data. Wait a few seconds after starting the game.

**Q: Can I combine temporal with instant conditions?**  
A: Yes! Use LOGIC nodes (AND/OR/XOR/NOT) to combine any conditions.

**Q: What's the maximum time window?**  
A: 10 seconds (limited by circular buffer size). For longer periods, use average conditions.

**Q: Are temporal conditions more CPU-intensive?**  
A: Slightly, but negligible. The circular buffer and rate calculations are highly optimized.

---

## Technical Details

### Implementation Files

- `src-tauri/src/state_history.rs` - Circular buffer & analysis
- `src-tauri/src/event_triggers.rs` - Condition evaluation
- `src-tauri/src/ui_patterns.rs` - Pattern parsing
- `src/components/nodes/InputNode.tsx` - UI controls

### Data Flow

```
Game State (10 Hz)
  ↓
StateSnapshot
  ↓
Circular Buffer (100 snapshots, 10 sec)
  ↓
TriggerManager.check_triggers()
  ↓
condition.evaluate_with_history(state, history)
  ↓
EventEngine → DeviceManager → Haptic Feedback
```

---

**🎉 Enjoy creating sophisticated temporal patterns!**

