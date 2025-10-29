# 📋 Полная документация событий Butt Thunder

## На основе официальной War Thunder localhost API документации

Источники:
- [War Thunder localhost API docs](https://github.com/lucasvmx/WarThunder-localhost-documentation)
- [WarThunder Vehicles API](https://github.com/Sgambe33/WarThunder-Vehicles-API)

---

## 🎯 Категории событий (75+ событий)

### 1. 💥 ПОПАДАНИЯ И УРОН

| Событие | Описание | Триггер |
|---------|----------|---------|
| `Hit` | Обычное попадание | state: "hit" |
| `CriticalHit` | Критическое попадание | state: "critical hit" |
| `PenetrationHit` | Пробитие брони | state: "penetration" |
| `Ricochet` | Рикошет | state: "ricochet" |
| `HitCamera` | Попадание в камеру | state: "hit camera" |

---

### 2. 🔧 ПОВРЕЖДЕНИЯ ТЕХНИКИ

#### Двигатель
| Событие | Описание | Триггер |
|---------|----------|---------|
| `EngineDestroyed` | Двигатель уничтожен | state: "engine destroyed" |
| `EngineDamaged` | Двигатель поврежден | state: "engine damaged" |
| `EngineOverheat` | Перегрев двигателя | `indicators.engine_temp > 250°` |
| `EngineFire` | Пожар двигателя | state: "engine fire" |
| `OilLeak` | Утечка масла | state: "oil leak" |
| `WaterLeak` | Утечка охлаждающей жидкости | state: "water leak" |

#### Экипаж
| Событие | Описание | Триггер |
|---------|----------|---------|
| `PilotKnockedOut` | Пилот выведен из строя | state: "pilot knocked" |
| `GunnerKnockedOut` | Наводчик выведен из строя | state: "gunner knocked" |
| `DriverKnockedOut` | Водитель выведен из строя | state: "driver knocked" |
| `CrewKnocked` | Любой член экипажа | state: "crew knocked" |

#### Танк
| Событие | Описание | Триггер |
|---------|----------|---------|
| `TrackBroken` | Гусеница сломана | state: "track broken" |
| `TurretJammed` | Башня заклинила | state: "turret jammed" |
| `GunBreach` | Затвор орудия поврежден | state: "gun breach" |
| `TransmissionDamaged` | Трансмиссия повреждена | state: "transmission" |
| `AmmunitionExploded` | Взрыв боекомплекта | state: "ammunition exploded" |
| `FuelTankHit` | Попадание в топливный бак | state: "fuel tank" |

#### Самолет
| Событие | Описание | Триггер |
|---------|----------|---------|
| `WingDamaged` | Крыло повреждено | state: "wing damaged" |
| `TailDamaged` | Хвост поврежден | state: "tail damaged" |
| `ElevatorDamaged` | Руль высоты поврежден | state: "elevator damaged" |
| `RudderDamaged` | Руль направления поврежден | state: "rudder damaged" |
| `AileronDamaged` | Элерон поврежден | state: "aileron damaged" |
| `GearDamaged` | Шасси повреждено | state: "gear damaged" |
| `FlapsDamaged` | Закрылки повреждены | state: "flaps damaged" |

---

### 3. ✈️ СОСТОЯНИЯ САМОЛЕТА

#### Аэродинамика
| Событие | Описание | Триггер |
|---------|----------|---------|
| `Stall` | Сваливание | state: "stall" |
| `Spin` | Штопор | state: "spin" |
| `FlatSpin` | Плоский штопор | state: "flat spin" |
| `Overspeed` | **Превышение макс скорости** | `indicators.ias > 800 км/ч` |
| `OverG` | **Превышение G-перегрузки** | `abs(indicators.g_load) > 10` |
| `HighAOA` | **Высокий угол атаки** | `indicators.aoa > 15°` |
| `CriticalAOA` | **Критический угол атаки** | `indicators.aoa > 20°` |
| `Mach1` | **Преодоление звукового барьера** | `indicators.mach > 0.98` |
| `CompressorStall` | Срыв компрессора | state: "compressor stall" |

#### Управление
| Событие | Описание | Триггер |
|---------|----------|---------|
| `GearUp` | Шасси убрано | state: "gear up" |
| `GearDown` | Шасси выпущено | state: "gear down" |
| `GearStuck` | Шасси заклинило | state: "gear stuck" |
| `FlapsExtended` | Закрылки выпущены | state: "flaps extended" |
| `FlapsRetracted` | Закрылки убраны | state: "flaps retracted" |
| `AirbrakeDeployed` | Воздушные тормоза | state: "airbrake" |
| `ParachuteDeployed` | Парашют выпущен | state: "parachute" |

---

### 4. ⚔️ БОЕВЫЕ ДЕЙСТВИЯ

#### Стрельба
| Событие | Описание | Триггер |
|---------|----------|---------|
| `Shooting` | Стрельба (общее) | state: "shooting" |
| `CannonFiring` | Стрельба из пушки | state: "cannon firing" |
| `MachineGunFiring` | Пулеметная очередь | state: "machine gun" |
| `RocketLaunched` | Запуск ракеты | state: "rocket launched" |
| `BombDropped` | Сброс бомбы | state: "bomb dropped" |
| `TorpedoDropped` | Сброс торпеды | state: "torpedo" |

#### Попадания игрока
| Событие | Описание | Триггер |
|---------|----------|---------|
| `TargetHit` | Попадание в цель | state: "target hit" |
| `TargetDestroyed` | Цель уничтожена | state: "target destroyed" |
| `TargetCritical` | Критическое попадание | state: "target critical" |
| `AircraftDestroyed` | Самолет сбит | state: "aircraft destroyed" |
| `TankDestroyed` | Танк уничтожен | state: "tank destroyed" |

---

### 5. ⛽ ТОПЛИВО И БОЕЗАПАС

| Событие | Описание | Триггер |
|---------|----------|---------|
| `LowFuel` | Мало топлива | `fuel / fuel_max < 10%` |
| `CriticalFuel` | Критически мало топлива | `fuel / fuel_max < 5%` |
| `OutOfFuel` | Топливо закончилось | state: "out of fuel" |
| `LowAmmo` | Мало боезапаса | `ammo < 20%` |
| `OutOfAmmo` | Боезапас закончился | state: "out of ammo" |

---

### 6. 🌍 ОКРУЖЕНИЕ И ПОЛЕТ

| Событие | Описание | Триггер |
|---------|----------|---------|
| `LowAltitude` | Низкая высота | `altitude < 100м && speed > 200` |
| `CriticalAltitude` | Критическая высота | `altitude < 50м` |
| `HighAltitude` | Большая высота | `altitude > 5000м` |
| `Touchdown` | Касание земли | state: "touchdown" |
| `Landed` | Приземлился | state: "landed" |
| `Takeoff` | Взлет | state: "takeoff" |

---

### 7. 🔧 СИСТЕМЫ И ЭКИПАЖ

| Событие | Описание | Триггер |
|---------|----------|---------|
| `FireExtinguished` | Пожар потушен | state: "fire extinguished" |
| `RepairCompleted` | Ремонт завершен | state: "repair completed" |
| `CrewReplenished` | Экипаж пополнен | state: "crew replenished" |
| `AutopilotEngaged` | Автопилот включен | state: "autopilot engaged" |
| `AutopilotDisengaged` | Автопилот выключен | state: "autopilot disengaged" |
| `TrimAdjusted` | Триммер настроен | state: "trim" |

---

### 8. 🎯 МИССИЯ

| Событие | Описание | Триггер |
|---------|----------|---------|
| `MissionStarted` | Миссия началась | state: "mission started" |
| `MissionSuccess` | Миссия успешна | state: "mission success" |
| `MissionFailed` | Миссия провалена | state: "mission failed" |
| `MissionObjectiveCompleted` | Задача выполнена | state: "objective completed" |
| `Respawn` | Респавн | state: "respawn" |

---

### 9. 👥 MULTIPLAYER

| Событие | Описание | Триггер |
|---------|----------|---------|
| `TeamKill` | Убийство союзника | state: "team kill" |
| `Assist` | Ассист | state: "assist" |
| `BaseCapture` | Захват базы | state: "base capture" |

---

### 10. ⚙️ НЕПРЕРЫВНЫЕ СОСТОЯНИЯ

| Событие | Описание | Триггер |
|---------|----------|---------|
| `EngineRunning` | Двигатель работает | `engine_rpm > 100` |

---

## 🎛️ Кастомные триггеры

### Что такое триггеры?

**Триггеры** — это события, которые срабатывают при выполнении определенных условий из индикаторов игры.

### Доступные условия:

```rust
// Скорость
SpeedAbove(f32)         // >X км/ч
IASAbove(f32)           // IAS >X
TASAbove(f32)           // TAS >X
MachAbove(f32)          // Mach >X

// Высота
AltitudeAbove(f32)      // >X м
AltitudeBelow(f32)      // <X м

// G-перегрузки
GLoadAbove(f32)         // >X g
GLoadBelow(f32)         // <X g

// Угол атаки
AOAAbove(f32)           // >X градусов
AOABelow(f32)           // <X градусов

// Температура
TempAbove(f32)          // >X градусов

// Топливо
FuelBelow(f32)          // <X%
FuelTimeBelow(f32)      // <X минут

// Боезапас
AmmoBelow(f32)          // <X%

// Повреждения
EngineDamageAbove(f32)  // >X (0.0-1.0)
ControlsDamageAbove(f32)

// Логические
And(A, B)               // A И B
Or(A, B)                // A ИЛИ B
Not(A)                  // НЕ A
```

### Примеры триггеров:

```rust
// Превышение 800 км/ч
EventTrigger {
    name: "Overspeed 800",
    condition: IASAbove(800.0),
    event: Overspeed,
}

// G-перегрузка >10g
EventTrigger {
    name: "Over G",
    condition: Or(
        GLoadAbove(10.0),
        GLoadBelow(-5.0)
    ),
    event: OverG,
}

// Низкая высота на скорости
EventTrigger {
    name: "Low Altitude",
    condition: And(
        AltitudeBelow(100.0),
        SpeedAbove(200.0)
    ),
    event: LowAltitude,
}
```

---

## 📊 War Thunder API эндпоинты

### localhost:8111/state
Основное состояние игры

```json
{
  "valid": true,
  "type": "aircraft",
  "state": ["hit", "critical", "fire"],
  ...
}
```

### localhost:8111/indicators
Индикаторы приборов

```json
{
  "speed": 450.0,
  "altitude": 1500.0,
  "rpm": 2500.0,
  "ias": 450.0,
  "tas": 480.0,
  "mach": 0.65,
  "aoa": 8.5,
  "g_load": 3.2,
  ...
}
```

### localhost:8111/hudmsg
Сообщения HUD (попадания, убийства)

### localhost:8111/gamechat
Чат игры

### localhost:8111/map_info.json
Информация о карте

### localhost:8111/map_obj.json
Объекты на карте

### localhost:8111/mission.json
Информация о миссии

---

## 🎨 Использование в профилях

```rust
// Пример профиля с новыми событиями
let mut aircraft_profile = HashMap::new();

// Попадания
aircraft_profile.insert(GameEvent::Hit, simple_hit_pattern);
aircraft_profile.insert(GameEvent::CriticalHit, critical_hit_pattern);

// Аэродинамика (НОВОЕ!)
aircraft_profile.insert(GameEvent::Overspeed, critical_pattern);
aircraft_profile.insert(GameEvent::OverG, critical_pattern);
aircraft_profile.insert(GameEvent::Stall, fire_pattern);
aircraft_profile.insert(GameEvent::Mach1, epic_pattern);

// Топливо (НОВОЕ!)
aircraft_profile.insert(GameEvent::LowFuel, warning_pattern);
aircraft_profile.insert(GameEvent::CriticalFuel, critical_pattern);

// Высота (НОВОЕ!)
aircraft_profile.insert(GameEvent::LowAltitude, warning_pattern);
```

---

## 🔮 Будущие улучшения

- [ ] Интеграция с **WarThunder Vehicles API** для данных о максимальных характеристиках техники
- [ ] Динамические триггеры на основе характеристик конкретного самолета
- [ ] Поддержка всех эндпоинтов WT API (hudmsg, gamechat, map_obj)
- [ ] UI конструктор кастомных триггеров
- [ ] Импорт/экспорт триггеров

---

**Источники:**
- [War Thunder localhost API docs](https://github.com/lucasvmx/WarThunder-localhost-documentation) ✅
- [WarThunder Vehicles API](https://github.com/Sgambe33/WarThunder-Vehicles-API) 🔄

**Версия:** 0.2.0  
**Дата:** 23 октября 2025

