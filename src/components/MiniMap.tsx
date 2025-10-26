import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
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
}

export function MiniMap() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [mapData, setMapData] = useState<MapData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isEnabled, setIsEnabled] = useState(false);
  const [mapImage, setMapImage] = useState<HTMLImageElement | null>(null);
  const [currentMapGen, setCurrentMapGen] = useState<number>(-1);

  useEffect(() => {
    if (!isEnabled) return;

    const interval = setInterval(async () => {
      try {
        const data = await invoke<MapData>('get_map_data');
        setMapData(data);
        setError(null);
        
        // Load map image if map generation changed
        if (data.info.valid && data.info.map_generation !== currentMapGen) {
          const img = new Image();
          img.crossOrigin = 'anonymous';
          img.onload = () => {
            setMapImage(img);
            setCurrentMapGen(data.info.map_generation);
          };
          img.onerror = () => {
            console.warn('Failed to load map image');
            setMapImage(null);
          };
          img.src = `http://127.0.0.1:8111/map.img?gen=${data.info.map_generation}`;
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
      ctx.drawImage(mapImage, 0, 0, canvas.width, canvas.height);
      
      // Draw semi-transparent overlay for better object visibility
      ctx.fillStyle = 'rgba(0, 0, 0, 0.2)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    // Draw grid
    drawGrid(ctx, canvas, mapData.info);

    // Draw objects
    for (const obj of mapData.objects) {
      if (obj.icon === 'Player') {
        // Draw player last (on top)
        continue;
      }
      drawMapObject(ctx, canvas, obj, mapData.info);
    }

    // Draw player
    const playerObj = mapData.objects.find(obj => obj.icon === 'Player');
    if (playerObj) {
      drawPlayer(ctx, canvas, playerObj, mapData.info);
    }
  }, [mapData, mapImage]);

  const drawGrid = (ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, info: MapInfo) => {
    // Make grid more visible when map image is present
    ctx.strokeStyle = mapImage ? 'rgba(255, 255, 255, 0.3)' : '#333';
    ctx.lineWidth = 1;
    ctx.beginPath();

    // Vertical lines
    for (let i = 0; i <= 10; i++) {
      const x = (canvas.width / 10) * i;
      ctx.moveTo(x, 0);
      ctx.lineTo(x, canvas.height);
    }

    // Horizontal lines
    for (let i = 0; i <= 10; i++) {
      const y = (canvas.height / 10) * i;
      ctx.moveTo(0, y);
      ctx.lineTo(canvas.width, y);
    }

    ctx.stroke();
  };

  const drawMapObject = (
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    obj: MapObject,
    info: MapInfo
  ) => {
    if (obj.x === undefined || obj.y === undefined) {
      // Line object (airfield)
      if (obj.sx !== undefined && obj.sy !== undefined && obj.ex !== undefined && obj.ey !== undefined) {
        const sx = obj.sx * canvas.width;
        const sy = obj.sy * canvas.height;
        const ex = obj.ex * canvas.width;
        const ey = obj.ey * canvas.height;

        ctx.strokeStyle = obj.color;
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(sx, sy);
        ctx.lineTo(ex, ey);
        ctx.stroke();
      }
      return;
    }

    const x = obj.x * canvas.width;
    const y = obj.y * canvas.height;

    // Draw icon based on type
    ctx.fillStyle = obj.color;
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1;

    const size = 8;

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
      ctx.arc(x, y, size * 2, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
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

    const x = obj.x * canvas.width;
    const y = obj.y * canvas.height;

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
    ctx.lineWidth = 2;

    const arrowSize = 12;
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
        <h3>Map</h3>
        <button 
          className={`btn btn-${isEnabled ? 'primary' : 'secondary'}`}
          onClick={() => setIsEnabled(!isEnabled)}
        >
          {isEnabled ? 'Enabled' : 'Disabled'}
        </button>
      </div>
      <div className="minimap-canvas-wrapper">
        <canvas
          ref={canvasRef}
          width={400}
          height={400}
          className="minimap-canvas"
        />
        {error && !isEnabled && (
          <div className="minimap-info">Enable to view map</div>
        )}
        {error && isEnabled && (
          <div className="minimap-error">
            {error.includes('connect') ? 'War Thunder not running' : 'Map unavailable'}
          </div>
        )}
        {mapData && isEnabled && (
          <div className="minimap-stats">
            <div>Objects: {mapData.objects.length}</div>
            {mapData.player_position && (
              <div>
                Position: ({mapData.player_position[0].toFixed(2)}, {mapData.player_position[1].toFixed(2)})
              </div>
            )}
            {mapData.player_heading !== null && (
              <div>Heading: {mapData.player_heading.toFixed(0)}°</div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}


