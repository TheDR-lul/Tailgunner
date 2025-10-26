import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Map } from 'lucide-react';
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

interface UnitDistance {
  distance: number;
  type: string;
  position: [number, number];
  bearing: number;  // Bearing/azimuth in degrees
  isEnemy: boolean; // true for enemies, false for allies
}

interface MapMarker {
  id: string;
  x: number;  // Normalized coords (0..1)
  y: number;
  label?: string;
  color?: string;
}

interface MeasurementData {
  distance: number;
  bearing: number;
  gridRef: string;
  point1: { x: number; y: number };
  point2: { x: number; y: number };
}

type MapMode = 'current' | 'full';
type MapTool = 'none' | 'measure' | 'distance' | 'marker';

export function MiniMap() {
  const { t } = useTranslation();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [mapData, setMapData] = useState<MapData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isEnabled, setIsEnabled] = useState(false);
  const [mapImage, setMapImage] = useState<HTMLImageElement | null>(null);
  const [currentMapGen, setCurrentMapGen] = useState<number>(-1);
  const [nearestEnemies, setNearestEnemies] = useState<UnitDistance[]>([]);
  const [nearestAllies, setNearestAllies] = useState<UnitDistance[]>([]);
  const [mapMode, setMapMode] = useState<MapMode>('current');
  const [zoomLevel, setZoomLevel] = useState<number>(1.0);
  const [panOffset, setPanOffset] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [followPlayer, setFollowPlayer] = useState<boolean>(false); // OFF по умолчанию
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [dragStart, setDragStart] = useState<{ x: number; y: number } | null>(null);
  const [activeTool, setActiveTool] = useState<MapTool>('none');
  const [markers, setMarkers] = useState<MapMarker[]>([]);
  const [measurePoint, setMeasurePoint] = useState<{ x: number; y: number } | null>(null);
  const [measurement, setMeasurement] = useState<MeasurementData | null>(null);
  const [updateInterval, setUpdateInterval] = useState<number>(() => 
    parseInt(localStorage.getItem('mapUpdateInterval') || '200')
  );
  const [canvasSize, setCanvasSize] = useState<number>(500);
  const canvasWrapperRef = useRef<HTMLDivElement>(null);

  // Listen for localStorage changes from DebugConsole
  useEffect(() => {
    const handleStorageChange = () => {
      const newInterval = parseInt(localStorage.getItem('mapUpdateInterval') || '200');
      setUpdateInterval(newInterval);
    };
    
    window.addEventListener('localStorageChange', handleStorageChange);
    return () => window.removeEventListener('localStorageChange', handleStorageChange);
  }, []);

  // Update canvas size based on wrapper size - FORCE SQUARE 1:1
  useEffect(() => {
    const wrapper = canvasWrapperRef.current;
    if (!wrapper) return;

    const updateSize = () => {
      const rect = wrapper.getBoundingClientRect();
      // Force square: use minimum of width/height
      const size = Math.min(rect.width, rect.height);
      setCanvasSize(size);
      
      // Force wrapper to be square
      wrapper.style.width = `${size}px`;
      wrapper.style.height = `${size}px`;
    };

    updateSize();
    const resizeObserver = new ResizeObserver(updateSize);
    resizeObserver.observe(wrapper);

    return () => resizeObserver.disconnect();
  }, []);

  // Clamp pan offset to keep map within bounds
  const clampPanOffset = (offset: { x: number; y: number }, canvasSize: number): { x: number; y: number } => {
    // If zoom <= 1.0, don't allow panning (map fills canvas or smaller)
    if (zoomLevel <= 1.0) {
      return { x: 0, y: 0 };
    }
    
    const scaledSize = canvasSize * zoomLevel;
    const maxOffset = (scaledSize - canvasSize) / 2;
    
    return {
      x: Math.max(-maxOffset, Math.min(maxOffset, offset.x)),
      y: Math.max(-maxOffset, Math.min(maxOffset, offset.y))
    };
  };

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

  // Handle canvas click for tools
  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!mapData || activeTool === 'none') return;
    
    const canvas = canvasRef.current;
    if (!canvas) return;

    // Get click position relative to canvas
    const rect = canvas.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const clickY = e.clientY - rect.top;

    // Convert from canvas pixel coords to map space (0..1)
    // Account for zoom and pan transforms
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Save current transform
    ctx.save();
    
    // Apply same transforms as drawing
    if (followPlayer && mapData.player_position) {
      const mapWidth = mapData.info.map_max[0] - mapData.info.map_min[0];
      const mapHeight = mapData.info.map_max[1] - mapData.info.map_min[1];
      const worldX = mapData.player_position[0] * mapWidth + mapData.info.map_min[0];
      const worldY = (1 - mapData.player_position[1]) * mapHeight + mapData.info.map_min[1];
      
      let canvasX, canvasY;
      if (mapMode === 'full') {
        canvasX = (worldX - mapData.info.map_min[0]) / mapWidth * canvas.width;
        canvasY = (mapData.info.map_max[1] - worldY) / mapHeight * canvas.height;
      } else {
        canvasX = (worldX - mapData.info.grid_zero[0]) / mapData.info.grid_size[0] * canvas.width;
        canvasY = (mapData.info.grid_zero[1] - worldY) / mapData.info.grid_size[1] * canvas.height;
      }
      
      ctx.translate(canvas.width / 2, canvas.height / 2);
      ctx.scale(zoomLevel, zoomLevel);
      ctx.translate(-canvasX, -canvasY);
    } else {
      ctx.translate(panOffset.x, panOffset.y);
      ctx.translate(canvas.width / 2, canvas.height / 2);
      ctx.scale(zoomLevel, zoomLevel);
      ctx.translate(-canvas.width / 2, -canvas.height / 2);
    }

    // Get inverse transform
    const transform = ctx.getTransform();
    const inverse = transform.inverse();
    
    // Transform click coords
    const transformedPoint = new DOMPoint(clickX, clickY).matrixTransform(inverse);
    
    ctx.restore();

    // Convert canvas coords to normalized map coords (0..1)
    let mapX, mapY;
    if (mapMode === 'full') {
      mapX = transformedPoint.x / canvas.width;
      mapY = transformedPoint.y / canvas.height;
    } else {
      mapX = transformedPoint.x / canvas.width;
      mapY = transformedPoint.y / canvas.height;
    }

    // Convert to API coords (full map 0..1)
    const mapWidth = mapData.info.map_max[0] - mapData.info.map_min[0];
    const mapHeight = mapData.info.map_max[1] - mapData.info.map_min[1];
    
    let worldX, worldY;
    if (mapMode === 'full') {
      worldX = mapX * mapWidth + mapData.info.map_min[0];
      worldY = mapData.info.map_max[1] - mapY * mapHeight;
    } else {
      worldX = mapX * mapData.info.grid_size[0] + mapData.info.grid_zero[0];
      worldY = mapData.info.grid_zero[1] - mapY * mapData.info.grid_size[1];
    }
    
    const apiX = (worldX - mapData.info.map_min[0]) / mapWidth;
    const apiY = 1 - (worldY - mapData.info.map_min[1]) / mapHeight;

    // Clamp to 0..1
    const clampedX = Math.max(0, Math.min(1, apiX));
    const clampedY = Math.max(0, Math.min(1, apiY));

    if (activeTool === 'measure') {
      if (!measurePoint) {
        // First click - set start point
        setMeasurePoint({ x: clampedX, y: clampedY });
        setMeasurement(null);
      } else {
        // Second click - set end point and calculate
        calculateMeasurement(measurePoint.x, measurePoint.y, clampedX, clampedY);
      }
    } else if (activeTool === 'distance') {
      // Measure from player to clicked point
      if (mapData.player_position) {
        calculateMeasurement(mapData.player_position[0], mapData.player_position[1], clampedX, clampedY);
      }
    } else if (activeTool === 'marker') {
      addMarker(clampedX, clampedY);
    }
  };

  const calculateMeasurement = (x1: number, y1: number, x2: number, y2: number) => {
    if (!mapData) return;

    const info = mapData.info;
    const mapWidth = info.map_max[0] - info.map_min[0];
    const mapHeight = info.map_max[1] - info.map_min[1];

    // Convert both points to world space (meters)
    const point1WorldX = x1 * mapWidth + info.map_min[0];
    const point1WorldY = (1 - y1) * mapHeight + info.map_min[1];
    const point2WorldX = x2 * mapWidth + info.map_min[0];
    const point2WorldY = (1 - y2) * mapHeight + info.map_min[1];

    // Calculate distance
    const dx = point2WorldX - point1WorldX;
    const dy = point2WorldY - point1WorldY;
    const distance = Math.sqrt(dx * dx + dy * dy);

    // Calculate bearing from point1 to point2
    const angleFromX = Math.atan2(dy, dx) * (180 / Math.PI);
    const bearing = (90 - angleFromX + 360) % 360;

    // Calculate grid reference for end point
    let gridRef = '?';
    if (mapMode === 'current') {
      const posX = point2WorldX - info.grid_zero[0];
      const posY = info.grid_zero[1] - point2WorldY;
      const gridX = Math.floor(posX / info.grid_steps[0]);
      const gridY = Math.floor(posY / info.grid_steps[1]);
      if (gridY >= 0 && gridY < 26) {
        const letter = String.fromCharCode(65 + gridY);
        gridRef = `${letter}-${gridX + 1}`;
      }
    }

    setMeasurement({ 
      distance, 
      bearing, 
      gridRef,
      point1: { x: x1, y: y1 },
      point2: { x: x2, y: y2 }
    });
  };

  const addMarker = (x: number, y: number) => {
    const newMarker: MapMarker = {
      id: Date.now().toString(),
      x,
      y,
      color: '#ff9933',
    };
    setMarkers(prev => [...prev, newMarker]);
  };

  const clearMarkers = () => {
    setMarkers([]);
    setMeasurePoint(null);
    setMeasurement(null);
  };

  // Handle zoom with mouse wheel
  useEffect(() => {
    const wrapper = canvasWrapperRef.current;
    if (!wrapper) return;

    const handleWheel = (e: WheelEvent) => {
      e.preventDefault();
      const delta = e.deltaY > 0 ? -0.1 : 0.1;
      const newZoom = Math.max(0.8, Math.min(6.0, zoomLevel + delta));
      setZoomLevel(newZoom);
      
      // Clamp pan offset to new zoom bounds
      const canvas = canvasRef.current;
      if (canvas) {
        setPanOffset(prev => clampPanOffset(prev, canvas.width));
      }
      setFollowPlayer(false); // Disable follow when zooming
    };

    wrapper.addEventListener('wheel', handleWheel, { passive: false });
    return () => wrapper.removeEventListener('wheel', handleWheel);
  }, [zoomLevel, clampPanOffset]);

  // Handle drag for panning
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const handleMouseDown = (e: MouseEvent) => {
      // Don't start dragging if tool is active
      if (activeTool !== 'none') return;
      
      setIsDragging(true);
      setDragStart({ x: e.clientX, y: e.clientY });
      setFollowPlayer(false); // Disable follow when dragging
      canvas.style.cursor = 'grabbing';
    };

    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging || !dragStart) return;

      const dx = e.clientX - dragStart.x;
      const dy = e.clientY - dragStart.y;
      
      // 1:1 movement - no acceleration, clamped to bounds
      setPanOffset(prev => {
        const newOffset = {
          x: prev.x + dx,
          y: prev.y + dy
        };
        // Use actual canvas size (assumes square canvas, use width)
        const canvasSize = canvas.width;
        return clampPanOffset(newOffset, canvasSize);
      });
      
      setDragStart({ x: e.clientX, y: e.clientY });
    };

    const handleMouseUp = () => {
      setIsDragging(false);
      setDragStart(null);
      canvas.style.cursor = 'grab';
    };

    canvas.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);

    return () => {
      canvas.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, dragStart, zoomLevel, clampPanOffset, activeTool]);

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
    }, updateInterval); // Use configurable update interval

    return () => clearInterval(interval);
  }, [isEnabled, currentMapGen, updateInterval]);

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

    // Apply zoom and pan transforms
    ctx.save();
    
    if (followPlayer && mapData.player_position) {
      // Follow mode: center on player
      // Convert player API coords to canvas coords
      const mapWidth = mapData.info.map_max[0] - mapData.info.map_min[0];
      const mapHeight = mapData.info.map_max[1] - mapData.info.map_min[1];
      const worldX = mapData.player_position[0] * mapWidth + mapData.info.map_min[0];
      const worldY = (1 - mapData.player_position[1]) * mapHeight + mapData.info.map_min[1];
      
      let canvasX, canvasY;
      if (mapMode === 'full') {
        canvasX = (worldX - mapData.info.map_min[0]) / mapWidth * canvas.width;
        canvasY = (mapData.info.map_max[1] - worldY) / mapHeight * canvas.height;
      } else {
        canvasX = (worldX - mapData.info.grid_zero[0]) / mapData.info.grid_size[0] * canvas.width;
        canvasY = (mapData.info.grid_zero[1] - worldY) / mapData.info.grid_size[1] * canvas.height;
      }
      
      // Center canvas, zoom, then center on player
      ctx.translate(canvas.width / 2, canvas.height / 2);
      ctx.scale(zoomLevel, zoomLevel);
      ctx.translate(-canvasX, -canvasY);
    } else {
      // Free mode: pan in screen space (before zoom)
      ctx.translate(panOffset.x, panOffset.y);
      ctx.translate(canvas.width / 2, canvas.height / 2);
      ctx.scale(zoomLevel, zoomLevel);
      ctx.translate(-canvas.width / 2, -canvas.height / 2);
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

    // Calculate nearby units (enemies and allies) distances
    const playerObj = mapData.objects.find(obj => obj.icon === 'Player');
    if (playerObj && playerObj.x !== undefined && playerObj.y !== undefined) {
      calculateNearbyUnits(playerObj, mapData.objects, mapData.info);
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
      drawPlayer(ctx, canvas, playerObj, mapData.info, zoomLevel);
    }

    // Draw markers and measurements (after zoom/pan transform)
    drawMarkersAndMeasurements(ctx, canvas, playerObj, mapData.info, zoomLevel);

    // Restore transform
    ctx.restore();
  }, [mapData, mapImage, mapMode, zoomLevel, panOffset, followPlayer, markers, measurePoint, measurement, activeTool, canvasSize]);

  const calculateNearbyUnits = (
    player: MapObject,
    objects: MapObject[],
    info: MapInfo
  ) => {
    if (!player.x || !player.y) return;

    const enemies: UnitDistance[] = [];
    const allies: UnitDistance[] = [];
    
    // Convert player API coords to world coords (meters)
    // NOTE: API Y is inverted! y=0 is North (top), y=1 is South (bottom)
    const mapWidth = info.map_max[0] - info.map_min[0];
    const mapHeight = info.map_max[1] - info.map_min[1];
    const playerWorldX = player.x * mapWidth + info.map_min[0];
    const playerWorldY = (1 - player.y) * mapHeight + info.map_min[1];

    for (const obj of objects) {
      // Check if object has position
      if (obj.x === undefined || obj.y === undefined) continue;
      
      // Skip player
      if (obj.icon === 'Player') continue;
      
      // Filter only vehicles (ground, aircraft, ships)
      if (obj.type !== 'ground_model' && obj.type !== 'aircraft' && obj.type !== 'Ship') continue;
      
      // Determine if enemy or ally
      // Enemy color: #fa0C00 (red)
      // Ally color: #174DFF (blue)
      const isEnemy = (obj.color_rgb && 
                       obj.color_rgb[0] > 240 && 
                       obj.color_rgb[1] < 20 && 
                       obj.color_rgb[2] < 20) ||
                      obj.color.toLowerCase() === '#fa0c00';
      
      const isAlly = (obj.color_rgb && 
                      obj.color_rgb[0] < 50 && 
                      obj.color_rgb[1] > 60 && 
                      obj.color_rgb[2] > 200) ||
                     obj.color.toLowerCase() === '#174dff';
      
      if (!isEnemy && !isAlly) continue;

      // Convert API coords to world coords (meters)
      const unitWorldX = obj.x * mapWidth + info.map_min[0];
      const unitWorldY = (1 - obj.y) * mapHeight + info.map_min[1]; // API Y inverted!
      
      // Calculate distance in real-world meters
      const distX = unitWorldX - playerWorldX;
      const distY = unitWorldY - playerWorldY;
      const distance = Math.sqrt(distX * distX + distY * distY);
      
      // Calculate bearing/azimuth (0° = North, clockwise)
      const angleFromX = Math.atan2(distY, distX) * (180 / Math.PI);
      const bearing = (90 - angleFromX + 360) % 360;

      const unit: UnitDistance = {
        distance,
        type: obj.icon || obj.type,
        position: [obj.x, obj.y],
        bearing,
        isEnemy,
      };
      
      if (isEnemy) {
        enemies.push(unit);
      } else {
        allies.push(unit);
      }
    }

    // Sort by distance and keep top 5
    enemies.sort((a, b) => a.distance - b.distance);
    allies.sort((a, b) => a.distance - b.distance);
    setNearestEnemies(enemies.slice(0, 5));
    setNearestAllies(allies.slice(0, 5));
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

    // Icon sizes - bigger in Current mode for better visibility
    const baseSize = mapMode === 'current' ? 5 : 3;
    const size = obj.type === 'capture_zone' ? (mapMode === 'current' ? 8 : 6) : baseSize;

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

  const drawMarkersAndMeasurements = (
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    player: MapObject | undefined,
    info: MapInfo,
    currentZoom: number
  ) => {
    // Helper to convert map coords (0..1) to canvas coords
    const toCanvasCoords = (x: number, y: number) => {
      const mapWidth = info.map_max[0] - info.map_min[0];
      const mapHeight = info.map_max[1] - info.map_min[1];
      const worldX = x * mapWidth + info.map_min[0];
      const worldY = (1 - y) * mapHeight + info.map_min[1];
      
      let visibleX, visibleY;
      if (mapMode === 'full') {
        visibleX = (worldX - info.map_min[0]) / mapWidth;
        visibleY = (info.map_max[1] - worldY) / mapHeight;
      } else {
        visibleX = (worldX - info.grid_zero[0]) / info.grid_size[0];
        visibleY = (info.grid_zero[1] - worldY) / info.grid_size[1];
      }
      return { x: visibleX * canvas.width, y: visibleY * canvas.height };
    };

    // Draw measurement line and points (scale inversely with zoom)
    if (measurement) {
      const point1Pos = toCanvasCoords(measurement.point1.x, measurement.point1.y);
      const point2Pos = toCanvasCoords(measurement.point2.x, measurement.point2.y);

      // Inverse scale - smaller when zoomed in
      const scale = 1 / currentZoom;

      ctx.save();
      // Draw line between points
      ctx.strokeStyle = '#ff9933';
      ctx.lineWidth = 2 * scale;
      ctx.setLineDash([5 * scale, 5 * scale]);
      ctx.beginPath();
      ctx.moveTo(point1Pos.x, point1Pos.y);
      ctx.lineTo(point2Pos.x, point2Pos.y);
      ctx.stroke();
      ctx.setLineDash([]);

      // Draw point 1 (start) - smaller
      ctx.fillStyle = '#ff9933';
      ctx.strokeStyle = '#000';
      ctx.lineWidth = 1.5 * scale;
      ctx.beginPath();
      ctx.arc(point1Pos.x, point1Pos.y, 3 * scale, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      // Draw point 2 (end) with crosshair - smaller
      ctx.beginPath();
      ctx.arc(point2Pos.x, point2Pos.y, 3 * scale, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      // Draw crosshair on end point
      const crossSize = 6 * scale;
      ctx.strokeStyle = '#ff9933';
      ctx.lineWidth = 1.5 * scale;
      ctx.beginPath();
      ctx.moveTo(point2Pos.x - crossSize, point2Pos.y);
      ctx.lineTo(point2Pos.x + crossSize, point2Pos.y);
      ctx.moveTo(point2Pos.x, point2Pos.y - crossSize);
      ctx.lineTo(point2Pos.x, point2Pos.y + crossSize);
      ctx.stroke();

      // Draw measurement info text above end point
      const fontSize = Math.max(6, 11 * scale); // Min 6px, more aggressive scaling
      ctx.font = `bold ${fontSize}px sans-serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'bottom';
      
      const distance = `${measurement.distance.toFixed(0)}m`;
      const bearing = `${measurement.bearing.toFixed(0)}°`;
      const gridRef = measurement.gridRef !== '?' ? measurement.gridRef : '';
      
      const line1 = `${distance} @ ${bearing}`;
      const line2 = gridRef;
      
      // Draw background for better readability (transparent)
      const padding = 4 * scale;
      const line1Width = ctx.measureText(line1).width;
      const line2Width = line2 ? ctx.measureText(line2).width : 0;
      const maxWidth = Math.max(line1Width, line2Width);
      const boxWidth = maxWidth + padding * 2;
      const boxHeight = (line2 ? 26 : 16) * scale;
      
      ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
      ctx.fillRect(
        point2Pos.x - boxWidth / 2,
        point2Pos.y - crossSize - boxHeight - 4 * scale,
        boxWidth,
        boxHeight
      );
      
      // Draw text with stroke for better readability
      ctx.lineWidth = 2 * scale;
      
      // Line 1: distance @ bearing
      ctx.strokeStyle = '#000';
      ctx.strokeText(line1, point2Pos.x, point2Pos.y - crossSize - 4 * scale);
      ctx.fillStyle = '#ff9933';
      ctx.fillText(line1, point2Pos.x, point2Pos.y - crossSize - 4 * scale);
      
      // Line 2: grid reference
      if (line2) {
        ctx.strokeStyle = '#000';
        ctx.strokeText(line2, point2Pos.x, point2Pos.y - crossSize - 4 * scale - 12 * scale);
        ctx.fillStyle = '#64c8ff';
        ctx.fillText(line2, point2Pos.x, point2Pos.y - crossSize - 4 * scale - 12 * scale);
      }

      ctx.restore();
    } else if (measurePoint && activeTool === 'measure') {
      // Only start point set for 'measure' tool, draw it - smaller
      const pointPos = toCanvasCoords(measurePoint.x, measurePoint.y);
      
      const scale = 1 / currentZoom;
      
      ctx.save();
      ctx.fillStyle = '#ff9933';
      ctx.strokeStyle = '#000';
      ctx.lineWidth = 1.5 * scale;
      ctx.beginPath();
      ctx.arc(pointPos.x, pointPos.y, 3 * scale, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
      ctx.restore();
    }

    // Draw markers (scale inversely with zoom)
    for (const marker of markers) {
      const pos = toCanvasCoords(marker.x, marker.y);
      
      const scale = 1 / currentZoom;
      
      ctx.save();
      ctx.fillStyle = marker.color || '#ff9933';
      ctx.strokeStyle = '#000';
      ctx.lineWidth = 1.5 * scale;
      
      // Draw marker pin - smaller
      const size = 4 * scale;
      ctx.beginPath();
      ctx.arc(pos.x, pos.y, size, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      // Calculate marker info (distance from player, bearing, grid)
      if (player && player.x !== undefined && player.y !== undefined) {
        const mapWidth = info.map_max[0] - info.map_min[0];
        const mapHeight = info.map_max[1] - info.map_min[1];
        
        // Convert player and marker coords to world space
        const playerWorldX = player.x * mapWidth + info.map_min[0];
        const playerWorldY = (1 - player.y) * mapHeight + info.map_min[1];
        const markerWorldX = marker.x * mapWidth + info.map_min[0];
        const markerWorldY = (1 - marker.y) * mapHeight + info.map_min[1];
        
        // Calculate distance
        const dx = markerWorldX - playerWorldX;
        const dy = markerWorldY - playerWorldY;
        const distance = Math.sqrt(dx * dx + dy * dy);
        
        // Calculate bearing
        const angleFromX = Math.atan2(dy, dx) * (180 / Math.PI);
        const bearing = (90 - angleFromX + 360) % 360;
        
        // Calculate grid reference
        let gridRef = '';
        if (mapMode === 'current') {
          const posX = markerWorldX - info.grid_zero[0];
          const posY = info.grid_zero[1] - markerWorldY;
          const gridX = Math.floor(posX / info.grid_steps[0]);
          const gridY = Math.floor(posY / info.grid_steps[1]);
          if (gridY >= 0 && gridY < 26) {
            const letter = String.fromCharCode(65 + gridY);
            gridRef = `${letter}-${gridX + 1}`;
          }
        }
        
        // Draw info text above marker
        const fontSize = Math.max(5, 10 * scale); // Min 5px
        ctx.font = `bold ${fontSize}px sans-serif`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'bottom';
        
        const distText = `${distance.toFixed(0)}m`;
        const bearText = `${bearing.toFixed(0)}°`;
        const line1 = `${distText} @ ${bearText}`;
        const line2 = gridRef;
        
        // Draw background
        const padding = 3 * scale;
        const line1Width = ctx.measureText(line1).width;
        const line2Width = line2 ? ctx.measureText(line2).width : 0;
        const maxWidth = Math.max(line1Width, line2Width);
        const boxWidth = maxWidth + padding * 2;
        const boxHeight = (line2 ? 22 : 13) * scale;
        
        ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
        ctx.fillRect(
          pos.x - boxWidth / 2,
          pos.y - size - boxHeight - 3 * scale,
          boxWidth,
          boxHeight
        );
        
        // Draw text
        ctx.lineWidth = 1.5 * scale;
        
        // Line 1: distance @ bearing
        ctx.strokeStyle = '#000';
        ctx.strokeText(line1, pos.x, pos.y - size - 3 * scale);
        ctx.fillStyle = '#ff9933';
        ctx.fillText(line1, pos.x, pos.y - size - 3 * scale);
        
        // Line 2: grid reference
        if (line2) {
          ctx.strokeStyle = '#000';
          ctx.strokeText(line2, pos.x, pos.y - size - 3 * scale - 10 * scale);
          ctx.fillStyle = '#64c8ff';
          ctx.fillText(line2, pos.x, pos.y - size - 3 * scale - 10 * scale);
        }
      }
      
      ctx.restore();
    }
  };

  const drawPlayer = (
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    obj: MapObject,
    info: MapInfo,
    currentZoom: number
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

    // Draw player as arrow with fixed size (compensate for zoom)
    ctx.save();
    ctx.translate(x, y);
    
    // Compensate zoom to keep icon fixed size
    const zoomCompensation = 1 / currentZoom;
    ctx.scale(zoomCompensation, zoomCompensation);
    
    ctx.rotate(angle);

    ctx.fillStyle = '#faC81E';
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1.5;

    // Bigger arrow in Current mode for better visibility
    const arrowSize = mapMode === 'current' ? 10 : 8;
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
      <div style={{ display: 'flex', gap: '16px', width: '100%', alignItems: 'flex-start' }}>
        {/* Left side - Map Canvas */}
        <div style={{ flex: '1 1 auto', minWidth: 0 }}>
          <div className="minimap-header">
            <div className="minimap-title">
              <Map size={18} />
              <h3>{t('map.title')}</h3>
            </div>
            <div style={{ display: 'flex', gap: '6px', alignItems: 'center' }}>
              <button
                className={`btn btn-${mapMode === 'current' ? 'primary' : 'secondary'}`}
                onClick={() => setMapMode('current')}
                disabled={!isEnabled}
                title="Current vehicle view"
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
              <button
                className={`btn btn-${followPlayer ? 'primary' : 'secondary'}`}
                onClick={() => {
                  setFollowPlayer(!followPlayer);
                  if (!followPlayer) {
                    setPanOffset({ x: 0, y: 0 });
                  }
                }}
                disabled={!isEnabled}
                title="Center map on player"
                style={{ fontSize: '11px', padding: '4px 8px' }}
              >
                📍 Follow
              </button>
              <button
                className="btn btn-secondary"
                onClick={() => {
                  setZoomLevel(1.0);
                  setPanOffset({ x: 0, y: 0 });
                  setFollowPlayer(false);
                }}
                disabled={!isEnabled}
                title="Reset view"
                style={{ fontSize: '11px', padding: '4px 8px' }}
              >
                🔄 Reset
              </button>
              <button 
                className={`btn btn-${isEnabled ? 'primary' : 'secondary'}`}
                onClick={() => setIsEnabled(!isEnabled)}
                title={t('map.rb_warning')}
              >
                {isEnabled ? t('map.enabled') : t('map.disabled')}
              </button>
            </div>
          </div>
      <div 
        className="minimap-canvas-wrapper" 
        ref={canvasWrapperRef}
        title={`Zoom: ${(zoomLevel * 100).toFixed(0)}% (scroll to zoom)`}
      >
        <canvas
          ref={canvasRef}
          width={canvasSize}
          height={canvasSize}
          className="minimap-canvas"
          style={{ cursor: isDragging ? 'grabbing' : activeTool !== 'none' ? 'crosshair' : 'grab' }}
          onClick={handleCanvasClick}
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
        </div>

        {/* Right side - Tools & Info */}
        <div style={{
          flex: '0 0 auto',
          width: '280px',
          minWidth: '250px',
          maxWidth: '350px',
          padding: '16px',
          paddingLeft: '0',
          display: 'flex',
          flexDirection: 'column',
          gap: '12px',
          overflowY: 'auto',
          maxHeight: '100%',
        }}>
          {/* Player Info */}
          <div style={{ 
            borderBottom: '1px solid var(--border)', 
            paddingBottom: '12px' 
          }}>
            <h4 style={{ margin: 0, fontSize: '14px', color: 'var(--text-primary)', marginBottom: '8px' }}>
              👤 Player Info
            </h4>
            {isEnabled && mapData ? (
              <div style={{ fontSize: '11px', color: 'var(--text-secondary)' }}>
                {mapData.player_grid && (
                  <div style={{ color: '#22c55e', fontWeight: 'bold' }}>
                    🎯 Grid: {mapData.player_grid}
                  </div>
                )}
                {mapData.player_heading !== null && (
                  <div>🧭 {mapData.player_heading.toFixed(0)}° {getCompassDirection(mapData.player_heading)}</div>
                )}
              </div>
            ) : (
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', fontStyle: 'italic' }}>
                —
              </div>
            )}
          </div>

          {/* Tools */}
          {isEnabled && (
            <div style={{ 
              borderBottom: '1px solid var(--border)', 
              paddingBottom: '12px' 
            }}>
              <h4 style={{ margin: 0, fontSize: '14px', color: 'var(--text-primary)', marginBottom: '8px' }}>
                🛠️ Tools
              </h4>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                <button
                  className={`btn btn-${activeTool === 'measure' ? 'primary' : 'secondary'}`}
                  onClick={() => {
                    setActiveTool(activeTool === 'measure' ? 'none' : 'measure');
                    if (activeTool === 'measure') clearMarkers();
                  }}
                  style={{ fontSize: '11px', padding: '6px 10px', justifyContent: 'flex-start' }}
                >
                  📏 Measure (2 points)
                </button>
                <button
                  className={`btn btn-${activeTool === 'distance' ? 'primary' : 'secondary'}`}
                  onClick={() => {
                    setActiveTool(activeTool === 'distance' ? 'none' : 'distance');
                    if (activeTool === 'distance') clearMarkers();
                  }}
                  style={{ fontSize: '11px', padding: '6px 10px', justifyContent: 'flex-start' }}
                >
                  📏 Distance from Player
                </button>
                <button
                  className={`btn btn-${activeTool === 'marker' ? 'primary' : 'secondary'}`}
                  onClick={() => setActiveTool(activeTool === 'marker' ? 'none' : 'marker')}
                  style={{ fontSize: '11px', padding: '6px 10px', justifyContent: 'flex-start' }}
                >
                  📍 Place Marker
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={clearMarkers}
                  disabled={markers.length === 0 && !measurePoint && !measurement}
                  style={{ fontSize: '11px', padding: '6px 10px', justifyContent: 'flex-start' }}
                >
                  🗑️ Clear All
                </button>
              </div>
            </div>
          )}

          {/* Measurement Info */}
          {measurement && (
            <div style={{
              background: 'rgba(255, 153, 51, 0.1)',
              border: '1px solid #ff9933',
              borderRadius: '4px',
              padding: '12px',
            }}>
              <div style={{ fontWeight: 'bold', marginBottom: '8px', color: '#ff9933', fontSize: '12px' }}>
                📏 Measurement
              </div>
              <div style={{ fontSize: '11px', color: 'var(--text-secondary)' }}>
                <div>Distance: <span style={{ fontWeight: 'bold', color: '#ff9933' }}>{measurement.distance.toFixed(0)}m</span></div>
                <div>Bearing: <span style={{ fontWeight: 'bold', color: '#ff9933' }}>{measurement.bearing.toFixed(0)}° {getCompassDirection(measurement.bearing)}</span></div>
                {measurement.gridRef !== '?' && (
                  <div>Grid: <span style={{ fontWeight: 'bold', color: '#ff9933' }}>{measurement.gridRef}</span></div>
                )}
              </div>
            </div>
          )}

          {/* Active Tool Hint */}
          {activeTool !== 'none' && !measurement && (
            <div style={{
              background: 'rgba(100, 200, 255, 0.1)',
              border: '1px solid #64c8ff',
              borderRadius: '4px',
              padding: '8px',
              fontSize: '10px',
              color: '#64c8ff',
              fontStyle: 'italic',
            }}>
              {activeTool === 'measure' && !measurePoint && '🖱️ Click to set start point'}
              {activeTool === 'measure' && measurePoint && !measurement && '🖱️ Click to set end point'}
              {activeTool === 'distance' && '🖱️ Click to measure from player'}
              {activeTool === 'marker' && '🖱️ Click to place marker'}
            </div>
          )}

          {/* Markers Count */}
          {markers.length > 0 && (
            <div style={{
              fontSize: '11px',
              color: 'var(--text-secondary)',
            }}>
              📍 {markers.length} marker{markers.length > 1 ? 's' : ''} placed
            </div>
          )}

          {/* Enemies & Allies in columns */}
          {isEnabled && mapData && (
            <div style={{ 
              borderTop: '1px solid var(--border)', 
              paddingTop: '12px',
              display: 'grid',
              gridTemplateColumns: '1fr 1fr',
              gap: '12px'
            }}>
              {/* Enemies Column */}
              <div>
                <div style={{ fontWeight: 'bold', marginBottom: '6px', color: '#ff6666', fontSize: '12px' }}>
                  🎯 Nearest Enemies
                </div>
                {nearestEnemies.length > 0 ? (
                  nearestEnemies.map((enemy, idx) => (
                    <div key={idx} style={{ fontSize: '10px', color: '#ff6666', marginBottom: '4px' }}>
                      {idx + 1}. {enemy.type}:<br/>
                      {enemy.distance.toFixed(0)}m @ {enemy.bearing.toFixed(0)}° {getCompassDirection(enemy.bearing)}
                    </div>
                  ))
                ) : (
                  <div style={{ fontSize: '10px', color: '#666', fontStyle: 'italic' }}>
                    —
                  </div>
                )}
              </div>

              {/* Allies Column */}
              <div>
                <div style={{ fontWeight: 'bold', marginBottom: '6px', color: '#64a8ff', fontSize: '12px' }}>
                  🛡️ Nearest Allies
                </div>
                {nearestAllies.length > 0 ? (
                  nearestAllies.map((ally, idx) => (
                    <div key={idx} style={{ fontSize: '10px', color: '#64a8ff', marginBottom: '4px' }}>
                      {idx + 1}. {ally.type}:<br/>
                      {ally.distance.toFixed(0)}m @ {ally.bearing.toFixed(0)}° {getCompassDirection(ally.bearing)}
                    </div>
                  ))
                ) : (
                  <div style={{ fontSize: '10px', color: '#666', fontStyle: 'italic' }}>
                    —
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}


