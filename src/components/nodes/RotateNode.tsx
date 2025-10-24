import { useState, useEffect } from 'react';
import { Handle, Position } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { RotateCw } from 'lucide-react';

interface RotateNodeData {
  duration: number;
  speed: number; // 0.0 to 1.0
  clockwise: boolean;
  mode: 'once' | 'continuous';
}

export function RotateNode({ data, id }: { data: RotateNodeData; id: string }) {
  const { t } = useTranslation();
  const [duration, setDuration] = useState(data.duration || 1.0);
  const [speed, setSpeed] = useState(data.speed || 0.5);
  const [clockwise, setClockwise] = useState(data.clockwise ?? true);
  const [mode, setMode] = useState(data.mode || 'once');
  
  useEffect(() => {
    data.duration = duration;
    data.speed = speed;
    data.clockwise = clockwise;
    data.mode = mode;
  }, [duration, speed, clockwise, mode, data]);
  
  return (
    <div 
      className="custom-node rotate-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #4c1d95 0%, #6d28d9 100%)',
        border: '2px solid #8b5cf6',
        minWidth: '180px'
      }}
    >
      <div className="node-header" style={{ background: 'rgba(139, 92, 246, 0.2)' }}>
        <RotateCw size={16} color="#8b5cf6" />
        <span style={{ color: '#8b5cf6' }}>Rotation</span>
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
          <div>
            <label style={{ fontSize: '10px', color: '#8b5cf6', marginBottom: '4px', display: 'block' }}>
              Mode:
            </label>
            <select 
              value={mode}
              onChange={(e) => setMode(e.target.value as any)}
              className="node-select"
              style={{
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid #8b5cf6',
                color: '#8b5cf6',
                fontSize: '11px'
              }}
            >
              <option value="once">Once</option>
              <option value="continuous">Continuous</option>
            </select>
          </div>
          
          <div>
            <label style={{ fontSize: '10px', color: '#8b5cf6', marginBottom: '4px', display: 'block' }}>
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
            />
          </div>
          
          <div>
            <label style={{ fontSize: '10px', color: '#8b5cf6', marginBottom: '4px', display: 'block' }}>
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
            />
          </div>
          
          <div>
            <label style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '6px',
              fontSize: '11px',
              color: '#8b5cf6',
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
          
          <div style={{
            padding: '6px',
            background: 'rgba(139, 92, 246, 0.2)',
            borderRadius: '4px',
            fontSize: '9px',
            color: '#8b5cf6',
            textAlign: 'center'
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

