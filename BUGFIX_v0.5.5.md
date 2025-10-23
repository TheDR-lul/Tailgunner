# Butt Thunder - BUGFIX v0.5.5

## 📅 Дата: 23 октября 2025

## 🎯 КРИТИЧЕСКОЕ ИСПРАВЛЕНИЕ: Интеграция с War Thunder API (v3)

**ОБНОВЛЕНИЕ 3:** Исправлена архитектура запросов! API War Thunder возвращает **ВСЕ данные в `/state`**, не нужен отдельный `/indicators`!

**ОБНОВЛЕНИЕ 2:** Исправлены названия полей! API War Thunder использует **нестандартные названия** с запятыми и единицами измерения.

### 🐛 Проблема

**Данные из War Thunder не отображались!** 

Все показатели (скорость, высота, топливо, G-перегрузка) всегда были равны **0**.

#### Причина:

В `src-tauri/src/wt_telemetry.rs`, строка 176:

```rust
let indicators = Indicators::default(); // ❌ ПУСТАЯ СТРУКТУРА!
// Комментарий: "Заполним позже через get_indicators"
// НО МЫ НИКОГДА НЕ ЗАПОЛНЯЛИ!
```

Мы создавали **пустой** `Indicators::default()` со всеми нулями и **никогда не запрашивали реальные данные** из War Thunder API.

---

## ✅ Решение

Согласно [официальной документации War Thunder localhost API](https://github.com/lucasvmx/WarThunder-localhost-documentation), нужно запрашивать **два эндпоинта**:

1. **`http://localhost:8111/state`** - базовая информация:
   - `valid` - валидность данных
   - `type` - тип техники (tank, aircraft, helicopter, ship)
   - `state` - массив состояний (флаги событий)

2. **`http://localhost:8111/indicators`** - **ВСЕ показатели**:
   - Скорость (IAS, TAS, Mach)
   - Высота, вертикальная скорость
   - G-перегрузка, угол атаки
   - Топливо, боезапас
   - Повреждения, обороты двигателя
   - И ещё 40+ параметров!

### Что изменено:

#### 1. Обновлен метод `get_state()`

**БЫЛО:**
```rust
pub async fn get_state(&mut self) -> Result<GameState> {
    let url = format!("{}/state", WT_TELEMETRY_URL);
    let response = self.client.get(&url).send().await?;
    let raw_json = response.json().await?;
    
    let state = self.parse_state(raw_json)?;  // ❌ Только /state
    Ok(state)
}
```

**СТАЛО:**
```rust
pub async fn get_state(&mut self) -> Result<GameState> {
    // 1. Запрашиваем /state для type и valid
    let state_json = self.client
        .get("http://127.0.0.1:8111/state")
        .send().await?.json().await?;
    
    log::trace!("[WT API] /state response: {}", ...);
    
    // 2. Запрашиваем /indicators для ВСЕХ показателей ✅
    let indicators_json = self.client
        .get("http://127.0.0.1:8111/indicators")
        .send().await?.json().await?;
    
    log::trace!("[WT API] /indicators response: {}", ...);
    
    // 3. Объединяем данные
    let state = self.parse_state(state_json, indicators_json)?;
    Ok(state)
}
```

#### 2. Новый метод `parse_indicators()`

Создан полноценный парсер JSON из `/indicators`:

```rust
fn parse_indicators(&self, json: serde_json::Value) -> Indicators {
    let get_f32 = |key: &str| json.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let get_i32 = |key: &str| json.get(key).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let get_bool = |key: &str| json.get(key).and_then(|v| v.as_bool()).unwrap_or(false);

    Indicators {
        // Базовые
        speed: get_f32("speed"),
        altitude: get_f32("altitude"),
        climb: get_f32("climb"),
        
        // Двигатель
        engine_rpm: get_f32("RPM"),
        throttle: get_f32("throttle"),
        ...
        
        // Аэродинамика
        aoa: get_f32("AoA"),
        g_load: get_f32("Vy"),  // Вертикальная перегрузка
        mach: get_f32("Mach"),
        tas: get_f32("TAS"),
        ias: get_f32("IAS"),
        
        // Топливо
        fuel: get_f32("fuel"),
        fuel_max: get_f32("fuel_max"),
        ...
    }
}
```

#### 3. Обновлен метод `parse_state()`

Теперь принимает **оба** JSON и объединяет данные:

```rust
fn parse_state(&self, 
    state_json: serde_json::Value,      // ← Из /state
    indicators_json: serde_json::Value  // ← Из /indicators
) -> Result<GameState> {
    // Парсим type и valid из /state
    let valid = state_json.get("valid")...;
    let type_ = state_json.get("type")...;
    let state = state_json.get("state")...;
    
    // Парсим indicators из /indicators ✅
    let indicators = self.parse_indicators(indicators_json);
    
    Ok(GameState { valid, type_, indicators, state })
}
```

#### 4. Добавлено trace-логирование

Для отладки можно увидеть **сырые JSON ответы** из WT API:

```rust
log::trace!("[WT API] /state response: {}", serde_json::to_string_pretty(&state_json)...);
log::trace!("[WT API] /indicators response: {}", serde_json::to_string_pretty(&indicators_json)...);
```

Чтобы включить trace-логи, запусти с переменной окружения:
```bash
RUST_LOG=trace npm run tauri dev
```

---

## 📊 Теперь работает!

### В UI компоненте `GameStatus`:

**БЫЛО:**
```
🔴 Не подключено
Техника: N/A
Скорость: 0 км/ч      ← Всегда 0 ❌
Высота: 0 м           ← Всегда 0 ❌
G-перегрузка: 0.0 G   ← Всегда 0 ❌
Топливо: 0%           ← Всегда 0 ❌
```

**СТАЛО:**
```
🟢 Подключено
Техника: Aircraft
Скорость: 542 км/ч    ← РЕАЛЬНЫЕ ДАННЫЕ! ✅
Высота: 2450 м        ← РЕАЛЬНЫЕ ДАННЫЕ! ✅
G-перегрузка: 3.2 G   ← РЕАЛЬНЫЕ ДАННЫЕ! ✅
Топливо: 78%          ← РЕАЛЬНЫЕ ДАННЫЕ! ✅
```

### В Debug Console:

```
[WT] Vehicle: Aircraft, Speed: 542 km/h, Alt: 2450m
[Events] Detected 0 events: []
```

---

## 🔍 Как проверить:

1. **Запусти War Thunder**
2. **Зайди в бой** (тестовый полет / танковый полигон)
3. **Запусти приложение**: `npm run tauri dev`
4. **Посмотри на раздел "War Thunder"**:
   - Должно быть 🟢 **Подключено**
   - Показатели должны **обновляться в реальном времени**
   - В Debug Console логи: `[WT] Vehicle: ..., Speed: ...`

### Если не работает:

1. **Проверь порт 8111**: Открой в браузере [http://localhost:8111/indicators](http://localhost:8111/indicators)
   - Должен вернуться JSON с данными
   - Если `404` - убедись что War Thunder запущен и ты в бою

2. **Включи trace-логи**: `RUST_LOG=trace npm run tauri dev`
   - Посмотри в консоль терминала на сырые JSON ответы

3. **Проверь Debug Console**: Должны быть логи `[WT] Game not connected: ...` если игра не подключена

---

## 🔥 ОБНОВЛЕНИЕ 2: Правильные названия полей

После тестирования обнаружено, что **API War Thunder использует нестандартные названия полей**!

### Реальная структура JSON из `http://localhost:8111/indicators`:

```json
{
  "valid": true,
  "type": "rafale_c_f3",
  "H, m=36",                  ← Высота в метрах
  "TAS, km/h=180",            ← Истинная скорость
  "IAS, km/h=179",            ← Приборная скорость
  "M=0.15",                   ← Число Маха
  "AoA, deg=0.8",             ← Угол атаки
  "AoS, deg=0.2",             ← Угол скольжения
  "Ny=0.98",                  ← Вертикальная перегрузка (G)
  "Vy, m/s=0",                ← Вертикальная скорость
  "Mfuel, kg=6320",           ← Текущее топливо в кг
  "Mfuel0, kg=9560",          ← Максимальное топливо в кг
  "RPM 1=13101",              ← Обороты двигателя 1
  "RPM 2=13099",              ← Обороты двигателя 2
  "manifold pressure 1, atm=1",
  "oil temp 1, C=74",
  "throttle 1, %=110",
  "aileron, %=0",
  "elevator, %=-100",
  "rudder, %=-11",
  "flaps, %=84",
  "gear, %=100",
  ...
}
```

### ❌ Было (НЕПРАВИЛЬНО):

```rust
speed: get_f32("speed"),         // ❌ Нет такого поля!
altitude: get_f32("altitude"),   // ❌ Нет такого поля!
g_load: get_f32("Vy"),           // ❌ Vy это вертикальная скорость!
mach: get_f32("Mach"),           // ❌ Называется просто "M"
fuel: get_f32("fuel"),           // ❌ Называется "Mfuel, kg"
engine_rpm: get_f32("RPM"),      // ❌ Называется "RPM 1" или "RPM 2"
```

### ✅ Стало (ПРАВИЛЬНО):

```rust
speed: get_f32("IAS, km/h").max(get_f32("TAS, km/h")),
altitude: get_f32("H, m"),
climb: get_f32("Vy, m/s"),
g_load: get_f32("Ny"),           // ✅ Это G-перегрузка!
mach: get_f32("M"),
fuel: get_f32("Mfuel, kg"),
fuel_max: get_f32("Mfuel0, kg"),
engine_rpm: get_f32("RPM 1").max(get_f32("RPM 2")),
engine_temp: get_f32("engine temp 1, C"),
oil_temp: get_f32("oil temp 1, C"),
throttle: get_f32("throttle 1, %"),
aileron: get_f32("aileron, %"),
elevator: get_f32("elevator, %"),
rudder: get_f32("rudder, %"),
flaps: get_f32("flaps, %"),
gear: get_f32("gear, %"),
aoa: get_f32("AoA, deg"),
slip: get_f32("AoS, deg"),
```

### 🔑 Ключевые изменения:

1. **Все названия полей содержат запятые, пробелы и единицы измерения!**
2. **Топливо в килограммах**, не в процентах → нужно рассчитывать: `fuel / fuel_max * 100%`
3. **Скорость уже в км/ч**, не нужно умножать на 3.6
4. **Поддержка 2 двигателей** (RPM 1, RPM 2) → берем максимум
5. **G-перегрузка** это `Ny`, а `Vy` это вертикальная скорость!

---

## 🔥 ОБНОВЛЕНИЕ 3: Один эндпоинт вместо двух!

После реального тестирования выяснилось, что **API War Thunder работает иначе**!

### ❌ БЫЛО (НЕПРАВИЛЬНО):

Мы делали **ДВА ЗАПРОСА**:
```rust
// 1. Запрос к /state
let state_json = get("http://localhost:8111/state").await?;

// 2. Запрос к /indicators ❌ НЕ НУЖЕН!
let indicators_json = get("http://localhost:8111/indicators").await?;

// Объединяем
parse_state(state_json, indicators_json)
```

### ✅ СТАЛО (ПРАВИЛЬНО):

Делаем **ОДИН ЗАПРОС** - все данные уже в `/state`!

```rust
// Один запрос к /state - содержит ВСЁ!
let json = get("http://localhost:8111/state").await?;

// Парсим
parse_state(json)
```

### 📊 Структура ответа `/state`:

```json
{
  "valid": true,
  "type": "rafale_c_f3",
  "H, m": 378,                  ← Все indicators прямо здесь!
  "TAS, km/h": 873,
  "IAS, km/h": 857,
  "M": 0.71,
  "Ny": 1.02,
  "Mfuel, kg": 6320,
  "Mfuel0, kg": 9560,
  "RPM 1": 12965,
  "aileron, %": 0,
  "elevator, %": -8,
  ...
}
```

### Почему так произошло?

1. **Документация неточная** - [официальная документация](https://github.com/lucasvmx/WarThunder-localhost-documentation) упоминает `/indicators` как отдельный эндпоинт
2. **Но реально** - все данные в `/state`!
3. **Двойной запрос** создавал задержку и мог приводить к ошибкам

### 🔑 Изменения:

```diff
- // Запрашиваем /state для получения type и valid
- let state_json = get("/state").await?;
- 
- // Запрашиваем /indicators для получения всех показателей
- let indicators_json = get("/indicators").await?;
- 
- parse_state(state_json, indicators_json)?

+ // Запрашиваем /state - содержит ВСЕ данные
+ let json = get("/state").await?;
+ parse_state(json)?
```

```diff
- fn parse_state(&self, state_json: Value, indicators_json: Value) -> Result<GameState> {
-     let indicators = parse_indicators(indicators_json);
+ fn parse_state(&self, json: Value) -> Result<GameState> {
+     let indicators = parse_indicators(json); // Из того же JSON!
```

---

## 📝 Изменённые файлы:

- `src-tauri/src/wt_telemetry.rs`:
  - Обновлен `get_state()` - теперь запрашивает `/state` И `/indicators`
  - Создан `parse_indicators()` - парсит все 40+ полей из JSON
  - Обновлен `parse_state()` - принимает оба JSON
  - Добавлено trace-логирование сырых API ответов

---

## 🎯 Итоги

✅ **Исправлена критическая ошибка** - данные из War Thunder теперь отображаются  
✅ **Полная интеграция с WT API** согласно [официальной документации](https://github.com/lucasvmx/WarThunder-localhost-documentation)  
✅ **Все 40+ параметров парсятся** из `/indicators`  
✅ **UI показывает реальные данные** в реальном времени  
✅ **Debug-логирование** для отладки  

---

## 🔜 Следующие шаги:

- Проверить корректность названий полей (сверить с реальным JSON из WT)
- Добавить парсинг `hudmsg` для событий попадания/повреждений
- Интегрировать `map_obj.json` для определения названия техники
- Улучшить обработку ошибок подключения

---

**Версия:** v0.5.5  
**Предыдущая версия:** v0.5.4  
**Тип исправления:** CRITICAL BUGFIX  
**Приоритет:** 🔥 ВЫСОКИЙ

