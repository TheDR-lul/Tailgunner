# Changelog v0.7.5 - UI Update & Cleanup

**Дата:** 25 октября 2025  
**Версия:** 0.7.5  
**Тип:** Feature, Refactoring

---

## 📝 Описание

Обновлен интерфейс отображения данных техники для работы с новым датамайном. Добавлена поддержка всех типов техники (самолеты, танки, корабли) с адаптивным отображением характеристик.

---

## ✨ Новые возможности

### 1. **Обновлен VehicleInfoCard**

**Было:**
- Старая структура от `wt_vehicles_api`
- Один универсальный вид для всей техники
- Ограниченный набор данных

**Стало:**
- Адаптивные карточки для каждого типа техники
- **AircraftCard**: Vne, Flutter Speed, G-Limits, Engine Power, Mass, Stall Speed
- **GroundCard**: Max Speed, Engine Power, Mass, Armor, Hull HP
- **ShipCard**: Max Speed (knots), Compartments, Total HP, Critical Modules

**Файлы:**
- `src/components/VehicleInfoCard.tsx` - полностью переписан
- `src/types.ts` - добавлены типы из датамайна

---

## 🎨 UI улучшения

### Самолеты (Aircraft):
```
✈️ Aircraft
┌─────────────────────┬─────────────────────┐
│ 🔴 Vne (Wing Rip)   │ 🌪️ Flutter Speed   │
│    790 km/h         │    730 km/h         │
├─────────────────────┼─────────────────────┤
│ 📊 G-Load Limits    │ ⚡ Engine Power     │
│    +9.0G / -4.0G    │    1200 HP          │
├─────────────────────┼─────────────────────┤
│ 🛡️ Mass             │ 🌪️ Stall Speed     │
│    3.0 t            │    150 km/h         │
└─────────────────────┴─────────────────────┘
```

### Танки (Ground):
```
🚜 mediumVehicle
┌─────────────────────┬─────────────────────┐
│ 🌪️ Max Speed        │ ⚡ Engine Power     │
│    60 km/h          │    650 HP           │
├─────────────────────┼─────────────────────┤
│ 🛡️ Mass             │ 🛡️ Armor           │
│    56.5 t           │    100 mm           │
├─────────────────────┼─────────────────────┤
│ 📊 Hull HP          │                     │
│    5000             │                     │
└─────────────────────┴─────────────────────┘
```

### Корабли (Ships):
```
⚓ battleship
┌─────────────────────┬─────────────────────┐
│ ⚓ Max Speed         │ 🛡️ Compartments    │
│    80 knots         │    15               │
├─────────────────────┼─────────────────────┤
│ 📊 Total HP         │ 🔴 Critical Modules │
│    68000            │    3                │
└─────────────────────┴─────────────────────┘
Critical Modules:
[engine_room (4700 HP)] [ammunition (4700 HP)]
```

---

## 🔧 Технические изменения

### TypeScript типы:
```typescript
// Новые типы в src/types.ts
export interface AircraftLimits { ... }
export interface GroundLimits { ... }
export interface ShipLimits { ... }

export type VehicleLimits = 
  | { Aircraft: AircraftLimits }
  | { Ground: GroundLimits }
  | { Ship: ShipLimits };
```

### Компонентная структура:
```tsx
<VehicleInfoCard>
  ├─ if 'Aircraft' → <AircraftCard />
  ├─ if 'Ground'   → <GroundCard />
  └─ if 'Ship'     → <ShipCard />
```

---

## 🎯 Данные техники

### ✈️ Самолеты:
- **Vne**: Never Exceed Speed (скорость разрыва крыльев)
- **Flutter Speed**: Скорость флаттера (предупреждение)
- **G-Load**: Лимиты перегрузок (+/-G)
- **Engine Power**: Мощность двигателя (HP)
- **Mass**: Взлетная масса
- **Stall Speed**: Скорость сваливания

### 🚜 Танки:
- **Max Speed**: Максимальная скорость
- **Engine Power**: Мощность двигателя (HP)
- **Mass**: Масса танка
- **Armor**: Толщина брони (лобовая)
- **Hull HP**: Прочность корпуса

### ⚓ Корабли:
- **Max Speed**: Максимальная скорость (узлы)
- **Compartments**: Количество модулей
- **Total HP**: Суммарная прочность
- **Critical Modules**: Критичные модули (двигатель, боезапас)

---

## 🔄 Интеграция с датамайном

**Автообновление:**
- Данные обновляются каждые 5 секунд
- Автоматическое определение типа техники
- Показывает только когда техника подключена

**Error handling:**
- Скрывается при "No vehicle connected"
- Показывает ошибку только при критичных проблемах
- Debug логирование в консоль

---

## 📦 Commit

**Название:** `v0.7.5: Update VehicleInfoCard for datamine, adaptive UI`

**Описание:**
```
- Rewrite VehicleInfoCard for new datamine structure
- Add adaptive cards for Aircraft/Ground/Ships
- Add TypeScript types for VehicleLimits
- Show type-specific data (Vne, Flutter, G-limits, Armor, etc)
- Improve visual layout with icons and colors
```

---

## 🎯 Итог

**UI обновлен под датамайн:**
- ✅ Адаптивное отображение по типам
- ✅ Полный набор характеристик
- ✅ Красивый визуал с иконками
- ✅ Автообновление данных

**Готово к тестированию!** 🚀

