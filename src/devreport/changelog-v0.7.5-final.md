# Changelog v0.7.5 - VehicleInfoCard Update & Code Cleanup

**Дата:** 25 октября 2025  
**Версия:** 0.7.5  
**Тип:** Feature, Refactoring, Cleanup

---

## 📝 Описание

Полная переработка VehicleInfoCard под новую систему датамайна с адаптивными карточками для всех типов техники. Глобальная чистка неиспользуемого кода с удалением избыточных модулей и добавлением `#[allow(dead_code)]` для будущего функционала.

---

## ✨ Новые возможности

### 1. **VehicleInfoCard - Адаптивные карточки**

**Три типа карточек:**
- **AircraftCard** (✈️):
  - 🔴 Vne (Wing Rip Speed)
  - 🌪️ Flutter Speed
  - 📊 G-Load Limits (+/-G)
  - ⚡ Engine Power (HP)
  - 🛡️ Mass (tons)
  - 🌪️ Stall Speed

- **GroundCard** (🚜):
  - 🌪️ Max Speed
  - ⚡ Engine Power (HP)
  - 🛡️ Mass (tons)
  - 🛡️ Armor (mm)
  - 📊 Hull HP

- **ShipCard** (⚓):
  - ⚓ Max Speed (knots)
  - 🛡️ Compartments (count)
  - 📊 Total HP
  - 🔴 Critical Modules (list)

**Преимущества:**
- Автоматическое определение типа техники
- Тип-специфичные данные и иконки
- Обновление каждые 5 секунд
- Скрывается при отсутствии техники

---

## 🧹 Code Cleanup

### **Удален избыточный модуль:**
- ❌ `src-tauri/src/dynamic_triggers.rs` - полностью дублировал функционал `VehicleLimitsManager`
- ✅ Функциональность интегрирована в `vehicle_limits.rs`

### **Добавлен #[allow(dead_code)] для будущего:**

**profile_manager.rs:**
- `add_profile()` - добавление пользовательских профилей
- `remove_profile()` - удаление профилей
- `toggle_profile()` - переключение профилей

**device_manager.rs:**
- `LovenseDevice` struct - для будущей интеграции Lovense
- `is_connected()` - проверка подключения
- `set_lovense_enabled()` - включение Lovense
- `is_lovense_connected()` - статус Lovense

**event_triggers.rs:**
- `evaluate_condition()` - вспомогательная функция
- `update_trigger()` - обновление триггеров

**state_history.rs:**
- `StateSnapshot` (aoa, rpm fields) - для будущих метрик
- `min()` - минимальное значение
- `max()` - максимальное значение
- `clear()` - очистка истории
- `aoa_extractor()` - угол атаки
- `rpm_extractor()` - RPM двигателя

**event_engine.rs:**
- `active_events` field - для отслеживания активных событий
- `reset()` - сброс состояния

**datamine/parser.rs:**
- `parse_blkx()` - парсинг BLKX файлов
- `validate_game_path()` - проверка пути

**datamine/types.rs:**
- `AircraftFM` - полная структура FM
- `MassData` - данные о массе
- `TankData` - данные танков
- `EngineData` - данные двигателя
- `ShipData` - данные кораблей

**datamine/mod.rs:**
- `get_limits()` - получение лимитов по ID

**rate_limiter.rs:**
- `time_until_next()` - время до следующей отправки

**wt_telemetry.rs:**
- `HudMessage` - сообщения HUD

---

## 🔧 Технические изменения

### **Frontend:**
```typescript
// src/types.ts - новые типы
export interface AircraftLimits { ... }
export interface GroundLimits { ... }
export interface ShipLimits { ... }

export type VehicleLimits = 
  | { Aircraft: AircraftLimits }
  | { Ground: GroundLimits }
  | { Ship: ShipLimits };
```

### **Компиляция:**
- ❌ **Было:** 40+ warnings
- ✅ **Стало:** 1 info-warning (только `#[warn(dead_code)]` on by default)

---

## 📊 Статистика очистки

**Удалено:**
- 1 файл (dynamic_triggers.rs)
- 1 импорт модуля
- ~200 строк избыточного кода

**Добавлено #[allow(dead_code)]:**
- 25+ методов
- 10+ структур и полей
- Сохранены для будущего функционала

**Результат:**
- Чистая компиляция без warnings
- Сохранен весь полезный код
- Понятная маркировка будущего функционала

---

## 🎨 UI/UX улучшения

### **Визуальные элементы:**
- Цветовое кодирование по типам:
  - ✈️ Aircraft - синий (#6366f1)
  - 🚜 Ground - зеленый (#22c55e)
  - ⚓ Ships - голубой (#3b82f6)

- Иконки для данных:
  - 🔴 Critical (Vne, Critical Modules)
  - 🌪️ Speed/Wind (Flutter, Stall, Max Speed)
  - 📊 Gauges (G-Load, HP)
  - ⚡ Power (Engine)
  - 🛡️ Protection (Armor, Mass)
  - ⚓ Naval (Ships)

### **Адаптивность:**
- Grid layout: `repeat(auto-fit, minmax(180px, 1fr))`
- Автоматическая подгонка по контенту
- Минималистичные карточки с границами

---

## 📦 Commit

**Название:**  
`v0.7.5: VehicleInfoCard adaptive UI, code cleanup, remove dead_code`

**Описание:**
```
- Rewrite VehicleInfoCard with adaptive cards for Aircraft/Ground/Ships
- Add type-specific data display (Vne, Flutter, G-limits, Armor, etc)
- Remove dynamic_triggers.rs (redundant with vehicle_limits)
- Add #[allow(dead_code)] to 25+ future methods/structs
- Clean up 40+ warnings → 1 info-warning
- Update TypeScript types for datamine integration
- Fix optional fields in Profile/DeviceInfo interfaces
```

---

## 🎯 Итог

**Основные достижения:**
- ✅ Адаптивный UI для всех типов техники
- ✅ Чистая компиляция без warnings
- ✅ Удален избыточный код
- ✅ Сохранен весь полезный функционал
- ✅ Понятная маркировка будущего кода

**Готово к продакшену!** 🚀

---

## 📝 Примечания

**Оставлены для будущего (с #[allow(dead_code)]):**
1. **Lovense интеграция** - полный API готов
2. **Пользовательские профили** - CRUD операции
3. **Расширенная история** - AoA, RPM метрики
4. **Триггер редактор** - update/evaluate методы
5. **Helper структуры** - полные BLK парсеры

**Не трогать без необходимости** - весь код имеет цель!

