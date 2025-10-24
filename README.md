# 🎯 Tailgunner
## powered by A.S.S. (Adaptive Sensory System)

**Transform your War Thunder experience into a full-body immersion system!**

Tailgunner is a revolutionary haptic feedback application that bridges War Thunder's telemetry API with advanced haptic devices, translating every game event into physical sensations. Feel every hit, every G-force, every critical moment through adaptive sensory feedback.

---

## 🚀 Key Features

### 🎮 **Full War Thunder Integration**
- ✅ **100% EAC-Safe** - Reads data only from `127.0.0.1:8111` (official API)
- ✅ **Real-time telemetry** - 10 updates per second
- ✅ **50+ Game Parameters** including:
  - **Flight**: IAS, TAS, Altitude, Mach, AoA, G-Load
  - **Controls**: Aileron, Elevator, Rudder, Flaps, Gear, Airbrake
  - **Engine**: RPM, Temperature, Oil Temp, Manifold Pressure, Throttle
  - **Weapons**: Ammo count, Cannon ready status
  - **Resources**: Fuel (kg & %), Low fuel warnings
  - **Tank-specific**: Stabilizer, Gear ratio, Cruise control, Driving direction
  - **Crew**: Total/alive crew, Gunner/Driver state
  - **Advanced**: Blisters, Gear lamps, Speed warnings

### 🎨 **Visual Pattern Constructor**
Node-based editor (similar to Blender/Unreal) for creating complex haptic patterns:

#### **Input Nodes:**
- **📊 Sensor Input** - 50+ game parameters grouped by category
- **⚡ Event Trigger** - Game events (Hit, Overspeed, LowFuel, etc.)

#### **Logic Nodes:**
- **🔍 Condition** - Single comparisons (>, <, =, ≥, ≤, ≠, between, outside)
- **🎯 Multi-Condition** - Complex conditions with AND/OR logic
- **⚙️ Logic Gates** - AND, OR, NOT, XOR operations

#### **Output Nodes:**
- **💥 Vibration** - Classic vibration patterns with ADSR curves
- **📏 Linear Motion** - For strokers/thrusters (position control)
- **🔄 Rotation** - For rotary devices (speed & direction)
- **📡 Output** - Send to devices

### 🎯 **Smart Profiles**
Automatic profile switching based on vehicle type:
- **🛡️ Tank RB** - Immersive realism with engine rumble, hit feedback
- **✈️ Aircraft** - G-force simulation, overspeed warnings, stall alerts
- **🎮 Light Background** - Subtle feedback for all vehicle types

### 🔧 **Advanced Device Support**
- **Buttplug.io Integration** - Universal device support via Intiface Central
- **Multiple Device Types:**
  - Vibration (classic haptics)
  - Linear actuators (strokers, thrusters)
  - Rotary devices (rotating toys)
- **Smart QoS** - Rate limiting (5-8 cmd/s) to prevent device overload
- **Fail-Safe** - Smooth fade-out on crash/disconnect

### 🌍 **Multilingual**
- 🇬🇧 English (primary)
- 🇷🇺 Russian
- Easy to add more languages

---

## 📦 Installation

### Prerequisites
1. **War Thunder** with localhost API enabled
2. **Intiface Central** ([Download](https://intiface.com/central/))
3. **Compatible haptic devices**

### Quick Start
```bash
# Clone repository
git clone https://github.com/yourusername/tailgunner.git
cd tailgunner

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build production version
npm run tauri build
```

---

## 🎮 How to Use

### 1. **Connect Devices**
- Launch **Intiface Central**
- Start server (default: `ws://localhost:12345`)
- Connect your haptic devices
- Launch **Tailgunner**
- Click "Initialize Devices"

### 2. **Start War Thunder**
- Enable localhost API in game settings
- Launch any battle (tank, aircraft, ship)
- Tailgunner will auto-detect vehicle type

### 3. **Create Custom Patterns**
- Open **Pattern Manager**
- Click **"Create Pattern"**
- Build your haptic flow:
  ```
  [Speed Sensor] → [Condition: > 800] → [Vibration Pattern] → [Output]
  ```
- Save and enable pattern

### 4. **Configure Triggers**
- Go to **Game Events** tab
- Expand any profile (Tank RB, Aircraft, etc.)
- Enable/disable triggers per event
- Adjust cooldowns and thresholds

---

## 🛠️ Architecture

### **Technology Stack**
- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **UI**: React Flow (node editor)
- **Haptics**: Buttplug.io via WebSocket
- **I18n**: react-i18next

### **Core Systems**

#### **A.S.S. (Adaptive Sensory System)**
The engine that powers Tailgunner:
- **Telemetry Reader** - Polls WT API at 100ms intervals
- **Event Engine** - Detects game events (hits, damage, state changes)
- **Trigger Manager** - Evaluates custom conditions
- **Profile Manager** - Switches haptic profiles based on vehicle
- **Rate Limiter** - QoS to prevent device spam
- **Device Manager** - Communicates with Buttplug.io

#### **Pattern System**
Visual node-based patterns are converted to Rust `EventTrigger` structures:
- **Condition evaluation** - Real-time checks against game state
- **Pattern execution** - ADSR envelopes, curves, continuous modes
- **Multi-device support** - Different patterns for different device types

---

## 🎯 Pattern Examples

### **Example 1: Speed Warning**
```
[IAS Sensor] → [Condition: > 800 km/h] → [Vibration: Pulsing] → [Output]
```
*Triggers when airspeed exceeds 800 km/h*

### **Example 2: Critical G-Load**
```
[G-Load Sensor] → [Multi-Condition: > 8G OR < -3G] → [Vibration: Sharp] → [Output]
```
*Triggers on extreme G-forces*

### **Example 3: Low Fuel + Enemy Hit**
```
[Fuel % Sensor] → [Condition: < 20%] ─┐
[Hit Event] ─────────────────────────┼→ [Logic: AND] → [Vibration: Critical] → [Output]
```
*Intense feedback when hit while low on fuel*

### **Example 4: Rotation on Engine Speed**
```
[RPM Sensor] → [Condition: > 2000] → [Rotate: Continuous] → [Output]
```
*Rotary device spins proportionally to engine RPM*

---

## 📊 Supported Parameters

### **Flight Dynamics**
- IAS, TAS, Altitude, Mach, AoA, G-Load (Ny)

### **Aircraft Controls**
- Aileron, Elevator, Rudder, Flaps, Gear, Airbrake
- Stick/Pedal raw inputs

### **Engine Telemetry**
- RPM (multi-engine support)
- Engine, Oil, Water temps
- Manifold pressure, Throttle position

### **Tank-Specific**
- Stabilizer state, Gear ratio
- Cruise control, Driving direction
- Speed warning indicators

### **Weapons & Resources**
- Ammo count, Cannon ready status
- Fuel (kg & percentage)
- Low fuel/ammo warnings

### **Crew Status**
- Total/Current crew count
- Crew distance, Gunner/Driver state

### **Advanced**
- Blisters (1-4), Gear lamps (up/down/off)
- Roll indicators available

---

## 🔐 Safety & Ethics

### **EAC-Safe Guarantee**
Tailgunner **ONLY** reads from War Thunder's official localhost API (`127.0.0.1:8111`). It does not:
- ❌ Inject code into game memory
- ❌ Modify game files
- ❌ Hook into DirectX/OpenGL
- ❌ Use any anti-cheat bypass techniques

### **Device Safety**
- **Rate Limiting**: Max 8 commands/second to prevent overheating
- **Fail-Safe Mode**: Devices stop smoothly if game crashes
- **Manual Override**: Emergency stop button in UI

---

## 🤝 Contributing

We welcome contributions! Areas of interest:
- **New node types** (e.g., Constrict, Inflate)
- **Additional game parameters**
- **Community pattern library**
- **Translations** (add your language!)
- **Device profiles** (optimize for specific hardware)

### **How to Contribute**
1. Fork repository
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

---

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🎖️ Credits

**Tailgunner** is powered by:
- [Tauri](https://tauri.app/) - Cross-platform framework
- [Buttplug.io](https://buttplug.io/) - Haptic device control
- [React Flow](https://reactflow.dev/) - Node editor
- [War Thunder API](https://github.com/lucasvmx/WarThunder-localhost-documentation) - Telemetry documentation

---

## 💬 Community

- **Discord**: [Join our server](#) *(coming soon)*
- **Reddit**: [r/Tailgunner](#) *(coming soon)*
- **Pattern Library**: [Share your patterns](#) *(coming soon)*

---

## 🚀 Roadmap

### **v0.2.0** *(Current)*
- ✅ 50+ game parameters
- ✅ Multi-condition nodes
- ✅ Linear & Rotate devices
- ✅ Full localization (EN/RU)

### **v0.3.0** *(Planned)*
- 🔄 Lovense direct API support
- 🔄 Community pattern library
- 🔄 AI pattern generator
- 🔄 VR headset integration

### **v1.0.0** *(Future)*
- 🔮 Multi-game support (DCS, IL-2, etc.)
- 🔮 Voice alerts & TTS
- 🔮 Discord Rich Presence
- 🔮 Tournament/Esports mode

---

## ⚠️ Disclaimer

Tailgunner is an **unofficial** third-party application and is **not affiliated with, endorsed by, or sponsored by Gaijin Entertainment**. War Thunder® is a registered trademark of Gaijin Entertainment. Use at your own risk.

---

<div align="center">

**Developed with ❤️ for the War Thunder community**

*Feel the Thunder. Be the Thunder.*

</div>
