# 🎮 Butt Thunder - Тактильный "Сабвуфер" для War Thunder

**Премиальный, безопасный и глубоко кастомизируемый haptic feedback для War Thunder**

---

## 🚀 Концепция

**Butt Thunder** — это десктопное приложение, которое превращает игровые события War Thunder в сложные тактильные ощущения на вибро-устройствах.

### ✨ Ключевые фичи:

- **🛡️ 100% EAC-Safe** — читает только публичные данные с `localhost:8111`, никакой инъекции в память
- **🎯 75+ игровых событий** — полная интеграция с War Thunder localhost API
- **⚡ Кастомные триггеры** — превышение скорости, G-перегрузки, угол атаки, и многое другое
- **✈️ Аэродинамические события** — Overspeed, OverG, Stall, Mach1, HighAOA
- **🎯 Автоматические профили** — автоопределение типа техники (танк/самолет/вертолет)
- **🎛️ Визуальный редактор паттернов** — ADSR синтезатор для создания уникальных тактильных текстур
- **📦 Готовые пресеты** — расширенные профили с новыми событиями
- **🔌 Широкая совместимость** — поддержка Buttplug.io (Lovense, Kiiroo, We-Vibe и др.)
- **⚡ Rate Limiting & Fail-Safe** — умное управление без перегрузки устройств

---

## 🏗️ Технологии

- **Frontend:** React + TypeScript + Vite
- **Backend:** Rust (Tauri 2)
- **Haptics:** Buttplug.io
- **HTTP Client:** Reqwest
- **Async Runtime:** Tokio

---

## 📋 Требования

### Системные требования:
- **OS:** Windows 10/11 (основная поддержка)
- **War Thunder** с включенным `localhost:8111` API
- **Intiface Desktop** (для Buttplug устройств) — [скачать](https://intiface.com/desktop/)

### Для разработки:
- **Rust** 1.70+ — [установить](https://www.rust-lang.org/)
- **Node.js** 18+ — [установить](https://nodejs.org/)
- **Tauri CLI** — устанавливается автоматически

---

## 🔧 Установка и запуск

### 1. Клонируйте репозиторий

```bash
git clone https://github.com/yourusername/butt_thunder.git
cd butt_thunder
```

### 2. Установите зависимости

```bash
npm install
```

### 3. Запустите в режиме разработки

```bash
npm run tauri dev
```

### 4. Сборка production версии

```bash
npm run tauri build
```

Собранное приложение будет в `src-tauri/target/release/`

---

## 🎮 Как использовать

### Шаг 1: Включите War Thunder API

В War Thunder включите локальный сервер:

1. Откройте настройки игры
2. Включите опцию "Localhost API" или создайте файл конфигурации
3. Перезапустите игру

Проверьте доступность: откройте в браузере `http://localhost:8111/state`

### Шаг 2: Запустите Intiface Desktop

1. Скачайте и установите [Intiface Desktop](https://intiface.com/desktop/)
2. Запустите приложение
3. Нажмите "Start Server"
4. Подключите ваши Bluetooth устройства

### Шаг 3: Запустите Butt Thunder

1. Откройте **Butt Thunder**
2. Нажмите **"Инициализировать"** в разделе "Устройства"
3. Выберите нужный профиль (или используйте автоматический)
4. Нажмите **"Запустить"**
5. Запустите War Thunder и начните бой! 🎯

---

## 🎛️ Структура модулей

### Rust Backend (`src-tauri/src/`)

```
wt_telemetry.rs     → Чтение данных с War Thunder API
pattern_engine.rs   → ADSR синтезатор паттернов
device_manager.rs   → Управление Buttplug устройствами
rate_limiter.rs     → QoS для предотвращения спама
event_engine.rs     → Детектор игровых событий
profile_manager.rs  → Автоматические профили
haptic_engine.rs    → Главный координатор
```

### React Frontend (`src/`)

```
components/
  Dashboard.tsx       → Управление системой
  DeviceList.tsx      → Список подключенных устройств
  ProfileList.tsx     → Профили и пресеты
  PatternEditor.tsx   → Визуальный редактор паттернов
api.ts                → Tauri API wrapper
types.ts              → TypeScript типы
```

---

## 🎨 Визуальный редактор паттернов

**"Синтезатор Ощущений"** — это уникальная фича Butt Thunder.

### ADSR Envelope:

- **Attack** — мгновенный удар (0-100% за X мс)
- **Hold** — удержание интенсивности
- **Decay** — плавное затухание
- **Burst** — количество повторений

### Примеры паттернов:

**Критическое попадание:**
```
Attack:  80ms  (0 → 100%)
Hold:    250ms (100%)
Decay:   400ms (100% → 0%)
Burst:   2x повтора
```

**Работающий двигатель:**
```
Attack:  100ms (0 → 30%)
Hold:    2000ms (30-35% пульсация)
Decay:   150ms
Burst:   0x (непрерывный)
```

---

## 🔒 Безопасность (EAC-Safe)

### Почему это безопасно:

1. **Только HTTP запросы** — приложение использует официальный API War Thunder (`localhost:8111`)
2. **Никакой инъекции** — нет доступа к памяти игры
3. **Нет изменения файлов** — игра остается нетронутой
4. **Публичный API** — Gaijin сами предоставляют эти данные для внешних приложений

⚠️ **Важно:** Мы НЕ можем дать 100% гарантию от бана (это зависит от политики Gaijin), но архитектура приложения спроектирована так, чтобы быть максимально безопасной.

---

## 📦 Готовые профили

### 1. **Танк RB - Иммерсивный реализм** (расширенный)
- Сильные удары от попаданий (Hit, CriticalHit, PenetrationHit)
- Глубокий рокот двигателя (EngineRunning)
- Интенсивная вибрация при пожаре (EngineFire, EngineDestroyed)
- Взрыв боекомплекта (AmmunitionExploded)
- Повреждения гусениц (TrackBroken)

### 2. **Самолет - Универсальный** (10 новых событий!)
- **Аэродинамика:** Overspeed, OverG, HighAOA, CriticalAOA, Mach1
- **Топливо:** LowFuel, CriticalFuel
- **Классика:** Stall, Spin, Hit, CriticalHit

### 3. **Легкий Фон**
- Ненавязчивая вибрация
- Подходит для длительных сессий
- Минимальная интенсивность

## 🎯 Кастомные триггеры (НОВОЕ!)

### Встроенные триггеры:
- **Overspeed** — IAS > 800 км/ч
- **OverG** — G-перегрузка > 10g
- **HighAOA** — Угол атаки > 15°
- **CriticalAOA** — Угол атаки > 20°
- **Mach1** — Mach > 0.98
- **LowFuel** — Топливо < 10%
- **CriticalFuel** — Топливо < 5%
- **LowAltitude** — Высота < 100м (на скорости)
- **EngineOverheat** — Температура > 250°
- **LowAmmo** — Боезапас < 20%

См. [EVENTS_DOCUMENTATION.md](EVENTS_DOCUMENTATION.md) для полного списка 75+ событий!

---

## 🛠️ Разработка

### Структура проекта:

```
butt_thunder/
├── src/              # React frontend
├── src-tauri/        # Rust backend
│   ├── src/          # Исходники
│   └── Cargo.toml    # Зависимости
├── public/           # Статика
└── package.json      # NPM зависимости
```

### Добавление нового события:

1. Добавьте вариант в `GameEvent` enum (`pattern_engine.rs`)
2. Добавьте маппинг в `EventEngine::map_wt_state_to_event()`
3. Создайте паттерн в `VibrationPattern::preset_your_event()`
4. Добавьте в профиль в `ProfileManager::load_default_profiles()`

### Тестирование без War Thunder:

Запустите mock сервер:

```bash
# TODO: Добавить mock server для тестирования
```

---

## 🤝 Contributing

Мы приветствуем вклад в проект!

1. Fork репозиторий
2. Создайте ветку (`git checkout -b feature/AmazingFeature`)
3. Commit изменения (`git commit -m 'Add some AmazingFeature'`)
4. Push в ветку (`git push origin feature/AmazingFeature`)
5. Откройте Pull Request

---

## 📚 Документация

- [README.md](README.md) — основная информация
- [ARCHITECTURE.md](ARCHITECTURE.md) — архитектура системы
- [EVENTS_DOCUMENTATION.md](EVENTS_DOCUMENTATION.md) — **НОВОЕ!** Полный список 75+ событий
- [CONTRIBUTING.md](CONTRIBUTING.md) — гайд для разработчиков
- [CHANGELOG.md](CHANGELOG.md) — история изменений

---

## 🔗 Источники данных

Проект использует официальные источники:
- [War Thunder localhost API docs](https://github.com/lucasvmx/WarThunder-localhost-documentation) ✅
- [WarThunder Vehicles API](https://github.com/Sgambe33/WarThunder-Vehicles-API) 🔄

---

## 📄 Лицензия

MIT License - см. [LICENSE](LICENSE)

---

## ⚠️ Disclaimer

Этот проект не аффилирован с Gaijin Entertainment. Используйте на свой риск.

**War Thunder** — торговая марка Gaijin Entertainment.

---

## 💬 Поддержка

- **Issues:** [GitHub Issues](https://github.com/yourusername/butt_thunder/issues)
- **Discord:** [Присоединиться](#) (TODO)
- **Email:** support@buttthunder.app (TODO)

---

**Сделано с ❤️ для War Thunder комьюнити**

🎮 Играй. Чувствуй. Побеждай.
