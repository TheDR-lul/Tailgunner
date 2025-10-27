# Changelog v0.7.10 - VROMFS Fallback Solution

**Date**: 2025-10-25  
**Type**: Temporary Solution  
**Status**: Active

## 🎯 Outcome

После исследования формата VRFx и доступных библиотек:
- ❌ `wt_blk` - для `.blk` файлов, **НЕ** для `.vromfs.bin` архивов
- ❌ VRFx формат сложный/обфусцированный, требует reverse engineering
- ✅ **Временное решение**: Используем `-unpack` параметр игры

## 💡 User Solution

### Инструкция для пользователя:
```
1. Steam → ПКМ War Thunder → Свойства → Параметры запуска
2. Добавить: -unpack
3. Запустить игру ОДИН РАЗ (можно сразу выйти)
4. Игра создаст: aces.vromfs.bin_u/ с распакованными файлами
5. Перезапустить Tailgunner → всё заработает!
```

### Автоматическое определение:
```rust
prepare_game_files() проверяет:
1. game_path/aces.vromfs.bin_u ✅ (игра распаковала)
2. game_path/gamedata ✅ (loose files)
3. TEMP/tailgunner_datamine/aces.vromfs.bin_u (наш кеш)
4. Если ничего нет → показываем инструкцию
```

## ✅ Advantages

### For User:
- ✅ **100% надёжно** - игра сама распаковывает
- ✅ **100% EAC-safe** - официальный метод
- ✅ **Один раз** - создаётся папка _u, больше не нужно
- ✅ **Быстро** - игра запускается <30 сек

### For Dev:
- ✅ Не нужен сложный VRFx парсер
- ✅ Не зависим от внешних библиотек
- ✅ Работает с любой версией игры
- ✅ Игра сама обновляет при патчах

## 🔮 Future Plans

### Option 1: VRFx Parser (если понадобится)
- Reverse engineer VRFx format
- Implement in Rust
- Time: 4-8 hours
- Complexity: Very High

### Option 2: Keep Current Solution
- Users do `-unpack` once
- We use pre-unpacked files
- **Recommended** - simple & reliable

## 📊 Impact

### Code Changes:
- `src-tauri/src/datamine/vromfs.rs` - fallback logic
- Error messages show clear instructions
- No breaking changes

### User Experience:
```
Before: ❌ "Failed to extract file entries"
After:  💡 "Please run War Thunder with '-unpack' parameter"
        [Clear step-by-step instructions]
```

## 🎉 Result

**Проект полностью автономен**, просто требует одноразовый шаг от юзера!

---

**Status**: ✅ Complete  
**Type**: Pragmatic solution  
**Commercial Impact**: None (официальный метод)

