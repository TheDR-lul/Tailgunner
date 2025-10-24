# 🚀 Butt Thunder - Roadmap & Improvements

## ✅ **Исправлено:**

### 1. **Парсинг названия техники (КРИТИЧНО)**
- ✅ Добавлено поле `vehicle_name` в `GameState`
- ✅ Извлечение реального названия из API (`"tankModels/sw_cv_90105_tml"` → `"sw_cv_90105_tml"`)
- ✅ Улучшенное определение типа техники (танк/самолет/вертолет/корабль)
- ✅ Отображение в UI и debug console

### 2. **Метод удаления паттернов**
- ✅ Добавлен `remove_trigger()` в `TriggerManager`
- ✅ Интегрирован в Tauri command `remove_pattern`
- ✅ Логирование успеха/ошибок

### 3. **Debug Info**
- ✅ Заполнены `triggers_count` и `patterns_active`
- ✅ Добавлено отображение названия техники в debug console

---

## ✅ **Реализовано в последнем обновлении:**

### **A. Расширенная телеметрия** ✅

#### **1. Вооружение**
- ✅ Добавлен парсинг `rockets_ready`, `bombs_ready`, `torpedoes_ready`
- ✅ `ammo_count` дифференцирован для танков (`first_stage_ammo`) и самолетов (`cannon ammo`)
- ✅ Добавлены `rocket_count`, `bomb_count`

#### **2. Расчет времени до конца топлива**
- ✅ Реализован расчет `fuel_time` (минуты остались)
- ✅ Отслеживание расхода топлива между тиками
- ✅ Формула: `fuel_time = current_fuel / consumption_per_sec / 60`

#### **3. Повреждения из /state**
- ✅ Парсинг массива `state: []` из API
- ✅ Определение `engine_damage`, `controls_damage`, `gear_damage`, `flaps_damage`
- ✅ Обнаружение по ключевым словам в state array

---

### **B. Lovense интеграция** ✅

- ✅ Добавлен HTTP-клиент для Lovense LAN API
- ✅ Реализованы методы `add_lovense_device()`, `remove_lovense_device()`
- ✅ Unified `send_vibration()` для Buttplug + Lovense
- ✅ Tauri commands для управления Lovense устройствами
- ✅ Конвертация интенсивности (0.0-1.0 → 0-20 для Lovense)

**Ссылки:**
- [Lovense LAN API Docs](https://developer.lovense.com/)

---

### **C. Temporal Conditions (Time-Based Triggers)** ✅

#### **State History System**
- ✅ Circular buffer для хранения истории состояний (100 снапшотов, 10 секунд)
- ✅ Автоматическая очистка старых данных
- ✅ Методы анализа: `dropped_by`, `increased_by`, `rate_of_change`, `average`, `min`, `max`

#### **Новые триггерные условия**
```rust
// Speed changes
SpeedDroppedBy { threshold: f32, window_seconds: f32 }     // ▼ Скорость упала
SpeedIncreasedBy { threshold: f32, window_seconds: f32 }   // ▲ Скорость выросла
AccelerationAbove { threshold: f32, window_seconds: f32 }  // ⇧ Ускорение
AccelerationBelow { threshold: f32, window_seconds: f32 }  // ⇩ Замедление

// Altitude changes
AltitudeDroppedBy { threshold: f32, window_seconds: f32 }  // Высота упала
AltitudeGainedBy { threshold: f32, window_seconds: f32 }   // Высота выросла
ClimbRateAbove { threshold: f32, window_seconds: f32 }     // Скорость набора

// G-load changes
GLoadSpiked { threshold: f32, window_seconds: f32 }        // Резкий скачок G
SuddenGChange { threshold: f32, window_seconds: f32 }      // Резкое изменение G

// Averages
AverageSpeedAbove { threshold: f32, window_seconds: f32 }  // Средняя скорость
AverageGLoadAbove { threshold: f32, window_seconds: f32 }  // Средняя G-нагрузка
FuelDepletingFast { threshold: f32, window_seconds: f32 }  // Быстрый расход топлива
```

#### **UI Integration**
- ✅ Добавлены временные операторы в InputNode: `dropped_by`, `increased_by`, `accel_above`, `accel_below`, `avg_above`
- ✅ Поле `window_seconds` (0.1-10.0 секунд) появляется автоматически
- ✅ Парсинг в `ui_patterns.rs` для конвертации в `TriggerCondition`
- ✅ Метод `evaluate_with_history()` для оценки условий

**Примеры использования:**
- **Hard Brake:** `Speed dropped_by 150 over 1.5 sec` → тяжелая вибрация
- **Aggressive Maneuver:** `G-Load increased_by 5.0 over 0.5 sec` → резкий импульс
- **Sustained High Speed:** `Speed avg_above 600 over 5.0 sec` → плавная вибрация

**Документация:** См. `TEMPORAL_CONDITIONS.md`

---

### **D. Dynamic Triggers (WT Vehicles API)** ✅

#### **Integration**
- ✅ API: `https://www.wtvehiclesapi.sgambe.serv00.net`
- ✅ Автоматическое получение характеристик техники при смене
- ✅ Кэширование данных для оффлайн работы
- ✅ `VehicleLimitsManager` для управления лимитами

#### **Автоматически создаваемые триггеры**
```rust
// Overspeed Warning (90% от max_speed)
dynamic_overspeed: "Overspeed Warning (2385+ km/h)"

// High G Warning (95% от max_positive_g)
dynamic_high_g: "High G Warning (11.9+ G)"

// Negative G Warning (95% от max_negative_g)
dynamic_negative_g: "Negative G Warning (-5.7 G)"
```

#### **Поддерживаемые характеристики**
- `max_speed_kmh` - максимальная скорость
- `max_positive_g` / `max_negative_g` - перегрузки
- `max_altitude_meters` - максимальная высота
- `fuel_capacity_kg` - емкость топлива
- `engine_power_hp` - мощность двигателя

**Пример для Rafale C F3:**
- Max Speed: 2650 km/h → warning at 2385 km/h
- Max +G: 12.5G → warning at 11.9G
- Max -G: -6.0G → warning at -5.7G

#### **UI Display**
- ✅ Отдельная секция "All Triggers" в Events tab
- ✅ Badges: "Built-in" (синий) / "Dynamic" (зелёный)
- ✅ Отображение события, cooldown, включено/выключено
- ✅ Grid layout для компактности

**Источник:** [WT Vehicles API](https://github.com/Sgambe33/WarThunder-Vehicles-API)

---

### **C. WT Vehicles API (wt_vehicles_api.rs)**

**Статус:** Модуль создан, но не используется

**Возможности:**
- `get_vehicle(name)` - получить полные данные техники
- `get_max_speed(name)` - макс. скорость
- `get_max_g_limits(name)` - максимальные G для техники

**Предложение:**
- **Динамические триггеры:**
  - Когда игрок меняет технику, получать её характеристики из API
  - Адаптировать пороги триггеров:
    - `Overspeed`: 90% от max_speed техники
    - `OverG`: 95% от max_g техники
  - Отображать в UI: "Max speed: 820 km/h, Current: 750 km/h (91%)"

- **Расширенная телеметрия:**
  - Показывать в GameStatus: класс техники, нацию, BR
  - Подсказки: "Rafale C F3 - French Jet Fighter, BR 13.7"

---

### **D. Event Engine (event_engine.rs)**

```rust
active_events: HashMap<GameEvent, f32>, // Никогда не читается
```

**Предложение:**
- Использовать для отслеживания активных событий и их интенсивности
- Реализовать "смешивание" паттернов:
  - Если одновременно `Hit` + `Overspeed` → наложить вибрации
  - Приоритеты: CriticalHit > Hit > Background
- Добавить метод `get_active_events()` для отображения в UI

---

### **E. Profile Manager (profile_manager.rs)**

**Неиспользуемые методы:**
```rust
add_profile()
remove_profile()
toggle_profile()
```

**Предложение:**
- **Создание пользовательских профилей:**
  - UI для создания нового профиля
  - Выбор типа техники + игровой режим
  - Привязка событий к паттернам
  
- **Импорт/экспорт профилей:**
  - JSON-формат для шаринга
  - Community presets в GitHub
  
- **Автопереключение:**
  - Определять тип техники из `vehicle_name`
  - Автоматически активировать профиль (Tank RB, Aircraft SB, etc.)

---

### **F. Rate Limiter (rate_limiter.rs)**

```rust
time_until_next() // Не используется
```

**Предложение:**
- Отображать в UI: "Next vibration in: 0.3s"
- Индикатор "cooldown" на активных паттернах
- Динамическая регулировка QoS:
  - High intensity mode: 10 cmd/s
  - Normal: 5 cmd/s
  - Battery saving: 2 cmd/s

---

## 🎯 **Новые фичи (идеи):**

### **1. Аудио-визуализация**
- Показывать график текущей интенсивности вибрации в реальном времени
- Sync с War Thunder: визуализация попаданий, выстрелов, урона

### **2. Statistica & Analytics**
- Сколько событий за бой
- Средняя интенсивность
- Самое частое событие
- Экспорт лога боя (CSV/JSON)

### **3. Voice Alerts**
- TTS: "Low fuel", "Critical damage", "Overspeed"
- Опция для стримеров: интеграция с OBS

### **4. Discord Rich Presence**
- Показывать статус: "In battle: Rafale C F3, Speed: 820 km/h"
- Кастомизируемый статус

### **5. Haptic Feedback Curves**
- Библиотека готовых кривых: Exponential, Logarithmic, Sine Wave
- AI-генератор паттернов по описанию: "короткий резкий удар"

### **6. Multiplayer Sync**
- Синхронизация паттернов между игроками в отряде
- Общие события: "Squad member destroyed", "Team victory"

### **7. VR Support**
- Интеграция с VR-гарнитурами War Thunder
- Дополнительные триггеры: Head tracking, Look direction

---

## 🐛 **Потенциальные улучшения:**

### **Performance**
- Кэширование данных с WT API (сейчас 10 запросов/сек)
- Оптимизация проверки триггеров (сейчас O(n) для каждого тика)

### **Error Handling**
- Graceful reconnect к War Thunder при обрыве связи
- Retry logic для устройств (если Intiface отвалился)

### **UI/UX**
- Onboarding tutorial для новых пользователей
- Tooltips на всех элементах
- Dark/Light theme toggle
- Compact mode для второго монитора

### **Testing**
- Модульные тесты для триггеров
- Mock War Thunder API для тестирования без игры
- E2E тесты с Playwright

---

## 📊 **Приоритеты (рекомендации):**

### 🔴 **Высокий приоритет:**
1. ✅ Парсинг названия техники (ГОТОВО)
2. Добавить парсинг боезапаса и повреждений
3. Интеграция с WT Vehicles API (динамические триггеры)
4. Lovense support (много пользователей просят)

### 🟡 **Средний приоритет:**
5. Пользовательские профили (UI для создания/редактирования)
6. Аудио-визуализация вибраций
7. Импорт/экспорт паттернов в community

### 🟢 **Низкий приоритет:**
8. Discord Rich Presence
9. Analytics & Statistics
10. VR support

---

## 💡 **Выводы:**

### **Что работает отлично:**
- ✅ EAC-Safe интеграция
- ✅ Node-based pattern editor
- ✅ Multi-language support
- ✅ Buttplug.io интеграция

### **Что нужно доделать:**
- 🔧 Расширенная телеметрия (боезапас, повреждения, fuel time)
- 🔧 Lovense support
- 🔧 Динамические триггеры на основе характеристик техники

### **Что можно добавить в будущем:**
- 🚀 Community patterns library
- 🚀 VR & Voice integration
- 🚀 Multiplayer sync

