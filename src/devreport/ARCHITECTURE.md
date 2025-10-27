# 🏗️ Архитектура Butt Thunder

## Обзор системы

```
┌─────────────────────────────────────────────────────────┐
│                    React Frontend (TypeScript)           │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐ │
│  │  Dashboard  │  │Pattern Editor│  │  Device List    │ │
│  └──────┬──────┘  └──────┬───────┘  └────────┬────────┘ │
│         └────────────────┴──────────────────┬─┘         │
│                   Tauri IPC Bridge          │           │
└─────────────────────────────────────────────┼───────────┘
                                              │
┌─────────────────────────────────────────────┼───────────┐
│                 Rust Backend (Tauri)        │           │
│                                             │           │
│  ┌──────────────────────────────────────────▼─────────┐ │
│  │           Haptic Engine (Координатор)              │ │
│  └──┬───┬────────┬───────────┬────────┬────────┬──────┘ │
│     │   │        │           │        │        │        │
│  ┌──▼───▼──┐  ┌──▼────┐  ┌───▼────┐ ┌─▼────┐ ┌─▼──────┐│
│  │   WT    │  │Pattern│  │Profile │ │Event │ │  Rate  ││
│  │Telemetry│  │Engine │  │Manager │ │Engine│ │Limiter ││
│  └────┬────┘  └───┬───┘  └────────┘ └──┬───┘ └────────┘│
│       │           │                     │                │
│       │           └─────────┬───────────┘                │
│       │                     │                            │
│  ┌────▼─────────────────────▼───────────────────────┐   │
│  │          Device Manager (Buttplug.io)            │   │
│  └─────────────────────────┬─────────────────────────┘  │
└────────────────────────────┼────────────────────────────┘
                             │
                ┌────────────▼────────────┐
                │  Вибро-устройства       │
                │  (Lovense, Kiiroo, etc) │
                └─────────────────────────┘
```

---

## Модули Backend (Rust)

### 1. **WTTelemetryReader** (`wt_telemetry.rs`)
**Цель:** Чтение данных из War Thunder

**Как работает:**
- Опрашивает `http://127.0.0.1:8111/state` каждые 100ms
- Парсит JSON ответ в структуру `GameState`
- Определяет тип техники (танк/самолет/вертолет)
- EAC-Safe: только HTTP запросы, никакой инъекции

**Структура данных:**
```rust
GameState {
    valid: bool,
    type_: VehicleType,  // Tank, Aircraft, Helicopter, Ship
    indicators: Indicators { speed, rpm, temp, ... },
    state: Vec<String>   // ["hit", "fire", "engine_destroyed"]
}
```

---

### 2. **PatternEngine** (`pattern_engine.rs`)
**Цель:** ADSR синтезатор вибрационных паттернов

**Как работает:**
- Определяет паттерны через ADSR огибающие:
  - **Attack** — резкий подъем (удар)
  - **Hold** — удержание интенсивности
  - **Decay** — плавное затухание
  - **Burst** — количество повторений
- Генерирует точки интерполяции для плавной вибрации
- Поддерживает кривые: Linear, EaseIn, EaseOut, EaseInOut

**Пример паттерна:**
```rust
VibrationPattern {
    attack: { 80ms, 0→100%, Linear },
    hold:   { 250ms, 100% },
    decay:  { 400ms, 100%→0%, EaseOut },
    burst:  { 2 повтора, 200ms пауза }
}
```

---

### 3. **EventEngine** (`event_engine.rs`)
**Цель:** Детектор игровых событий

**Как работает:**
- Сравнивает текущее и предыдущее состояние `GameState`
- Детектирует новые события (hit, critical_hit, fire_started)
- Маппит строки WT в enum `GameEvent`
- Поддерживает непрерывные события (engine_running)

**События:**
```rust
GameEvent::Hit                  // Попадание
GameEvent::CriticalHit          // Критическое попадание
GameEvent::FireStarted          // Пожар
GameEvent::EngineRunning        // Двигатель работает
```

---

### 4. **ProfileManager** (`profile_manager.rs`)
**Цель:** Автоматическое переключение профилей

**Как работает:**
- Определяет тип техники из `GameState`
- Автоматически выбирает подходящий профиль
- Профиль = набор маппингов `GameEvent → VibrationPattern`
- Поддерживает готовые пресеты и кастомные профили

**Встроенные профили:**
- **Tank RB** — реалистичный для танков
- **Aircraft** — для самолетов
- **Light Background** — легкая вибрация для всех

---

### 5. **DeviceManager** (`device_manager.rs`)
**Цель:** Управление вибро-устройствами

**Как работает:**
- Использует библиотеку [Buttplug.io](https://buttplug.io)
- Поддерживает Lovense, Kiiroo, We-Vibe, и др.
- Подключается через `ButtplugInProcessClientConnector`
- Отправляет команды вибрации (0.0-1.0 интенсивность)

**API:**
```rust
device_manager.init_buttplug().await;
device_manager.scan_devices().await;
device_manager.send_vibration(0.75).await;
device_manager.stop_all().await;  // Fail-safe
```

---

### 6. **RateLimiter** (`rate_limiter.rs`)
**Цель:** QoS для предотвращения спама команд

**Как работает:**
- Ограничивает частоту до 8 команд/сек
- Группирует команды для стабильности Bluetooth
- Предотвращает перегрузку устройств

**Логика:**
```rust
if rate_limiter.try_send() {
    device.vibrate(intensity).await;
}
```

---

### 7. **HapticEngine** (`haptic_engine.rs`)
**Цель:** Главный координатор системы

**Как работает:**
1. **Опрос WT:** Читает `GameState` каждые 100ms
2. **Автопрофиль:** Выбирает профиль по типу техники
3. **Детектор:** Находит новые события
4. **Паттерн:** Получает `VibrationPattern` из профиля
5. **Отправка:** Выполняет паттерн на устройствах

**Главный цикл:**
```rust
loop {
    let state = telemetry.get_state().await;
    profile_manager.auto_select_profile(&state.type_);
    let events = event_engine.detect_events(&state);
    
    for event in events {
        if let Some(pattern) = profile_manager.get_pattern(&event) {
            execute_pattern(pattern).await;
        }
    }
}
```

---

## Модули Frontend (React + TypeScript)

### 1. **Dashboard** (`components/Dashboard.tsx`)
- Кнопки Start/Stop
- Статус системы (активна/остановлена)
- Тестовая вибрация
- EAC-Safe бейдж

### 2. **PatternEditor** (`components/PatternEditor.tsx`)
- Визуализация паттернов (SVG график)
- Выбор пресетов
- Отображение параметров ADSR
- Экспорт/импорт паттернов

### 3. **DeviceList** (`components/DeviceList.tsx`)
- Инициализация Buttplug
- Список подключенных устройств
- Обновление списка

### 4. **ProfileList** (`components/ProfileList.tsx`)
- Список доступных профилей
- Индикатор активного профиля
- Бейджи типа техники и режима игры

---

## Потоки данных

### Инициализация
```
1. User: Нажимает "Инициализировать"
2. Frontend: invoke('init_devices')
3. Backend: DeviceManager.init_buttplug()
4. Backend: Подключение к Buttplug.io
5. Backend: Сканирование устройств
6. Frontend: Обновление списка устройств
```

### Главный цикл (при запуске)
```
┌─────────────────────────────────────────┐
│ HapticEngine.start()                    │
└────────────┬────────────────────────────┘
             │
             ▼
┌────────────────────────────────────────┐
│ 1. WTTelemetry.get_state()             │
│    → GameState                          │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 2. ProfileManager.auto_select()        │
│    → Выбор профиля по типу техники     │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 3. EventEngine.detect_events()         │
│    → Vec<GameEvent>                    │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 4. ProfileManager.get_pattern()        │
│    → VibrationPattern                  │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 5. PatternEngine.generate_points()     │
│    → Vec<(Duration, Intensity)>        │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 6. RateLimiter.try_send()              │
│    → Проверка QoS                      │
└────────┬───────────────────────────────┘
         │
         ▼
┌────────────────────────────────────────┐
│ 7. DeviceManager.send_vibration()      │
│    → Buttplug → Устройства             │
└────────────────────────────────────────┘
```

---

## Безопасность (EAC-Safe)

### Почему это безопасно:

1. **Только HTTP API** — используется официальный `localhost:8111`
2. **Никакой инъекции** — нет доступа к памяти процесса игры
3. **Нет модификации файлов** — игра остается нетронутой
4. **Публичный API** — Gaijin сами предоставляют эти данные

### Что мы НЕ делаем:

- ❌ Читаем/пишем память процесса
- ❌ Внедряем DLL
- ❌ Модифицируем файлы игры
- ❌ Перехватываем Direct3D/Vulkan
- ❌ Используем читы или эксплойты

### Что мы делаем:

- ✅ Читаем HTTP API `localhost:8111`
- ✅ Парсим JSON ответы
- ✅ Отправляем команды на внешние устройства
- ✅ Работаем полностью изолированно от игры

---

## Технологии

| Компонент | Технология | Версия |
|-----------|-----------|--------|
| Frontend | React | 19.1 |
| Language | TypeScript | 5.8 |
| Build Tool | Vite | 7.0 |
| Desktop | Tauri | 2.0 |
| Backend Language | Rust | 1.70+ |
| HTTP Client | Reqwest | 0.12 |
| Async Runtime | Tokio | 1.48 |
| Haptics | Buttplug.io | 9.0 |
| Icons | Lucide React | 0.462 |

---

## Производительность

### Таймеры

- **WT Polling:** 100ms (10 Hz)
- **UI Status Update:** 2000ms (0.5 Hz)
- **Rate Limit:** 125ms (8 Hz max)
- **Pattern Sampling:** 50ms (20 Hz)

### Память

- **Rust Backend:** ~10-15 MB
- **React Frontend:** ~30-50 MB
- **Tauri Overhead:** ~20-30 MB
- **Общий footprint:** ~60-95 MB

### CPU

- **Idle:** <1%
- **Активный бой:** 2-5%
- **Пики (события):** 8-12%

---

## Расширяемость

### Добавление нового события

1. Добавить вариант в `GameEvent` enum
2. Добавить маппинг в `EventEngine::map_wt_state_to_event()`
3. Создать пресет паттерн
4. Добавить в профиль

### Добавление нового устройства

- Buttplug.io поддерживает [90+ устройств](https://iostindex.com)
- Lovense поддержка через LAN API (TODO)
- Новые протоколы через Buttplug плагины

### Добавление нового профиля

```rust
let mut custom_profile_mappings = HashMap::new();
custom_profile_mappings.insert(
    GameEvent::Hit,
    VibrationPattern::preset_simple_hit()
);

profile_manager.add_profile(Profile {
    id: "custom".to_string(),
    name: "Custom Profile".to_string(),
    vehicle_type: VehicleType::Tank,
    game_mode: GameMode::Any,
    event_mappings: custom_profile_mappings,
    enabled: true,
});
```

---

## Отладка

### Логи

```bash
# Запуск с логами
RUST_LOG=debug npm run tauri dev
```

### Уровни логов:
- `info` — общая информация
- `warn` — предупреждения
- `error` — ошибки
- `debug` — детальная отладка

### Примеры логов:
```
[INFO] Buttplug client connected
[INFO] Scanning for devices...
[INFO] Auto-selected profile: Танк RB - Иммерсивный
[INFO] Haptic engine started
[WARN] Failed to send vibration to Lovense: Timeout
```

---

## Зависимости

### Rust (Cargo.toml)
```toml
tauri = "2"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
buttplug = "9.0"
anyhow = "1"
serde = { version = "1", features = ["derive"] }
```

### TypeScript (package.json)
```json
{
  "react": "^19.1.0",
  "@tauri-apps/api": "^2",
  "lucide-react": "^0.462.0",
  "clsx": "^2.1.1"
}
```

---

## Лицензии

- **Butt Thunder:** MIT
- **Buttplug.io:** BSD-3-Clause
- **React:** MIT
- **Tauri:** MIT/Apache-2.0

---

**Сделано с ❤️ для War Thunder комьюнити**

