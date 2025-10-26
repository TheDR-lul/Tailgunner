# 🗺️ Исправление размеров сетки карт

## 🎯 Проблема

War Thunder API (`localhost:8111`) возвращает **неправильные размеры сетки**:

| Карта | API говорит | Реально | Ошибка |
|-------|-------------|---------|--------|
| **Attica** | 200m × 200m | **225m × 225m** | ❌ **25m** |
| American Desert | 200m × 200m | **175m × 175m** | ❌ **25m** |
| Abandoned Factory | 200m × 200m | **150m × 150m** | ❌ **50m** |

Это приводит к неправильным координатам сетки (A-Z, 1-20).

---

## ✅ Решение

### **1. База данных карт** (`src-tauri/src/map_database.rs`)

Создана база с **реальными размерами** для каждой карты:

```rust
pub struct MapGridInfo {
    pub name: String,
    pub ground_grid_step: Option<f32>,  // Реальный размер!
    pub air_grid_step: Option<f32>,
    pub naval_grid_step: Option<f32>,
    pub map_size: [f32; 2],
}
```

**Примеры:**
- ✅ Attica: `225m × 225m` (не 200m!)
- ✅ American Desert: `175m × 175m`
- ✅ Abandoned Factory: `150m × 150m`
- ✅ European Province: `200m × 200m` (правильно)

### **2. Интеграция с map_module** (`src-tauri/src/map_module.rs`)

```rust
// Приоритет: база данных → API
let (grid_step_x, grid_step_y) = if let Some(correct) = self.correct_grid_step {
    (correct[0], correct[1])  // ✓ Используем правильные!
} else {
    (self.info.grid_steps[0], self.info.grid_steps[1])  // Fallback
};
```

### **3. UI Инфобокс** (`src/components/MapInfoBox.tsx`)

Компонент показывает:
- ✅ Реальные размеры сетки из игры
- ⚠️ Предупреждение если API неточен
- 📊 Информация о карте (размер, режимы)

**Пример:**
```
[i] Информация о карте

Карта: Attica
Размер: 4.1km × 4.1km

Размеры сетки (из игры):
🚜 Танки: 225m × 225m

⚠️ API неточен!
API даёт: 200m × 200m ✗
Реально: 225m × 225m ✓

Используем правильные размеры из базы данных
```

---

## 🛠️ Инструменты

### **Парсер карт** (`tools/parse_wt_maps.py`)

Извлекает реальные размеры из файлов игры:

```bash
python tools/parse_wt_maps.py
```

**Результат:**
- `wt_maps_database.json` - JSON с данными
- `map_database_generated.rs` - Rust код

**См. подробности:** [`tools/README_MAP_PARSER.md`](tools/README_MAP_PARSER.md)

---

## 📊 Текущая база (12 карт)

### **Танковые:**
1. ✅ **Attica** - 225m (исправлено!)
2. ✅ European Province - 200m
3. ✅ Mozdok - 200m
4. ✅ Fulda Gap - 200m (большая)
5. ✅ **Abandoned Factory** - 150m (исправлено!)
6. ✅ Karelia - 200m
7. ✅ Sinai - 200m (большая)
8. ✅ **American Desert** - 175m (исправлено!)
9. ✅ Fields of Poland - 200m
10. ✅ Maginot Line - 200m

### **Авиационные:**
1. ✅ Berlin - 13.1km

### **Морские:**
1. ✅ Bay of Naples - 800m

---

## 🚀 Использование

### **Добавить новую карту:**

1. Собрать дамп API:
```javascript
invoke('dump_wt_api')
```

2. Найти `map_min`, `map_max`, `grid_steps`

3. Добавить в `map_database.rs`:
```rust
self.maps.insert("new_map".to_string(), MapGridInfo {
    name: "new_map".to_string(),
    localized_name: "New Map".to_string(),
    ground_grid_step: Some(225.0),  // Реальный размер!
    // ...
});
```

### **Или использовать парсер:**

```bash
python tools/parse_wt_maps.py
# → Автоматически извлечёт ВСЕ карты из игры!
```

---

## 🎮 Как это работает

### **Детекция карты:**
1. API даёт координаты: `map_min`, `map_max`, `grid_zero`
2. Ищем в базе по координатам
3. Если найдено → используем правильные размеры
4. Если нет → fallback на API (с предупреждением)

### **Отображение сетки:**
```typescript
// Используем правильные размеры из базы!
const gridStepX = mapData.correct_grid_step?.[0] ?? info.grid_steps[0];
const gridStepY = mapData.correct_grid_step?.[1] ?? info.grid_steps[1];
```

### **Расчет координат:**
```rust
// Attica: 225m вместо 200m из API!
let grid_x = (abs_x / grid_step_x).floor() as i32;
let grid_y = (abs_y / grid_step_y).floor() as i32;
// → Правильная позиция на сетке A-Z, 1-20
```

---

## 📈 Результат

✅ **Точные координаты сетки**
✅ **Синхронизация с игровой картой**
✅ **Автоочистка фида** при смене боя
✅ **Инфобокс с реальной информацией**
✅ **База данных карт** (расширяемая)
✅ **Парсер для автоматизации**

---

## 🔄 Изменённые файлы

### **Rust Backend:**
- ✅ `src-tauri/src/map_database.rs` (новый)
- ✅ `src-tauri/src/map_module.rs` (обновлён)
- ✅ `src-tauri/src/map_detection.rs` (обновлён)
- ✅ `src-tauri/src/lib.rs` (новая команда)

### **React Frontend:**
- ✅ `src/components/MapInfoBox.tsx` (новый)
- ✅ `src/components/MapInfoBox.css` (новый)
- ✅ `src/components/MiniMap.tsx` (обновлён)
- ✅ `src/components/GameChat.tsx` (очистка фида)

### **Tools:**
- ✅ `tools/parse_wt_maps.py` (новый)
- ✅ `tools/README_MAP_PARSER.md` (новый)

---

## 🎯 TODO

- [ ] Добавить все 150+ карт из игры
- [ ] Автообновление при патче WT
- [ ] Извлечение миникарт (`.dds` текстуры)
- [ ] Поддержка других языков

---

**Статус:** ✅ **Готово к использованию!**

**Проверено на:** Attica, European Province, Mozdok, Fulda Gap

