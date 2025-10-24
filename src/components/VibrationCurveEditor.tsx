import { useState, useRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { EditableNumberInput } from './EditableNumberInput';

interface CurvePoint {
  x: number;
  y: number;
}

interface VibrationCurveEditorProps {
  duration: number;
  curve: CurvePoint[];
  onDurationChange: (duration: number) => void;
  onCurveChange: (curve: CurvePoint[]) => void;
}

export function VibrationCurveEditor({ 
  duration, 
  curve, 
  onDurationChange, 
  onCurveChange 
}: VibrationCurveEditorProps) {
  const { t } = useTranslation();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [draggedPointIndex, setDraggedPointIndex] = useState<number | null>(null);
  const [hoveredPointIndex, setHoveredPointIndex] = useState<number | null>(null);
  
  const POINT_RADIUS = 6;
  const HOVER_RADIUS = 8;
  
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
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    
    const canvasX = (e.clientX - rect.left) * scaleX;
    const canvasY = (e.clientY - rect.top) * scaleY;
    
    const x = Math.max(0, Math.min(1, canvasX / canvas.width));
    const y = Math.max(0, Math.min(1, 1 - (canvasY / canvas.height)));
    
    return { x, y };
  };
  
  // Find nearest point to click
  const findNearestPoint = (clickPoint: CurvePoint): number | null => {
    const canvas = canvasRef.current;
    if (!canvas) return null;
    
    const threshold = 15;
    const width = canvas.width;
    const height = canvas.height;
    
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
      setDraggedPointIndex(nearestIndex);
    } else {
      const newCurve = [...curve, clickPoint].sort((a, b) => a.x - b.x);
      onCurveChange(newCurve);
    }
  };
  
  // Mouse move - drag point or hover
  const handleMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    e.stopPropagation();
    
    const movePoint = getPointFromEvent(e);
    
    if (draggedPointIndex !== null) {
      const newCurve = [...curve];
      newCurve[draggedPointIndex] = movePoint;
      newCurve.sort((a, b) => a.x - b.x);
      
      const newIndex = newCurve.findIndex(p => p.x === movePoint.x && p.y === movePoint.y);
      setDraggedPointIndex(newIndex);
      
      onCurveChange(newCurve);
    } else {
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
      onCurveChange(newCurve);
    }
  };
  
  // Reset curve
  const clearCurve = () => {
    onCurveChange([
      { x: 0.0, y: 1.0 },
      { x: 1.0, y: 0.0 }
    ]);
  };
  
  return (
    <div style={{ 
      display: 'flex', 
      flexDirection: 'column', 
      gap: '8px',
      padding: '8px',
      background: 'var(--bg-tertiary)',
      borderRadius: 'var(--radius-sm)',
      border: '1px solid var(--border)'
    }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
          {t('trigger_settings.vibration_duration', 'Duration (sec)')}
        </label>
        <EditableNumberInput
          value={duration}
          onChange={onDurationChange}
          min={0.1}
          max={10}
          decimals={1}
          suffix="s"
          style={{
            width: '60px',
            padding: '4px 6px',
            fontSize: '11px',
            background: 'var(--bg-primary)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-sm)',
            color: 'var(--text-primary)',
            textAlign: 'center'
          }}
        />
      </div>

      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center',
        fontSize: '9px',
        color: 'var(--text-muted)'
      }}>
        <span>{t('trigger_settings.curve_hint', 'Click = add • Drag = move • 2× click = delete')}</span>
        <button 
          onClick={clearCurve}
          style={{
            padding: '2px 8px',
            fontSize: '9px',
            background: 'var(--bg-secondary)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-sm)',
            cursor: 'pointer',
            color: 'var(--text-secondary)'
          }}
        >
          {t('common.reset', 'Reset')}
        </button>
      </div>

      <div style={{ position: 'relative', display: 'flex', gap: '4px' }}>
        <div style={{ 
          position: 'relative',
          width: '30px',
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'space-between',
          fontSize: '9px',
          color: 'var(--text-muted)',
          paddingTop: '4px',
          paddingBottom: '4px'
        }}>
          <span style={{ textAlign: 'right' }}>100%</span>
          <span style={{ textAlign: 'right' }}>50%</span>
          <span style={{ textAlign: 'right' }}>0%</span>
        </div>

        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '4px', maxWidth: '280px' }}>
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
            style={{ 
              cursor: draggedPointIndex !== null ? 'grabbing' : 
                      hoveredPointIndex !== null ? 'grab' : 'crosshair',
              borderRadius: '4px',
              width: '100%',
              height: 'auto',
              aspectRatio: '2 / 1',
              display: 'block'
            }}
          />
          
          <div style={{ 
            display: 'flex', 
            justifyContent: 'space-between',
            fontSize: '9px',
            color: 'var(--text-muted)',
            paddingLeft: '4px',
            paddingRight: '4px'
          }}>
            <span>0s</span>
            <span>{t('nodes.vibration.time', 'Time')}</span>
            <span>{duration.toFixed(1)}s</span>
          </div>
        </div>
      </div>

      <div style={{ 
        fontSize: '9px', 
        color: 'var(--text-muted)', 
        textAlign: 'center',
        paddingTop: '4px',
        borderTop: '1px solid var(--border)'
      }}>
        {t('trigger_settings.points', 'Points')}: {curve.length}
      </div>
    </div>
  );
}

