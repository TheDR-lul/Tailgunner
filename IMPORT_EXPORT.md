# Import/Export System

## Overview

System for importing and exporting custom vibration patterns with native file dialogs.

---

## Export

### Before (Browser-style) ❌
```typescript
// Saved to Downloads folder automatically
const blob = new Blob([JSON.stringify(config, null, 2)]);
const a = document.createElement('a');
a.download = 'pattern.json';
a.click();
```

**Problem:** No control over save location, files pile up in Downloads.

### After (Native Dialog) ✅
```typescript
const { save } = await import('@tauri-apps/plugin-dialog');
const filePath = await save({
  title: 'Export Pattern',
  defaultPath: `${patternName}.json`,
  filters: [{ name: 'Pattern File', extensions: ['json'] }]
});

if (filePath) {
  const { writeTextFile } = await import('@tauri-apps/plugin-fs');
  await writeTextFile(filePath, jsonContent);
}
```

**Benefits:**
- ✅ User chooses save location
- ✅ Native OS dialog (Windows/macOS/Linux)
- ✅ File filters show only .json
- ✅ Suggested filename
- ✅ Debug console feedback

---

## Import

### Before (Browser-style) ❌
```typescript
// Hidden file input
const input = document.createElement('input');
input.type = 'file';
input.accept = '.json';
input.click();
// FileReader API to read content
```

**Problem:** Non-native dialog, extra DOM manipulation.

### After (Native Dialog) ✅
```typescript
const { open } = await import('@tauri-apps/plugin-dialog');
const filePath = await open({
  title: 'Import Pattern',
  filters: [{ name: 'Pattern File', extensions: ['json'] }],
  multiple: false
});

if (filePath) {
  const { readTextFile } = await import('@tauri-apps/plugin-fs');
  const content = await readTextFile(filePath);
  const config = JSON.parse(content);
  // Load pattern...
}
```

**Benefits:**
- ✅ Native OS dialog
- ✅ File filters
- ✅ Direct file system access (faster)
- ✅ Debug console feedback
- ✅ Better error handling

---

## File Format

### Pattern JSON Structure
```json
{
  "name": "My Custom Pattern",
  "nodes": [
    {
      "id": "input-1",
      "type": "input",
      "position": { "x": 100, "y": 200 },
      "data": {
        "indicator": "speed",
        "operator": ">",
        "value": 500,
        "window_seconds": 1.0
      }
    },
    {
      "id": "vibration-1",
      "type": "vibration",
      "position": { "x": 400, "y": 200 },
      "data": {
        "duration": 1.0,
        "points": [
          { "time": 0.0, "intensity": 0.0 },
          { "time": 0.5, "intensity": 1.0 },
          { "time": 1.0, "intensity": 0.0 }
        ]
      }
    },
    {
      "id": "output-1",
      "type": "output",
      "position": { "x": 700, "y": 200 },
      "data": {}
    }
  ],
  "edges": [
    {
      "id": "edge-1",
      "source": "input-1",
      "sourceHandle": "value",
      "target": "vibration-1",
      "targetHandle": "trigger"
    },
    {
      "id": "edge-2",
      "source": "vibration-1",
      "sourceHandle": "output",
      "target": "output-1",
      "targetHandle": "input"
    }
  ]
}
```

---

## Required Plugins

### Cargo.toml
```toml
[dependencies]
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
```

### lib.rs
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    // ...
```

### capabilities/default.json
```json
{
  "permissions": [
    "dialog:default",
    "fs:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file"
  ]
}
```

---

## User Experience

### Export Flow
1. User clicks "Export" button in Pattern Editor
2. Native save dialog opens
3. **User chooses location and filename**
4. File is saved
5. Debug console shows: `✅ Pattern exported to: C:/Users/.../pattern.json`

### Import Flow
1. User clicks "Import" button in Pattern Editor
2. Native open dialog opens
3. **User selects .json file**
4. Pattern loads into editor
5. Debug console shows: `✅ Pattern imported: My Pattern`

---

## Error Handling

### Export Errors
```typescript
try {
  // Export logic
} catch (error) {
  console.error('Export failed:', error);
  debugLog('error', `❌ Export failed: ${error}`);
}
```

### Import Errors
```typescript
try {
  // Import logic
} catch (error) {
  console.error('Import failed:', error);
  debugLog('error', `❌ Import failed: ${error}`);
  alert('Failed to import pattern');
}
```

**Common Issues:**
- Invalid JSON format
- Missing required fields (name, nodes, edges)
- File system permission errors
- User cancels dialog (not an error)

---

## File Location

### Before ❌
```
C:/Users/Username/Downloads/
├─ pattern.json
├─ pattern (1).json
├─ pattern (2).json
└─ pattern (3).json
```
Files accumulate in Downloads, hard to organize.

### After ✅
```
User chooses location:

Option 1: Project folder
C:/Users/Username/Documents/ButtThunder/Patterns/
├─ tank_combat.json
├─ aircraft_dogfight.json
└─ custom_brake.json

Option 2: Desktop
C:/Users/Username/Desktop/
└─ my_pattern.json

Option 3: Any folder
```

---

## Future Improvements

Possible additions:
- [ ] Default patterns directory (e.g., `AppData/ButtThunder/Patterns/`)
- [ ] Recent files list
- [ ] Pattern preview in file dialog
- [ ] Batch import/export (multiple files)
- [ ] Pattern templates/presets
- [ ] Cloud sync (optional)
- [ ] Pattern marketplace/sharing

---

## Compatibility

**Supported Platforms:**
- ✅ Windows (native dialog)
- ✅ macOS (native dialog)
- ✅ Linux (native dialog)

**File Extension:**
- `.json` (human-readable, easy to share)

**Encoding:**
- UTF-8 (supports international characters)

---

## Testing

### Manual Test Checklist
- [ ] Export creates file in chosen location
- [ ] Export with special characters in filename
- [ ] Export with long filename
- [ ] Import loads pattern correctly
- [ ] Import with invalid JSON shows error
- [ ] Import with missing fields shows error
- [ ] Cancel export dialog (no error)
- [ ] Cancel import dialog (no error)
- [ ] Debug console shows success messages
- [ ] Debug console shows error messages

### Test Patterns
Create test patterns with:
- Simple condition (INPUT → VIBRATION → OUTPUT)
- Complex logic (INPUT → LOGIC → VIBRATION → OUTPUT)
- Temporal condition (speed dropped by X in Y seconds)
- Multiple nodes (3+ inputs, 2+ vibrations)

---

**✅ Native file dialogs implemented!**  
**Users can now save patterns anywhere they want!** 📁

