import { useState, useRef, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';

interface CurvePoint {
  x: number;
  y: number;
}

export interface VibrationNodeData {
  duration: number;
  curve: CurvePoint[];
  randomMin?: number;
  randomMax?: number;
  mode?: 'once' | 'continuous' | 'repeat';
  repeatCount?: number;
}

export function VibrationNode({ data, id, selected }: { data: VibrationNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [duration, setDuration] = useState(data.duration || 1.0);
  const [curve, setCurve] = useState<CurvePoint[]>(data.curve || [
    { x: 0.4, y: 0.6 },
    { x: 0.6, y: 0.8 }
  ]);
  const [mode, setMode] = useState<'once' | 'continuous' | 'repeat'>(data.mode || 'once');
  const [repeatCount, setRepeatCount] = useState(data.repeatCount || 3);
  const [enableRandom, setEnableRandom] = useState(!!data.randomMin);
  const [randomMin, setRandomMin] = useState(data.randomMin || 0.3);
  const [randomMax, setRandomMax] = useState(data.randomMax || 0.8);
  
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [draggedPointIndex, setDraggedPointIndex] = useState<number | null>(null);
  const [hoveredPointIndex, setHoveredPointIndex] = useState<number | null>(null);
  
  // Drawing constants
  const POINT_RADIUS = 6;
  const HOVER_RADIUS = 8;
  
  // Sync with data prop when changed (loading saved pattern)
  useEffect(() => {
    if (data.duration !== undefined) setDuration(data.duration);
    if (data.curve) setCurve(data.curve);
    if (data.mode) setMode(data.mode);
    if (data.repeatCount !== undefined) setRepeatCount(data.repeatCount);
    if (data.randomMin !== undefined) {
      setEnableRandom(true);
      setRandomMin(data.randomMin);
    }
    if (data.randomMax !== undefined) setRandomMax(data.randomMax);
  }, [data]);
  
  // Update data prop when local state changes
  useEffect(() => {
    data.duration = duration;
    data.curve = curve;
    data.mode = mode;
    data.repeatCount = repeatCount;
    if (enableRandom) {
      data.randomMin = randomMin;
      data.randomMax = randomMax;
    }
  }, [duration, curve, mode, repeatCount, enableRandom, randomMin, randomMax, data]);
  
  // Draw graph
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Clear
    ctx.clearRect(0, 0, width, height);
    
    // Background
    ctx.fillStyle = '#1a1d29';
    ctx.fillRect(0, 0, width, height);
    
    // Grid
    ctx.strokeStyle = 'rgba(148, 163, 184, 0.1)';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 10; i++) {
      const x = (width / 10) * i;
      const y = (height / 10) * i;
      
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
      
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }
    
    // Center line
    ctx.strokeStyle = 'rgba(148, 163, 184, 0.2)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(0, height / 2);
    ctx.lineTo(width, height / 2);
    ctx.stroke();
    
    // Curve
    if (curve.length > 0) {
      ctx.strokeStyle = '#00d4ff';
      ctx.lineWidth = 3;
      ctx.lineCap = 'round';
      ctx.lineJoin = 'round';
      ctx.shadowColor = 'rgba(0, 212, 255, 0.6)';
      ctx.shadowBlur = 10;
      
      ctx.beginPath();
      curve.forEach((point, index) => {
        const canvasX = point.x * width;
        const canvasY = (1 - point.y) * height;
        
        if (index === 0) {
          ctx.moveTo(canvasX, canvasY);
        } else {
          ctx.lineTo(canvasX, canvasY);
        }
      });
      ctx.stroke();
      
      // Reset shadow for points
      ctx.shadowColor = 'transparent';
      ctx.shadowBlur = 0;
      
      // Points
      curve.forEach((point, index) => {
        const canvasX = point.x * width;
        const canvasY = (1 - point.y) * height;
        const isHovered = index === hoveredPointIndex;
        const isDragged = index === draggedPointIndex;
        
        // Outer circle (hover)
        if (isHovered || isDragged) {
          ctx.fillStyle = 'rgba(0, 212, 255, 0.2)';
          ctx.beginPath();
          ctx.arc(canvasX, canvasY, HOVER_RADIUS, 0, Math.PI * 2);
          ctx.fill();
        }
        
        // Main point
        ctx.fillStyle = isDragged ? '#8b5cf6' : (isHovered ? '#00d4ff' : 'white');
        ctx.strokeStyle = isDragged ? '#a78bfa' : '#00d4ff';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(canvasX, canvasY, POINT_RADIUS, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
      });
    }
  }, [curve, hoveredPointIndex, draggedPointIndex]);
  
  // Get point coordinates from mouse event
  const getPointFromEvent = (e: React.MouseEvent<HTMLCanvasElement>): CurvePoint => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: 0, y: 0 };
    
    const rect = canvas.getBoundingClientRect();
    // Use real browser size (with zoom)
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    const y = Math.max(0, Math.min(1, 1 - ((e.clientY - rect.top) / rect.height)));
    
    return { x, y };
  };
  
  // Find nearest point to click
  const findNearestPoint = (clickPoint: CurvePoint): number | null => {
    const canvas = canvasRef.current;
    if (!canvas) return null;
    
    const rect = canvas.getBoundingClientRect();
    const threshold = 15; // pixels
    const width = rect.width;
    const height = rect.height;
    
    let nearestIndex: number | null = null;
    let minDistance = threshold;
    
    curve.forEach((point, index) => {
      const canvasX = point.x * width;
      const canvasY = (1 - point.y) * height;
      const clickX = clickPoint.x * width;
      const clickY = (1 - clickPoint.y) * height;
      
      const distance = Math.sqrt(
        Math.pow(canvasX - clickX, 2) + 
        Math.pow(canvasY - clickY, 2)
      );
      
      if (distance < minDistance) {
        minDistance = distance;
        nearestIndex = index;
      }
    });
    
    return nearestIndex;
  };
  
  // Mouse down - start dragging or add point
  const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const clickPoint = getPointFromEvent(e);
    const nearestIndex = findNearestPoint(clickPoint);
    
    if (nearestIndex !== null) {
      // Start dragging existing point
      setDraggedPointIndex(nearestIndex);
    } else {
      // Add new point
      const newCurve = [...curve, clickPoint].sort((a, b) => a.x - b.x);
      setCurve(newCurve);
    }
  };
  
  // Mouse move - drag point or hover
  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const movePoint = getPointFromEvent(e);
    
    if (draggedPointIndex !== null) {
      // Drag point
      const newCurve = [...curve];
      newCurve[draggedPointIndex] = movePoint;
      
      // Re-sort by X
      newCurve.sort((a, b) => a.x - b.x);
      
      // Find new index of dragged point after sorting
      const newIndex = newCurve.findIndex(p => p.x === movePoint.x && p.y === movePoint.y);
      setDraggedPointIndex(newIndex);
      
      setCurve(newCurve);
    } else {
      // Detect hover
      const nearestIndex = findNearestPoint(movePoint);
      setHoveredPointIndex(nearestIndex);
    }
  };
  
  // Mouse up - end dragging
  const handleMouseUp = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDraggedPointIndex(null);
  };
  
  // Mouse leave - reset hover
  const handleMouseLeave = () => {
    setDraggedPointIndex(null);
    setHoveredPointIndex(null);
  };
  
  // Double click - delete point
  const handleDoubleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const clickPoint = getPointFromEvent(e);
    const nearestIndex = findNearestPoint(clickPoint);
    
    if (nearestIndex !== null && curve.length > 2) {
      const newCurve = curve.filter((_, index) => index !== nearestIndex);
      setCurve(newCurve);
    }
  };
  
  // Reset curve
  const clearCurve = () => {
    setCurve([
      { x: 0.4, y: 0.6 },
      { x: 0.6, y: 0.8 }
    ]);
  };
  
  return (
    <div className="node-vibration" onClick={(e) => e.stopPropagation()}>
      <NodeResizer 
        isVisible={selected} 
        minWidth={250} 
        minHeight={200}
        color="rgba(255, 153, 51, 0.8)"
      />
      <Handle type="target" position={Position.Left} id="trigger" />
      
      <div className="node-header">💥 {t('nodes.vibration.title')}</div>
      
      <div className="node-body" onClick={(e) => e.stopPropagation()}>
        <div className="vibration-controls">
          <div className="mode-controls">
            <label>
              {t('nodes.vibration.mode')}:
              <select
                value={mode}
                onChange={(e) => setMode(e.target.value as 'once' | 'continuous' | 'repeat')}
                className="node-input-field"
                onClick={(e) => e.stopPropagation()}
                onMouseDown={(e) => e.stopPropagation()}
              >
                <option value="once">{t('nodes.vibration.mode_once')}</option>
                <option value="continuous">{t('nodes.vibration.mode_continuous')}</option>
                <option value="repeat">{t('nodes.vibration.mode_repeat')}</option>
              </select>
            </label>
            
            {mode === 'repeat' && (
              <label>
                {t('nodes.vibration.repeat_count')}:
                <input
                  type="number"
                  value={repeatCount}
                  onChange={(e) => setRepeatCount(parseInt(e.target.value))}
                  className="node-input-field"
                  min="1"
                  max="100"
                  onClick={(e) => e.stopPropagation()}
                  onMouseDown={(e) => e.stopPropagation()}
                />
              </label>
            )}
          </div>
          
          <label>
            {t('nodes.vibration.duration')} ({t('nodes.vibration.seconds')}):
            <input
              type="number"
              value={duration}
              onChange={(e) => setDuration(parseFloat(e.target.value))}
              className="node-input-field"
              min="0.1"
              max="10"
              step="0.1"
              onClick={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
            />
          </label>
          
          <div className="curve-editor">
            <div className="curve-header">
              <span className="curve-hint">
                Click = add • Drag = move • 2× click = delete
              </span>
              <button 
                className="btn-clear-curve" 
                onClick={(e) => { e.stopPropagation(); clearCurve(); }}
              >
                Reset
              </button>
            </div>
            
            {/* Intensity curve editor */}
            <div className="curve-graph-container">
              <div className="curve-y-axis">
                <span className="axis-label">{t('nodes.vibration.intensity')}</span>
                <span className="axis-value">100%</span>
                <span className="axis-value" style={{ position: 'absolute', top: '50%' }}>50%</span>
                <span className="axis-value" style={{ position: 'absolute', bottom: '0' }}>0%</span>
              </div>
              
              <div className="curve-graph-area">
                <div 
                  className="canvas-wrapper"
                  onMouseDown={(e) => e.stopPropagation()}
                  onMouseMove={(e) => e.stopPropagation()}
                  onMouseUp={(e) => e.stopPropagation()}
                >
                  <canvas
                    ref={canvasRef}
                    width={280}
                    height={140}
                    onMouseDown={handleMouseDown}
                    onMouseMove={handleMouseMove}
                    onMouseUp={handleMouseUp}
                    onMouseLeave={handleMouseLeave}
                    onDoubleClick={handleDoubleClick}
                    onContextMenu={(e) => e.preventDefault()}
                    draggable={false}
                    style={{ 
                      cursor: draggedPointIndex !== null ? 'grabbing' : 
                              hoveredPointIndex !== null ? 'grab' : 'crosshair',
                      pointerEvents: 'auto'
                    }}
                  />
                </div>
                
                <div className="curve-x-axis">
                  <span className="axis-value">0s</span>
                  <span className="axis-label">{t('nodes.vibration.time')}</span>
                  <span className="axis-value">{duration.toFixed(1)}s</span>
                </div>
              </div>
            </div>
            
            <div className="curve-legend">
              <span>Points: {curve.length}</span>
            </div>
          </div>
          
          <div className="random-controls">
            <label>
              <input
                type="checkbox"
                checked={enableRandom}
                onChange={(e) => setEnableRandom(e.target.checked)}
                onClick={(e) => e.stopPropagation()}
                onMouseDown={(e) => e.stopPropagation()}
              />
              {t('nodes.vibration.random')}
            </label>
            
            {enableRandom && (
              <div className="random-range">
                <input
                  type="number"
                  value={randomMin}
                  onChange={(e) => setRandomMin(parseFloat(e.target.value))}
                  className="node-input-field"
                  min="0"
                  max="1"
                  step="0.01"
                  onClick={(e) => e.stopPropagation()}
                  onMouseDown={(e) => e.stopPropagation()}
                />
                <span>-</span>
                <input
                  type="number"
                  value={randomMax}
                  onChange={(e) => setRandomMax(parseFloat(e.target.value))}
                  className="node-input-field"
                  min="0"
                  max="1"
                  step="0.01"
                  onClick={(e) => e.stopPropagation()}
                  onMouseDown={(e) => e.stopPropagation()}
                />
              </div>
            )}
          </div>
        </div>
      </div>
      <Handle type="source" position={Position.Right} id="vibrate" />
    </div>
  );
}
