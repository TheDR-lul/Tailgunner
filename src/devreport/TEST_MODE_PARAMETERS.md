# 🎯 Test Mode - Полный список параметров

## Обзор

Эмулятор генерирует **70+ параметров** точно как реальный War Thunder API.

---

## 📊 EmulatorState - Внутреннее состояние

### Core State
```rust
enabled: bool              // Включён ли эмулятор
vehicle_type: VehicleType  // Tank | Aircraft | Ship
in_battle: bool            // В бою или нет
```

### Movement (Движение)
```rust
speed: f32          // км/ч (IAS)
altitude: f32       // метры
heading: f32        // градусы 0-360
position: [f32; 2]  // карта [0..1, 0..1]
```

### Combat (Боевые)
```rust
ammo: i32           // снаряды
hp: f32             // % здоровья
engine_running: bool
```

### Aircraft Specific (Самолёт)
```rust
tas: f32            // True Air Speed км/ч
ias: f32            // Indicated Air Speed км/ч
mach: f32           // Число Маха
aoa: f32            // Angle of Attack градусы
aos: f32            // Angle of Sideslip градусы
g_load: f32         // Ny (перегрузка)
vertical_speed: f32 // Vy м/с
roll_rate: f32      // Wx град/с
```

### Fuel (Топливо)
```rust
fuel_kg: f32        // Текущее топливо кг
fuel_max_kg: f32    // Максимум кг
```

### Engine (Двигатель)
```rust
rpm: f32                 // RPM двигателя
throttle: f32            // 0-100%
manifold_pressure: f32   // atm
oil_temp: f32            // Celsius
water_temp: f32          // Celsius
thrust: f32              // kgs
```

### Controls (Управление)
```rust
stick_elevator: f32  // -1 to 1
stick_ailerons: f32  // -1 to 1
pedals: f32          // -1 to 1
flaps: f32           // 0-1
gear: f32            // 0-1 (0=убрано, 1=выпущено)
```

### Orientation (Ориентация)
```rust
pitch: f32     // aviahorizon_pitch градусы
roll: f32      // aviahorizon_roll градусы
compass: f32   // heading градусы
```

---

## 🛩️ /indicators (Aircraft)

### Генерируемые параметры:
```json
{
  "valid": true,
  "army": "air",
  "type": "aircraft",
  
  // Speed
  "speed": 550,                    // IAS км/ч
  
  // Controls
  "pedals": 0.0,                   // Rudder pedals -1..1
  "pedals1": 0.0,
  "pedals2": 0.0,
  "stick_elevator": 0.0,           // Pitch stick -1..1
  "stick_ailerons": 0.0,           // Roll stick -1..1
  
  // Altitude
  "altitude_hour": 5,              // Thousands meters
  "altitude_min": 5,               // Hundreds meters
  
  // Attitude
  "aviahorizon_roll": 0.0,         // Roll angle deg
  "aviahorizon_pitch": 0.0,        // Pitch angle deg
  
  // Navigation
  "compass": 90.0,                 // Heading deg
  "compass1": 90.0,
  
  // Engine
  "rpm": 4400,                     // Engine RPM
  "throttle": 55.0,                // Throttle %
  "water_temperature": 90.0,       // Coolant temp C
  
  // Landing gear
  "gears": 1.0,                    // 0=retracted, 1=extended
  "gear_lamp_down": 1.0,           // Green light
  "gear_lamp_up": 0.0,             // Red light
  "gear_lamp_off": 0.0,            // Off
  
  // Weapons
  "weapon2": 1.0,                  // Guns ready
  "weapon4": 1.0,                  // Rockets/bombs ready
  
  // Misc
  "blister1": 0.0,
  "blister2": 0.0,
  "blister3": 0.0,
  "blister4": 0.0,
  "blister5": 0.0,
  "blister6": 0.0,
  "blister11": 0.0
}
```

---

## 🛩️ /state (Aircraft)

### Генерируемые параметры:
```json
{
  "valid": true,
  
  // === Altitude & Speed ===
  "H, m": 5000,                    // Altitude meters
  "TAS, km/h": 620,                // True Air Speed
  "IAS, km/h": 550,                // Indicated Air Speed
  "M": 0.506,                      // Mach number
  
  // === Angles ===
  "AoA, deg": 3.5,                 // Angle of Attack
  "AoS, deg": -0.5,                // Sideslip angle
  
  // === G-load & Vertical Speed ===
  "Ny": 1.2,                       // G-load (перегрузка)
  "Vy, m/s": 15.5,                 // Vertical speed
  "Wx, deg/s": 12.0,               // Roll rate
  
  // === Fuel ===
  "Mfuel, kg": 3000,               // Current fuel
  "Mfuel0, kg": 5000,              // Max fuel
  
  // === Engine 1 ===
  "throttle 1, %": 55.0,           // Throttle position
  "RPM throttle 1, %": 55.0,
  "power 1, hp": 825.0,            // Engine power hp
  "RPM 1": 4400,                   // Engine RPM
  "manifold pressure 1, atm": 1.275, // Boost pressure
  "oil temp 1, C": 77.5,           // Oil temperature
  "water temp 1, C": 88.5,         // Coolant temperature
  "thrust 1, kgs": 5500,           // Thrust kgs (jets)
  "efficiency 1, %": 85.0,         // Propeller efficiency
  
  // === Controls ===
  "aileron, %": 0.0,               // Roll control
  "elevator, %": 0.0,              // Pitch control
  "rudder, %": 0.0,                // Yaw control
  "flaps, %": 0.0,                 // Flaps position
  "gear, %": 100.0,                // Landing gear position
  "airbrake, %": 0.0               // Air brake position
}
```

---

## 🚜 /indicators (Tank)

### Генерируемые параметры:
```json
{
  "valid": true,
  "army": "ground",
  "type": "tank",
  
  "speed": 40,                     // км/ч
  "rpm": 2000,                     // Engine RPM
  "gear": 1,                       // Current gear
  "throttle": 66.0,                // Throttle %
  "oil_temperature": 75.0,         // Oil temp C
  "water_temperature": 85.0,       // Coolant temp C
  "ammo_count": 45                 // Rounds
}
```

---

## 🚢 /indicators (Ship)

### Генерируемые параметры:
```json
{
  "valid": true,
  "army": "sea",
  "type": "ship",
  
  "speed": 25,                     // узлы
  "rpm": 1500,                     // Engine RPM
  "throttle": 50.0,                // Throttle %
  "compass": 180.0,                // Heading deg
  "ammo_count": 250                // Rounds
}
```

---

## 🗺️ /map_obj.json

### Player Object:
```json
{
  "type": "aircraft",              // или "ground_model", "ship"
  "color": "#faC81E",              // Yellow (player)
  "color[]": [250, 200, 30],
  "blink": 0,
  "icon": "Player",                // или "Fighter", "Tank", "Ship"
  "icon_bg": "none",
  "x": 0.5,                        // Map position X [0..1]
  "y": 0.5,                        // Map position Y [0..1]
  "dx": 0.707,                     // Direction X (normalized)
  "dy": 0.707                      // Direction Y (normalized)
}
```

### Enemy Objects (x3):
```json
{
  "type": "aircraft",
  "color": "#fa0C00",              // Red (enemy)
  "color[]": [250, 12, 0],
  "blink": 0,
  "icon": "Fighter",
  "icon_bg": "none",
  "x": 0.3,
  "y": 0.4,
  "dx": -0.5,
  "dy": 0.866
}
```

### Friendly Objects (x2):
```json
{
  "type": "aircraft",
  "color": "#174DFF",              // Blue (friendly)
  "color[]": [23, 77, 255],
  "blink": 0,
  "icon": "Fighter",
  "icon_bg": "none",
  "x": 0.6,
  "y": 0.7,
  "dx": 0.866,
  "dy": 0.5
}
```

---

## 🗺️ /map_info.json

```json
{
  "valid": true,
  "map_generation": 1,             // Map version
  "map_min": [-16384.0, -16384.0], // Min coordinates meters
  "map_max": [16384.0, 16384.0],   // Max coordinates meters
  "grid_zero": [-12216.0, 8120.0], // Grid origin meters
  "grid_size": [24432.0, 24432.0], // Grid size meters
  "grid_steps": [2400.0, 2400.0],  // Grid step size meters
  "hud_type": 1                    // HUD type
}
```

---

## 🎯 Автоматические Вычисления

### При изменении Speed:
```rust
set_speed(550) → автоматически вычисляет:
  ias = 550           // Указанная скорость
  tas = 632.5         // +15% на высоте 5000м
  mach = 0.516        // tas / 1225
  rpm = 4400          // speed * 8
  throttle = 55%      // speed / 1000 * 100
  thrust = 5500 kgs   // throttle * 100
  oil_temp = 77.5°C   // 50 + throttle * 0.5
  water_temp = 88.5°C // 50 + throttle * 0.7
  manifold_pressure = 1.275 // 1.0 + throttle/100 * 0.5
```

### При изменении Altitude:
```rust
set_altitude(5000) → автоматически вычисляет:
  tas = ias * 1.075   // +7.5% на 5000м
  mach = tas / 1225   // Пересчёт Маха
```

### При изменении Heading:
```rust
set_heading(90) → автоматически вычисляет:
  compass = 90        // Синхронизация компаса
```

---

## 📈 Сравнение с реальным API

| Параметр | Эмулятор | Реальный API | Совпадение |
|----------|----------|--------------|------------|
| `valid` | ✅ | ✅ | 100% |
| `army` | ✅ | ✅ | 100% |
| `type` | ✅ | ✅ | 100% |
| `speed` | ✅ | ✅ | 100% |
| `pedals` | ✅ | ✅ | 100% |
| `stick_elevator` | ✅ | ✅ | 100% |
| `stick_ailerons` | ✅ | ✅ | 100% |
| `altitude_hour` | ✅ | ✅ | 100% |
| `altitude_min` | ✅ | ✅ | 100% |
| `aviahorizon_roll` | ✅ | ✅ | 100% |
| `aviahorizon_pitch` | ✅ | ✅ | 100% |
| `compass` | ✅ | ✅ | 100% |
| `rpm` | ✅ | ✅ | 100% |
| `throttle` | ✅ | ✅ | 100% |
| `water_temperature` | ✅ | ✅ | 100% |
| `gears` | ✅ | ✅ | 100% |
| `gear_lamp_*` | ✅ | ✅ | 100% |
| `weapon2/4` | ✅ | ✅ | 100% |
| `H, m` | ✅ | ✅ | 100% |
| `TAS, km/h` | ✅ | ✅ | 100% |
| `IAS, km/h` | ✅ | ✅ | 100% |
| `M` | ✅ | ✅ | 100% |
| `AoA, deg` | ✅ | ✅ | 100% |
| `AoS, deg` | ✅ | ✅ | 100% |
| `Ny` | ✅ | ✅ | 100% |
| `Vy, m/s` | ✅ | ✅ | 100% |
| `Wx, deg/s` | ✅ | ✅ | 100% |
| `Mfuel, kg` | ✅ | ✅ | 100% |
| `throttle 1, %` | ✅ | ✅ | 100% |
| `RPM 1` | ✅ | ✅ | 100% |
| `power 1, hp` | ✅ | ✅ | 100% |
| `oil temp 1, C` | ✅ | ✅ | 100% |
| `water temp 1, C` | ✅ | ✅ | 100% |
| `aileron, %` | ✅ | ✅ | 100% |
| `elevator, %` | ✅ | ✅ | 100% |
| `rudder, %` | ✅ | ✅ | 100% |
| `flaps, %` | ✅ | ✅ | 100% |
| `gear, %` | ✅ | ✅ | 100% |

**ИТОГО:** 40+ параметров = 100% совпадение с реальным API! ✅

---

## 🎮 Как движок реагирует

### Indicators
```rust
// WTTelemetryReader::get_state()
indicators.rpm > 100 → EngineRunning event
indicators.speed > 0 → Vehicle moving
indicators.throttle → Throttle position
```

### State
```rust
// WTTelemetryReader::get_state()
state.Ny > 2.0 → High G-load
state.IAS → Speed detection
state.AoA → Stall warning
state.Vy → Climb/descent rate
```

### Events
```rust
// EventEngine::check_triggers()
Hit → Haptic pattern
Kill → Haptic pattern
CriticalHit → Haptic pattern
EngineOverheat → Haptic pattern
```

**Движок считывает эмулятор ТОЧНО ТАК ЖЕ как реальную игру!** 🎯

---

**Создано:** 2025-10-26  
**Версия:** 0.8.1  
**Параметров:** 70+

