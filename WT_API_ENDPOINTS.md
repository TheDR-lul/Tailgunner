# War Thunder Localhost API - Complete Endpoint List

## Base URL
`http://127.0.0.1:8111`

## Discovered Endpoints (from source code)

### **1. `/indicators`** ✅ **ИСПОЛЬЗУЕТСЯ**
**Response:** JSON
**Description:** Vehicle indicators (speed, RPM, fuel, ammo, etc.)
**Usage:** Real-time telemetry for gauges

### **2. `/state`** ✅ **ИСПОЛЬЗУЕТСЯ**
**Response:** JSON
**Description:** Flight/driving state (IAS, TAS, AoA, G-load, control surfaces, engine parameters)
**Usage:** Real-time flight dynamics

### **3. `/mission.json`**
**Response:** JSON
**Description:** Mission objectives (primary, secondary)
**Fields:**
- `objectives[]` - array of mission objectives
  - `status` - "in_progress", "completed", "failed"
  - `primary` - boolean
  - `text` - objective description

### **4. `/map_obj.json`**
**Response:** JSON
**Description:** Map objects (tanks, aircraft, objectives, capture zones)
**Fields:**
- Array of objects with:
  - `type` - "airfield", "respawn_base_fighter", etc.
  - `x`, `y` - position (0.0-1.0)
  - `dx`, `dy` - direction
  - `icon` - "Player", "Airdefence", "Structure", etc.
  - `color` - RGB color
  - `blink` - 0/1/2 (no blink / normal / heavy)

### **5. `/map_info.json`**
**Response:** JSON
**Description:** Map information and grid
**Fields:**
- `map_min` - [x, y] min coordinates
- `map_max` - [x, y] max coordinates
- `grid_steps` - [x, y] grid cell size
- `map_generation` - map version ID

### **6. `/map.img?gen=<generation>`**
**Response:** Image (PNG/JPG)
**Description:** Map image
**Usage:** Visual map representation

### **7. `/gamechat?lastId=<id>`**
**Response:** JSON array
**Description:** Game chat messages
**Fields:**
- `id` - message ID
- `time` - game time in seconds
- `sender` - player name (or null for system)
- `msg` - message text
- `enemy` - boolean (enemy team)
- `mode` - "Team", "All", "Squad", "Private"

### **8. `/hudmsg?lastEvt=<id>&lastDmg=<id>`**
**Response:** JSON
**Description:** HUD messages (events and damage)
**Fields:**
- `events[]` - array of event messages
- `damage[]` - array of damage messages
  - `id` - message ID
  - `msg` - message text
  - `time` - game time

---

## **💡 ПОТЕНЦИАЛЬНЫЕ ПРИМЕНЕНИЯ**

### **1. `/hudmsg` - HUD DAMAGE MESSAGES** 🎯
**ВАЖНО:** Может содержать информацию о хитах!

```json
{
  "damage": [
    {"id": 1, "msg": "Hit! 50 HP", "time": 123},
    {"id": 2, "msg": "Critical hit! Engine damaged", "time": 125}
  ]
}
```

**Применение:**
- Детекция хитов по новым damage messages
- Парсинг текста: "Hit", "Critical", "Penetration", "Ricochet"

### **2. `/gamechat` - СИСТЕМНЫЕ СООБЩЕНИЯ**
**Потенциал:** Kill feed, достижения

```json
[
  {"id": 1, "msg": "Player123 destroyed Enemy456", "sender": null, "time": 100}
]
```

### **3. `/map_obj.json` - ОБЪЕКТЫ НА КАРТЕ**
**Применение:**
- Детекция врагов рядом → vibration
- Детекция угроз (AAA, SAM) → warning
- Расстояние до capture zone

### **4. `/mission.json` - OBJECTIVES**
**Применение:**
- Vibration при выполнении objective
- Warning при провале

---

## **🚀 ЧТО ДЕЛАТЬ ДАЛЬШЕ**

1. **Проверить `/hudmsg`** - может содержать информацию о хитах!
2. Реализовать парсинг damage messages
3. Добавить triggers на:
   - Proximity warnings (враги рядом)
   - Objective completion
   - Kill events

