# Changelog v0.7.7 - Debugging Improvements & ConditionNode Fix

**Дата:** 25 октября 2025  
**Версия:** 0.7.7  
**Тип:** Bugfix, Enhancement

---

## 📝 Описание

Критичные улучшения логирования для отладки проблем с поиском техники, замена небезопасных `unwrap()` на `expect()`, и исправление дубликата функциональности между `ConditionNode` и `LogicNode`.

---

## 🐛 Исправленные баги

### 1. **Vehicle не находился в датамайне - плохое логирование**

**Проблема:**
- Техника не показывалась в Vehicle Information Card
- Fuzzy search работал молча, непонятно что искал
- Невозможно было понять почему техника не найдена

**Исправление:**
```rust
// src-tauri/src/datamine/database.rs
log::info!("[Database] Searching for vehicle: '{}'", identifier);
log::debug!("[Database] Generated {} alternatives: {:?}", alternatives.len(), alternatives);

for (idx, alt_id) in alternatives.iter().enumerate() {
    log::debug!("[Database] Try {}/{}: '{}'", idx + 1, alternatives.len(), alt_id);
    
    if let Ok(aircraft) = self.get_aircraft(alt_id) {
        log::info!("[Database] ✅ Found AIRCRAFT: '{}' (matched: '{}')", 
            aircraft.display_name, alt_id);
        return Some(VehicleLimits::Aircraft(aircraft));
    }
    // ... same for ground/ships
}

log::error!("[Database] ❌ NO MATCH for '{}' after trying {} alternatives", 
    identifier, alternatives.len());
```

**Теперь в логах:**
```
[Vehicle Info] 🔍 Fetching data for vehicle: 'sw_strv_122b_plss'
[Vehicle Info] 🔎 Searching for: 'sw_strv_122b_plss'
[Database] Searching for vehicle: 'sw_strv_122b_plss'
[Database] Try 1/7: 'sw_strv_122b_plss'
[Database] Try 2/7: 'sw strv 122b plss'
[Database] Try 3/7: 'swstrv122bplss'
[Database] ✅ Found GROUND: 'Strv 122B PLSS' (matched: 'swstrv122bplss')
```

**Результат:**
- ✅ Видно все попытки поиска
- ✅ Понятно какой identifier сматчился
- ✅ Легко отлаживать проблемы

---

### 2. **ConditionNode и LogicNode были дубликатами**

**Проблема:**
- Оба node делали одно и то же (AND/OR/XOR/NOT)
- Непонятно зачем два одинаковых node
- Запутанный UX

**Исправление:**

**LogicNode** (остался как есть):
- Для объединения РЕЗУЛЬТАТОВ условий
- AND / OR / XOR / NOT логика
- Входы: A, B (boolean)
- Выход: boolean

**ConditionNode** (ПЕРЕПИСАН):
- Для СРАВНЕНИЯ параметров
- Параметр: Speed / Altitude / G-Load / Fuel / RPM
- Оператор: > / < / >= / <= / =
- Значение: число
- Пример: `Speed > 500 km/h`

**Схема использования:**
```
[InputNode: Speed] → [ConditionNode: Speed > 500] → [LogicNode: AND] → [OutputNode]
                                                        ↑
[InputNode: Altitude] → [ConditionNode: Alt < 100] ────┘
```

**Результат:**
- ✅ Понятное разделение
- ✅ ConditionNode для сравнений
- ✅ LogicNode для комбинирования

---

### 3. **Небезопасные unwrap() заменены на expect()**

**Проблема:**
- 6x `unwrap()` в критичных местах
- При панике нет информации о причине
- Сложно отлаживать крашы

**Исправление:**

**pattern_engine.rs:**
```rust
// Было:
let start_point = points.first().unwrap();

// Стало:
let start_point = points.first()
    .expect("Curve must have at least one point");
```

**vehicle_limits.rs:**
```rust
// Было:
Self::new().unwrap()

// Стало:
Self::new()
    .expect("Failed to initialize VehicleLimitsManager: database connection error")
```

**state_history.rs:**
```rust
// Было:
.min_by(|a, b| a.partial_cmp(b).unwrap())

// Стало:
.min_by(|a, b| a.partial_cmp(b).expect("NaN in state history"))
```

**Результат:**
- ✅ Понятные сообщения при панике
- ✅ Легче отлаживать
- ✅ Лучший DX (Developer Experience)

---

## 🔧 Технические улучшения

### **Fuzzy Search Logging:**
- Логирование каждой попытки поиска
- Индексы (1/7, 2/7, ...)
- Эмодзи для быстрого сканирования (🔍 🔎 ✅ ❌)

### **ConditionNode Redesign:**
```typescript
interface ConditionNodeData {
  param: 'speed' | 'altitude' | 'g_load' | 'fuel' | 'rpm';
  operator: '>' | '<' | '=' | '>=' | '<=';
  value: number;
}
```

**UI улучшения:**
- 3 селектора: Parameter / Operator / Value
- Live formula preview: `Speed > 500 km/h`
- Цветовое кодирование по параметру
- Шаг 0.1 для G-Load, 1 для остальных

---

## 📊 Статистика изменений

**Файлы:**
- `src-tauri/src/datamine/database.rs` - detailed logging (+20 lines)
- `src-tauri/src/lib.rs` - logging, cleanup (+5 lines)
- `src-tauri/src/pattern_engine.rs` - unwrap → expect (3x)
- `src-tauri/src/vehicle_limits.rs` - unwrap → expect (1x)
- `src-tauri/src/state_history.rs` - unwrap → expect (2x)
- `src/components/nodes/ConditionNode.tsx` - REWRITE (160 lines)

**Строки:**
- +180 строк (logging + ConditionNode)
- -6 unwrap()
- +6 expect()

---

## 🧪 Тестирование

### **Сценарии:**
1. ✅ Техника не найдена → лог показывает все попытки
2. ✅ Техника найдена → лог показывает какой identifier сматчился
3. ✅ ConditionNode: создать `Speed > 500` → работает
4. ✅ LogicNode + ConditionNode → правильная логика
5. ✅ Panic с expect → показывает осмысленное сообщение

---

## 📦 Commit

**Название:**  
`v0.7.7: Add fuzzy search logging, fix ConditionNode, replace unwrap`

**Описание:**
```
Debugging:
- Add detailed logging to fuzzy search (all alternatives)
- Add emojis for quick log scanning (🔍 🔎 ✅ ❌)
- Use vehicle_name AS IS (fuzzy search handles variations)

Bugfix:
- Rewrite ConditionNode for comparisons (Speed > 500)
- LogicNode now distinct (AND/OR for combining results)
- Replace 6x unwrap() with expect() + meaningful messages

Files:
- src-tauri/src/datamine/database.rs: detailed logging
- src-tauri/src/lib.rs: cleanup vehicle search
- src-tauri/src/pattern_engine.rs: expect() messages
- src-tauri/src/vehicle_limits.rs: expect() messages
- src-tauri/src/state_history.rs: expect() messages
- src/components/nodes/ConditionNode.tsx: REWRITE
```

---

## 🎯 Итог

**Основные достижения:**
- ✅ Отладка техники теперь простая (видны все попытки)
- ✅ ConditionNode и LogicNode разделены по назначению
- ✅ Все unwrap() заменены на expect() с сообщениями
- ✅ Готов к продакшену

**Патч готов!** 🚀

---

## 📝 Для пользователя

**Если техника не показывается:**
1. Запусти с логами: `RUST_LOG=debug npm run tauri dev`
2. Посмотри в терминал:
   - Какой identifier искался
   - Какие альтернативы пробовались
   - Сматчилось ли что-то
3. Если не нашлась → нужно запустить datamine parsing:
   ```
   datamine_auto_init() или datamine_parse()
   ```

**Новый ConditionNode:**
- Теперь для сравнений: `Speed > 500`, `Altitude < 100`
- LogicNode для комбинирования: `A AND B`
- Можно строить сложные условия!

