import { useState, useRef, useEffect } from 'react';
import { Handle, Position } from 'reactflow';

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

export function VibrationNode({ data, id }: { data: VibrationNodeData; id: string }) {
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
  
  // Константы для отрисовки
  const POINT_RADIUS = 6;
  const HOVER_RADIUS = 8;
  
  // Синхронизация с data при изменении (загрузка сохраненного паттерна)
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
  
  // Обновляем data при изменении локальных состояний
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
  
  // Отрисовка графика
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Очистка
    ctx.clearRect(0, 0, width, height);
    
    // Фон
    ctx.fillStyle = '#1a1d29';
    ctx.fillRect(0, 0, width, height);
    
    // Сетка
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
    
    // Центральная линия
    ctx.strokeStyle = 'rgba(148, 163, 184, 0.2)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(0, height / 2);
    ctx.lineTo(width, height / 2);
    ctx.stroke();
    
    // Кривая
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
      
      // Сброс shadow для точек
      ctx.shadowColor = 'transparent';
      ctx.shadowBlur = 0;
      
      // Точки
      curve.forEach((point, index) => {
        const canvasX = point.x * width;
        const canvasY = (1 - point.y) * height;
        const isHovered = index === hoveredPointIndex;
        const isDragged = index === draggedPointIndex;
        
        // Внешний круг (hover)
        if (isHovered || isDragged) {
          ctx.fillStyle = 'rgba(0, 212, 255, 0.2)';
          ctx.beginPath();
          ctx.arc(canvasX, canvasY, HOVER_RADIUS, 0, Math.PI * 2);
          ctx.fill();
        }
        
        // Основная точка
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
  
  // Получить координаты точки из события мыши
  const getPointFromEvent = (e: React.MouseEvent<HTMLCanvasElement>): CurvePoint => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: 0, y: 0 };
    
    const rect = canvas.getBoundingClientRect();
    // Используем реальный размер в браузере (с учетом зума)
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    const y = Math.max(0, Math.min(1, 1 - ((e.clientY - rect.top) / rect.height)));
    
    return { x, y };
  };
  
  // Найти ближайшую точку к клику
  const findNearestPoint = (clickPoint: CurvePoint): number | null => {
    const canvas = canvasRef.current;
    if (!canvas) return null;
    
    const rect = canvas.getBoundingClientRect();
    const threshold = 15; // пиксели
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
  
  // Mouse down - начало перетаскивания или добавление точки
  const handleMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const clickPoint = getPointFromEvent(e);
    const nearestIndex = findNearestPoint(clickPoint);
    
    if (nearestIndex !== null) {
      // Начинаем перетаскивание существующей точки
      setDraggedPointIndex(nearestIndex);
    } else {
      // Добавляем новую точку
      const newCurve = [...curve, clickPoint].sort((a, b) => a.x - b.x);
      setCurve(newCurve);
    }
  };
  
  // Mouse move - перетаскивание точки или hover
  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const movePoint = getPointFromEvent(e);
    
    if (draggedPointIndex !== null) {
      // Перетаскиваем точку
      const newCurve = [...curve];
      newCurve[draggedPointIndex] = movePoint;
      
      // Пересортировываем по X
      newCurve.sort((a, b) => a.x - b.x);
      
      // Находим новый индекс перетаскиваемой точки после сортировки
      const newIndex = newCurve.findIndex(p => p.x === movePoint.x && p.y === movePoint.y);
      setDraggedPointIndex(newIndex);
      
      setCurve(newCurve);
    } else {
      // Определяем hover
      const nearestIndex = findNearestPoint(movePoint);
      setHoveredPointIndex(nearestIndex);
    }
  };
  
  // Mouse up - конец перетаскивания
  const handleMouseUp = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDraggedPointIndex(null);
  };
  
  // Mouse leave - сброс hover
  const handleMouseLeave = () => {
    setDraggedPointIndex(null);
    setHoveredPointIndex(null);
  };
  
  // Двойной клик - удаление точки
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
  
  // Сброс кривой
  const clearCurve = () => {
    setCurve([
      { x: 0.4, y: 0.6 },
      { x: 0.6, y: 0.8 }
    ]);
  };
  
  return (
    <div className="node-vibration" onClick={(e) => e.stopPropagation()}>
      <Handle type="target" position={Position.Left} id="trigger" />
      
      <div className="node-header">💥 Вибрация</div>
      
      <div className="node-body" onClick={(e) => e.stopPropagation()}>
        <div className="vibration-controls">
          <div className="mode-controls">
            <label>
              Режим:
              <select
                value={mode}
                onChange={(e) => setMode(e.target.value as 'once' | 'continuous' | 'repeat')}
                className="node-input-field"
                onClick={(e) => e.stopPropagation()}
                onMouseDown={(e) => e.stopPropagation()}
              >
                <option value="once">Разовая</option>
                <option value="continuous">Постоянная (пока условие)</option>
                <option value="repeat">Циклическая</option>
              </select>
            </label>
            
            {mode === 'repeat' && (
              <label>
                Повторений:
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
            Длительность (сек):
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
                Клик = добавить • Перетащи = двигать • 2× клик = удалить
              </span>
              <button 
                className="btn-clear-curve" 
                onClick={(e) => { e.stopPropagation(); clearCurve(); }}
              >
                Сбросить
              </button>
            </div>
            
            {/* График с подписями осей */}
            <div className="curve-graph-container">
              <div className="curve-y-axis">
                <span className="axis-label">Интенсивность</span>
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
                  <span className="axis-label">Время</span>
                  <span className="axis-value">{duration.toFixed(1)}s</span>
                </div>
              </div>
            </div>
            
            <div className="curve-legend">
              <span>Точек: {curve.length}</span>
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
              Рандомная интенсивность
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
