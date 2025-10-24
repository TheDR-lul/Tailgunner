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

## 🔧 **Заготовки в коде (можно доделать):**

### **A. Расширенная телеметрия (wt_telemetry.rs)**

#### **1. Вооружение**
```rust
// TODO: Парсить из API
rockets_ready: 0,
bombs_ready: 0,
torpedoes_ready: 0,
ammo_count: 0,
rocket_count: 0,
bomb_count: 0,
```

**Предложение:**
- Найти названия полей в War Thunder API для боеприпасов
- Добавить парсинг в `parse_indicators()`
- Создать события: `LowAmmo`, `OutOfAmmo`, `NoRockets`

#### **2. Расчет времени до конца топлива**
```rust
fuel_time: 0.0, // TODO: вычислить из Mfuel / расход
```

**Предложение:**
- Отслеживать изменение `Mfuel` за секунду = расход
- Вычислять: `fuel_time = current_fuel / fuel_consumption_per_sec / 60` (минуты)
- Добавить событие: `FuelRunningOut` (< 2 минуты)

#### **3. Повреждения из /state**
```rust
engine_damage: 0.0, // TODO: из /state
controls_damage: 0.0,
gear_damage: 0.0,
flaps_damage: 0.0,
```

**Предложение:**
- Парсить массив `state: []` из API (например: `["damaged", "engine_damaged", "gear_damaged"]`)
- Вычислять процент повреждений по количеству тегов
- События: `MinorDamage`, `ModerateDamage`, `CriticalDamage`

---

### **B. Lovense интеграция (device_manager.rs)**

```rust
lovense_enabled: false, // Никогда не используется
```

**Статус:** Заготовка для будущей интеграции Lovense API

**Предложение:**
- Добавить HTTP-клиент для Lovense LAN API
- Реализовать `init_lovense()` и `scan_lovense_devices()`
- Добавить UI-переключатель: Buttplug / Lovense / Оба
- Создать единый интерфейс для отправки вибраций на оба типа устройств

**Ссылки:**
- [Lovense LAN API Docs](https://developer.lovense.com/)

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

