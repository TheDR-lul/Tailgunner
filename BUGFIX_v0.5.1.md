# 🐛 Bugfix v0.5.1 - Исправление критических багов UI

## Исправленные проблемы

### 1. ✅ **Выбор индикатора в InputNode**

**Проблема:** В InputNode был только "speed", нельзя было выбрать другие индикаторы.

**Решение:**
```typescript
const INDICATORS = [
  { id: 'speed', label: 'Скорость (IAS)', unit: 'км/ч' },
  { id: 'altitude', label: 'Высота', unit: 'м' },
  { id: 'g_load', label: 'G-перегрузка', unit: 'G' },
  { id: 'aoa', label: 'Угол атаки', unit: '°' },
  { id: 'engine_rpm', label: 'Обороты двигателя', unit: 'RPM' },
  { id: 'engine_temp', label: 'Температура двигателя', unit: '°C' },
  { id: 'fuel', label: 'Топливо', unit: 'кг' },
  { id: 'ammo_count', label: 'Боезапас', unit: 'шт' },
  { id: 'mach', label: 'Число Маха', unit: 'M' },
  { id: 'tas', label: 'TAS', unit: 'км/ч' },
];

<select 
  value={indicator}
  onChange={(e) => setIndicator(e.target.value)}
  className="node-select"
>
  {INDICATORS.map(ind => (
    <option key={ind.id} value={ind.id}>{ind.label}</option>
  ))}
</select>
```

---

### 2. ✅ **Редактирование графика вибрации**

**Проблема:** При клике на canvas вибрации, двигалась вся нода вместо добавления точки.

**Решение:** Добавлен `stopPropagation()` для всех событий:

```typescript
const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
  e.stopPropagation(); // ⭐ Останавливаем всплытие!
  
  const canvas = canvasRef.current;
  if (!canvas) return;
  
  const rect = canvas.getBoundingClientRect();
  const x = (e.clientX - rect.left) / canvas.width;
  const y = 1 - ((e.clientY - rect.top) / canvas.height);
  
  const newCurve = [...curve, { x, y }].sort((a, b) => a.x - b.x);
  setCurve(newCurve);
};

// Также добавлено в JSX
<canvas
  ref={canvasRef}
  width={200}
  height={100}
  onClick={handleCanvasClick}
  onMouseDown={(e) => e.stopPropagation()} // ⭐ И здесь!
  style={{ cursor: 'crosshair' }}
/>
```

**Дополнительно:**
- Добавлена кнопка "Сбросить" для очистки кривой
- Улучшен визуал: hover эффект на canvas

---

### 3. ✅ **Консоль отладки налезает на интерфейс**

**Проблема:** Debug Console с `z-index: 900` перекрывала модальные окна.

**Решение:**
```css
.debug-console {
  z-index: 100; /* Было 900! */
  height: 42px; /* Уменьшена высота */
  box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.3);
}

.debug-console.expanded {
  height: 280px; /* Было 300px */
}
```

---

### 4. ✅ **Масштабирование интерфейса**

**Проблема:** UI не адаптировался к разным разрешениям экрана.

**Решение:** Добавлены responsive media queries:

```css
/* 1440px и меньше */
@media (max-width: 1440px) {
  .main-content {
    grid-template-columns: 320px 1fr;
    gap: 16px;
    padding: 16px;
  }
}

/* 1024px и меньше - планшеты */
@media (max-width: 1024px) {
  .main-content {
    grid-template-columns: 1fr; /* Одна колонка! */
    padding: 12px;
    padding-bottom: 50px;
  }
  
  .main-sidebar,
  .main-body {
    max-height: none; /* Убираем ограничение высоты */
  }
}

/* 768px и меньше - мобильные */
@media (max-width: 768px) {
  .modal-content {
    width: 100vw;
    height: 100vh;
    border-radius: 0;
  }
  
  .modal-toolbar {
    flex-direction: column;
  }
  
  .pattern-name-input {
    width: 100%;
  }
}
```

**Добавлены max-height для скролла:**
```css
.main-sidebar,
.main-body {
  max-height: calc(100vh - 140px);
  overflow-y: auto;
}
```

---

### 5. ✅ **Паника Buttplug (Option::unwrap())**

**Проблема:** Краш при инициализации Buttplug из-за `unwrap()` на `None`.

**Решение:** Добавлена proper error handling:

```rust
pub async fn init_buttplug(&self) -> Result<()> {
    // Проверка, не подключен ли уже
    if self.buttplug_client.read().await.is_some() {
        log::info!("Buttplug client already initialized");
        return Ok(());
    }
    
    let client = ButtplugClient::new("Haptic Feedback System");
    
    // Пробуем с опциями
    match ButtplugInProcessClientConnector::new_with_options(
        "Haptic Feedback System", 
        false
    ) {
        Some(connector) => {
            match client.connect(connector).await {
                Ok(_) => {
                    log::info!("Buttplug client connected successfully");
                    *self.buttplug_client.write().await = Some(client);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Failed to connect: {}", e);
                    Err(anyhow::anyhow!("Connection failed: {}", e))
                }
            }
        }
        None => {
            // Fallback к default
            log::warn!("Fallback to default connector");
            let connector = ButtplugInProcessClientConnector::default();
            match client.connect(connector).await {
                Ok(_) => {
                    log::info!("Connected (fallback)");
                    *self.buttplug_client.write().await = Some(client);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Buttplug not available: {}", e);
                    Err(anyhow::anyhow!("Not available: {}", e))
                }
            }
        }
    }
}
```

---

### 6. ✅ **Дополнительные улучшения**

#### **Custom Scrollbar:**
```css
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--accent);
}
```

#### **Стили для select:**
```css
.node-select {
  padding: 6px 8px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text);
  cursor: pointer;
  transition: all 0.2s;
  width: 100%;
}

.node-select:hover {
  border-color: var(--accent);
}

.node-select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.2);
}
```

#### **StopPropagation на всех инпутах:**
Добавлено `onClick={(e) => e.stopPropagation()}` и `onMouseDown={(e) => e.stopPropagation()}` для:
- Всех `<input>`
- Всех `<select>`
- Всех `<canvas>`
- Всех контейнеров нод

---

## 📊 Изменённые файлы

| Файл | Изменения |
|------|-----------|
| `src/components/nodes/InputNode.tsx` | ✅ Добавлен select с 10 индикаторами |
| `src/components/nodes/VibrationNode.tsx` | ✅ stopPropagation + кнопка "Сбросить" |
| `src/components/nodes/ConditionNode.tsx` | ✅ stopPropagation на всех элементах |
| `src/styles/modal.css` | ✅ Responsive + z-index fix |
| `src/styles/nodes.css` | ✅ Стили для select + улучшения |
| `src/styles/scrollbar.css` | ✅ Новый файл - custom scrollbar |
| `src/App.css` | ✅ Grid layout fix |
| `src/App.tsx` | ✅ Импорт scrollbar.css |
| `src-tauri/src/device_manager.rs` | ✅ Buttplug error handling |

---

## 🎯 Результат

### **До:**
- ❌ Только "speed" в InputNode
- ❌ Нода двигается вместо рисования кривой
- ❌ Консоль налезает на всё
- ❌ UI не масштабируется
- ❌ Краш Buttplug при инициализации

### **После:**
- ✅ 10 индикаторов на выбор (speed, altitude, g_load, etc.)
- ✅ Кривая рисуется без движения ноды
- ✅ Консоль не налезает (z-index: 100)
- ✅ Responsive дизайн (1440px, 1024px, 768px)
- ✅ Graceful handling Buttplug ошибок

---

## 🚀 Тестирование

### **Проверь:**
1. Открой редактор паттернов
2. Добавь InputNode → кликни на select → выбери "G-перегрузка"
3. Добавь VibrationNode → кликни на canvas → добавь точку (нода НЕ должна двигаться!)
4. Сверни/разверни Debug Console → она НЕ должна налезать
5. Измени размер окна → UI должен адаптироваться
6. Инициализируй Intiface → НЕ должно быть panic

---

**Версия:** 0.5.1  
**Дата:** 23 октября 2025  
**Статус:** ✅ Все баги исправлены!

