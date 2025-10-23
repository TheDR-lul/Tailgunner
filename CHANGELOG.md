# Changelog

## [0.2.0] - 2025-10-23 (РАСШИРЕНИЕ СОБЫТИЙ)

### Добавлено

#### 🎯 Система событий расширена до 75+ событий
- **Категоризация:** 10 категорий событий
- **Попадания:** Ricochet, HitCamera
- **Повреждения двигателя:** EngineDamaged, EngineOverheat, OilLeak, WaterLeak
- **Экипаж:** PilotKnockedOut, GunnerKnockedOut, DriverKnockedOut
- **Танк:** TurretJammed, GunBreach, TransmissionDamaged, AmmunitionExploded, FuelTankHit
- **Самолет повреждения:** WingDamaged, TailDamaged, ElevatorDamaged, RudderDamaged, AileronDamaged, GearDamaged, FlapsDamaged

#### ✈️ Аэродинамические события (НОВОЕ!)
- **Overspeed** — превышение максимальной скорости (триггер: IAS > 800)
- **OverG** — критическая G-перегрузка (триггер: |G| > 10)
- **HighAOA** — высокий угол атаки (триггер: AOA > 15°)
- **CriticalAOA** — критический угол атаки (триггер: AOA > 20°)
- **Mach1** — преодоление звукового барьера (триггер: Mach > 0.98)
- **FlatSpin** — плоский штопор
- **CompressorStall** — срыв компрессора

#### ⛽ События топлива и боезапаса (НОВОЕ!)
- **LowFuel** — мало топлива (<10%)
- **CriticalFuel** — критически мало (<5%)
- **OutOfFuel** — закончилось топливо
- **LowAmmo** — мало боезапаса (<20%)
- **OutOfAmmo** — закончился боезапас

#### 🌍 События окружения (НОВОЕ!)
- **LowAltitude** — низкая высота (<100м)
- **CriticalAltitude** — критическая высота (<50м)
- **HighAltitude** — большая высота (>5000м)
- **Touchdown** — касание земли
- **Landed** — приземление
- **Takeoff** — взлет

#### 🎯 Система триггеров (event_triggers.rs) - НОВЫЙ МОДУЛЬ!
- **TriggerCondition** enum — условия для кастомных событий
  - SpeedAbove/Below
  - AltitudeAbove/Below
  - GLoadAbove/Below
  - AOAAbove/Below
  - IASAbove, TASAbove, MachAbove
  - FuelBelow, AmmoBelow
  - EngineDamageAbove, ControlsDamageAbove
  - Логические: And, Or, Not
  
- **TriggerManager** — менеджер триггеров
  - 10 встроенных триггеров
  - Cooldown система (предотвращение спама)
  - Поддержка кастомных триггеров

#### 📊 Расширенные Indicators
- **Базовые:** climb (скороподъемность)
- **Двигатель:** manifold_pressure, mixture, radiator, compressor_stage, magneto
- **Управление:** pitch, roll, yaw, aileron, elevator, rudder, airbrake
- **Аэродинамика:** aoa, slip, g_load, mach, tas, ias
- **Вооружение:** cannon_ready, machine_gun_ready, rockets_ready, bombs_ready, torpedoes_ready
- **Боезапас:** ammo_count, rocket_count, bomb_count
- **Топливо:** fuel, fuel_max, fuel_time
- **Повреждения:** engine_damage, controls_damage, gear_damage, flaps_damage

#### 🎮 Новые профили событий
- **Танк RB:** +3 новых события (PenetrationHit, AmmunitionExploded, TrackBroken)
- **Самолет:** +10 новых событий (Overspeed, OverG, HighAOA, CriticalAOA, LowFuel, CriticalFuel, Mach1, и др.)

#### 📚 Документация
- **EVENTS_DOCUMENTATION.md** — полная документация всех 75+ событий
  - Таблицы с описаниями
  - Триггеры для каждого события
  - Примеры использования
  - Ссылки на официальную документацию WT API

### Изменено
- EventEngine расширен для поддержки всех новых событий
- HapticEngine интегрирован с TriggerManager
- ProfileManager обновлен с новыми паттернами

### Технические детали
- Интеграция с [War Thunder localhost API docs](https://github.com/lucasvmx/WarThunder-localhost-documentation)
- Подготовка к интеграции с [WarThunder Vehicles API](https://github.com/Sgambe33/WarThunder-Vehicles-API)
- +300 строк кода в event_triggers.rs
- +150 строк в расширенном маппинге событий

---

## [0.1.0] - 2025-10-23 (MVP)

### Добавлено
- Базовая система событий (15 событий)
- ADSR синтезатор паттернов
- Buttplug.io интеграция
- 3 готовых профиля
- React UI
- Полная документация

---

[0.2.0]: https://github.com/yourusername/butt_thunder/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yourusername/butt_thunder/releases/tag/v0.1.0
