import { useState, useRef, useEffect } from 'react';
import { Handle, Position } from 'reactflow';

interface Point {
  x: number;
  y: number;
}

interface VibrationNodeData {
  duration: number;
  curve: Point[];
  randomize?: { min: number; max: number };
}

export function VibrationNode({ data, id }: { data: VibrationNodeData; id: string }) {
  const [duration, setDuration] = useState(data.duration || 1.0);
  const [curve, setCurve] = useState<Point[]>(data.curve || [
    { x: 0, y: 0 },
    { x: 0.1, y: 1.0 },
    { x: 0.5, y: 0.8 },
    { x: 1.0, y: 0 }
  ]);
  const [randomMin, setRandomMin] = useState(data.randomize?.min || 0);
  const [randomMax, setRandomMax] = useState(data.randomize?.max || 0);
  const [enableRandom, setEnableRandom] = useState(!!data.randomize);
  
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  useEffect(() => {
    drawCurve();
  }, [curve]);
  
  const drawCurve = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Очистка
    ctx.clearRect(0, 0, width, height);
    
    // Фон
    ctx.fillStyle = 'rgba(30, 30, 40, 0.5)';
    ctx.fillRect(0, 0, width, height);
    
    // Сетка
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 4; i++) {
      const y = (height / 4) * i;
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }
    
    // Кривая
    ctx.strokeStyle = 'var(--accent)';
    ctx.lineWidth = 2;
    ctx.beginPath();
    
    curve.forEach((point, index) => {
      const x = point.x * width;
      const y = height - (point.y * height);
      
      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });
    
    ctx.stroke();
    
    // Точки
    curve.forEach((point) => {
      const x = point.x * width;
      const y = height - (point.y * height);
      
      ctx.fillStyle = 'var(--accent)';
      ctx.beginPath();
      ctx.arc(x, y, 4, 0, Math.PI * 2);
      ctx.fill();
    });
  };
  
  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.stopPropagation(); // Останавливаем всплытие события!
    
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / canvas.width;
    const y = 1 - ((e.clientY - rect.top) / canvas.height);
    
    // Добавляем новую точку
    const newCurve = [...curve, { x, y }].sort((a, b) => a.x - b.x);
    setCurve(newCurve);
  };
  
  const clearCurve = () => {
    setCurve([
      { x: 0, y: 0 },
      { x: 0.5, y: 1.0 },
      { x: 1.0, y: 0 }
    ]);
  };
  
  return (
    <div className="node-vibration" onClick={(e) => e.stopPropagation()}>
      <Handle type="target" position={Position.Left} id="trigger" />
      
      <div className="node-header">💥 Вибрация</div>
      
      <div className="node-body" onClick={(e) => e.stopPropagation()}>
        <div className="vibration-controls">
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
            />
          </label>
          
          <div className="curve-editor">
            <div className="curve-header">
              <span className="curve-hint">Клик = добавить точку</span>
              <button 
                className="btn-clear-curve" 
                onClick={(e) => { e.stopPropagation(); clearCurve(); }}
              >
                Сбросить
              </button>
            </div>
            <canvas
              ref={canvasRef}
              width={200}
              height={100}
              onClick={handleCanvasClick}
              onMouseDown={(e) => e.stopPropagation()}
              style={{ cursor: 'crosshair' }}
            />
          </div>
          
          <div className="random-controls">
            <label>
              <input
                type="checkbox"
                checked={enableRandom}
                onChange={(e) => setEnableRandom(e.target.checked)}
              />
              Рандомная интенсивность
            </label>
            
            {enableRandom && (
              <div className="random-range">
                <input
                  type="number"
                  value={randomMin}
                  onChange={(e) => setRandomMin(parseFloat(e.target.value))}
                  placeholder="Мин %"
                  min="0"
                  max="100"
                  step="5"
                />
                <span>—</span>
                <input
                  type="number"
                  value={randomMax}
                  onChange={(e) => setRandomMax(parseFloat(e.target.value))}
                  placeholder="Макс %"
                  min="0"
                  max="100"
                  step="5"
                />
              </div>
            )}
          </div>
        </div>
      </div>
      
      <Handle type="source" position={Position.Right} id="output" />
    </div>
  );
}

