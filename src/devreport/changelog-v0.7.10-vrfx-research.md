# Changelog v0.7.10 - VRFx Format Research

**Date**: 2025-10-25  
**Type**: Research & Integration  
**Status**: In Progress

## 🎯 Goal
Полная автономная распаковка VROMFS VRFx архивов War Thunder

## 📊 Findings

### Format Detection
```
Archive: G:\SteamLibrary\steamapps\common\War Thunder\aces.vromfs.bin
- Size: 12,626,760 bytes (12 MB)
- Magic: VRFx (новый формат War Thunder)
- Version: 1129316352
- Files claimed: 1467621245 ❌ (невозможное значение!)
- Data offset: 0x2310056 (36765782) ❌ (больше размера файла!)
```

### Hex Dump Analysis
```
0000: 56 52 46 78 00 00 50 43 A0 A3 0B 01 20 AA C0 C0  VRFx..PC.... ...
0010: 08 00 00 00 56 00 31 02 7D 1F 7A 57 8F 90 AF 53  ....V.1.}.zW...S
0020: 5E AB 29 7B 4A 0E 95 16 20 00 00 00 BA 55 00 90  ^.){J... ....U..
0030: 48 15 F0 AD 02 0C AE 1B 2C 3F 85 98 AA C0 D6 EA  H.......,?......
```

### Problem
Наш простой парсер **НЕ РАБОТАЕТ** с VRFx:
- ❌ Формат отличается от старого VRFs
- ❌ Может быть зашифрован/обфусцирован
- ❌ Неизвестная структура subheader

## 🔍 Solutions Researched

### Option 1: wt_blk Library ✅ CHOSEN
**Repository**: https://github.com/Warthunder-Open-Source-Foundation/wt_blk  
**License**: MIT ✅ (коммерческое использование разрешено!)  
**Language**: Rust  
**Status**: Added to Cargo.toml

**Pros:**
- ✅ MIT лицензия - можно продавать
- ✅ Rust native
- ✅ Поддерживается сообществом
- ✅ Специально для War Thunder

**Cons:**
- ⚠️ API сложный, нужно изучить
- ⚠️ Документация не полная

### Option 2: Manual VRFx Parser ❌ REJECTED
**Estimated time**: 2-4 hours  
**Complexity**: Very High  
**Reason**: VRFx format is complex/obfuscated, reverse engineering needed

### Option 3: User Manual Unpacking ⚠️ FALLBACK
**Method**: Game's `-unpack` parameter  
**Instructions**:
```
1. Steam → ПКМ War Thunder → Properties → Launch Options
2. Add: -unpack
3. Launch game once
4. Game creates: aces.vromfs.bin_u folder
5. Restart Tailgunner
```

## 📦 Integration Status

### Added Dependencies
```toml
wt_blk = "0.3.1"  # MIT License
```

### Files Modified
- `src-tauri/Cargo.toml` - added wt_blk
- `src-tauri/src/datamine/vromfs.rs` - API stub (in progress)

### Current State
```rust
// ✅ Compiles
// ⚠️ wt_blk API not yet integrated
// 🔄 Need to study: wt_blk::vromf module
```

## 🚧 Next Steps

1. **Study wt_blk API** (current step)
   - Open local docs: `cargo doc --package wt_blk --open`
   - Find correct module for VROMFS unpacking
   - Implement unpack_with_wt_blk()

2. **Test Unpacking**
   - Test on real aces.vromfs.bin
   - Verify extracted files structure
   - Measure performance

3. **Error Handling**
   - Graceful fallback to manual unpacking
   - Clear error messages for user

## ⏱️ Time Estimate
- wt_blk integration: 30-60 minutes
- Testing & debugging: 15-30 minutes
- Total: ~1-1.5 hours

## 📝 License Compliance

### wt_blk (MIT License)
```
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software... to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software...
```

**Action Required**: Add wt_blk attribution to LICENSE file

---

**Status**: 🔄 In Progress  
**Blocked By**: wt_blk API documentation study

