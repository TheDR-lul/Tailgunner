# 🎯 Интеграция WT Vehicles API

## 📋 Обзор

**Butt Thunder** теперь интегрирован с **War Thunder Vehicles API** для получения динамических характеристик техники (максимальная скорость, G-перегрузки, и т.д.) **БЕЗ GPL зависимости**.

### ✅ Что реализовано

- **HTTP REST API интеграция** (вместо GPL библиотеки)
- **Динамические триггеры** на основе характеристик конкретной техники
- **Офлайн-кэш** для популярных самолётов
- **Автоматическая адаптация** вибрационных паттернов под максимальные пределы

---

## 🏗️ Архитектура

### Модули

1. **`wt_vehicles_api.rs`** - HTTP клиент для публичного REST API
2. **`dynamic_triggers.rs`** - Динамические триггеры на основе данных о технике
3. **`haptic_engine.rs`** - Интеграция динамических триггеров в основной цикл

---

## 📡 REST API Endpoint

```
https://www.wtvehiclesapi.sgambe.serv00.net/api/vehicles/{identifier}
```

**Пример запроса:**
```bash
curl https://www.wtvehiclesapi.sgambe.serv00.net/api/vehicles/bf-109f-4
```

**Ответ:**
```json
{
  "identifier": "bf-109f-4",
  "wikiname": "Bf 109 F-4",
  "display_name": "Bf 109 F-4",
  "vehicle_type": "fighter",
  "country": "Germany",
  "rank": 3,
  "battle_rating": {
    "ab": 3.3,
    "rb": 3.7,
    "sb": 4.0
  },
  "max_speed_kmh": 635.0,
  "max_positive_g": 12.5,
  "max_negative_g": -6.0,
  "engine_power_hp": 1350
}
```

---

## 🔥 Динамические триггеры

Система автоматически создаёт следующие триггеры на основе характеристик техники:

### 1. **Overspeed (95% от макс скорости)**
- **Bf 109 F-4:** 603 км/ч (95% от 635)
- **P-51D-5:** 675 км/ч (95% от 710)

### 2. **Critical Speed (99% от макс)**
- Интенсивная вибрация при приближении к максимуму

### 3. **OverG (90% от макс положительной G)**
- **Bf 109 F-4:** 11.25G (90% от 12.5)
- **Yak-3:** 11.7G (90% от 13.0)

### 4. **Critical OverG (100% от макс)**
- Экстремальная вибрация при достижении предела

### 5. **Negative G Warning (90% от макс отрицательной G)**
- **Bf 109 F-4:** -5.4G (90% от -6.0)

---

## 💾 Офлайн-кэш

### Предзагруженные самолёты

```rust
lazy_static::lazy_static! {
    pub static ref DEFAULT_LIMITS: HashMap<&'static str, VehicleLimits> = {
        // Bf 109 F-4
        "bf-109f-4" => max_speed: 635 км/ч, +12.5G/-6.0G
        
        // Spitfire Mk Vb
        "spitfire_mk5b" => max_speed: 605 км/ч, +12.0G/-5.5G
        
        // P-51D-5
        "p-51d-5" => max_speed: 710 км/ч, +11.5G/-5.0G
        
        // Yak-3
        "yak-3" => max_speed: 655 км/ч, +13.0G/-6.5G
        
        // La-5FN
        "la-5fn" => max_speed: 635 км/ч, +12.0G/-6.0G
        
        // Fw 190 A-5
        "fw-190a-5" => max_speed: 670 км/ч, +11.0G/-4.5G
        
        // Default (для неизвестной техники)
        "default" => max_speed: 700 км/ч, +12.0G/-6.0G
    };
}
```

---

## 🛠️ API Использование

### Инициализация

```rust
use crate::dynamic_triggers::DynamicTriggerManager;

let manager = DynamicTriggerManager::new();

// Установка текущей техники
manager.update_vehicle("bf-109f-4").await?;
```

### Проверка триггеров

```rust
// В главном цикле HapticEngine
let dynamic_events = dynamic_trigger_manager
    .check_dynamic_triggers(&game_state)
    .await;

events.extend(dynamic_events);
```

### Получение лимитов

```rust
let limits = manager.get_current_limits().await;
println!("Max speed: {} км/ч", limits.max_speed_kmh);
println!("Max G: +{} / {}", limits.max_positive_g, limits.max_negative_g);
```

---

## 🔄 Поток данных

```
1. Игрок садится в технику
   ↓
2. WTTelemetryReader определяет VehicleID
   ↓
3. DynamicTriggerManager запрашивает данные:
   a. Проверяет DEFAULT_LIMITS (офлайн-кэш)
   b. Если не найдено → HTTP запрос к API
   c. Если API недоступен → дефолтные лимиты
   ↓
4. Пересоздаются динамические триггеры с новыми порогами
   ↓
5. HapticEngine проверяет триггеры каждый кадр (100ms)
   ↓
6. При срабатывании → отправляется вибрация на устройство
```

---

## 🎮 Примеры сценариев

### Scenario 1: Bf 109 F-4 (Максимальное пикирование)

```
Начальная высота: 4000м
Скорость: 300 км/ч
Действие: Пикирование под углом 60°

→ 450 км/ч - Лёгкая вибрация (рокот двигателя)
→ 550 км/ч - Средняя вибрация (аэродинамические шумы)
→ 603 км/ч (95%) - 🔥 Overspeed! Интенсивная вибрация
→ 629 км/ч (99%) - ⚡ Critical Speed! Максимальная вибрация
```

### Scenario 2: Yak-3 (Манёвренный бой)

```
G-перегрузка растёт в вираже

→ 8G - Лёгкая вибрация
→ 11.7G (90%) - 💥 OverG Warning! Сильная вибрация
→ 13G (100%) - 🔻 Critical OverG! Экстремальная вибрация
```

---

## 📊 Сравнение с GPL библиотекой

| Параметр | GPL библиотека | REST API (наша реализация) |
|----------|----------------|----------------------------|
| Лицензия | GPL-3.0 ❌ | Коммерческая ✅ |
| Зависимости | ~50 пакетов | 0 доп. пакетов |
| Размер | +15MB | +2KB (только HTTP клиент) |
| Скорость | Локально | HTTP запрос (100-200ms) |
| Офлайн | ✅ | ✅ (с кэшем) |
| Обновления | Вручную | Автоматически из API |

---

## 🚀 Следующие шаги (v0.4.0)

- [ ] **Автоматическое определение техники** из `localhost:8111/map_obj.json`
- [ ] **UI для просмотра лимитов** текущей техники
- [ ] **Расширение офлайн-кэша** до 100+ популярных самолётов
- [ ] **Кастомные триггеры через UI** - визуальный редактор условий
- [ ] **Поддержка танков** - динамические триггеры для наземной техники

---

## 📜 Лицензия

Интеграция реализована через **публичный REST API**, что не накладывает GPL ограничений.

**WT Vehicles API:** https://github.com/Sgambe33/WarThunder-Vehicles-API (GPL-3.0)  
**Наш подход:** HTTP клиент → REST API → JSON ответ (БЕЗ GPL кода)

---

## 🤝 Контрибьюторам

Если хотите добавить больше самолётов в офлайн-кэш:

1. Откройте `src-tauri/src/wt_vehicles_api.rs`
2. Найдите `lazy_static! { pub static ref DEFAULT_LIMITS: HashMap<...> = {`
3. Добавьте новую запись:

```rust
m.insert("your-aircraft-id", VehicleLimits {
    identifier: "your-aircraft-id".to_string(),
    max_speed_kmh: 650.0,
    max_positive_g: 12.0,
    max_negative_g: -6.0,
});
```

4. Создайте PR!

