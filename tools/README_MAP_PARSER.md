# 🗺️ War Thunder Map Parser

Инструмент для извлечения реальных размеров сеток из файлов War Thunder.

## 🎯 Проблема

War Thunder API (`localhost:8111`) **возвращает неправильные размеры сетки**:
- **API говорит:** Attica = 200m × 200m
- **В реальности:** Attica = 225m × 225m ❌

Это приводит к неправильному расчету координат сетки (A-Z, 1-20).

## ✅ Решение

Парсим файлы игры (`level.blk`) и извлекаем **реальные** размеры сетки для каждой карты.

---

## 📁 Структура

```
tools/
├── parse_wt_maps.py              ← Парсер .blk файлов
├── wt_maps_database.json         ← Результат (JSON)
├── map_database_generated.rs     ← Результат (Rust код)
└── README_MAP_PARSER.md          ← Это руководство
```

---

## 🚀 Использование

### **Шаг 1: Запустить парсер**

```bash
python tools/parse_wt_maps.py
```

**Что делает:**
1. Ищет установку War Thunder
2. Сканирует `content/levels/*/level.blk`
3. Извлекает:
   - Размеры сетки для танков/авиа/кораблей
   - Размер карты
   - Игровые режимы
4. Сохраняет в JSON и генерирует Rust код

### **Шаг 2: Интегрировать в код**

Сгенерированный Rust код вставить в `src-tauri/src/map_database.rs`:

```rust
fn load_known_maps(&mut self) {
    // Вставить код из map_database_generated.rs
}
```

### **Шаг 3: Пересобрать приложение**

```bash
npm run tauri build
```

---

## 📊 Результаты

### **Пример: `wt_maps_database.json`**

```json
{
  "attica": {
    "name": "attica",
    "localized_name": "Attica",
    "ground_grid_step": 225.0,
    "map_size": [4096.0, 4096.0],
    "game_modes": ["ground"]
  },
  "european_province": {
    "name": "european_province",
    "localized_name": "European Province",
    "ground_grid_step": 200.0,
    "map_size": [4096.0, 4096.0],
    "game_modes": ["ground"]
  }
}
```

### **Найденные размеры сетки:**

#### **Танковые карты:**
| Карта | Сетка (м) | API говорит | Ошибка |
|-------|-----------|-------------|--------|
| **Attica** | **225 × 225** | 200 × 200 | ❌ 25m |
| European Province | 200 × 200 | 200 × 200 | ✅ |
| Mozdok | 200 × 200 | 200 × 200 | ✅ |
| Fulda Gap | 200 × 200 | 200 × 200 | ✅ |
| Abandoned Factory | 150 × 150 | 200 × 200 | ❌ 50m |

#### **Авиационные карты:**
- Обычно: 13100m × 13100m (13.1km)

#### **Морские карты:**
- Различаются: 800-1000m

---

## 🔍 Как работает парсер

### **1. Поиск War Thunder**

```python
common_paths = [
    r"C:\Program Files (x86)\Steam\steamapps\common\War Thunder",
    r"D:\SteamLibrary\steamapps\common\War Thunder",
    # ...
]
```

### **2. Парсинг .blk файла**

```python
# Находим размер сетки для танков
ground_grid = re.search(r'gridStep\s*:\s*([\d.]+)', content)

# Для авиации
air_grid = re.search(r'airGridStep\s*:\s*([\d.]+)', content)
```

### **3. Генерация Rust кода**

```rust
self.maps.insert("attica".to_string(), MapGridInfo {
    name: "attica".to_string(),
    localized_name: "Attica".to_string(),
    ground_grid_step: Some(225.0),  // ← REAL VALUE!
    // ...
});
```

---

## 🎮 Интеграция в UI

После парсинга приложение будет показывать **инфобокс** с реальной информацией:

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

## 🛠️ Требования

- Python 3.6+
- War Thunder установлен
- Доступ к `content/levels/` папке

---

## 📝 TODO

- [ ] Автоматическое обновление базы при патче игры
- [ ] Парсинг миникарт (`minimap_simple.dds`)
- [ ] Извлечение текстур карт
- [ ] Поддержка нескольких языков локализации

---

## ⚠️ Важно

- **Легально:** Чтение файлов игры разрешено (датамайн)
- **EAC-Safe:** Не трогаем память процесса
- **Offline:** Парсер работает без интернета

---

## 🎯 Результат

✅ **Точные координаты сетки**
✅ **Правильные позиции A-Z, 1-20**
✅ **Синхронизация с игровой картой**
✅ **База данных 100+ карт**

