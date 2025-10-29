# Аудит событий War Thunder API

## ✅ УЖЕ РЕАЛИЗОВАНО И РАБОТАЕТ

### HUD События (из `/hudmsg/damage`):
- ✅ **Kill (destroyed)** - `HudEvent::Kill` → `GameEvent::TargetDestroyed`
- ✅ **Set Afire** - `HudEvent::SetAfire` → `GameEvent::TargetSetOnFire`
- ✅ **Severely Damaged** - `HudEvent::SeverelyDamaged` → `GameEvent::TargetSeverelyDamaged`
- ✅ **Shot Down** - `HudEvent::ShotDown` → `GameEvent::AircraftDestroyed`
- ✅ **Crashed** - `HudEvent::Crashed` → `GameEvent::Crashed`
- ✅ **Engine Overheated** - `HudEvent::EngineOverheated` → `GameEvent::EngineOverheat`
- ✅ **Oil Overheated** - `HudEvent::OilOverheated` → `GameEvent::OilOverheated`
- ✅ **Achievement** - `HudEvent::Achievement` → `GameEvent::Achievement`
- ✅ **Taking Damage** - `HudEvent::TakeDamage` → `GameEvent::Hit`
- ✅ **Chat Messages** - `HudEvent::ChatMessage` → `GameEvent::ChatMessage` / `TeamChatMessage` / etc.

## ❌ НЕ РЕАЛИЗОВАНО (найдено в дампах):

### Критические события:
1. ❌ **"critically damaged"** - критический урон
   - Пример: `"Bobby_Kotick_UA (F-2A) critically damaged ebbestahl (JA37DI)"`
   - Нужно: `HudEvent::CriticallyDamaged(String)` → `GameEvent::CriticalHit`

### Достижения в бою:
2. ❌ **"has delivered the first strike!"** - первый удар
   - Пример: `"抓一只性感母蛙 (F/A-18A) has delivered the first strike!"`
   - Нужно: парсить как специальное achievement → `GameEvent::FirstStrike`

3. ❌ **"Fighter Rescuer"** achievement
   - Пример: `"⋇Jace36647 (F-4S) has achieved \"Fighter Rescuer\""`
   - Нужно: парсить как `GameEvent::ShipRescuer` или специальный `Achievement("Rescuer")`

4. ❌ **"Avenger"** achievement
   - Пример: `"сухой понос (F-4J) has achieved \"Avenger\""`
   - Нужно: новый `GameEvent::Avenger`?

5. ❌ **"Double strike!"** achievement
   - Пример: `"сухой понос (F-4J) has achieved \"Double strike!\""`
   - Нужно: новый `GameEvent::DoubleStrike`?

6. ❌ **"Rank does not matter"** achievement
   - Пример: `"⋇Jace36647 (F-4S) has achieved \"Rank does not matter\""`

### Чат из `/gamechat`:
7. ❌ **Детальные chat сообщения из `/gamechat`**
   - Сейчас: парсятся только из `/hudmsg/damage`
   - Проблема: `/hudmsg` не содержит `sender`, `enemy`, `mode`
   - Нужно: добавить `get_chat_messages()` для парсинга `/gamechat`

### Дополнительные события из GameEvent (не найдены в дампах):
8. ❓ **TargetHit** - игрок попал по врагу (без убийства)
   - Нет в дампах, возможно не логируется в HUD?

9. ❓ **ShipDestroyed**, **TankDestroyed**, **VehicleDestroyed**
   - Возможно нужно определять по типу техники из `destroyed`?

10. ❓ **Shooting** (RPM spike detection)
    - Требует анализа `indicators.rpm` в `event_engine.rs`

11. ❓ **CrewKnocked** (экипаж убит/ранен)
    - Требует отслеживания `indicators.crew_current`

12. ❓ **Mission events** (MissionObjectiveCompleted, MissionFailed, MissionSuccess)
    - Требует парсинга `/mission.json`

13. ❓ **BaseCapture** (захват точки)
    - Требует отслеживания `/map_obj.json`

14. ❓ **TeamKill** (убийство союзника)
    - Нет в дампах, возможно специальное сообщение?

15. ❓ **PlayerDisconnected**
    - Видно в дампах: `"has disconnected from the game."` и `"td! kd?NET_PLAYER_DISCONNECT_FROM_GAME"`
    - Сейчас фильтруется в коде (строка 516-518 wt_telemetry.rs)

## 📊 СТАТИСТИКА:

- **Реализовано:** 10 событий
- **Нужно добавить (критично):** 7 событий (critically damaged, first strike, rescuer, chat parsing)
- **Нужно проверить:** 8 событий (mission, crew, shooting, etc.)

---

## 🎯 ПРИОРИТЕТЫ:

### HIGH (важно для геймплея):
1. ✅ "critically damaged" - часто встречается в боях
2. ✅ Парсинг `/gamechat` - для точных chat триггеров
3. ✅ "first strike" achievement

### MEDIUM (улучшение UX):
4. ⚙️ Fighter Rescuer, Avenger, Double Strike achievements
5. ⚙️ TargetHit (попадания без убийства)
6. ⚙️ Mission events

### LOW (второстепенное):
7. 📝 BaseCapture tracking
8. 📝 TeamKill detection
9. 📝 Shooting detection (RPM spikes)
10. 📝 CrewKnocked tracking

---

## 📝 ПЛАН РЕАЛИЗАЦИИ:

### Этап 1: Критические события (HIGH)
- [ ] Добавить парсинг "critically damaged" в `parse_hud_message()`
- [ ] Добавить `HudEvent::CriticallyDamaged(String)`
- [ ] Обновить match в `haptic_engine.rs`

### Этап 2: Chat парсинг (HIGH)
- [ ] Добавить `get_chat_messages()` в `wt_telemetry.rs`
- [ ] Парсить `/gamechat?lastId=X`
- [ ] Мержить chat events с `hud_events` в `GameState`

### Этап 3: Достижения (MEDIUM)
- [ ] Парсить "first strike", "rescuer", "avenger" в `parse_hud_message()`
- [ ] Добавить соответствующие GameEvent варианты

### Этап 4: UI для тестирования
- [ ] Добавить отправку chat сообщений в эмуляторе
- [ ] Добавить триггер "critically damaged" в UI

---

## 🔍 ДОПОЛНИТЕЛЬНЫЕ НАХОДКИ:

### Формат HUD сообщений:
```
ATTACKER (VEHICLE) ACTION VICTIM (VEHICLE)
"Bobby_Kotick_UA (F-2A) critically damaged ebbestahl (JA37DI)"
"抓一只性感母蛙 (F/A-18A) shot down ⋇SgtSquatch1655 (MiG-29)"
```

### Формат достижений:
```
PLAYER (VEHICLE) has achieved "ACHIEVEMENT_NAME"
"⋇Jace36647 (F-4S) has achieved \"Fighter Rescuer\""
"сухой понос (F-4J) has achieved \"Double strike!\""
```

### Формат disconnects:
```
PLAYER has disconnected from the game.
PLAYERtd! kd?NET_PLAYER_DISCONNECT_FROM_GAME
```


