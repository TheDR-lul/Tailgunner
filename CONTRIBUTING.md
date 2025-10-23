# 🤝 Contributing to Butt Thunder

Спасибо за ваш интерес к проекту! Мы приветствуем вклад от сообщества.

## 📋 Как внести вклад

### 1. Fork и Clone

```bash
# Fork через GitHub UI
git clone https://github.com/YOUR_USERNAME/butt_thunder.git
cd butt_thunder
```

### 2. Установка зависимостей

```bash
# Установка Rust (если еще не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Установка Node.js зависимостей
npm install
```

### 3. Создание ветки

```bash
git checkout -b feature/amazing-feature
```

### 4. Разработка

#### Запуск в dev режиме:

```bash
npm run tauri dev
```

#### Проверка Rust кода:

```bash
cargo check --manifest-path=src-tauri/Cargo.toml
cargo clippy --manifest-path=src-tauri/Cargo.toml
```

#### Проверка TypeScript кода:

```bash
npm run build  # Проверяет TypeScript
```

### 5. Коммит изменений

Используйте понятные commit message:

```bash
git commit -m "feat: Добавлен новый паттерн для критических попаданий"
git commit -m "fix: Исправлена утечка памяти в DeviceManager"
git commit -m "docs: Обновлен README с новыми инструкциями"
```

**Префиксы:**
- `feat:` — новая фича
- `fix:` — исправление бага
- `docs:` — документация
- `refactor:` — рефакторинг
- `test:` — тесты
- `chore:` — обслуживание

### 6. Push и Pull Request

```bash
git push origin feature/amazing-feature
```

Затем создайте Pull Request через GitHub UI.

---

## 🎯 Что можно улучшить

### Приоритетные задачи:

1. **Lovense LAN API интеграция** — прямое подключение без Buttplug
2. **Mock сервер для тестирования** — эмулятор War Thunder API
3. **UI для создания кастомных паттернов** — интерактивный редактор ADSR
4. **Система импорта/экспорта профилей** — JSON формат
5. **Настройки приложения** — выбор порта, частоты опроса и т.д.
6. **Тесты** — unit и integration тесты
7. **i18n** — поддержка английского языка

### Идеи для фич:

- Интеграция с другими играми (DCS, IL-2)
- Поддержка многоканальных устройств (разные зоны вибрации)
- WebSocket API для стримеров (Twitch интеграция)
- Статистика событий (сколько попаданий за сессию)
- Облачные профили (синхронизация между устройствами)

---

## 📝 Стиль кода

### Rust

```rust
// Хорошо
pub struct VibrationPattern {
    pub name: String,
    attack: EnvelopeStage,
}

impl VibrationPattern {
    pub fn new(name: String) -> Self {
        Self {
            name,
            attack: EnvelopeStage::default(),
        }
    }
}

// Плохо
pub struct vibrationPattern {  // snake_case для типов
    Name: String,  // PascalCase для полей
}
```

**Правила:**
- `PascalCase` для типов и трейтов
- `snake_case` для функций и переменных
- Документируйте публичные API с `///`
- Используйте `clippy` для проверки

### TypeScript

```typescript
// Хорошо
export interface DeviceInfo {
  id: number;
  name: string;
  connected: boolean;
}

export async function getDevices(): Promise<DeviceInfo[]> {
  return invoke<DeviceInfo[]>('get_devices');
}

// Плохо
export interface device_info {  // PascalCase для интерфейсов
  ID: number;  // camelCase для полей
}
```

**Правила:**
- `PascalCase` для типов и интерфейсов
- `camelCase` для переменных и функций
- Используйте `async/await` вместо `.then()`
- Типизируйте всё

### React

```tsx
// Хорошо
export function Dashboard() {
  const [isRunning, setIsRunning] = useState(false);
  
  return (
    <div className="card">
      <h3>Dashboard</h3>
    </div>
  );
}

// Плохо
export default function dashboard() {  // PascalCase для компонентов
  const is_running = useState(false);  // camelCase для переменных
  return <div style={{color: 'red'}}></div>;  // используйте CSS классы
}
```

---

## 🧪 Тестирование

### Unit тесты (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiting() {
        let limiter = RateLimiter::new();
        assert!(limiter.try_send());
        assert!(!limiter.try_send());  // Должно быть заблокировано
    }
}
```

Запуск:
```bash
cargo test --manifest-path=src-tauri/Cargo.toml
```

### Интеграционные тесты

TODO: Добавить интеграционные тесты с mock War Thunder API

---

## 🐛 Баг репорты

При создании issue укажите:

1. **Версия приложения**
2. **ОС и версия** (Windows 10/11)
3. **Шаги для воспроизведения**
4. **Ожидаемое поведение**
5. **Фактическое поведение**
6. **Логи** (если есть)

### Пример:

```markdown
**Версия:** 0.1.0
**ОС:** Windows 11 23H2
**Устройство:** Lovense Edge 2

**Шаги:**
1. Запустить приложение
2. Нажать "Инициализировать"
3. Запустить движок

**Ожидается:** Вибрация при попадании
**Фактически:** Вибрация не работает

**Логи:**
[WARN] Failed to send vibration: Timeout
```

---

## 📚 Дополнительные ресурсы

### Документация

- [Tauri](https://tauri.app/)
- [Buttplug.io](https://buttplug.io/docs/)
- [React](https://react.dev/)
- [Rust](https://www.rust-lang.org/learn)

### Полезные ссылки

- [War Thunder LocalHost API](https://localhost.warthunder.com/help)
- [Buttplug Устройства](https://iostindex.com/)
- [Tauri IPC](https://tauri.app/develop/calling-rust/)

---

## ⚖️ Лицензия

Внося вклад в проект, вы соглашаетесь, что ваш код будет лицензирован под MIT License.

---

**Спасибо за ваш вклад! 🎮💜**

