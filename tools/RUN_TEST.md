# ⚡ БЫСТРЫЙ ТЕСТ

## 🎯 Проверяем можно ли детектить карту по картинке

### 1. Зайди в игру
```
✅ Запусти War Thunder
✅ Зайди в бой на Campania (как на скриншоте)
✅ Открой карту (M)
```

### 2. Запусти скрипт
```bash
cd G:\projects\Butt_Thunder
python tools/check_map_image.py
```

### 3. Что будет:
```
[1/3] Getting image from API...
✓ API image: 1234567 bytes
✓ SHA256: abc123def456...
✓ Saved to: tools/api_map_dump.png

[2/3] Finding War Thunder...
✓ Found War Thunder at: C:\...\War Thunder

[3/3] Searching for matching images...
Found 500 image files
Comparing hashes...
  Checked 100/500...
  Checked 200/500...

✓✓✓ MATCH FOUND! ✓✓✓  ← Если повезёт!
  File: content/levels/avg_campania/minimap_simple.dds
  Hash: abc123def456...

✓ We can use image hash for map detection!
```

### 4. Результаты:

**✅ Если нашёл совпадение:**
- Значит API возвращает оригинальные файлы
- Можем использовать SHA256 хеш
- Соберём хеши всех карт
- Добавим в код

**❌ Если НЕ нашёл:**
- API генерирует картинки динамически
- SHA256 не подойдёт
- Попробуем перцептивный хеш (pHash)
- Или придумаем что-то другое

---

## 🔍 Если не нашёл:

Посмотри файл:
```
tools/api_map_dump.png
```

Сравни с картами в:
```
War Thunder\content\levels\avg_campania\
```

Похожи? Может просто масштаб другой?

---

## 🚀 ЗАПУСКАЙ!

```bash
python tools/check_map_image.py
```

И напиши что получилось! 🎯

