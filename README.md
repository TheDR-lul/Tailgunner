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

### 🎯 **Smart Profiles & Dynamic Triggers**
Automatic profile switching based on vehicle type:
- **🛡️ Tank RB** - Immersive realism with engine rumble, hit feedback
- **✈️ Aircraft** - G-force simulation, overspeed warnings, stall alerts
- **🎮 Light Background** - Subtle feedback for all vehicle types

**Advanced Trigger System:**
- **Custom Curve Editor** - Draw your own vibration intensity curves
- **Per-Trigger Settings** - Adjust cooldown, duration, and intensity individually
- **Built-in Triggers** - Pre-configured for fuel warnings, engine damage, ammo alerts
- **Profile Filtering** - Only shows events with active triggers for clarity

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

### 🎨 **Enhanced UI/UX**
- **Interactive Curve Editor** - Draw custom vibration patterns with mouse
  - Add/remove points with double-click
  - Drag points to adjust intensity over time
  - Real-time visual feedback
  - Canvas-based rendering for smooth curves
- **Smart Number Inputs** - Keyboard-friendly value editing
  - Type values directly instead of only using arrows
  - Backspace/Delete support
  - Enter to confirm, Escape to cancel
  - Auto-formatting with units (seconds, etc.)
- **Filtered Views** - Cleaner interface
  - Only shows triggers with configured events
  - Hides empty profiles for clarity

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
- **All Triggers** - View and configure all available triggers
  - Enable/disable individual triggers
  - Adjust cooldown (seconds between activations)
  - Customize vibration duration
  - Draw custom intensity curves with the interactive editor
- **Active Profiles** - Configure profile-specific event mappings
  - Only shows events with configured triggers
  - Per-profile customization for different vehicle types
  - Real-time preview of trigger settings

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
- **Trigger Manager** - Evaluates custom conditions with persistent settings
  - Custom curve point storage and serialization
  - Per-trigger cooldown and pattern management
  - Real-time condition evaluation with state history
- **Profile Manager** - Switches haptic profiles based on vehicle
- **Pattern Engine** - Converts curves to ADSR envelopes
  - Custom curve-to-pattern conversion
  - ADSR (Attack, Decay, Sustain, Release) generation
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

### **Example 5: Custom Vibration Curve for Low Fuel**
```
Trigger: Low Fuel (<10%)
Cooldown: 30.0s
Duration: 2.5s
Curve: Custom drawn curve (gentle ramp up → sustained pulse → fade out)
```
*Creates a unique sensation using the interactive curve editor*

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
- **Rate Limiting**: Max 8 commands/second to prevent overheating, XD
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

**Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)**

This project is **free and open source** for personal, educational, and non-commercial use.

✅ **You CAN:**
- Use the software for personal gaming and entertainment
- Modify and customize for your own use
- Share modifications with the community
- Use in educational settings
- Contribute improvements back to the project

❌ **You CANNOT:**
- Use in commercial products or services
- Sell the software or derivative works
- Offer as a paid service
- Use to generate revenue in any form

See [LICENSE](LICENSE) for full details.

For commercial licensing inquiries, please contact the maintainers.

---

## 🎖️ Credits

**Tailgunner** is powered by:
- [Tauri](https://tauri.app/) - Cross-platform framework
- [Buttplug.io](https://buttplug.io/) - Haptic device control
- [React Flow](https://reactflow.dev/) - Node editor
- [War Thunder API](https://github.com/lucasvmx/WarThunder-localhost-documentation) - Telemetry documentation (Temp)

---

## 💬 Community

- **Reddit**:*(coming soon)*
- **Pattern Library**: [Share your patterns](#) *(coming soon)*

---

## 🚀 Roadmap

### **v0.7.0** *(Current)*
- ✅ 50+ game parameters
- ✅ Multi-condition nodes
- ✅ Linear & Rotate devices
- ✅ Full localization (EN/RU)
- ✅ Interactive vibration curve editor
- ✅ Per-trigger cooldown/duration/intensity settings
- ✅ Persistent trigger configuration
- ✅ Profile filtering (shows only events with triggers)
- ✅ Editable number inputs with keyboard support

### **v0.8.0** *(Planned)*
- 🔄 On-screen HUD overlay
  - Real-time game state visualization
  - Active trigger indicators
  - Device status display
  - Customizable position and opacity
- 🔄 Community pattern library
- 🔄 Enhanced curve editor with presets
- 🔄 Trigger import/export functionality
- 🔄 WT wiki parcer

### **v1.0.0** *(Future)*
- TODO

---

## ⚠️ Disclaimer

Tailgunner is an **unofficial** third-party application and is **not affiliated with, endorsed by, or sponsored by Gaijin Entertainment**. War Thunder® is a registered trademark of Gaijin Entertainment. Use at your own risk.

---

<div align="center">

**Developed with ❤️ for the War Thunder community**

*Feel the Thunder. Be the Thunder.*

</div>
