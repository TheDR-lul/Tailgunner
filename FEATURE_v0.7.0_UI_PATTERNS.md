# Butt Thunder - FEATURE v0.7.0: Пользовательские паттерны

## 📅 Дата: 23 октября 2025

## 🎯 ИНТЕГРАЦИЯ: Паттерны из UI редактора → Rust движок

### ✅ РЕАЛИЗОВАНО:

#### 1. **Rust модуль для UI паттернов** ✨

**Новый файл:** `src-tauri/src/ui_patterns.rs`

**Структуры:**
```rust
pub struct UIPattern {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub nodes: Vec<UINode>,    // Ноды из React Flow
    pub edges: Vec<UIEdge>,    // Связи между нодами
}

pub struct UINode {
    pub id: String,
    pub type_: String,         // "input" | "vibration"
    pub data: serde_json::Value,
}

pub struct UIEdge {
    pub source: String,
    pub target: String,
}
```

**Конвертация:**
```rust
impl UIPattern {
    // UI паттерн → EventTrigger для движка
    pub fn to_trigger(&self) -> Option<EventTrigger> {
        // 1. Парсим InputNode → TriggerCondition
        // 2. Парсим VibrationNode → VibrationPattern
        // 3. Создаём EventTrigger
    }
}
```

---

#### 2. **Tauri команды** 🔗

```rust
// Добавить паттерн в движок
#[tauri::command]
async fn add_pattern(pattern: UIPattern) -> Result<String, String>

// Удалить паттерн из движка
#[tauri::command]
async fn remove_pattern(id: String) -> Result<String, String>

// Включить/выключить паттерн
#[tauri::command]
async fn toggle_pattern(id: String, enabled: bool) -> Result<String, String>
```

**Процесс:**
```
UI создаёт паттерн → invoke('add_pattern') → UIPattern::to_trigger() 
→ TriggerManager.add_trigger() → Паттерн активен в движке!
```

---

#### 3. **React интеграция** ⚛️

**Обновлён:** `src/hooks/PatternsProvider.tsx`

**Изменения:**
```typescript
// При создании паттерна
const addPattern = async (name, nodes, edges) => {
  // 1. Сохранить в localStorage
  const newPattern = { id, name, enabled: true, nodes, edges };
  saveToLocalStorage(newPattern);
  
  // 2. НОВОЕ! Синхронизировать с Rust
  await invoke('add_pattern', { pattern: newPattern });
  
  debugLog('success', '✅ Pattern synced to Rust engine');
};

// При переключении паттерна
const togglePattern = async (id) => {
  // 1. Обновить в localStorage
  updateLocalStorage(id, { enabled: !enabled });
  
  // 2. НОВОЕ! Синхронизировать с Rust
  await invoke('toggle_pattern', { id, enabled: !enabled });
  
  debugLog('success', `✅ Pattern ${enabled ? 'enabled' : 'disabled'}`);
};
```

**Автосинхронизация при загрузке:**
```typescript
useEffect(() => {
  // Загружаем из localStorage
  const savedPatterns = loadFromLocalStorage();
  
  // Синхронизируем все с Rust движком
  savedPatterns.forEach(async (pattern) => {
    await invoke('add_pattern', { pattern });
  });
}, []);
```

---

#### 4. **Парсинг показателей** 📊

**Поддерживаемые показатели:**

| Показатель | Операторы | Описание |
|-----------|-----------|----------|
| `speed` | `>`, `<` | Скорость (км/ч) |
| `altitude` | `>`, `<` | Высота (м) |
| `rpm` | `>` | Обороты двигателя |
| `temperature` | `>` | Температура (°C) |
| `g_load` | `>`, `<` | G-перегрузка |
| `aoa` | `>`, `<` | Угол атаки (°) |
| `ias` | `>` | Приборная скорость (км/ч) |
| `tas` | `>` | Истинная скорость (км/ч) |
| `mach` | `>` | Число Маха |
| `fuel` | `<` | Топливо (%) |
| `ammo` | `<` | Боезапас (%) |

**Пример парсинга:**
```rust
// UI: indicator="speed", operator=">", value=600
// Rust: TriggerCondition::SpeedAbove(600.0)

// UI: indicator="g_load", operator=">", value=8
// Rust: TriggerCondition::GLoadAbove(8.0)
```

---

#### 5. **Парсинг паттернов вибрации** 🎵

**Из UI:**
```javascript
{
  duration: 1.5,  // секунды
  curve: [
    { x: 0.0, y: 0.4 },
    { x: 0.5, y: 0.8 },
    { x: 1.0, y: 0.2 }
  ],
  mode: "once",     // "once" | "continuous" | "repeat"
  repeatCount: 3
}
```

**В Rust:**
```rust
VibrationPattern {
    attack: EnvelopeStage {
        duration_ms: 375,          // 1/4 длительности
        start_intensity: 0.0,
        end_intensity: 0.6,        // Средняя из curve
        curve: Curve::EaseIn,
    },
    hold: EnvelopeStage {
        duration_ms: 750,          // 1/2 длительности
        start_intensity: 0.6,
        end_intensity: 0.6,
        curve: Curve::Linear,
    },
    decay: EnvelopeStage {
        duration_ms: 375,          // 1/4 длительности
        start_intensity: 0.6,
        end_intensity: 0.0,
        curve: Curve::EaseOut,
    },
    burst: BurstConfig {
        repeat_count: 3,           // из repeatCount
        pause_between_ms: 100,
    },
}
```

---

#### 6. **Логирование** 📝

**Rust логи:**
```
[UI Pattern] Converting 'My Custom Pattern' to trigger
[UI Pattern] Found InputNode: {"indicator":"speed","operator":">","value":600}
[UI Pattern] Parsed condition: SpeedAbove(600.0)
[UI Pattern] Found VibrationNode: {"duration":1.5,"curve":[...],"mode":"once"}
[UI Pattern] Parsed pattern: VibrationPattern { ... }
[UI Pattern] ✅ Pattern 'My Custom Pattern' added to engine
```

**UI логи (Debug Console):**
```
✅ Pattern created: My Custom Pattern (total: 3)
✅ Pattern 'My Custom Pattern' synced to Rust engine
```

---

### 🎮 КАК ИСПОЛЬЗОВАТЬ:

#### 1. Создать паттерн в UI:

```
1. Открой редактор паттернов
2. Добавь InputNode:
   - Показатель: Speed
   - Оператор: >
   - Значение: 600
3. Добавь VibrationNode:
   - Длительность: 1.5s
   - График: рисуй кривую интенсивности
   - Режим: Once
4. Соедини ноды
5. Сохрани паттерн (имя: "Overspeed 600")
```

**Что происходит:**
```
React → localStorage → invoke('add_pattern') 
→ UIPattern::to_trigger() → TriggerManager 
→ Триггер добавлен в движок!
```

#### 2. Тестировать паттерн:

```
1. Зайди в War Thunder
2. Разгонись >600 км/ч
3. В терминале увидишь:
   [UI Pattern] Converting 'Overspeed 600' to trigger
   [Triggers] ✅ TRIGGERED: Overspeed 600 -> Custom
   [Pattern] Executing custom pattern: 1500ms
   [Device] 🎮 Отправка вибрации...
4. ПОЧУВСТВУЕШЬ ВИБРАЦИЮ! 🎉
```

#### 3. Управление паттернами:

```typescript
// В UI (PatternManager)
<button onClick={() => togglePattern(pattern.id)}>
  {pattern.enabled ? 'ВКЛ' : 'ВЫКЛ'}
</button>

// Автоматически вызывается:
await invoke('toggle_pattern', { id, enabled: false });

// Rust:
TriggerManager.toggle_trigger(id, false);
```

---

### 📊 СТАТИСТИКА:

**Встроенные триггеры:**
- **Было:** 10 встроенных триггеров (enabled: true по умолчанию)
- **Стало:** 10 встроенных триггеров (enabled: false по умолчанию)

**Пользовательские триггеры:**
- ✅ Создание через UI редактор
- ✅ Автоматическая синхронизация с Rust
- ✅ Конвертация React Flow → EventTrigger
- ✅ Парсинг всех показателей (11 типов)
- ✅ Парсинг паттернов вибрации (ADSR)
- ✅ Поддержка режимов (once/continuous/repeat)

**Технические метрики:**
- Новых файлов: 1 (`ui_patterns.rs`)
- Новых Tauri команд: 3 (`add_pattern`, `remove_pattern`, `toggle_pattern`)
- Поддерживаемых показателей: 11
- Поддерживаемых операторов: 2 (`>`, `<`)
- Строк кода: ~200 (Rust) + ~60 (TypeScript)

---

### 🔧 ТЕХНИЧЕСКИЕ ДЕТАЛИ:

**Архитектура:**
```
┌─────────────────────────────────────────┐
│ UI (React Flow)                         │
│ ┌─────────────────────────────────────┐ │
│ │ [InputNode: Speed > 600]            │ │
│ │         ↓                           │ │
│ │ [VibrationNode: Curve, 1.5s]        │ │
│ └─────────────────────────────────────┘ │
│         ↓ invoke('add_pattern')         │
└─────────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────┐
│ Rust (ui_patterns.rs)                   │
│ ┌─────────────────────────────────────┐ │
│ │ UIPattern::to_trigger()             │ │
│ │ - parse_condition()                 │ │
│ │ - parse_vibration_pattern()         │ │
│ │ → EventTrigger                      │ │
│ └─────────────────────────────────────┘ │
│         ↓                               │
└─────────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────┐
│ HapticEngine                            │
│ ┌─────────────────────────────────────┐ │
│ │ TriggerManager                      │ │
│ │ - встроенные триггеры (10)         │ │
│ │ - пользовательские триггеры (N)    │ │
│ │                                     │ │
│ │ check_triggers() → events           │ │
│ │ execute_pattern() → vibration       │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

**Конвертация данных:**
```
UI Node Data:
{
  indicator: "speed",
  operator: ">",
  value: 600,
}
         ↓ parse_condition()
Rust TriggerCondition:
TriggerCondition::SpeedAbove(600.0)

──────────────────────────────────

UI Vibration Data:
{
  duration: 1.5,
  curve: [{ x: 0.4, y: 0.6 }, ...],
  mode: "once",
}
         ↓ parse_vibration_pattern()
Rust VibrationPattern:
VibrationPattern {
  attack: { duration_ms: 375, intensity: 0.0 → 0.6 },
  hold:   { duration_ms: 750, intensity: 0.6 → 0.6 },
  decay:  { duration_ms: 375, intensity: 0.6 → 0.0 },
  burst:  { repeat_count: 1 },
}
```

---

### 🎯 ИТОГИ:

✅ **Встроенные триггеры отключены по умолчанию**  
✅ **Пользовательские паттерны синхронизируются с Rust**  
✅ **React Flow nodes конвертируются в EventTrigger**  
✅ **Паттерны загружаются при старте**  
✅ **Переключение паттернов работает**  
✅ **Логирование на всех уровнях**  

**Теперь пользовательские паттерны РЕАЛЬНО РАБОТАЮТ!** 🚀

---

## 📋 ОБНОВЛЕНИЯ:

**v0.6.1:**
- ✅ Исправлен баг с отсутствием паттернов в профиле "Легкий Фон"
- ✅ Добавлены паттерны для всех 10 встроенных триггеров

**v0.7.0:**
- ✅ Встроенные триггеры отключены по умолчанию
- ✅ Создан модуль `ui_patterns.rs`
- ✅ Добавлены Tauri команды для UI паттернов
- ✅ Интеграция React Flow → Rust движок
- ✅ Автосинхронизация при загрузке
- ✅ Парсинг всех показателей и паттернов

---

**Версия:** v0.7.0  
**Предыдущая версия:** v0.6.1  
**Тип:** FEATURE - Интеграция UI паттернов  
**Приоритет:** ✅ ЗАВЕРШЕНО


