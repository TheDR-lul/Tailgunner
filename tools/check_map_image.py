#!/usr/bin/env python3
"""
Check if /map.img from API matches images in War Thunder game files
"""

import os
import hashlib
import requests
from pathlib import Path

def find_wt_installation():
    """Find War Thunder installation"""
    common_paths = [
        Path(r"C:\Program Files (x86)\Steam\steamapps\common\War Thunder"),
        Path(r"C:\Program Files\Steam\steamapps\common\War Thunder"),
        Path(r"D:\SteamLibrary\steamapps\common\War Thunder"),
        Path(r"D:\Steam\steamapps\common\War Thunder"),
        Path(r"E:\SteamLibrary\steamapps\common\War Thunder"),
        Path(r"E:\Games\War Thunder"),
        Path(r"F:\Games\War Thunder"),
        Path(r"G:\Games\War Thunder"),
        Path.home() / "Games" / "War Thunder",
        Path(r"C:\Games\War Thunder"),
    ]
    
    for path in common_paths:
        if path.exists():
            print(f"[OK] Found War Thunder at: {path}")
            return path
    
    return None

def calculate_hash(data):
    """Calculate SHA256 hash"""
    return hashlib.sha256(data).hexdigest()

def get_api_map_image():
    """Get current map image from API"""
    try:
        response = requests.get("http://127.0.0.1:8111/map.img", timeout=5)
        if response.status_code == 200:
            return response.content
        else:
            print(f"[ERROR] API returned status {response.status_code}")
            return None
    except Exception as e:
        print(f"[ERROR] Failed to get API image: {e}")
        return None

def find_map_images(wt_path):
    """Find all map images in WT directory"""
    images = []
    
    print(f"\n[3/3] Searching for map images...")
    print(f"     War Thunder stores data in .vromfs.bin archives")
    print(f"     This means maps are PACKED, not as loose files")
    print(f"")
    print(f"[INFO] Checking for unpacked data...")
    
    # Check for unpacked VROMFS (if user ran game with -unpack)
    unpacked_dirs = list(wt_path.glob("*_u"))
    if unpacked_dirs:
        print(f"[OK] Found {len(unpacked_dirs)} unpacked archive(s)")
        for unpacked in unpacked_dirs:
            print(f"     - {unpacked.name}")
    else:
        print(f"[WARN] No unpacked data found")
        print(f"")
        print(f"[INFO] War Thunder stores maps in aces.vromfs.bin archive")
        print(f"       To extract: Launch game with -unpack parameter")
        print(f"       Or use: https://github.com/klensy/wt-tools")
        
    # Look for ANY image files recursively
    print(f"\n[INFO] Searching for ANY image files in installation...")
    for ext in ['*.dds', '*.png', '*.jpg', '*.tga', '*.bmp']:
        for img_path in wt_path.rglob(ext):
            images.append(img_path)
            if len(images) % 50 == 0:
                print(f"     Found {len(images)} images so far...")
                
    return images

def main():
    print("=" * 60)
    print("War Thunder Map Image Checker")
    print("=" * 60)
    
    # Step 1: Get API image
    print("\n[1/3] Getting image from API...")
    api_image = get_api_map_image()
    if not api_image:
        print("[ERROR] Cannot get API image. Make sure War Thunder is running!")
        return
    
    api_hash = calculate_hash(api_image)
    api_size = len(api_image)
    print(f"[OK] API image: {api_size} bytes")
    print(f"[OK] SHA256: {api_hash}")
    print(f"[OK] First 16 chars: {api_hash[:16]}")
    
    # Save API image for manual inspection
    output_file = Path(__file__).parent / "api_map_dump.png"
    with open(output_file, 'wb') as f:
        f.write(api_image)
    print(f"[OK] Saved to: {output_file}")
    
    # Step 2: Use known WT path
    print("\n[2/3] Checking War Thunder installation...")
    wt_path = Path(r"G:\SteamLibrary\steamapps\common\War Thunder")
    
    if not wt_path.exists():
        print(f"[ERROR] War Thunder not found at: {wt_path}")
        return
    
    print(f"[OK] Found War Thunder at: {wt_path}")
    
    # Step 3: Search for matching images
    print("\n[3/3] Searching for matching images...")
    images = find_map_images(wt_path)
    
    print(f"\nFound {len(images)} image files")
    print("Comparing hashes...")
    
    matches = []
    for i, img_path in enumerate(images):
        if i % 100 == 0:
            print(f"  Checked {i}/{len(images)}...")
        
        try:
            with open(img_path, 'rb') as f:
                img_data = f.read()
            
            img_hash = calculate_hash(img_data)
            
            # Check if hashes match
            if img_hash == api_hash:
                matches.append(img_path)
                print(f"\n>>> MATCH FOUND! <<<")
                print(f"  File: {img_path}")
                print(f"  Hash: {img_hash}")
            
            # Also check size for potential matches
            elif len(img_data) == api_size:
                print(f"\n[WARN] Same size (but different hash): {img_path}")
                
        except Exception as e:
            pass
    
    # Results
    print("\n" + "=" * 60)
    print("RESULTS:")
    print("=" * 60)
    
    if matches:
        print(f"\n[SUCCESS] Found {len(matches)} exact match(es)!")
        for match in matches:
            print(f"  - {match}")
        print(f"\n[SUCCESS] The /map.img API returns actual game files!")
        print(f"[SUCCESS] We can use image hash for map detection!")
    else:
        print(f"\n[FAIL] No exact matches found")
        print(f"[FAIL] The API might be generating images dynamically")
        print(f"[WARN] Image hash detection may not work reliably")
        print(f"\nAPI image saved to: {output_file}")
        print(f"You can inspect it manually")

if __name__ == "__main__":
    main()

