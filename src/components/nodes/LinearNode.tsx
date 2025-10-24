import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { MoveVertical } from 'lucide-react';

interface LinearNodeData {
  duration: number;
  position: number; // 0.0 to 1.0
  mode: 'once' | 'continuous';
}

export function LinearNode({ data, id, selected }: { data: LinearNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [duration, setDuration] = useState(data.duration || 1.0);
  const [position, setPosition] = useState(data.position || 0.5);
  const [mode, setMode] = useState(data.mode || 'once');
  
  useEffect(() => {
    data.duration = duration;
    data.position = position;
    data.mode = mode;
  }, [duration, position, mode, data]);
  
  return (
    <div 
      className="custom-node linear-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '2px solid rgba(93, 138, 168, 0.4)',
        minWidth: '180px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={180} 
        minHeight={140}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(93, 138, 168, 0.2)' }}>
        <MoveVertical size={16} color="#5d8aa8" />
        <span style={{ color: '#5d8aa8' }}>Linear Motion</span>
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
          
          <div>
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
            />
          </div>
          
          <div>
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Position: {(position * 100).toFixed(0)}%
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={position}
              onChange={(e) => setPosition(parseFloat(e.target.value))}
              style={{ width: '100%' }}
              onClick={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
            />
          </div>
          
          <div style={{
            padding: '6px',
            background: 'rgba(16, 185, 129, 0.2)',
            borderRadius: '4px',
            fontSize: '9px',
            color: '#94a3b8',
            textAlign: 'center'
          }}>
            📏 Linear actuators (strokers, thrusters)
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="output"
        style={{ 
          background: '#10b981', 
          width: 12, 
          height: 12, 
          border: '2px solid #10b981',
          boxShadow: '0 0 8px #10b98188'
        }}
      />
    </div>
  );
}

