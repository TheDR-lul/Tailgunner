# 🤝 Contributing to Tailgunner

Thank you for your interest in the project! We welcome contributions from the community.

## 📋 How to Contribute

### 1. Fork and Clone

```bash
# Fork via GitHub UI
git clone https://github.com/YOUR_USERNAME/tailgunner.git
cd tailgunner
```

### 2. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js dependencies
npm install
```

### 3. Create a Branch

```bash
git checkout -b feature/amazing-feature
```

### 4. Development

#### Run in dev mode:

```bash
npm run tauri dev
```

#### Check Rust code:

```bash
cargo check --manifest-path=src-tauri/Cargo.toml
cargo clippy --manifest-path=src-tauri/Cargo.toml
```

#### Check TypeScript code:

```bash
npm run build  # Validates TypeScript
```

### 5. Commit Changes

Use clear commit messages:

```bash
git commit -m "feat: Add new pattern for critical hits"
git commit -m "fix: Fix memory leak in DeviceManager"
git commit -m "docs: Update README with new instructions"
```

**Prefixes:**
- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation
- `refactor:` — refactoring
- `test:` — tests
- `chore:` — maintenance
- `style:` — code style (no functional changes)

### 6. Push and Pull Request

```bash
git push origin feature/amazing-feature
```

Then create a Pull Request via GitHub UI.

---

## 🎯 What Can Be Improved

### Priority Tasks:

1. **On-screen HUD overlay** — Real-time game state and trigger visualization
2. **Mock server for testing** — War Thunder API emulator
3. **Enhanced curve editor** — Preset curves, copy/paste functionality
4. **Profile import/export system** — JSON format with validation
5. **Application settings** — Port selection, polling frequency, etc.
6. **Tests** — Unit and integration tests
7. **Additional translations** — Support for more languages

### Feature Ideas:

- Integration with other games (DCS, IL-2)
- Multi-channel device support (different vibration zones)
- WebSocket API for streamers (Twitch integration)
- Event statistics (hits per session, G-force records)
- Cloud profiles (sync between devices)
- Voice alerts and TTS announcements
- VR headset haptic integration

---

## 📝 Code Style

### Rust

```rust
// Good
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

// Bad
pub struct vibrationPattern {  // Use PascalCase for types
    Name: String,  // Use snake_case for fields
}
```

**Rules:**
- `PascalCase` for types and traits
- `snake_case` for functions and variables
- Document public APIs with `///`
- Use `clippy` for linting
- **English only** for comments and documentation

### TypeScript

```typescript
// Good
export interface DeviceInfo {
  id: number;
  name: string;
  connected: boolean;
}

export async function getDevices(): Promise<DeviceInfo[]> {
  return invoke<DeviceInfo[]>('get_devices');
}

// Bad
export interface device_info {  // Use PascalCase for interfaces
  ID: number;  // Use camelCase for fields
}
```

**Rules:**
- `PascalCase` for types and interfaces
- `camelCase` for variables and functions
- Use `async/await` instead of `.then()`
- Type everything (avoid `any`)
- **English only** for comments and variable names

### React

```tsx
// Good
export function Dashboard() {
  const [isRunning, setIsRunning] = useState(false);
  
  return (
    <div className="card">
      <h3>Dashboard</h3>
    </div>
  );
}

// Bad
export default function dashboard() {  // Use PascalCase for components
  const is_running = useState(false);  // Use camelCase for variables
  return <div style={{color: 'red'}}></div>;  // Use CSS classes
}
```

**Rules:**
- `PascalCase` for component names
- `camelCase` for props and state
- Prefer CSS classes over inline styles
- Use functional components with hooks
- **English only** for component names and props

---

## 🧪 Testing

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiting() {
        let limiter = RateLimiter::new();
        assert!(limiter.try_send());
        assert!(!limiter.try_send());  // Should be blocked
    }
}
```

Run tests:
```bash
cargo test --manifest-path=src-tauri/Cargo.toml
```

### Integration Tests

TODO: Add integration tests with mock War Thunder API

---

## 🐛 Bug Reports

When creating an issue, include:

1. **Application version**
2. **OS and version** (Windows 10/11, Linux, etc.)
3. **Steps to reproduce**
4. **Expected behavior**
5. **Actual behavior**
6. **Logs** (if available)

### Example:

```markdown
**Version:** 0.7.0
**OS:** Windows 11 23H2
**Device:** Lovense Edge 2

**Steps:**
1. Launch application
2. Click "Initialize Devices"
3. Start engine

**Expected:** Vibration on hit
**Actual:** No vibration

**Logs:**
[WARN] Failed to send vibration: Timeout
```

---

## 🎨 UI/UX Guidelines

When contributing to the frontend:

1. **Maintain consistency** - Follow existing design patterns
2. **Accessibility** - Ensure keyboard navigation works
3. **Responsive design** - Test on different window sizes
4. **Internationalization** - Use `t()` for all user-facing text
5. **Performance** - Avoid unnecessary re-renders

### Translation Guidelines

When adding new UI text:

1. Add keys to both `src/i18n/locales/en.json` and `ru.json`
2. Use descriptive key names: `trigger_settings.cooldown` not `ts.cd`
3. Provide context in comments if meaning is ambiguous
4. Test with language switcher to ensure layout works in both languages

---

## 📚 Additional Resources

### Documentation

- [Tauri](https://tauri.app/)
- [Buttplug.io](https://buttplug.io/docs/)
- [React](https://react.dev/)
- [Rust](https://www.rust-lang.org/learn)

### Useful Links

- [War Thunder LocalHost API](https://localhost.warthunder.com/help)
- [Buttplug Devices](https://iostindex.com/)
- [Tauri IPC](https://tauri.app/develop/calling-rust/)
- [React Flow Docs](https://reactflow.dev/)

---

## 🚀 Project Structure

```
tailgunner/
├── src/                    # Frontend (React + TypeScript)
│   ├── components/         # React components
│   ├── hooks/             # Custom React hooks
│   ├── i18n/              # Translations
│   └── styles/            # CSS files
├── src-tauri/             # Backend (Rust + Tauri)
│   └── src/
│       ├── device_manager.rs    # Buttplug.io integration
│       ├── event_engine.rs      # Game event detection
│       ├── event_triggers.rs    # Trigger evaluation
│       ├── haptic_engine.rs     # Main haptic engine
│       ├── pattern_engine.rs    # Pattern generation
│       ├── profile_manager.rs   # Profile switching
│       ├── ui_patterns.rs       # UI pattern parsing
│       ├── wt_telemetry.rs      # War Thunder API
│       └── lib.rs               # Tauri commands
└── public/                # Static assets
```

---

## ⚖️ License

By contributing to this project, you agree that your code will be licensed under the MIT License.

---

**Thank you for your contribution! 🎮💜**
