#!/usr/bin/env python3
"""
War Thunder Map Parser
Extracts grid sizes and map information from level.blk files
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Optional

def find_wt_installation() -> Optional[Path]:
    """Find War Thunder installation directory"""
    common_paths = [
        Path(r"C:\Program Files (x86)\Steam\steamapps\common\War Thunder"),
        Path(r"D:\SteamLibrary\steamapps\common\War Thunder"),
        Path(r"E:\Games\War Thunder"),
        Path.home() / "Games" / "War Thunder"
    ]
    
    for path in common_paths:
        if path.exists():
            print(f"✓ Found War Thunder at: {path}")
            return path
    
    return None

def parse_blk_file(file_path: Path) -> Dict:
    """Parse .blk file and extract grid information"""
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        
        data = {}
        
        # Extract map size
        size_match = re.search(r'mapSize\s*:\s*\[\s*([\d.]+)\s*,\s*([\d.]+)\s*\]', content)
        if size_match:
            data['map_size'] = [float(size_match.group(1)), float(size_match.group(2))]
        
        # Extract grid step for ground vehicles
        ground_grid = re.search(r'gridStep\s*:\s*([\d.]+)', content)
        if ground_grid:
            data['ground_grid_step'] = float(ground_grid.group(1))
        
        # Extract grid step for air vehicles
        air_grid = re.search(r'airGridStep\s*:\s*([\d.]+)', content)
        if air_grid:
            data['air_grid_step'] = float(air_grid.group(1))
        
        # Extract grid step for naval
        naval_grid = re.search(r'navalGridStep\s*:\s*([\d.]+)', content)
        if naval_grid:
            data['naval_grid_step'] = float(naval_grid.group(1))
        
        # Extract game modes
        modes = []
        if 'ground' in content.lower() or 'tank' in content.lower():
            modes.append('ground')
        if 'air' in content.lower():
            modes.append('air')
        if 'naval' in content.lower():
            modes.append('naval')
        
        data['game_modes'] = modes if modes else ['unknown']
        
        return data
    except Exception as e:
        print(f"  ✗ Error parsing {file_path.name}: {e}")
        return {}

def extract_all_maps(wt_path: Path) -> Dict[str, Dict]:
    """Extract information from all map level.blk files"""
    levels_path = wt_path / "content" / "levels"
    
    if not levels_path.exists():
        print(f"✗ Levels directory not found: {levels_path}")
        return {}
    
    maps = {}
    
    # Find all level.blk files
    for level_dir in levels_path.iterdir():
        if not level_dir.is_dir():
            continue
        
        level_blk = level_dir / "level.blk"
        if not level_blk.exists():
            continue
        
        map_name = level_dir.name
        print(f"  Parsing: {map_name}")
        
        map_data = parse_blk_file(level_blk)
        if map_data:
            # Clean up map name
            localized_name = map_name.replace('avg_', '').replace('_', ' ').title()
            
            maps[map_name] = {
                'name': map_name,
                'localized_name': localized_name,
                **map_data
            }
    
    return maps

def generate_rust_code(maps: Dict[str, Dict]) -> str:
    """Generate Rust code for map database"""
    code = []
    
    for map_name, data in sorted(maps.items()):
        ground_step = data.get('ground_grid_step')
        air_step = data.get('air_grid_step')
        naval_step = data.get('naval_grid_step')
        map_size = data.get('map_size', [4096.0, 4096.0])
        
        code.append(f'''
        // {data['localized_name']}
        self.maps.insert("{map_name}".to_string(), MapGridInfo {{
            name: "{map_name}".to_string(),
            localized_name: "{data['localized_name']}".to_string(),
            ground_grid_step: {f"Some({ground_step})" if ground_step else "None"},
            air_grid_step: {f"Some({air_step})" if air_step else "None"},
            naval_grid_step: {f"Some({naval_step})" if naval_step else "None"},
            map_size: [{map_size[0]}, {map_size[1]}],
            game_modes: vec![{', '.join(f'"{m}".to_string()' for m in data['game_modes'])}],
        }});''')
    
    return '\n'.join(code)

def main():
    print("=" * 60)
    print("War Thunder Map Parser")
    print("=" * 60)
    
    # Find War Thunder installation
    wt_path = find_wt_installation()
    if not wt_path:
        print("\n✗ War Thunder not found!")
        print("Please specify path manually:")
        manual_path = input("Enter path: ").strip('"')
        wt_path = Path(manual_path)
        
        if not wt_path.exists():
            print("✗ Invalid path!")
            return
    
    print(f"\n📁 Scanning levels...")
    maps = extract_all_maps(wt_path)
    
    if not maps:
        print("\n✗ No maps found!")
        return
    
    print(f"\n✓ Found {len(maps)} maps!")
    
    # Save JSON
    output_json = Path(__file__).parent / "wt_maps_database.json"
    with open(output_json, 'w', encoding='utf-8') as f:
        json.dump(maps, f, indent=2, ensure_ascii=False)
    print(f"\n✓ Saved JSON: {output_json}")
    
    # Generate Rust code
    rust_code = generate_rust_code(maps)
    output_rust = Path(__file__).parent / "map_database_generated.rs"
    with open(output_rust, 'w', encoding='utf-8') as f:
        f.write(f"// Auto-generated from parse_wt_maps.py\n")
        f.write(f"// Total maps: {len(maps)}\n\n")
        f.write(rust_code)
    print(f"✓ Saved Rust code: {output_rust}")
    
    # Show some statistics
    print("\n" + "=" * 60)
    print("Statistics:")
    print("=" * 60)
    
    ground_maps = [m for m in maps.values() if 'ground' in m['game_modes']]
    air_maps = [m for m in maps.values() if 'air' in m['game_modes']]
    naval_maps = [m for m in maps.values() if 'naval' in m['game_modes']]
    
    print(f"Ground maps: {len(ground_maps)}")
    print(f"Air maps: {len(air_maps)}")
    print(f"Naval maps: {len(naval_maps)}")
    
    # Show unique grid steps
    ground_steps = set(m.get('ground_grid_step') for m in ground_maps if m.get('ground_grid_step'))
    print(f"\nGround grid steps found: {sorted(ground_steps)}")
    
    # Show examples
    print("\n" + "=" * 60)
    print("Example maps:")
    print("=" * 60)
    
    for map_name, data in list(maps.items())[:5]:
        print(f"\n{data['localized_name']}:")
        if data.get('ground_grid_step'):
            print(f"  Ground grid: {data['ground_grid_step']}m")
        if data.get('map_size'):
            print(f"  Size: {data['map_size'][0]}m × {data['map_size'][1]}m")

if __name__ == "__main__":
    main()

