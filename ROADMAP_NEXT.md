# Butt Thunder - ROADMAP: Следующие шаги

## 📅 Текущая версия: v0.6.0

## ✅ ЧТО УЖЕ РАБОТАЕТ (v0.6.0):

### Встроенные триггеры
- ✅ 10 готовых триггеров с описаниями
- ✅ UI для включения/выключения триггеров
- ✅ Визуальная индикация активных триггеров
- ✅ Подробное логирование (`RUST_LOG=debug`)
- ✅ Cooldown система для предотвращения спама

### UI/UX
- ✅ Редактор паттернов (React Flow)
- ✅ InputNode (выбор показателя, условие, порог)
- ✅ VibrationNode (график интенсивности, режимы)
- ✅ Debug Console с логами
- ✅ Game Status (показатели в реальном времени)
- ✅ Переключатель языков (RU/EN)
- ✅ Сохранение паттернов в localStorage

### Интеграции
- ✅ War Thunder API (`http://localhost:8111/state`)
- ✅ Intiface Central (автоподключение)
- ✅ Парсинг всех показателей игры
- ✅ Rust логирование

---

## 🚧 ЧТО **НЕ РАБОТАЕТ** (КРИТИЧНО!):

### ❌ Паттерны из UI **НЕ** интегрированы с Rust движком!

**Проблема:**
- Ты создаёшь паттерн в UI редакторе (React Flow)
- Сохраняется в `localStorage`
- НО Rust движок про него **НЕ ЗНАЕТ**!
- Движок использует **ТОЛЬКО встроенные триггеры** из `event_triggers.rs`

**Что происходит сейчас:**

```
┌─────────────────────────────────────────┐
│ UI (React)                              │
│ ┌─────────────────────────────────────┐ │
│ │ [InputNode: Speed > 600]            │ │
│ │         ↓                           │ │
│ │ [VibrationNode: Curve, 1s]          │ │
│ └─────────────────────────────────────┘ │
│         │                               │
│         ↓ localStorage                  │
│    (НЕ ПЕРЕДАЁТСЯ В RUST!)             │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ Rust (HapticEngine)                     │
│ ┌─────────────────────────────────────┐ │
│ │ TriggerManager                      │ │
│ │ - Overspeed (800 км/ч)              │ │
│ │ - OverG (10G)                       │ │
│ │ - ...                               │ │
│ │ (ИСПОЛЬЗУЕТ ТОЛЬКО ЭТИ!)            │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

**Нужно:**

```
┌─────────────────────────────────────────┐
│ UI (React)                              │
│ ┌─────────────────────────────────────┐ │
│ │ Pattern: "My Custom Pattern"        │ │
│ │ [InputNode: Speed > 600]            │ │
│ │         ↓                           │ │
│ │ [VibrationNode: Curve, 1s]          │ │
│ └─────────────────────────────────────┘ │
│         │                               │
│         ↓ invoke('add_pattern', {...})  │
│    (ПЕРЕДАТЬ В RUST!)                  │
└─────────────────────────────────────────┘
                ↓
┌─────────────────────────────────────────┐
│ Rust (HapticEngine)                     │
│ ┌─────────────────────────────────────┐ │
│ │ TriggerManager                      │ │
│ │ - Overspeed (встроенный)            │ │
│ │ - OverG (встроенный)                │ │
│ │ - My Custom Pattern (из UI!)        │ │
│ │   Condition: Speed > 600            │ │
│ │   Pattern: Curve[0.4,0.6,0.8], 1s   │ │
│ └─────────────────────────────────────┘ │
│         │                               │
│         ↓ check_triggers()              │
│    (ТРИГГЕРЫ СРАБАТЫВАЮТ!)             │
└─────────────────────────────────────────┘
```

---

## 🎯 ROADMAP v0.7.0: Интеграция UI паттернов с Rust

### Задача #1: Rust структуры для UI паттернов

**Цель:** Создать структуры для хранения паттернов из UI в Rust.

**Файлы:** `src-tauri/src/ui_patterns.rs` (НОВЫЙ!)

```rust
// src-tauri/src/ui_patterns.rs

use serde::{Deserialize, Serialize};
use crate::event_triggers::{TriggerCondition, GameEvent};

/// Паттерн созданный в UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPattern {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub nodes: Vec<UINode>,
    pub edges: Vec<UIEdge>,
}

/// Нода из React Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UINode {
    pub id: String,
    pub type_: String, // "input" | "vibration"
    pub data: serde_json::Value,
}

/// Связь между нодами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIEdge {
    pub source: String,
    pub target: String,
}

impl UIPattern {
    /// Конвертировать в EventTrigger для движка
    pub fn to_trigger(&self) -> Option<EventTrigger> {
        // TODO: Парсинг nodes/edges → TriggerCondition
        // TODO: Создание VibrationPattern из VibrationNode
        None
    }
}
```

---

### Задача #2: Tauri команды для паттернов

**Цель:** Создать команды для добавления/удаления/переключения паттернов из UI.

**Файлы:** `src-tauri/src/lib.rs`

```rust
// src-tauri/src/lib.rs

use ui_patterns::UIPattern;

#[tauri::command]
async fn add_pattern(
    state: tauri::State<'_, AppState>,
    pattern: UIPattern
) -> Result<String, String> {
    let engine = state.engine.lock().await;
    let mut manager = engine.trigger_manager.write().await;
    
    // Конвертируем UIPattern → EventTrigger
    if let Some(trigger) = pattern.to_trigger() {
        manager.add_trigger(trigger);
        log::info!("[UI Patterns] Added: {}", pattern.name);
        Ok(format!("Pattern '{}' added", pattern.name))
    } else {
        Err("Failed to convert pattern".to_string())
    }
}

#[tauri::command]
async fn remove_pattern(
    state: tauri::State<'_, AppState>,
    id: String
) -> Result<String, String> {
    // TODO
    Ok("Pattern removed".to_string())
}

#[tauri::command]
async fn toggle_pattern(
    state: tauri::State<'_, AppState>,
    id: String,
    enabled: bool
) -> Result<String, String> {
    // Используем toggle_trigger - он уже работает!
    toggle_trigger(state, id, enabled).await
}

// Добавить в invoke_handler:
.invoke_handler(tauri::generate_handler![
    // ... existing ...
    add_pattern,
    remove_pattern,
    toggle_pattern,
])
```

---

### Задача #3: Конвертация React Flow → Rust

**Цель:** Парсинг nodes/edges из React Flow в Rust структуры.

**Алгоритм:**

1. **Найти InputNode:**
   - Парсим `data.indicator` → получаем показатель (Speed, Altitude, etc.)
   - Парсим `data.operator` → получаем оператор (`>`, `<`, etc.)
   - Парсим `data.value` → получаем порог
   - Создаём `TriggerCondition::SpeedAbove(600.0)`

2. **Найти VibrationNode:**
   - Парсим `data.curve` → получаем точки кривой
   - Парсим `data.duration` → получаем длительность
   - Парсим `data.mode` → получаем режим (once/continuous/repeat)
   - Создаём `VibrationPattern { attack, hold, decay, release }`

3. **Проверить edges:**
   - Убедиться что InputNode → VibrationNode
   - Если есть несколько InputNode → AND/OR логика

**Файлы:** `src-tauri/src/ui_patterns.rs`

```rust
impl UIPattern {
    pub fn to_trigger(&self) -> Option<EventTrigger> {
        // 1. Найти InputNode
        let input_node = self.nodes.iter()
            .find(|n| n.type_ == "input")?;
        
        // 2. Парсинг условия
        let indicator = input_node.data["indicator"].as_str()?;
        let operator = input_node.data["operator"].as_str()?;
        let value = input_node.data["value"].as_f64()? as f32;
        
        let condition = match (indicator, operator) {
            ("speed", ">") => TriggerCondition::SpeedAbove(value),
            ("speed", "<") => TriggerCondition::SpeedBelow(value),
            ("altitude", ">") => TriggerCondition::AltitudeAbove(value),
            // TODO: все остальные показатели
            _ => return None,
        };
        
        // 3. Найти VibrationNode
        let vibration_node = self.nodes.iter()
            .find(|n| n.type_ == "vibration")?;
        
        // 4. Парсинг паттерна
        let duration = vibration_node.data["duration"].as_f64()? as u64;
        let curve = vibration_node.data["curve"].as_array()?;
        
        // Простейший вариант - берем первую точку как attack
        let intensity = curve.first()?["y"].as_f64()? as f32;
        
        let pattern = VibrationPattern {
            attack: 200,
            hold: duration,
            decay: 300,
            release: 100,
            intensity,
        };
        
        // 5. Создать EventTrigger
        Some(EventTrigger {
            id: self.id.clone(),
            name: self.name.clone(),
            description: format!("Custom pattern: {}", self.name),
            condition,
            event: GameEvent::Custom(pattern),
            cooldown_ms: 1000,
            enabled: self.enabled,
            is_builtin: false,
        })
    }
}
```

---

### Задача #4: Синхронизация UI ↔ Rust

**Цель:** При сохранении паттерна в UI → вызывать Rust команду.

**Файлы:** `src/hooks/usePatterns.ts`

```typescript
// src/hooks/usePatterns.ts

export function usePatterns() {
  const savePattern = async (pattern: Pattern) => {
    // 1. Сохранить в localStorage (как сейчас)
    localStorage.setItem(`pattern_${pattern.id}`, JSON.stringify(pattern));
    
    // 2. НОВОЕ! Отправить в Rust
    try {
      await invoke('add_pattern', { pattern });
      console.log(`✅ Pattern '${pattern.name}' synced to Rust`);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `Паттерн '${pattern.name}' синхронизирован`);
      }
    } catch (error) {
      console.error('Failed to sync pattern:', error);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `Ошибка синхронизации: ${error}`);
      }
    }
  };
  
  return { savePattern, ... };
}
```

---

### Задача #5: Тестирование

**Цель:** Убедиться что паттерны из UI реально срабатывают.

**Шаги:**

1. Создать паттерн в UI: `Speed > 600 → Vibration (0.8 intensity, 1s)`
2. Сохранить → видеть в консоли: `✅ Pattern 'Test' synced to Rust`
3. Запустить War Thunder
4. Разогнаться >600 км/ч
5. Увидеть в логах:
   ```
   [Triggers] ✅ TRIGGERED: Test -> Custom
   [Pattern] Executing custom pattern: 1s, 0.8 intensity
   ```
6. Почувствовать вибрацию на устройстве!

---

## 📋 ЧЕКЛИСТ ДЛЯ v0.7.0:

- [ ] Создать `src-tauri/src/ui_patterns.rs`
- [ ] Добавить структуры `UIPattern`, `UINode`, `UIEdge`
- [ ] Реализовать `UIPattern::to_trigger()`
- [ ] Добавить Tauri команды `add_pattern`, `remove_pattern`, `toggle_pattern`
- [ ] Обновить `invoke_handler` в `lib.rs`
- [ ] Обновить `usePatterns.ts` - вызывать `add_pattern` при сохранении
- [ ] Протестировать: создать паттерн в UI → увидеть в Rust логах
- [ ] Протестировать: паттерн срабатывает в War Thunder
- [ ] Добавить UI индикацию синхронизации (иконка "облако")
- [ ] Обновить документацию

---

## 🎮 КАК ТЕСТИРОВАТЬ СЕЙЧАС (v0.6.0):

### Встроенные триггеры:

1. Запусти:
   ```bash
   RUST_LOG=debug npm run tauri dev
   ```

2. Включи триггер "Превышение 800 км/ч" в UI

3. Зайди в War Thunder, разгонись >800 км/ч

4. Увидишь в терминале:
   ```
   [WT Parser] IAS=825, TAS=830, H=100, Fuel=6100/9560
   [Triggers] Превышение 800 км/ч - Result: true
   [Triggers] ✅ TRIGGERED: Превышение 800 км/ч -> Overspeed
   ```

---

## 🔮 БУДУЩЕЕ (v0.8.0+):

- [ ] Сохранение состояния триггеров (не сбрасываются при перезапуске)
- [ ] Профили для триггеров (танк/самолёт)
- [ ] Экспорт/импорт паттернов
- [ ] Marketplace для шаринга паттернов
- [ ] Динамические триггеры (адаптация к максимальной скорости техники)
- [ ] Интеграция с WT Vehicles API (автоматические пороги для каждого самолёта)

---

**Текущая версия:** v0.6.0  
**Следующая версия:** v0.7.0 (UI → Rust интеграция)  
**Приоритет:** 🔥 КРИТИЧНО - без этого кастомные паттерны не работают!


