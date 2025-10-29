# Changelog v0.7.8 - Vehicle Database UI

**Дата:** 25 октября 2025  
**Версия:** 0.7.8  
**Тип:** Feature

---

## 📝 Описание

Добавлена секция **Vehicle Database** в Dashboard с кнопкой для автоматического сбора данных техники из файлов War Thunder.

---

## ✨ Новые возможности

### **Vehicle Database Section в Dashboard**

**Локация:** Dashboard → Vehicle Database

**Когда база пуста:**
```
┌─────────────────────────────────┐
│ 💿 Vehicle Database             │
├─────────────────────────────────┤
│                                 │
│        💿                       │
│   Database is empty             │
│   Build from War Thunder files  │
│   to show vehicle data          │
│                                 │
│ [Build Vehicle Database]        │
│                                 │
└─────────────────────────────────┘
```

**Когда база заполнена:**
```
┌─────────────────────────────────┐
│ 💿 Vehicle Database             │
├─────────────────────────────────┤
│  ┌───────┬───────┬───────┐      │
│  │  500  │  300  │  100  │      │
│  │ ✈️ Air │ 🚜 Tank│ ⚓ Ship│      │
│  └───────┴───────┴───────┘      │
│                                 │
│ [Rebuild Database]              │
│                                 │
└─────────────────────────────────┘
```

**Функциональность:**
1. **Автоматический поиск War Thunder:**
   - Проверяет реестр Windows
   - Ищет по стандартным путям
   - Находит `aces.vromfs.bin_u`

2. **Парсинг файлов:**
   - Читает `.blkx` файлы
   - Извлекает данные техники:
     - ✈️ Aircraft: Vne, Flutter, G-limits, Engine, Mass
     - 🚜 Ground: Max Speed, Engine, Mass, Armor, HP
     - ⚓ Ships: Max Speed, Compartments, HP, Critical modules

3. **SQLite база:**
   - Сохраняет в локальную БД
   - Быстрый доступ
   - Не требует интернет

4. **Статистика:**
   - Количество самолетов
   - Количество танков
   - Количество кораблей

---

## 🎨 UI/UX

### **Состояния кнопки:**

**Idle (база пуста):**
```tsx
<button className="btn btn-primary btn-sm">
  <Database size={16} />
  Build Vehicle Database
</button>
```

**Loading:**
```tsx
<button className="btn btn-primary btn-sm" disabled>
  <Database size={16} className="spin" />
  Building Database...
</button>
```

**Success:**
- Кнопка становится "Rebuild Database"
- Показываются статистики (Aircraft / Ground / Ships)
- Debug log: `✅ Database built: 500 aircraft, 300 ground, 100 ships`

**Error:**
- Debug log: `❌ Database error: War Thunder not found`
- Кнопка остается активной для повторной попытки

---

## 🔧 Технические детали

### **Dashboard.tsx:**
```typescript
const [dbStats, setDbStats] = useState<[number, number, number] | null>(null);
const [isParsingDb, setIsParsingDb] = useState(false);

// Check DB stats every 3s
useEffect(() => {
  const checkDbStats = async () => {
    const stats = await api.datamineGetStats(); // [aircraft, ground, ships]
    setDbStats(stats);
  };
  
  checkDbStats();
  const interval = setInterval(checkDbStats, 3000);
  return () => clearInterval(interval);
}, []);

// Build database
const handleInitDatabase = async () => {
  setIsParsingDb(true);
  
  const result = await api.datamineAutoInit();
  // "Database built: 500 aircraft, 300 ground, 100 ships"
  
  const stats = await api.datamineGetStats();
  setDbStats(stats);
  
  setIsParsingDb(false);
};
```

### **Backend flow:**
```rust
datamine_auto_init()
  ↓
1. Check if database exists
   - If has data → return stats
   ↓
2. Auto-detect War Thunder
   - Windows Registry
   - Common paths
   ↓
3. Parse all files
   - Aircraft: gamedata/flightmodels/fm/*.blkx
   - Ground: gamedata/units/tankmodels/*.blkx
   - Ships: gamedata/units/ships/*.blkx
   ↓
4. Save to SQLite
   - tables: aircraft, ground, ships
   ↓
5. Return stats
   - [500, 300, 100]
```

---

## 📊 Статистика изменений

**Файлы:**
- `src/components/Dashboard.tsx` - добавлена секция (+120 lines)
- `package.json`, `Cargo.toml`, `tauri.conf.json` - версия 0.7.8

**Строки:**
- +120 строк UI
- +2 useState hooks
- +1 button handler

---

## 🧪 Тестирование

### **Сценарии:**
1. ✅ Первый запуск → показывает "Database is empty"
2. ✅ Клик "Build" → парсинг начинается, кнопка disabled
3. ✅ Парсинг успешен → показывает статистики
4. ✅ War Thunder не найден → показывает ошибку в Debug Console
5. ✅ Клик "Rebuild" → обновляет базу (если вышел патч WT)
6. ✅ Статистики обновляются каждые 3 секунды

---

## 📦 Commit

**Название:**  
`v0.7.8: Add Vehicle Database UI to Dashboard`

**Описание:**
```
Feature:
- Add "Vehicle Database" section to Dashboard
- Button to auto-build database from WT files
- Show stats: Aircraft / Ground / Ships count
- Auto-refresh stats every 3 seconds
- Loading state with spinner
- Empty state placeholder

UI:
- Grid layout for stats (3 columns)
- Color coding (blue/green/blue for types)
- Rebuild button when database exists

Files:
- src/components/Dashboard.tsx: new section (+120 lines)
- package.json, Cargo.toml, tauri.conf.json: v0.7.8
```

---

## 🎯 Итог

**Теперь НЕ НУЖНО:**
- ❌ Вручную искать War Thunder
- ❌ Вызывать команды в Debug Console
- ❌ Искать где включить парсинг

**Нужно ТОЛЬКО:**
- ✅ Нажать одну кнопку "Build Vehicle Database"
- ✅ Подождать 10-30 секунд
- ✅ База готова!

**Патч готов!** 🚀

---

## 📝 Для пользователя

**Инструкция:**

1. **Открой Dashboard** (главный экран)
2. **Найди секцию "Vehicle Database"** (внизу под Safety badge)
3. **Нажми "Build Vehicle Database"**
4. **Подожди 10-30 секунд** (в зависимости от ПК)
5. **Готово!** Увидишь статистику:
   - ✈️ 500 Aircraft
   - 🚜 300 Ground
   - ⚓ 100 Ships

**Теперь Vehicle Information Card покажет данные техники!**

Если War Thunder не найден автоматически:
- Запусти игру хотя бы раз
- Или укажи путь вручную (пока нет UI, но скоро добавим)

