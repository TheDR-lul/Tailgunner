# Анализ недоделок в Tailgunner

**Дата:** 25 октября 2025  
**Версия:** 0.7.6

---

## 🔍 Найденные недоделки

### 1. **❌ Advanced Pattern Nodes не реализованы в бэкенде**

**Файлы:**
- `src/components/nodes/ConditionNode.tsx` ✅ (UI готов)
- `src/components/nodes/LogicNode.tsx` ✅ (UI готов)
- `src/components/nodes/MultiConditionNode.tsx` ✅ (UI готов)
- `src-tauri/src/pattern_engine.rs` ❌ (нет обработки)

**Проблема:**
- UI позволяет создавать Condition/Logic/MultiCondition ноды
- Rust backend их не обрабатывает, они игнорируются
- Паттерны с этими нодами не работают автоматически

**Что нужно:**
1. Добавить обработку Condition nodes в pattern_engine.rs
2. Реализовать логику AND/OR/XOR/NOT
3. Связать с event_triggers.rs
4. Добавить конвертацию UIPattern → EventTrigger для condition nodes

**Приоритет:** 🔴 HIGH (фича есть в UI, но не работает)

---

### 2. **⚠️ unwrap() вместо expect() в критических местах**

**Файлы:**
- `src-tauri/src/pattern_engine.rs:186-192` (3x unwrap())
- `src-tauri/src/vehicle_limits.rs:199` (1x unwrap())
- `src-tauri/src/state_history.rs:169, 183` (2x unwrap())

**Проблема:**
- unwrap() не дает информации при панике
- Сложно отлаживать крашы

**Что нужно:**
```rust
// Было:
let start_point = points.first().unwrap();

// Должно быть:
let start_point = points.first()
    .expect("Curve must have at least one point");
```

**Приоритет:** 🟡 MEDIUM (code quality, debugging)

---

### 3. **⚠️ Datamine fallback значения**

**Файлы:**
- `src-tauri/src/datamine/aircraft.rs:49`
- `src-tauri/src/datamine/ground.rs`
- `src-tauri/src/datamine/naval.rs`

**Проблема:**
```rust
let vne = json["Vne"].as_f64().unwrap_or(800.0) as f32;
```
- Используются fallback значения (800.0 km/h для Vne)
- Если данные отсутствуют, подставляются "средние"
- Пользователь не знает что данные неточные

**Что нужно:**
1. Добавить флаг `data_quality: DataQuality` в типы
2. Логировать когда используются fallback
3. В UI показывать предупреждение (⚠️ Estimated data)

**Приоритет:** 🟡 MEDIUM (data accuracy)

---

### 4. **📝 VehicleInfoCard не показывает "data quality"**

**Файлы:**
- `src/components/VehicleInfoCard.tsx`

**Проблема:**
- Нет индикации качества данных
- Пользователь не знает, точные ли данные или fallback
- Нет предупреждения когда техника не найдена в датамайне

**Что нужно:**
1. Добавить бейдж с качеством данных:
   - ✅ **Complete** - все данные найдены
   - ⚠️ **Partial** - некоторые fallback
   - ❌ **Limited** - много fallback

2. Tooltip с пояснением

**Приоритет:** 🟢 LOW (nice to have, UX)

---

### 5. **🎮 Pattern Export/Import не сохраняет metadata**

**Файлы:**
- `src-tauri/src/ui_patterns.rs`
- `src/components/PatternEditorModal.tsx`

**Проблема:**
- При export/import паттернов теряется:
  - Дата создания
  - Автор
  - Версия
  - Совместимость с версиями приложения

**Что нужно:**
```rust
pub struct UIPatternExport {
    pub pattern: UIPattern,
    pub metadata: PatternMetadata,
}

pub struct PatternMetadata {
    pub created_at: String,
    pub author: String,
    pub app_version: String,
    pub description: String,
}
```

**Приоритет:** 🟢 LOW (future-proofing)

---

### 6. **🔍 Fuzzy search не логирует попытки**

**Файлы:**
- `src-tauri/src/datamine/database.rs:219-248`

**Проблема:**
- `generate_identifier_alternatives()` создает альтернативы
- Но не логирует какие пробовались
- Сложно отлаживать когда техника не находится

**Что нужно:**
```rust
for (idx, alt_id) in alternatives.iter().enumerate() {
    log::debug!("[Database] Trying alternative {}/{}: '{}'", 
        idx + 1, alternatives.len(), alt_id);
    
    if let Ok(aircraft) = self.get_aircraft(alt_id) {
        log::info!("[Database] ✅ Match found: '{}'", alt_id);
        return Some(VehicleLimits::Aircraft(aircraft));
    }
}
```

**Приоритет:** 🟢 LOW (debugging)

---

### 7. **📊 GameStatus не показывает Engine RPM**

**Файлы:**
- `src/components/GameStatus.tsx`
- `src-tauri/src/haptic_engine.rs` (GameStatusInfo)

**Проблема:**
- Телеметрия включает engine_rpm
- GameStatus не отображает
- Полезно для отладки и мониторинга

**Что нужно:**
Добавить карточку с RPM (как Speed/Altitude):
```tsx
<div className="stat-item">
  <Zap size={18} className="stat-icon" />
  <div className="stat-content">
    <span className="stat-label">Engine RPM</span>
    <span className="stat-value">{status.engine_rpm}</span>
  </div>
</div>
```

**Приоритет:** 🟢 LOW (nice to have)

---

## ✅ Что предлагаю сделать

### **Сейчас (v0.7.7 патч):**
1. ✅ Заменить все unwrap() на expect() с осмысленными сообщениями
2. ✅ Добавить detailed logging в fuzzy search
3. ✅ Исправить отступы (indent) в changelog файлах

### **Следующий минорный релиз (v0.8.0):**
1. 🔴 Реализовать Condition/Logic nodes в pattern_engine
2. 🟡 Добавить DataQuality флаг и показывать в UI
3. 🟢 Добавить Engine RPM в GameStatus

### **Backlog (v0.9.0+):**
1. Pattern metadata (export/import)
2. Advanced pattern validation
3. Pattern marketplace/sharing

---

## 📝 Примечания

**Что НЕ является недоделкой:**
- `#[allow(dead_code)]` методы - специально оставлены для будущего
- Debug логи в коде - нужны для отладки
- DebugConsole в UI - рабочий инструмент

**Приоритизация:**
- 🔴 HIGH - блокирует функциональность, нужно исправить срочно
- 🟡 MEDIUM - влияет на качество, желательно скоро
- 🟢 LOW - улучшения, можно отложить

---

## 🎯 Итог

**Критичных блокеров:** 0  
**Важных недоделок:** 1 (Advanced Pattern Nodes)  
**Средних недоделок:** 2 (unwrap, fallback values)  
**Мелких улучшений:** 4 (logging, UI, metadata)

**Рекомендация:**
Сделать патч 0.7.7 с:
- Заменой unwrap → expect
- Улучшенным логированием fuzzy search
- Мелкими фиксами

Затем запланировать 0.8.0 с реализацией Condition nodes.

