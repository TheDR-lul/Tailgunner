import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { GitBranch } from 'lucide-react';

interface ConditionNodeData {
  logic: 'AND' | 'OR' | 'XOR' | 'NOT';
  description?: string;
}

export function ConditionNode({ data, id, selected }: { data: ConditionNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [logic, setLogic] = useState(data.logic || 'AND');
  
  useEffect(() => {
    data.logic = logic;
  }, [logic, data]);
  
  const LOGIC_OPS = [
    { value: 'AND', label: 'AND (All true)', symbol: '∧', color: '#4ade80', desc: 'All conditions must be true' },
    { value: 'OR', label: 'OR (Any true)', symbol: '∨', color: '#fbbf24', desc: 'At least one condition true' },
    { value: 'XOR', label: 'XOR (Only one)', symbol: '⊕', color: '#a855f7', desc: 'Exactly one condition true' },
    { value: 'NOT', label: 'NOT (Inverse)', symbol: '¬', color: '#f87171', desc: 'Invert the result' },
  ];
  
  const selectedLogic = LOGIC_OPS.find(op => op.value === logic);
  
  return (
    <div 
      className="custom-node condition-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: `2px solid ${selectedLogic?.color || 'rgba(93, 138, 168, 0.4)'}`,
        minWidth: '180px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={180} 
        minHeight={120}
        color={selectedLogic?.color || 'rgba(93, 138, 168, 0.8)'}
      />
      
      {/* Multiple Inputs */}
      <Handle 
        type="target" 
        position={Position.Left} 
        id="input-a"
        style={{ 
          top: '30%',
          background: '#ff9933', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(255, 153, 51, 0.6)'
        }}
      />
      <span style={{
        position: 'absolute',
        left: '16px',
        top: 'calc(30% - 8px)',
        fontSize: '9px',
        color: '#ff9933',
        fontWeight: 'bold',
        pointerEvents: 'none'
      }}>A</span>
      
      <Handle 
        type="target" 
        position={Position.Left} 
        id="input-b"
        style={{ 
          top: '70%',
          background: '#ff9933', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(255, 153, 51, 0.6)'
        }}
      />
      <span style={{
        position: 'absolute',
        left: '16px',
        top: 'calc(70% - 8px)',
        fontSize: '9px',
        color: '#ff9933',
        fontWeight: 'bold',
        pointerEvents: 'none'
      }}>B</span>
      
      <div className="node-header" style={{ background: `${selectedLogic?.color}20` }}>
        <GitBranch size={16} style={{ color: selectedLogic?.color }} />
        <span style={{ color: selectedLogic?.color }}>LOGIC</span>
      </div>
      
      <div className="node-body">
        <select 
          value={logic} 
          onChange={(e) => setLogic(e.target.value as any)}
          className="node-select"
          onClick={(e) => e.stopPropagation()}
          style={{
            background: 'rgba(0, 0, 0, 0.3)',
            border: `1px solid ${selectedLogic?.color}`,
            color: selectedLogic?.color,
            fontSize: '13px',
            fontWeight: 'bold',
            textAlign: 'center',
            padding: '8px'
          }}
        >
          {LOGIC_OPS.map(op => (
            <option key={op.value} value={op.value}>
              {op.symbol} {op.value}
            </option>
          ))}
        </select>
        
        <div style={{ 
          fontSize: '9px', 
          color: '#64748b', 
          textAlign: 'center',
          padding: '4px',
          marginTop: '4px'
        }}>
          {selectedLogic?.desc}
        </div>
        
        <div style={{
          fontSize: '10px',
          color: selectedLogic?.color,
          textAlign: 'center',
          padding: '6px',
          marginTop: '4px',
          background: `${selectedLogic?.color}10`,
          borderRadius: '4px',
          fontWeight: 'bold'
        }}>
          {logic === 'AND' && 'A ∧ B'}
          {logic === 'OR' && 'A ∨ B'}
          {logic === 'XOR' && 'A ⊕ B'}
          {logic === 'NOT' && '¬A'}
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="output" 
        style={{ 
          background: selectedLogic?.color, 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: `0 0 8px ${selectedLogic?.color}99`
        }}
      />
    </div>
  );
}
