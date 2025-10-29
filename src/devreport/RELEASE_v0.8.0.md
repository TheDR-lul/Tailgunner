# 🎉 Tailgunner v0.8.0 - Pattern Editor Overhaul

**Release Date:** October 25, 2025  
**Type:** Major Feature Update

---

## 🚀 **MAJOR CHANGES**

### ✅ **Pattern Editor Complete Redesign**

**1. InputNode Simplification**
- ❌ **REMOVED:** Operator and value from InputNode
- ✅ **NEW:** InputNode now ONLY selects telemetry indicator (speed, altitude, G-load, etc)
- ✅ All conditions are now handled by dedicated ConditionNode

**2. ConditionNode - Single Threshold**
- ✅ Operators: `>`, `<`, `=`, `>=`, `<=`
- ✅ Visual feedback with color-coded operators
- ✅ Live value preview

**3. MultiConditionNode - Complex Logic**
- ✅ Multiple thresholds with AND/OR logic
- ✅ Add/remove conditions dynamically
- ✅ Visual grouping

**4. OutputNode - Device Routing**
- ✅ Device mode selection: All / By Type / Specific
- ✅ Filter by device type: Vibrators / Linear / Rotators
- ✅ Runtime device list support

**5. EventNode Enhancements**
- ✅ Chat message filtering by text
- ✅ Player/Clan filtering for kills, achievements
- ✅ No more dummy conditions - uses `AlwaysTrue`

---

## 🔧 **BACKEND IMPROVEMENTS**

### Pattern Parsing
```
INPUT → CONDITION → OUTPUT ✅
INPUT → MULTI → OUTPUT ✅
INPUT → LOGIC → OUTPUT ✅
EVENT → OUTPUT ✅
```

- ✅ `parse_input_node()` - Returns base indicator condition
- ✅ `apply_condition_to_input()` - Applies operator/value from ConditionNode
- ✅ `parse_multicondition_node()` - Handles AND/OR logic
- ✅ `TriggerCondition::AlwaysTrue` - For event-based patterns

### Wiki Data Persistence
- ✅ `update_aircraft_wiki_data()` in database.rs
- ✅ Automatic saving of Wiki-scraped data (G-load, flap speeds, gear speeds)
- ✅ `data_source` field tracks: "datamine", "wiki", "datamine+wiki"

---

## 🎨 **UI/UX POLISH**

- ✅ Removed non-functional nodes (old ConditionNode variants)
- ✅ Cleaned up node palette
- ✅ Improved node visual consistency
- ✅ Better debug logging for pattern parsing

---

## 🐛 **BUG FIXES**

- ✅ Fixed EventNode using dummy `IASAbove(0.0)` condition
- ✅ Fixed InputNode storing unused operator/value fields
- ✅ Fixed Wiki data not being saved to database
- ✅ Removed compilation warnings for unused imports

---

## 📦 **BUILD OPTIMIZATIONS**

- ✅ Cleaned up debug build artifacts
- ✅ Removed temporary build files
- ✅ Optimized release build size

---

## 🔮 **FUTURE ENHANCEMENTS**

### OutputNode Device Filtering
- 🚧 Basic parsing implemented
- 🚧 Full device filtering requires DeviceManager integration
- 🚧 Planned for v0.8.x

---

## 📋 **TECHNICAL DETAILS**

### Added Files
- None (all changes to existing files)

### Modified Files
- `src/components/nodes/InputNode.tsx` - Simplified to indicator-only
- `src/components/nodes/ConditionNode.tsx` - Restored & enhanced
- `src/components/nodes/MultiConditionNode.tsx` - Restored & enhanced
- `src/components/nodes/OutputNode.tsx` - Restored & enhanced
- `src/components/nodes/EventNode.tsx` - Added text/player filtering
- `src/components/PatternEditorModal.tsx` - Updated node palette
- `src-tauri/src/ui_patterns.rs` - Complete rewrite of pattern parsing
- `src-tauri/src/event_triggers.rs` - Added `AlwaysTrue` condition
- `src-tauri/src/datamine/database.rs` - Added `update_aircraft_wiki_data()`
- `src-tauri/src/lib.rs` - Wiki data persistence
- `package.json` - Version bump to 0.8.0
- `src-tauri/Cargo.toml` - Version bump to 0.8.0
- `src-tauri/tauri.conf.json` - Version bump to 0.8.0

### Database Schema Changes
- None (existing schema supports new features)

---

## 🎯 **MIGRATION GUIDE**

### For Users
1. **Existing patterns may break!** 
   - Old InputNode patterns with embedded conditions will no longer work
   - Recreate patterns using new ConditionNode workflow

2. **New workflow:**
   ```
   INPUT (indicator) → CONDITION (operator + value) → VIBRATION
   ```

3. **Event patterns:**
   ```
   EVENT (with filters) → VIBRATION
   ```

### For Developers
- `TriggerCondition::AlwaysTrue` replaces dummy conditions
- `parse_input_node()` returns placeholder condition (will be overwritten by ConditionNode)
- `apply_condition_to_input()` maps operator to correct condition variant

---

## 🙏 **CONTRIBUTORS**

- **AI Assistant**: Pattern Editor redesign, backend refactoring
- **User (@wingsofprey)**: QA, testing, feature requests

---

## 📝 **CHANGELOG SUMMARY**

```
v0.8.0 (2025-10-25)
├─ Pattern Editor: Complete node architecture redesign
├─ Backend: Robust pattern parsing with proper condition handling
├─ Database: Wiki data persistence for aircraft
├─ UI: Cleaned up non-functional elements
└─ Build: Optimized artifacts cleanup
```

---

**Full commit history:** [See Git log](https://github.com/your-repo/tailgunner)

---

🎮 **Ready for production testing!**

