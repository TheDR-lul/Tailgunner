import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import './MiniMap.css';

interface MapObject {
  type: string;
  color: string;
  color_rgb: number[];
  blink: number;
  icon: string;
  icon_bg: string;
  x?: number;
  y?: number;
  dx?: number;
  dy?: number;
  sx?: number;
  sy?: number;
  ex?: number;
  ey?: number;
}

interface MapInfo {
  grid_size: number[];
  grid_steps: number[];
  grid_zero: number[];
  map_min: number[];
  map_max: number[];
  map_generation: number;
  hud_type: number;
  valid: boolean;
}

interface MapData {
  objects: MapObject[];
  info: MapInfo;
  player_position: [number, number] | null;
  player_heading: number | null;
  map_name: string | null;
  player_grid: string | null;
}

interface EnemyDistance {
  distance: number;
  type: string;
  position: [number, number];
  bearing: number;  // Bearing/azimuth to enemy in degrees
}

type MapMode = 'current' | 'full';

export function MiniMap() {
  const { t } = useTranslation();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [mapData, setMapData] = useState<MapData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isEnabled, setIsEnabled] = useState(false);
  const [mapImage, setMapImage] = useState<HTMLImageElement | null>(null);
  const [currentMapGen, setCurrentMapGen] = useState<number>(-1);
  const [nearestEnemies, setNearestEnemies] = useState<EnemyDistance[]>([]);
  const [mapMode, setMapMode] = useState<MapMode>('current');

  const getCompassDirection = (heading: number): string => {
    if (heading >= 337.5 || heading < 22.5) return 'N';
    if (heading >= 22.5 && heading < 67.5) return 'NE';
    if (heading >= 67.5 && heading < 112.5) return 'E';
    if (heading >= 112.5 && heading < 157.5) return 'SE';
    if (heading >= 157.5 && heading < 202.5) return 'S';
    if (heading >= 202.5 && heading < 247.5) return 'SW';
    if (heading >= 247.5 && heading < 292.5) return 'W';
    if (heading >= 292.5 && heading < 337.5) return 'NW';
    return '';
  };

  useEffect(() => {
    if (!isEnabled) return;

    const interval = setInterval(async () => {
      try {
        const data = await invoke<MapData>('get_map_data');
        setMapData(data);
        setError(null);
        
        // Load map image if map generation changed
        if (data.info.valid && data.info.map_generation !== currentMapGen) {
          try {
            // Load image through Tauri backend to avoid CORS issues
            const base64Image = await invoke<string>('get_map_image', { 
              mapGeneration: data.info.map_generation 
            });
            
            const img = new Image();
            img.onload = () => {
              setMapImage(img);
              setCurrentMapGen(data.info.map_generation);
            };
            img.onerror = () => {
              console.warn('Failed to load map image');
              setMapImage(null);
            };
            img.src = base64Image;
          } catch (error) {
            console.warn('Failed to fetch map image:', error);
            setMapImage(null);
          }
        }
      } catch (err) {
        setError(String(err));
      }
    }, 500); // Update every 500ms

    return () => clearInterval(interval);
  }, [isEnabled, currentMapGen]);

  useEffect(() => {
    if (!mapData || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    if (!mapData.info.valid) {
      // Draw "No Data" message
      ctx.fillStyle = '#666';
      ctx.font = '14px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('No map data', canvas.width / 2, canvas.height / 2);
      return;
    }

    // Draw map image as background
    if (mapImage) {
      const mapWidth = mapData.info.map_max[0] - mapData.info.map_min[0];
      const mapHeight = mapData.info.map_max[1] - mapData.info.map_min[1];
      
      let cropLeft, cropTop, cropWidth, cropHeight;
      
      if (mapMode === 'full') {
        // Full map mode: show entire map without crop
        cropLeft = 0;
        cropTop = 0;
        cropWidth = 1;
        cropHeight = 1;
      } else {
        // Current mode: crop to visible grid area (tank/aircraft view)
        cropLeft = (mapData.info.grid_zero[0] - mapData.info.map_min[0]) / mapWidth;
        cropWidth = mapData.info.grid_size[0] / mapWidth;
        cropHeight = mapData.info.grid_size[1] / mapHeight;
        
        // grid_zero[1] is the TOP of visible area in WT coords (Y goes up)
        const gridTop = mapData.info.grid_zero[1];
        // Convert WT Y coords to image Y coords (invert)
        cropTop = (mapData.info.map_max[1] - gridTop) / mapHeight;
      }
      
      // Draw cropped portion of the image
      ctx.drawImage(
        mapImage,
        cropLeft * mapImage.width,  // Source X
        cropTop * mapImage.height,   // Source Y
        cropWidth * mapImage.width,  // Source Width
        cropHeight * mapImage.height, // Source Height
        0,                            // Dest X
        0,                            // Dest Y
        canvas.width,                 // Dest Width
        canvas.height                 // Dest Height
      );
      
      // Draw semi-transparent overlay for better object visibility
      ctx.fillStyle = 'rgba(0, 0, 0, 0.2)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    // Draw grid
    drawGrid(ctx, canvas, mapData.info);

    // Calculate enemy distances
    const playerObj = mapData.objects.find(obj => obj.icon === 'Player');
    if (playerObj && playerObj.x !== undefined && playerObj.y !== undefined) {
      calculateEnemyDistances(playerObj, mapData.objects, mapData.info);
    }

    // Draw objects
    for (const obj of mapData.objects) {
      if (obj.icon === 'Player') {
        // Draw player last (on top)
        continue;
      }
      drawMapObject(ctx, canvas, obj, mapData.info);
    }

    // Draw player
    if (playerObj) {
      drawPlayer(ctx, canvas, playerObj, mapData.info);
    }
  }, [mapData, mapImage, mapMode]);

  const calculateEnemyDistances = (
    player: MapObject,
    objects: MapObject[],
    info: MapInfo
  ) => {
    if (!player.x || !player.y) return;

    const enemies: EnemyDistance[] = [];
    
    // Convert player API coords to world coords (meters)
    // NOTE: API Y is inverted! y=0 is North (top), y=1 is South (bottom)
    const mapWidth = info.map_max[0] - info.map_min[0];
    const mapHeight = info.map_max[1] - info.map_min[1];
    const playerWorldX = player.x * mapWidth + info.map_min[0];
    const playerWorldY = (1 - player.y) * mapHeight + info.map_min[1];

    for (const obj of objects) {
      // Check if object is enemy (red color) and has position
      if (obj.x === undefined || obj.y === undefined) continue;
      
      // Skip player
      if (obj.icon === 'Player') continue;
      
      // Enemy color is #fa0C00 (red), NOT player color #faC81E (yellow)
      const isEnemy = (obj.color_rgb && 
                       obj.color_rgb[0] > 240 && 
                       obj.color_rgb[1] < 20 && 
                       obj.color_rgb[2] < 20) ||
                      obj.color.toLowerCase() === '#fa0c00';
      
      if (!isEnemy) continue;
      if (obj.type !== 'ground_model' && obj.type !== 'aircraft' && obj.type !== 'Ship') continue;

      // Convert enemy API coords to world coords (meters)
      const enemyWorldX = obj.x * mapWidth + info.map_min[0];
      const enemyWorldY = (1 - obj.y) * mapHeight + info.map_min[1]; // API Y inverted!
      
      // Calculate distance in real-world meters
      const distX = enemyWorldX - playerWorldX;
      const distY = enemyWorldY - playerWorldY;
      const distance = Math.sqrt(distX * distX + distY * distY);
      
      // Calculate bearing/azimuth to enemy (0° = North, clockwise)
      const angleFromX = Math.atan2(distY, distX) * (180 / Math.PI);
      const bearing = (90 - angleFromX + 360) % 360;

      enemies.push({
        distance,
        type: obj.icon || obj.type,
        position: [obj.x, obj.y],
        bearing,
      });
    }

    // Sort by distance and keep top 5
    enemies.sort((a, b) => a.distance - b.distance);
    setNearestEnemies(enemies.slice(0, 5));
  };

  const drawGrid = (ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, info: MapInfo) => {
    // Use grid size based on map mode
    const mapSizeX = mapMode === 'full' 
      ? info.map_max[0] - info.map_min[0] 
      : info.grid_size[0];
    const mapSizeY = mapMode === 'full'
      ? info.map_max[1] - info.map_min[1]
      : info.grid_size[1];
    
    // Use grid_steps from API
    const gridStepX = info.grid_steps[0];
    const gridStepY = info.grid_steps[1];
    
    // Calculate EXACT number of grid squares (NOT rounded!)
    // e.g. 1600/225 = 7.111... means 7 full squares + 11% of 8th
    const gridCountX = mapSizeX / gridStepX;
    const gridCountY = mapSizeY / gridStepY;
    
    // Number of FULL squares for labels
    const fullGridX = Math.floor(gridCountX);
    const fullGridY = Math.floor(gridCountY);
    
    // Make grid more visible when map image is present
    ctx.strokeStyle = mapImage ? 'rgba(255, 255, 255, 0.25)' : '#333';
    ctx.lineWidth = 1;
    ctx.beginPath();

    // Draw vertical lines (including partial last line if needed)
    for (let i = 0; i <= Math.ceil(gridCountX); i++) {
      if (i <= gridCountX) {
        const x = (canvas.width / gridCountX) * i;
        ctx.moveTo(x, 0);
        ctx.lineTo(x, canvas.height);
      }
    }

    // Draw horizontal lines (including partial last line if needed)
    for (let i = 0; i <= Math.ceil(gridCountY); i++) {
      if (i <= gridCountY) {
        const y = (canvas.height / gridCountY) * i;
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
      }
    }

    ctx.stroke();
    
    // Draw grid labels (like in War Thunder)
    ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
    ctx.font = '10px monospace';
    
    // Top numbers (1, 2, 3, ...) - only for FULL squares
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    for (let i = 0; i < fullGridX && i < 20; i++) {
      const x = (canvas.width / gridCountX) * (i + 0.5);
      ctx.fillText((i + 1).toString(), x, 2);
    }
    
    // Left letters (A, B, C, ...) - only for FULL squares
    ctx.textAlign = 'left';
    ctx.textBaseline = 'middle';
    for (let i = 0; i < fullGridY && i < 26; i++) {
      const y = (canvas.height / gridCountY) * (i + 0.5);
      const letter = String.fromCharCode(65 + i); // A=65, B=66, ...
      ctx.fillText(letter, 2, y);
    }
  };

  const drawMapObject = (
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    obj: MapObject,
    info: MapInfo
  ) => {
    // Convert API coords (relative to full map) to canvas coords (relative to cropped visible area)
    // Same logic as image cropping!
    const mapWidth = info.map_max[0] - info.map_min[0];
    const mapHeight = info.map_max[1] - info.map_min[1];
    
    const toCanvasCoords = (apiX: number, apiY: number) => {
      // 1. Convert normalized API coords to world coords (meters)
      // NOTE: API Y is inverted! y=0 is North (top), y=1 is South (bottom)
      const worldX = apiX * mapWidth + info.map_min[0];
      const worldY = (1 - apiY) * mapHeight + info.map_min[1];
      
      if (mapMode === 'full') {
        // Full map mode: convert world coords to normalized 0..1 directly
        const visibleX = (worldX - info.map_min[0]) / mapWidth;
        const visibleY = (info.map_max[1] - worldY) / mapHeight;
        return { x: visibleX, y: visibleY };
      } else {
        // Current mode: convert to visible area coords (grid_zero/grid_size)
        const visibleX = (worldX - info.grid_zero[0]) / info.grid_size[0];
        const visibleY = (info.grid_zero[1] - worldY) / info.grid_size[1];
        return { x: visibleX, y: visibleY };
      }
    };
    
    if (obj.x === undefined || obj.y === undefined) {
      // Line object (airfield)
      if (obj.sx !== undefined && obj.sy !== undefined && obj.ex !== undefined && obj.ey !== undefined) {
        const start = toCanvasCoords(obj.sx, obj.sy);
        const end = toCanvasCoords(obj.ex, obj.ey);
        
        const sx = start.x * canvas.width;
        const sy = start.y * canvas.height;
        const ex = end.x * canvas.width;
        const ey = end.y * canvas.height;

        ctx.strokeStyle = obj.color;
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(sx, sy);
        ctx.lineTo(ex, ey);
        ctx.stroke();
      }
      return;
    }

    // Convert API coords to canvas coords
    const pos = toCanvasCoords(obj.x, obj.y);
    const x = pos.x * canvas.width;
    const y = pos.y * canvas.height;

    // Draw icon based on type
    ctx.fillStyle = obj.color;
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 0.5;

    // Smaller marker sizes
    const size = obj.type === 'capture_zone' ? 6 : 3;

    if (obj.icon === 'Ship') {
      // Triangle for ships
      ctx.beginPath();
      ctx.moveTo(x, y - size);
      ctx.lineTo(x - size / 2, y + size / 2);
      ctx.lineTo(x + size / 2, y + size / 2);
      ctx.closePath();
      ctx.fill();
      ctx.stroke();
    } else if (obj.type === 'capture_zone') {
      // Circle for capture zones
      ctx.beginPath();
      ctx.arc(x, y, size, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
    } else if (obj.type === 'respawn_base_tank' || obj.type === 'respawn_base_fighter' || obj.type === 'respawn_base_bomber') {
      // Tiny dots for spawn points
      ctx.beginPath();
      ctx.arc(x, y, 1.5, 0, Math.PI * 2);
      ctx.fill();
    } else {
      // Square for other objects
      ctx.fillRect(x - size / 2, y - size / 2, size, size);
      ctx.strokeRect(x - size / 2, y - size / 2, size, size);
    }
  };

  const drawPlayer = (
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    obj: MapObject,
    info: MapInfo
  ) => {
    if (obj.x === undefined || obj.y === undefined) return;

    // Convert API coords to canvas coords
    const mapWidth = info.map_max[0] - info.map_min[0];
    const mapHeight = info.map_max[1] - info.map_min[1];
    const worldX = obj.x * mapWidth + info.map_min[0];
    const worldY = (1 - obj.y) * mapHeight + info.map_min[1]; // API Y inverted!
    
    let visibleX, visibleY;
    if (mapMode === 'full') {
      visibleX = (worldX - info.map_min[0]) / mapWidth;
      visibleY = (info.map_max[1] - worldY) / mapHeight;
    } else {
      visibleX = (worldX - info.grid_zero[0]) / info.grid_size[0];
      visibleY = (info.grid_zero[1] - worldY) / info.grid_size[1];
    }
    
    const x = visibleX * canvas.width;
    const y = visibleY * canvas.height;

    // Calculate heading angle
    let angle = 0;
    if (obj.dx !== undefined && obj.dy !== undefined && (obj.dx !== 0 || obj.dy !== 0)) {
      angle = Math.atan2(obj.dy, obj.dx);
    }

    // Draw player as arrow
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(angle);

    ctx.fillStyle = '#faC81E';
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1.5;

    const arrowSize = 8;
    ctx.beginPath();
    ctx.moveTo(arrowSize, 0);
    ctx.lineTo(-arrowSize / 2, -arrowSize / 2);
    ctx.lineTo(-arrowSize / 2, arrowSize / 2);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    ctx.restore();
  };

  return (
    <div className="minimap-container">
      <div className="minimap-header">
        <h3>{t('map.title')}</h3>
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <div style={{ display: 'flex', gap: '4px' }}>
            <button
              className={`btn btn-${mapMode === 'current' ? 'primary' : 'secondary'}`}
              onClick={() => setMapMode('current')}
              disabled={!isEnabled}
              title="Current vehicle view (tank/aircraft)"
              style={{ fontSize: '11px', padding: '4px 8px' }}
            >
              Current
            </button>
            <button
              className={`btn btn-${mapMode === 'full' ? 'primary' : 'secondary'}`}
              onClick={() => setMapMode('full')}
              disabled={!isEnabled}
              title="Full map view"
              style={{ fontSize: '11px', padding: '4px 8px' }}
            >
              Full
            </button>
          </div>
          <button 
            className={`btn btn-${isEnabled ? 'primary' : 'secondary'}`}
            onClick={() => setIsEnabled(!isEnabled)}
            title={t('map.rb_warning')}
          >
            {isEnabled ? t('map.enabled') : t('map.disabled')}
          </button>
        </div>
      </div>
      <div className="minimap-canvas-wrapper">
        <canvas
          ref={canvasRef}
          width={400}
          height={400}
          className="minimap-canvas"
        />
        {error && !isEnabled && (
          <div className="minimap-info">{t('map.disabled')}</div>
        )}
        {error && isEnabled && (
          <div className="minimap-error">
            {error.includes('connect') ? 'War Thunder not running' : t('map.no_data')}
          </div>
        )}
      </div>
      {mapData && isEnabled && (
        <div className="minimap-stats-bottom">
          {mapData.map_name && (
            <div style={{ fontWeight: 'bold', fontSize: '12px', marginBottom: '4px', color: '#64c8ff' }}>
              📍 {mapData.map_name}
            </div>
          )}
          <div style={{ display: 'flex', gap: '16px', flexWrap: 'wrap' }}>
            <div>
              <div>{t('map.objects')} {mapData.objects.length}</div>
              {mapData.player_grid && (
                <div style={{ fontSize: '11px', color: '#22c55e', fontWeight: 'bold' }}>
                  🎯 Grid: {mapData.player_grid}
                </div>
              )}
              {mapData.player_position && (
                <div style={{ fontSize: '10px', color: '#888' }}>
                  ({mapData.player_position[0].toFixed(3)}, {mapData.player_position[1].toFixed(3)})
                </div>
              )}
              {mapData.player_heading !== null && (
                <div>🧭 {mapData.player_heading.toFixed(0)}° {getCompassDirection(mapData.player_heading)}</div>
              )}
            </div>
            {nearestEnemies.length > 0 && (
              <div className="minimap-enemies">
                <div style={{ fontWeight: 'bold', marginBottom: '4px' }}>{t('map.nearest_enemies')}</div>
                {nearestEnemies.map((enemy, idx) => (
                  <div key={idx} style={{ fontSize: '11px', color: '#ff6666' }}>
                    {idx + 1}. {enemy.type}: {enemy.distance.toFixed(0)}m @ {enemy.bearing.toFixed(0)}° {getCompassDirection(enemy.bearing)}
                  </div>
                ))}
              </div>
            )}
            {nearestEnemies.length === 0 && mapData.info.valid && (
              <div style={{ fontSize: '11px', color: '#888', fontStyle: 'italic' }}>
                {t('map.no_enemies')}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}


