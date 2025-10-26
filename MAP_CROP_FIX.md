# Map Crop Fix - Matching In-Game View

## Problem

The application was showing the **full map** from the API, while War Thunder in-game shows only a **cropped portion** (zoom level).

**Example (American Desert / Iberian Castle):**
- Full map: `4096×4096m`
- Visible in-game: `1600×1600m` (**39.1%** of full map)
- Crop offset: `grid_zero = [1050, 2650]`

## Solution

### API Parameters

War Thunder API provides crop information:

```json
{
  "grid_size": [1600, 1600],   // Visible area size
  "grid_zero": [1050, 2650],   // Top-left corner offset
  "grid_steps": [225, 225],     // Grid cell size
  "map_min": [0, 0],            // Full map bounds
  "map_max": [4096, 4096]       // Full map bounds
}
```

**Key insight:** `grid_size` defines the **visible area**, not just grid dimensions!

### Changes Made

#### 1. Backend (`src-tauri/src/map_module.rs`)

**Grid reference calculation:**

API coords are **already relative to visible area** (0..1), so grid calculation is simple:

```rust
// API coords are relative to visible area (0..1)
// Convert to position in meters from top-left
let pos_x = x * grid_size[0];
let pos_y = y * grid_size[1];

// Calculate grid cell
let grid_x = (pos_x / grid_step_x).floor() as i32;  // Column (0, 1, 2, ...)
let grid_y = (pos_y / grid_step_y).floor() as i32;  // Row (0=A, 1=B, ...)

// Format as "A-1", "B-3", etc.
let letter = ('A' as u8 + grid_y) as char;
let number = grid_x + 1;
```

**Functions updated:**
- `MapData::get_player_grid_reference()` - Calculate grid reference (e.g. "C-4")

#### 2. Frontend (`src/components/MiniMap.tsx`)

**Image cropping:**
```typescript
// Calculate crop region from full API image
const cropLeft = (grid_zero[0] - map_min[0]) / mapWidth;
const cropTop = (grid_zero[1] - map_min[1]) / mapHeight;
const cropWidth = grid_size[0] / mapWidth;
const cropHeight = grid_size[1] / mapHeight;

// Y coordinate inversion: WT Y goes bottom-to-top, image Y goes top-to-bottom
const wtTop = grid_zero[1];
const cropTop = (mapHeight - wtTop) / mapHeight;

// Draw only visible portion
ctx.drawImage(
  mapImage,
  cropLeft * mapImage.width,    // Source X
  cropTop * mapImage.height,    // Source Y
  cropWidth * mapImage.width,   // Source Width
  cropHeight * mapImage.height, // Source Height
  0, 0, canvas.width, canvas.height
);
```

**Object coordinate conversion:**

**CRITICAL:** API coordinates are **ALREADY relative to the visible area**, NOT the full map!

The coordinates can be:
- Negative (object is above/left of visible area)
- Greater than 1 (object is below/right of visible area)  
- Between 0 and 1 (object is visible on screen)

```typescript
// API coords are directly usable - no conversion needed!
const x = obj.x * canvas.width;
const y = obj.y * canvas.height;

// For distance calculations, use grid_size (visible area size in meters)
const dx = enemy.x - player.x;  // Normalized difference
const distanceMeters = dx * grid_size[0];  // Convert to meters
```

**Functions updated:**
- `drawMapObject()` - Convert API coords to visible area coords
- `drawPlayer()` - Convert API coords to visible area coords
- `drawGrid()` - Use `grid_size` instead of `map_max - map_min`
- `calculateEnemyDistances()` - Convert API coords to world coords for distance calculation

### Result

✅ **Application now matches in-game view:**
- Shows same zoomed area as War Thunder
- Objects positioned correctly
- Grid labels aligned
- Distances calculated accurately

### Technical Details

**Crop Factor:**
```
crop_factor = grid_size / (map_max - map_min)
            = 1600 / 4096
            = 0.390625 (39.1%)
```

**Normalized Crop Region:**
```
left:   (grid_zero[0] - map_min[0]) / map_width  = 0.2563
top:    (grid_zero[1] - map_min[1]) / map_height = 0.6470
right:  left + crop_factor                        = 0.6470
bottom: top + crop_factor                         = 1.0376
```

### Notes

- This fix applies to **all maps** (different maps have different `grid_size` and `grid_zero`)
- Crop factor varies by game mode:
  - Ground battles (tanks): typically `1600×1600m` on `4096×4096m` maps
  - Air battles: different zoom levels
  - Naval battles: different zoom levels
- The API image itself is **always the full map**, we crop it client-side

### Files Modified

- `src-tauri/src/map_module.rs` - Backend coordinate calculations
- `src/components/MiniMap.tsx` - Frontend image cropping and rendering
- `tools/test_crop.py` - Analysis script for crop parameters

### Testing

1. Start War Thunder and enter a battle
2. Open application map view
3. Compare with in-game tactical map
4. Verify:
   - ✅ Same zoom level
   - ✅ Player position matches
   - ✅ Enemy positions match
   - ✅ Grid labels aligned
   - ✅ Distances accurate

