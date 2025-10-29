import { useState, useEffect, useRef } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { RotateCw } from 'lucide-react';

interface CurvePoint {
  x: number;
  y: number;
}

interface RotateNodeData {
  duration: number;
  speed: number; // 0.0 to 1.0
  clockwise: boolean;
  mode: 'once' | 'continuous';
  curve?: CurvePoint[]; // Speed curve for variable rotation
}

export function RotateNode({ data, id, selected }: { data: RotateNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [duration, setDuration] = useState(data.duration || 1.0);
  const [speed, setSpeed] = useState(data.speed || 0.5);
  const [clockwise, setClockwise] = useState(data.clockwise ?? true);
  const [mode, setMode] = useState(data.mode || 'once');
  const [curve, setCurve] = useState<CurvePoint[]>(data.curve || [
    { x: 0.0, y: 0.0 },
    { x: 0.5, y: 0.5 },
    { x: 1.0, y: 1.0 }
  ]);
  
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [draggedPointIndex, setDraggedPointIndex] = useState<number | null>(null);
  const [hoveredPointIndex, setHoveredPointIndex] = useState<number | null>(null);
  
  const POINT_RADIUS = 6;
  const HOVER_RADIUS = 8;
  
  useEffect(() => {
    data.duration = duration;
    data.speed = speed;
    data.clockwise = clockwise;
    data.mode = mode;
    data.curve = curve;
  }, [duration, speed, clockwise, mode, curve, data]);
  
  // Draw curve on canvas
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
    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, width, height);
    
    // Grid
    ctx.strokeStyle = '#1e293b';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 4; i++) {
      const y = (height / 4) * i;
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
      
      const x = (width / 4) * i;
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    
    // Curve line
    ctx.strokeStyle = '#8b5cf6';
    ctx.lineWidth = 2;
    ctx.beginPath();
    
    const sortedPoints = [...curve].sort((a, b) => a.x - b.x);
    sortedPoints.forEach((point, i) => {
      const x = point.x * width;
      const y = height - (point.y * height);
      
      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });
    ctx.stroke();
    
    // Draw points
    curve.forEach((point, i) => {
      const x = point.x * width;
      const y = height - (point.y * height);
      
      const isHovered = hoveredPointIndex === i;
      const isDragged = draggedPointIndex === i;
      const radius = (isHovered || isDragged) ? HOVER_RADIUS : POINT_RADIUS;
      
      ctx.fillStyle = isDragged ? '#a78bfa' : isHovered ? '#8b5cf6' : '#7c3aed';
      ctx.beginPath();
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.fill();
      
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = 2;
      ctx.stroke();
    });
    
    // Labels
    ctx.fillStyle = '#64748b';
    ctx.font = '10px monospace';
    ctx.fillText('Time →', 5, height - 5);
    ctx.save();
    ctx.translate(10, height / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillText('Speed →', 0, 0);
    ctx.restore();
    
  }, [curve, hoveredPointIndex, draggedPointIndex]);
  
  const handleCanvasMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = 1 - ((e.clientY - rect.top) / rect.height);
    
    // Check if clicked on existing point
    for (let i = 0; i < curve.length; i++) {
      const point = curve[i];
      const distance = Math.sqrt(
        Math.pow((point.x - x) * rect.width, 2) + 
        Math.pow((point.y - y) * rect.height, 2)
      );
      
      if (distance < HOVER_RADIUS) {
        setDraggedPointIndex(i);
        return;
      }
    }
    
    // Add new point
    const newCurve = [...curve, { x, y }].sort((a, b) => a.x - b.x);
    setCurve(newCurve);
  };
  
  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const rect = canvas.getBoundingClientRect();
    const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    const y = Math.max(0, Math.min(1, 1 - ((e.clientY - rect.top) / rect.height)));
    
    if (draggedPointIndex !== null) {
      const newCurve = [...curve];
      newCurve[draggedPointIndex] = { x, y };
      setCurve(newCurve.sort((a, b) => a.x - b.x));
      return;
    }
    
    // Check hover
    let foundHover = false;
    for (let i = 0; i < curve.length; i++) {
      const point = curve[i];
      const distance = Math.sqrt(
        Math.pow((point.x - x) * rect.width, 2) + 
        Math.pow((point.y - y) * rect.height, 2)
      );
      
      if (distance < HOVER_RADIUS) {
        setHoveredPointIndex(i);
        foundHover = true;
        break;
      }
    }
    
    if (!foundHover) {
      setHoveredPointIndex(null);
    }
  };
  
  const handleCanvasMouseUp = () => {
    setDraggedPointIndex(null);
  };
  
  const handleCanvasMouseLeave = () => {
    setDraggedPointIndex(null);
    setHoveredPointIndex(null);
  };
  
  const handleCanvasDoubleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = 1 - ((e.clientY - rect.top) / rect.height);
    
    // Remove point if double-clicked
    for (let i = 0; i < curve.length; i++) {
      const point = curve[i];
      const distance = Math.sqrt(
        Math.pow((point.x - x) * rect.width, 2) + 
        Math.pow((point.y - y) * rect.height, 2)
      );
      
      if (distance < HOVER_RADIUS && curve.length > 2) {
        const newCurve = curve.filter((_, idx) => idx !== i);
        setCurve(newCurve);
        return;
      }
    }
  };
  
  return (
    <div 
      className="custom-node rotate-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '2px solid rgba(93, 138, 168, 0.4)',
        minWidth: '200px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={200} 
        minHeight={280}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(93, 138, 168, 0.2)' }}>
        <RotateCw size={16} color="#5d8aa8" />
        <span style={{ color: '#5d8aa8' }}>Rotation</span>
      </div>
      <div className="node-body">
        <Handle 
          type="target" 
          position={Position.Left} 
          id="trigger"
          style={{ 
            background: '#a855f7', 
            width: 12, 
            height: 12, 
            border: '2px solid #a855f7',
            boxShadow: '0 0 8px #a855f788'
          }}
        />
        
        <div style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div className="nodrag">
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Mode:
            </label>
            <select 
              value={mode}
              onChange={(e) => setMode(e.target.value as any)}
              className="node-select"
              style={{
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid rgba(93, 138, 168, 0.5)',
                color: '#94a3b8',
                fontSize: '11px'
              }}
            >
              <option value="once">Once</option>
              <option value="continuous">Continuous</option>
            </select>
          </div>
          
          <div className="nodrag">
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Duration (s): {duration.toFixed(1)}
            </label>
            <input
              type="range"
              min="0.1"
              max="5"
              step="0.1"
              value={duration}
              onChange={(e) => setDuration(parseFloat(e.target.value))}
              style={{ width: '100%' }}
              onClick={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
              onPointerDown={(e) => e.stopPropagation()}
            />
          </div>
          
          <div className="nodrag">
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Speed: {(speed * 100).toFixed(0)}%
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={speed}
              onChange={(e) => setSpeed(parseFloat(e.target.value))}
              style={{ width: '100%' }}
              onClick={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
              onPointerDown={(e) => e.stopPropagation()}
            />
          </div>
          
          <div className="nodrag">
            <label style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '6px',
              fontSize: '11px',
              color: '#94a3b8',
              cursor: 'pointer'
            }}>
              <input
                type="checkbox"
                checked={clockwise}
                onChange={(e) => setClockwise(e.target.checked)}
                onClick={(e) => e.stopPropagation()}
              />
              Clockwise {clockwise ? '↻' : '↺'}
            </label>
          </div>
          
          <div className="nodrag" style={{ marginTop: '8px' }}>
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Speed Curve:
            </label>
            <canvas
              ref={canvasRef}
              width={160}
              height={100}
              onMouseDown={handleCanvasMouseDown}
              onMouseMove={handleCanvasMouseMove}
              onMouseUp={handleCanvasMouseUp}
              onMouseLeave={handleCanvasMouseLeave}
              onDoubleClick={handleCanvasDoubleClick}
              style={{
                width: '100%',
                height: 'auto',
                border: '1px solid rgba(139, 92, 246, 0.3)',
                borderRadius: '4px',
                cursor: draggedPointIndex !== null ? 'grabbing' : hoveredPointIndex !== null ? 'grab' : 'crosshair',
                background: '#0f172a'
              }}
            />
            <div style={{
              fontSize: '9px',
              color: '#64748b',
              marginTop: '4px',
              textAlign: 'center'
            }}>
              Click: Add • Drag: Move • Double-click: Remove
            </div>
          </div>
          
          <div style={{
            padding: '6px',
            background: 'rgba(139, 92, 246, 0.2)',
            borderRadius: '4px',
            fontSize: '9px',
            color: '#94a3b8',
            textAlign: 'center',
            marginTop: '8px'
          }}>
            🔄 Rotary devices (rotating toys)
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="output"
        style={{ 
          background: '#8b5cf6', 
          width: 12, 
          height: 12, 
          border: '2px solid #8b5cf6',
          boxShadow: '0 0 8px #8b5cf688'
        }}
      />
    </div>
  );
}

