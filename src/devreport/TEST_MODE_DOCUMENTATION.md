# 🧪 Test Mode - Документация

## Обзор

**Test Mode** позволяет эмулировать War Thunder API локально для тестирования паттернов и функционала без запуска игры.

---

## ⚙️ Архитектура

### Порты
- **8111** - Реальный War Thunder API (игра)
- **8112** - Эмулятор API (Test Mode)

### Компоненты

```
┌─────────────────────────────────────────────────┐
│                  Приложение                      │
│                                                   │
│  ┌─────────────┐        ┌──────────────────┐   │
│  │   UI (React)│◄──────►│  Haptic Engine   │   │
│  │             │        │                  │   │
│  │  - Test Mode│        │  ┌──────────────┐│   │
│  │    Button   │        │  │ WTTelemetry  ││   │
│  │  - Emulator │        │  │  Reader      ││   │
│  │    Panel    │        │  └──────┬───────┘│   │
│  └──────┬──────┘        └─────────┼────────┘   │
│         │                          │             │
│         │                          │             │
│         ▼                          ▼             │
│  ┌──────────────────────────────────────────┐  │
│  │         Tauri Backend (Rust)              │  │
│  │                                            │  │
│  │  ┌────────────┐      ┌─────────────────┐│  │
│  │  │ API Server │      │   API Emulator  ││  │
│  │  │(Axum HTTP) │      │   State Manager ││  │
│  │  │Port: 8112  │◄────►│                 ││  │
│  │  └────────────┘      └─────────────────┘│  │
│  └──────────────────────────────────────────┘  │
└───────────────────────────────────────────────┘
            │                     │
            ▼                     ▼
    HTTP :8112            HTTP :8111
   (Эмулятор)          (Реальная игра)
```

---

## 🚀 Использование

### 1. Включение Test Mode

```typescript
// Клик на кнопку 🧪 в header
// Или через API Emulator панель

await api.emulatorSetEnabled(true);
```

**Что происходит:**
1. ✅ Запускается HTTP сервер на порту `8112`
2. ✅ Телеметрия переключается с `:8111` → `:8112`
3. ✅ Индикатор в header загорается 🟢
4. ✅ API Emulator панель становится активной

### 2. Настройка параметров

```typescript
// Тип техники
await api.emulatorSetVehicleType('Aircraft'); // Tank | Aircraft | Ship

// Параметры полета
await api.emulatorSetSpeed(550);      // км/ч
await api.emulatorSetAltitude(5000);  // метры
await api.emulatorSetHeading(90);     // градусы

// Боевые параметры
await api.emulatorSetAmmo(100);       // кол-во снарядов
await api.emulatorSetHp(75);          // %
await api.emulatorSetInBattle(true);  // в бою

// Позиция на карте
await api.emulatorSetPosition(0.5, 0.5); // x, y [0..1]
```

### 3. Триггер событий

```typescript
// События
await api.emulatorTriggerEvent('Hit');         // Попадание
await api.emulatorTriggerEvent('Kill');        // Уничтожение
await api.emulatorTriggerEvent('CriticalHit'); // Критический урон
await api.emulatorTriggerEvent('Shooting');    // Стрельба
await api.emulatorTriggerEvent('EngineOverheat'); // Перегрев

// Результат: паттерны активируются как в реальной игре!
```

### 4. Чат сообщения

```typescript
// Отправить сообщение
await api.emulatorSendChat('Test message', 'Team');

// Режимы: 'Team' | 'All' | 'Squad'
// Сообщения появятся в Game Feed компоненте
```

---

## 📡 API Endpoints (Port 8112)

### Telemetry Endpoints

#### `GET /status`
```json
{
  "type": "aircraft",  // tank | aircraft | ship
  "state": "in_battle",
  "valid": true
}
```

#### `GET /indicators`
```json
{
  "valid": true,
  "type": "aircraft",
  "speed": 550,
  "altitude": 5000,
  "throttle": 85,
  "rpm": 2400,
  // ... все indicators параметры
}
```

#### `GET /state`
```json
{
  "valid": true,
  "H, m": 5000,
  "IAS, km/h": 550,
  "TAS, km/h": 620,
  "AoA, deg": 3.5,
  "Ny": 1.2,
  // ... все state параметры
}
```

### Map Endpoints

#### `GET /map_info.json`
```json
{
  "valid": true,
  "grid_size": [24432.0, 24432.0],
  "grid_steps": [2400.0, 2400.0],
  "grid_zero": [-12216.0, 8120.0],
  "map_generation": 3
}
```

#### `GET /map_obj.json`
```json
[
  {
    "type": "ground_model",
    "icon": "Player",
    "color": "#faC81E",
    "x": 0.5,
    "y": 0.5,
    "dx": 0.707,
    "dy": 0.707
  },
  // ... другие объекты
]
```

### Chat Endpoints

#### `GET /gamechat?lastId=0`
```json
[
  {
    "id": 1,
    "time": 120,
    "sender": "TestPlayer",
    "mode": "Team",
    "msg": "Test message",
    "enemy": false
  }
]
```

#### `POST /gamechat/send`
**Request:**
```json
{
  "message": "Hello team!",
  "mode": "Team"
}
```

**Response:**
```json
{
  "id": 2,
  "time": 125,
  "sender": "TestPlayer",
  "mode": "Team",
  "msg": "Hello team!",
  "enemy": false
}
```

### HUD Endpoints

#### `GET /hudmsg?lastEvt=0&lastDmg=0`
```json
{
  "events": [
    {
      "id": 100,
      "time": 60,
      "msg": "Hit"
    }
  ],
  "damage": []
}
```

### Mission Endpoints

#### `GET /mission.json`
```json
{
  "status": "running",
  "objectives": [
    {
      "primary": true,
      "status": "in_progress",
      "text": "Test Objective: Destroy enemy forces"
    }
  ]
}
```

---

## 🔧 Код: Реализация

### Backend (Rust)

#### API Server (`src-tauri/src/api_server.rs`)
```rust
pub async fn start_server(emulator: Arc<APIEmulator>) {
    let app = Router::new()
        .route("/status", get(status_handler))
        .route("/indicators", get(indicators_handler))
        .route("/state", get(state_handler))
        .route("/map_obj.json", get(map_obj_handler))
        .route("/map_info.json", get(map_info_handler))
        .route("/gamechat", get(gamechat_handler))
        .route("/gamechat/send", post(gamechat_send_handler))
        // ... другие endpoints
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8112").await?;
    axum::serve(listener, app).await?;
}
```

#### Telemetry Switching (`src-tauri/src/wt_telemetry.rs`)
```rust
impl WTTelemetryReader {
    pub fn set_emulator_mode(&mut self, enabled: bool) {
        if enabled {
            self.base_url = "http://127.0.0.1:8112".to_string();
            log::info!("🧪 Switched to EMULATOR mode");
        } else {
            self.base_url = "http://127.0.0.1:8111".to_string();
            log::info!("🎮 Switched to REAL mode");
        }
        self.last_state = None;
        self.hud_initialized = false;
    }
}
```

### Frontend (React)

#### UI Component (`src/components/APIEmulator.tsx`)
```tsx
export function APIEmulator() {
  const [state, setState] = useState<EmulatorState>({ ... });
  const [chatMessage, setChatMessage] = useState('');
  
  const toggleEnabled = async () => {
    await api.emulatorSetEnabled(!state.enabled);
  };
  
  const sendChat = async () => {
    await api.emulatorSendChat(chatMessage, chatMode);
    setChatMessage('');
  };
  
  return (
    <div className="api-emulator-container">
      {/* Controls */}
      <button onClick={toggleEnabled}>
        {state.enabled ? 'Disable' : 'Enable'} Test Mode
      </button>
      
      {/* Parameters */}
      <input type="range" onChange={e => setSpeed(+e.target.value)} />
      
      {/* Events */}
      <button onClick={() => triggerEvent('Hit')}>Trigger Hit</button>
      
      {/* Chat */}
      <input value={chatMessage} onChange={e => setChatMessage(e.target.value)} />
      <button onClick={sendChat}>Send</button>
    </div>
  );
}
```

---

## 🎯 Примеры использования

### Тестирование паттернов

```typescript
// 1. Включаем Test Mode
await api.emulatorSetEnabled(true);

// 2. Настраиваем параметры
await api.emulatorSetVehicleType('Aircraft');
await api.emulatorSetSpeed(500);
await api.emulatorSetInBattle(true);

// 3. Триггерим события
await api.emulatorTriggerEvent('Shooting');  // Ожидаем вибрацию
await api.emulatorTriggerEvent('Hit');       // Ожидаем вибрацию
await api.emulatorTriggerEvent('Kill');      // Ожидаем вибрацию

// 4. Проверяем чат
await api.emulatorSendChat('Test kill message', 'Team');
// Должно появиться в Game Feed
```

### Тестирование разных типов техники

```typescript
// Танк
await api.emulatorSetVehicleType('Tank');
await api.emulatorSetSpeed(40);
await api.emulatorTriggerEvent('TankDestroyed');

// Самолет
await api.emulatorSetVehicleType('Aircraft');
await api.emulatorSetSpeed(550);
await api.emulatorSetAltitude(5000);
await api.emulatorTriggerEvent('AircraftDestroyed');

// Корабль
await api.emulatorSetVehicleType('Ship');
await api.emulatorSetSpeed(30);
await api.emulatorTriggerEvent('ShipDestroyed');
```

---

## 🐛 Отладка

### Логи
```bash
# В консоли Rust
[Emulator] 🧪 API server started on http://localhost:8112
[Telemetry] 🧪 Switched to EMULATOR mode (port 8112)
[Emulator] Triggered event: Hit
[Emulator] Chat sent: Test message
```

### Проверка сервера
```bash
# Проверить что сервер работает
curl http://localhost:8112/

# Проверить indicators
curl http://localhost:8112/indicators

# Проверить state
curl http://localhost:8112/state

# Отправить чат
curl -X POST http://localhost:8112/gamechat/send \
  -H "Content-Type: application/json" \
  -d '{"message":"Test","mode":"Team"}'
```

---

## ⚠️ Важные моменты

### 1. Порты не конфликтуют
- **8111** - только реальная игра
- **8112** - только эмулятор
- Можно запустить оба одновременно

### 2. Автоматическое переключение
- При включении Test Mode → телеметрия переключается на `:8112`
- При выключении → возвращается на `:8111`

### 3. Сброс состояния
- При переключении режима сбрасывается `last_state`
- HUD события начинаются заново
- Это предотвращает "старые" события

### 4. Синхронизация UI
- Кнопка в header ↔ панель внизу синхронизированы
- Используют один источник состояния (`api.emulatorGetState()`)
- Polling каждую секунду

---

## 📝 TODO / Future

- [ ] Добавить больше параметров из дампов
- [ ] Улучшить генерацию карты (реальные изображения)
- [ ] Добавить `/damage` endpoint с данными
- [ ] Graceful shutdown для Axum сервера
- [ ] Сохранение состояния эмулятора между сессиями
- [ ] Preset'ы для разных сценариев
- [ ] Record/Replay режим событий

---

## 📊 Статистика

- **Endpoints:** 12
- **Emulated Parameters:** ~50+
- **Event Types:** 15+
- **Vehicle Types:** 3 (Tank, Aircraft, Ship)
- **Chat Modes:** 3 (Team, All, Squad)

---

**Создано:** 2025-10-26  
**Версия:** 0.8.1  
**Порт:** 8112 (Emulator) | 8111 (Real)

