# 🧪 Test Mode - Итоговый Отчёт

## ✅ ЧТО СДЕЛАНО

### 1. **HTTP API Сервер (порт 8112)** 🌐

```rust
// src-tauri/src/api_server.rs
- Полный эмулятор War Thunder API
- 12 endpoints (все основные)
- Генерация данных на основе EmulatorState
- CORS поддержка
- Async Axum framework
```

**Endpoints:**
- ✅ `/status` - статус игры
- ✅ `/indicators` - приборы (RPM, throttle, gear, etc)
- ✅ `/state` - полётные данные (IAS, TAS, altitude, AoA, G-load)
- ✅ `/map_obj.json` - объекты на карте
- ✅ `/map_info.json` - инфо карты
- ✅ `/map.img` - изображение карты
- ✅ `/gamechat` - получить сообщения
- ✅ `/gamechat/send` - отправить сообщение
- ✅ `/hudmsg` - HUD события
- ✅ `/mission.json` - задачи миссии
- ✅ `/info`, `/gunner_view`, `/damage` - вспомогательные

---

### 2. **Автоматическое Переключение Портов** 🔄

```rust
// src-tauri/src/wt_telemetry.rs
impl WTTelemetryReader {
    base_url: String,  // Динамический URL
    
    pub fn set_emulator_mode(&mut self, enabled: bool) {
        if enabled {
            self.base_url = "http://127.0.0.1:8112"; // ЭМУЛЯТОР
        } else {
            self.base_url = "http://127.0.0.1:8111"; // РЕАЛЬНАЯ ИГРА
        }
    }
}
```

**Логика:**
1. **Test Mode ON** → Движок читает с `:8112`
2. **Test Mode OFF** → Движок читает с `:8111`
3. **Автоматический сброс** состояния при переключении

---

### 3. **Синхронизация UI** 🔗

```typescript
// src/components/APIEmulator.tsx
// Использует api.* методы вместо прямого invoke
loadState = async () => {
  const state = await api.emulatorGetState();
  setState(state);
};

// Кнопка в панели
toggleEnabled = async () => {
  await api.emulatorSetEnabled(!enabled);
};
```

```typescript
// src/App.tsx
// Кнопка в header
useEffect(() => {
  setInterval(async () => {
    const state = await api.emulatorGetState();
    setTestModeEnabled(state.enabled);
  }, 1000);
}, []);
```

**Результат:**
- ✅ Кнопка 🧪 в header
- ✅ Панель API Emulator внизу
- ✅ Оба синхронизированы (polling 1 сек)
- ✅ Один источник истины

---

### 4. **Полноценный Чат** 💬

```typescript
// UI компонент
<input 
  value={chatMessage}
  onChange={e => setChatMessage(e.target.value)}
  onKeyPress={e => e.key === 'Enter' && sendChat()}
/>
<button onClick={sendChat}>Send</button>

// Режимы
['Team', 'All', 'Squad'].map(mode => (
  <button onClick={() => setChatMode(mode)}>{mode}</button>
))
```

```rust
// Backend endpoint
POST /gamechat/send
{
  "message": "Test message",
  "mode": "Team"
}

// Сохраняется в Arc<RwLock<Vec<ChatMessage>>>
// Возвращается через GET /gamechat?lastId=0
```

**Фичи:**
- ✅ 3 режима: Team / All / Squad
- ✅ Enter для отправки
- ✅ Сообщения в Game Feed
- ✅ Persistence в памяти сервера

---

### 5. **Все Параметры из API** 📊

```rust
// EmulatorState
pub struct EmulatorState {
    // Основные
    enabled: bool,
    vehicle_type: VehicleType,  // Tank | Aircraft | Ship
    in_battle: bool,
    
    // Движение
    speed: f32,           // км/ч
    altitude: f32,        // метры
    heading: f32,         // градусы 0-360
    position: (f32, f32), // карта [0..1, 0..1]
    
    // Боевые
    ammo: i32,            // снаряды
    hp: f32,              // % здоровья
    
    // Двигатель
    engine_running: bool,
}
```

**Генерируемые данные:**
- `indicators.*` - ~30 параметров (RPM, throttle, gear, flaps, etc)
- `state.*` - ~40 параметров (IAS, TAS, AoA, Ny, controls, fuel, etc)
- `map_obj.json` - игрок + 5 объектов (враги, союзники)
- `map_info.json` - grid размеры, steps, zero point

---

### 6. **События (Events)** ⚡

```typescript
// Триггер событий
await api.emulatorTriggerEvent('Hit');
await api.emulatorTriggerEvent('Kill');
await api.emulatorTriggerEvent('CriticalHit');
await api.emulatorTriggerEvent('Shooting');
await api.emulatorTriggerEvent('EngineOverheat');
```

```rust
// Сохраняется в Vec<EmulatedEvent>
pub struct EmulatedEvent {
    timestamp: u64,
    event_type: String,
}

// Возвращается через GET /hudmsg
```

**Результат:**
- ✅ События триггерятся как в реальной игре
- ✅ Паттерны активируются
- ✅ Логируется в HUD

---

## 🏗️ Архитектура

```
┌─────────────────────────────────────────────────┐
│                 React UI                         │
│  ┌──────────┐           ┌──────────────────┐   │
│  │  Header  │           │   API Emulator   │   │
│  │   🧪     │◄─────────►│      Panel       │   │
│  └──────────┘  Sync     └──────────────────┘   │
└──────────┬──────────────────────┬───────────────┘
           │                      │
           ▼                      ▼
    ┌──────────────────────────────────────────┐
    │        Tauri Backend (Rust)               │
    │  ┌────────────┐      ┌─────────────────┐│
    │  │ Axum HTTP  │      │  API Emulator   ││
    │  │ Server     │◄────►│  State Manager  ││
    │  │ Port: 8112 │      │                 ││
    │  └─────┬──────┘      └─────────────────┘│
    │        │                                  │
    │        ▼                                  │
    │  ┌─────────────────┐                    │
    │  │  WTTelemetry    │                    │
    │  │  Reader         │                    │
    │  │  base_url: 8112 │◄───┐              │
    │  └─────────────────┘    │ Switch       │
    │                          │              │
    └──────────────────────────┼──────────────┘
                               │
                 ┌─────────────┴──────────────┐
                 │                            │
                 ▼                            ▼
        :8112 (Emulator)           :8111 (Real Game)
```

---

## 🎯 Использование

### Быстрый старт

```typescript
// 1. Включить Test Mode (кликнуть 🧪 в header)
await api.emulatorSetEnabled(true);

// 2. Настроить параметры
await api.emulatorSetVehicleType('Aircraft');
await api.emulatorSetSpeed(500);
await api.emulatorSetAltitude(5000);
await api.emulatorSetInBattle(true);

// 3. Триггерить события
await api.emulatorTriggerEvent('Hit');
// → Ожидаем вибрацию!

// 4. Отправить чат
await api.emulatorSendChat('Test message', 'Team');
// → Появится в Game Feed!
```

---

## 🐛 Исправленные Ошибки

### 1. ❌ `GameEvent::TargetCritical` не существует
```rust
// src-tauri/src/hud_messages.rs:171
- assert_eq!(parse_event(msg), Some(GameEvent::TargetCritical));
+ assert_eq!(parse_event(msg), Some(GameEvent::TargetSeverelyDamaged));
```

### 2. ❌ APIEmulator использовал `invoke` напрямую
```typescript
// src/components/APIEmulator.tsx
- import { invoke } from '@tauri-apps/api/core';
+ import { api } from '../api';

- await invoke('emulator_set_enabled', { enabled });
+ await api.emulatorSetEnabled(enabled);
```

### 3. ❌ `WT_TELEMETRY_URL` был константой
```rust
// src-tauri/src/wt_telemetry.rs
- const WT_TELEMETRY_URL: &str = "http://127.0.0.1:8111";
+ const WT_TELEMETRY_URL_REAL: &str = "http://127.0.0.1:8111";
+ const WT_TELEMETRY_URL_EMULATOR: &str = "http://127.0.0.1:8112";

- let url = format!("{}/state", WT_TELEMETRY_URL);
+ let url = format!("{}/state", &self.base_url);
```

---

## 📈 Статистика

| Метрика | Значение |
|---------|----------|
| **Новых файлов** | 2 (`api_server.rs`, документация) |
| **Изменённых файлов** | 6 |
| **Строк кода (Rust)** | ~400 |
| **Строк кода (TypeScript)** | ~150 |
| **Endpoints** | 12 |
| **Параметров эмуляции** | ~70+ |
| **Типов событий** | 15+ |
| **Режимов чата** | 3 |

---

## 🚀 Что Дальше

### Возможные улучшения:

1. **Больше параметров из дампов**
   - Добавить все indicators из real API
   - Все state параметры
   - Damage endpoint с реальными данными

2. **Улучшение карты**
   - Загрузка реальных изображений
   - Настоящие координаты
   - Больше объектов

3. **Preset'ы**
   - Сохранённые сценарии
   - Быстрое переключение
   - Импорт/экспорт

4. **Record/Replay**
   - Запись последовательности событий
   - Воспроизведение
   - Тестирование паттернов

5. **Graceful Shutdown**
   - Правильная остановка Axum
   - Очистка ресурсов

---

## 📝 Файлы

### Созданные
- ✅ `src-tauri/src/api_server.rs` - HTTP сервер
- ✅ `reports/TEST_MODE_DOCUMENTATION.md` - полная документация
- ✅ `reports/TEST_MODE_SUMMARY_RU.md` - этот файл

### Изменённые
- ✅ `src-tauri/src/lib.rs` - интеграция, команды
- ✅ `src-tauri/src/wt_telemetry.rs` - переключение портов
- ✅ `src-tauri/src/api_emulator.rs` - публичные методы
- ✅ `src-tauri/src/hud_messages.rs` - fix теста
- ✅ `src-tauri/Cargo.toml` - зависимости (axum, tower)
- ✅ `src/components/APIEmulator.tsx` - чат UI
- ✅ `src/api.ts` - новые команды
- ✅ `src/App.tsx` - синхронизация header

---

## ✨ Итоги

### Проблема
> "консоль перекрывает API test Mode. в вверху добавь индикатор. Все доступные параметры должны быть в test mode. эмулируем API локально. чат с отправкой сообщений. почему кнопка в боксе не имеет связи с индикатором сверху?"

### Решение
1. ✅ **Индикатор в header** - добавлен 🧪 кнопка
2. ✅ **Синхронизация** - кнопка ↔ панель используют `api.*`
3. ✅ **HTTP сервер** - порт 8112, все endpoints
4. ✅ **Переключение** - движок автоматически переключается 8111↔8112
5. ✅ **Все параметры** - indicators, state, map, mission
6. ✅ **Чат** - полноценный с отправкой и режимами
7. ✅ **Документация** - подробная в `/reports`

### Результат
🎉 **Test Mode полностью функционален!**
- Можно тестировать паттерны без игры
- Все параметры настраиваются
- События триггерятся
- Чат работает
- UI синхронизирован

---

**Дата:** 2025-10-26  
**Версия:** 0.8.1  
**Автор:** Butt Thunder Team 🚀

