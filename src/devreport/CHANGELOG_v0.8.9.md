# Changelog v0.8.9

**Release Date:** October 27, 2025

## 🐛 Bug Fixes

### Game Feed & Event Filtering

#### Fixed Game Feed not clearing on battle change
- **Issue:** Game Feed was not clearing old messages when starting a new battle
- **Fix:** Removed unnecessary `if (lastBattleId !== 0)` check that was blocking message cleanup
- **Impact:** Game Feed now correctly clears when `map_generation` changes (new battle detected)
- **File:** `src/components/GameChat.tsx`

#### Fixed triggers firing on old events after app restart
- **Issue:** When restarting the application mid-battle, all accumulated HUD events (100+ messages) would be processed, causing triggers to fire inappropriately
- **Root Cause:** War Thunder API does not clear old events - it only resets the `time` field to 0 when starting a new battle
- **Solution:** Implemented two-level filtering system:

**Backend Filter (Rust):**
- Added `last_battle_time` tracking in `WTTelemetryReader`
- **INIT PHASE:** On first connection, finds max `time` and max `id`, sets baseline, skips all old events
  - Logs: `[HUD] ✅ Initialized with baseline ID=177, time=385s`
- **NEW BATTLE DETECTION:** Detects time reset when `time` decreases by >60 seconds
  - Logs: `[HUD] 🔄 NEW BATTLE DETECTED! Time reset: 531s → 91s (diff: -440s)`
  - Clears message cache: `[HUD] 🧹 Clearing old events cache (X entries)`
  - Skips entire event batch: `[HUD] ⏭️ Skipping batch of 160 events (new battle initialization)`
- **NORMAL PHASE:** Only processes events with `time >= last_battle_time` and `id > last_id`
- **Files:** `src-tauri/src/wt_telemetry.rs`

**Frontend Filter (TypeScript):**
- **Bulk Load Detection:** If receiving >5 messages at once when message list is empty → old events
  - Logs: `[GameChat] ⏭️ Ignoring 140 old HUD messages (bulk load detected)`
- Prevents display of old messages in UI while still updating `lastId` for subsequent polls
- Applied to both HUD events and chat messages
- **Files:** `src/components/GameChat.tsx`

**Example Flow:**
```
1. User restarts app mid-battle
2. First poll: Backend returns 140 events (ids 33-177)
3. Backend: Sets baseline ID=177, time=385s, skips display
4. Frontend: Receives 140 messages, detects bulk load, ignores them
5. Next poll: Only NEW events (1-3 at a time) are processed
```

#### Enhanced chat message initialization
- Added message count logging: `[Chat] ✅ Initialized with baseline ID=95, skipping 30 old messages`
- Improved visibility into chat message filtering during startup
- **File:** `src-tauri/src/wt_telemetry.rs`

## 🔧 Technical Details

### Backend Changes
- Added `last_battle_time: u32` to `WTTelemetryReader` struct
- Enhanced `get_hud_events()` with time reset detection logic
- Improved initialization logging for both HUD and chat streams
- Better duplicate message prevention using time-based cache retention

### Frontend Changes
- Added bulk load detection in `GameChat.tsx` for both HUD and chat messages
- Threshold: >5 messages when `prev.length === 0` triggers filtering
- Messages are filtered but `lastId` is still updated to track position in stream

## 📊 Impact
- **Before:** 100+ trigger activations on app restart
- **After:** 0 trigger activations on old events
- **Performance:** No performance impact, actually improves by skipping old event processing
- **User Experience:** Clean Game Feed showing only current battle events

## 🧪 Testing
Tested scenarios:
1. ✅ App restart during active battle → old events ignored
2. ✅ Battle transition (map change) → feed clears, new events processed
3. ✅ Cold start in hangar → no false events
4. ✅ Multiple battle transitions → consistent behavior

---

## Files Modified
- `src-tauri/src/wt_telemetry.rs` - Backend event filtering logic
- `src/components/GameChat.tsx` - Frontend bulk load detection
- `src-tauri/Cargo.toml` - Version bump to 0.8.9
- `src-tauri/tauri.conf.json` - Version bump to 0.8.9

