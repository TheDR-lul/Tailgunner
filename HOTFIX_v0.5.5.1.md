# Butt Thunder - HOTFIX v0.5.5.1

## 📅 Дата: 23 октября 2025

## 🎯 HOTFIX: Логирование триггеров + Сохранение графика вибрации

### 🐛 Проблемы:

1. **Триггеры не срабатывают** - непонятно почему
2. **График вибрации сбрасывается** при повторном открытии редактора паттерна

---

## ✅ Что исправлено:

### 1. **Подробное логирование триггеров** 🔍

Добавлено детальное логирование в `event_triggers.rs`:

```rust
[Triggers] Checking 8 triggers
[Triggers] Превышение 800 км/ч - Condition: IASAbove(800.0), Result: false
  IASAbove: 392 > 800 = false
[Triggers] G-перегрузка >10G - Condition: Or(...), Result: false
  GLoadAbove: 1.14 > 10 = false
[Triggers] Угол атаки >15° - Condition: AOAAbove(15.0), Result: false
  AOAAbove: 2.9 > 15 = false
[Triggers] ✅ TRIGGERED: Преодоление Mach 1.0 -> Mach1
```

**Теперь видно:**
- Сколько триггеров проверяется
- Какие условия
- Реальные значения показателей
- Результат проверки (true/false)
- Какие триггеры сработали

### 2. **Сохранение настроек графика вибрации** 💾

**БЫЛО:**
```tsx
// При каждом открытии редактора - сброс на defaults
const [curve, setCurve] = useState(data.curve || defaultCurve);
// data.curve игнорировался при повторном открытии!
```

**СТАЛО:**
```tsx
// Синхронизация с data при загрузке
useEffect(() => {
  if (data.curve) setCurve(data.curve);
  if (data.duration !== undefined) setDuration(data.duration);
  if (data.mode) setMode(data.mode);
  ...
}, [data]);

// Обновление data при изменении
useEffect(() => {
  data.curve = curve;
  data.duration = duration;
  data.mode = mode;
  ...
}, [curve, duration, mode, ...]);
```

**Теперь:**
- ✅ График сохраняется при закрытии редактора
- ✅ Настройки восстанавливаются при повторном открытии
- ✅ Режим вибрации (once/continuous/repeat) не сбрасывается
- ✅ Количество повторений сохраняется

### 3. **Подсказка в Debug Console** 💡

Добавлено уведомление в UI:

```
💡 Rust логи (триггеры, парсер) → смотри в терминале!
   Для детальных логов запусти: RUST_LOG=debug npm run tauri dev
```

---

## 📊 Как использовать логи:

### Обычный режим (info логи):
```bash
npm run tauri dev
```

Покажет:
- `[WT] Vehicle: Aircraft, Speed: 392 km/h, Alt: 50m`
- `[Triggers] ✅ TRIGGERED: Превышение 800 км/ч -> Overspeed`
- `[Pattern] Executing pattern for event Overspeed: Attack=200ms, Hold=500ms`

### Детальный режим (debug логи):
```bash
RUST_LOG=debug npm run tauri dev
```

Покажет дополнительно:
- `[WT Parser] IAS=392, TAS=393, H=50, Fuel=6320/9560`
- `[Triggers] Превышение 800 км/ч - Condition: IASAbove(800.0), Result: false`
- `  IASAbove: 392 > 800 = false`

### Максимальный детализированный режим (trace логи):
```bash
RUST_LOG=trace npm run tauri dev
```

Покажет ВСЁ включая:
- `[WT API] /state response: { "valid": true, "H, m": 50, ... }`
- `[Triggers] Checking 8 triggers`
- `[Triggers] Skipping disabled trigger: ...`
- `[Triggers] ... on cooldown (1500/5000ms)`

---

## 🔍 Отладка триггеров:

### Проверь что триггеры работают правильно:

1. **Запусти с debug логами:**
   ```bash
   RUST_LOG=debug npm run tauri dev
   ```

2. **Зайди в War Thunder в бой**

3. **Смотри в терминале логи:**
   ```
   [WT Parser] IAS=392, TAS=393, H=50, Fuel=6320/9560
   [Triggers] Превышение 800 км/ч - Result: false
     IASAbove: 392 > 800 = false  ← Скорость недостаточна!
   ```

4. **Разгонись выше 800 км/ч:**
   ```
   [WT Parser] IAS=825, TAS=830, H=100, Fuel=6100/9560
   [Triggers] ✅ TRIGGERED: Превышение 800 км/ч -> Overspeed
   [Pattern] Executing pattern for event Overspeed
   ```

### Почему триггер не срабатывает?

Смотри на логи:

1. **Значение меньше порога:**
   ```
   IASAbove: 392 > 800 = false
   ```
   → Нужна скорость >800 км/ч

2. **На cooldown:**
   ```
   [Triggers] ... on cooldown (1500/5000ms)
   ```
   → Подожди окончания cooldown

3. **Триггер выключен:**
   ```
   [Triggers] Skipping disabled trigger: ...
   ```
   → Включи триггер в настройках

---

## 📝 Изменённые файлы:

- **`src-tauri/src/event_triggers.rs`**:
  - Добавлено логирование в `check_triggers()`
  - Добавлено детальное логирование в `evaluate_condition()`
  - Выводятся реальные значения показателей и результаты проверки
  
- **`src/components/nodes/VibrationNode.tsx`**:
  - Добавлен `useEffect` для синхронизации с `data` при загрузке
  - Добавлен `useEffect` для обновления `data` при изменении
  - Теперь график сохраняется между открытиями редактора
  
- **`src/components/DebugConsole.tsx`**:
  - Добавлена подсказка о Rust логах в терминале
  - Добавлена команда для запуска с debug логами
  
- **`src-tauri/src/lib.rs`**:
  - Добавлена команда `get_debug_info` (для будущего расширения)

---

## 🎯 Итоги:

✅ **Логирование триггеров** - видно какие проверяются, какие значения, почему не срабатывают  
✅ **График вибрации сохраняется** - настройки не теряются при редактировании  
✅ **Подсказка в UI** - пользователь знает где смотреть Rust логи  
✅ **Отладка упрощена** - можно быстро понять почему триггер не работает  

---

**Версия:** v0.5.5.1  
**Предыдущая версия:** v0.5.5  
**Тип исправления:** HOTFIX  
**Приоритет:** 🔧 СРЕДНИЙ


