import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Scale } from 'lucide-react';

interface ConditionNodeData {
  operator: '>' | '<' | '=' | '>=' | '<=';
  value: number;
  usePercentage?: boolean; // Use % of max value from vehicle limits
  continuous?: boolean; // Vibrate continuously while condition is true
}

export function ConditionNode({ data, id, selected }: { data: ConditionNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [operator, setOperator] = useState(data.operator || '>');
  const [value, setValue] = useState(data.value || 100);
  const [usePercentage, setUsePercentage] = useState(data.usePercentage || false);
  const [continuous, setContinuous] = useState(data.continuous || false);
  
  useEffect(() => {
    data.operator = operator;
    data.value = value;
    data.usePercentage = usePercentage;
    data.continuous = continuous;
  }, [operator, value, usePercentage, continuous, data]);
  
  const OPERATORS = [
    { value: '>', symbol: '>', label: 'Greater than', color: '#4ade80' },
    { value: '<', symbol: '<', label: 'Less than', color: '#f87171' },
    { value: '>=', symbol: '≥', label: 'Greater or equal', color: '#22c55e' },
    { value: '<=', symbol: '≤', label: 'Less or equal', color: '#fb923c' },
    { value: '=', symbol: '=', label: 'Equal', color: '#a855f7' },
  ];
  
  const selectedOp = OPERATORS.find(op => op.value === operator);
  
  return (
    <div 
      className="custom-node condition-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: `2px solid ${selectedOp?.color || 'rgba(93, 138, 168, 0.4)'}`,
        minWidth: '160px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={160} 
        minHeight={100}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: `${selectedOp?.color}22` }}>
        <Scale size={16} color={selectedOp?.color} />
        <span style={{ color: selectedOp?.color }}>Condition</span>
      </div>
      <div className="node-body">
        <Handle 
          type="target" 
          position={Position.Left} 
          id="input"
          style={{ 
            background: '#3b82f6', 
            width: 10, 
            height: 10, 
            border: '2px solid #3b82f6',
            boxShadow: '0 0 6px #3b82f688'
          }}
        />
        
        <div style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <select 
            value={operator}
            onChange={(e) => setOperator(e.target.value as any)}
            className="node-select"
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: `1px solid ${selectedOp?.color}`,
              color: selectedOp?.color,
              fontSize: '12px',
              fontWeight: 'bold'
            }}
          >
            {OPERATORS.map(op => (
              <option key={op.value} value={op.value}>
                {op.symbol} {op.label}
              </option>
            ))}
          </select>
          
          <input
            type="number"
            value={value}
            onChange={(e) => setValue(parseFloat(e.target.value) || 0)}
            className="node-input-field"
            onClick={(e) => e.stopPropagation()}
            onMouseDown={(e) => e.stopPropagation()}
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: `1px solid ${selectedOp?.color}`,
              color: '#fff',
              padding: '6px 10px',
              borderRadius: '4px',
              fontSize: '14px',
              fontWeight: 'bold',
              textAlign: 'center'
            }}
          />
          
          <div style={{
            padding: '5px',
            background: `${selectedOp?.color}22`,
            borderRadius: '4px',
            fontSize: '9px',
            color: selectedOp?.color,
            textAlign: 'center',
            fontWeight: 600
          }}>
            Value {selectedOp?.symbol} {value}{usePercentage ? '%' : ''}
          </div>
          
          {/* Percentage mode toggle */}
          <label style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            fontSize: '9px',
            color: '#94a3b8',
            marginTop: '6px',
            cursor: 'pointer',
            padding: '4px',
            background: usePercentage ? 'rgba(34, 197, 94, 0.15)' : 'transparent',
            borderRadius: '4px'
          }}>
            <input
              type="checkbox"
              checked={usePercentage}
              onChange={(e) => setUsePercentage(e.target.checked)}
              style={{ accentColor: '#22c55e' }}
            />
            📊 % of max value
          </label>
          
          {/* Continuous mode toggle */}
          <label style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            fontSize: '9px',
            color: '#94a3b8',
            marginTop: '4px',
            cursor: 'pointer',
            padding: '4px',
            background: continuous ? 'rgba(139, 92, 246, 0.15)' : 'transparent',
            borderRadius: '4px'
          }}>
            <input
              type="checkbox"
              checked={continuous}
              onChange={(e) => setContinuous(e.target.checked)}
              style={{ accentColor: '#8b5cf6' }}
            />
            🔄 Continuous
          </label>
          
          {continuous && (
            <div style={{
              fontSize: '8px',
              color: '#8b5cf6',
              marginTop: '4px',
              padding: '4px',
              background: 'rgba(139, 92, 246, 0.1)',
              borderRadius: '3px',
              textAlign: 'center',
              lineHeight: '1.3'
            }}>
              ⚡ Vibrates while condition is true
            </div>
          )}
        </div>
        
        <Handle 
          type="source" 
          position={Position.Right} 
          id="output"
          style={{ 
            background: selectedOp?.color, 
            width: 12, 
            height: 12, 
            border: `2px solid ${selectedOp?.color}`,
            boxShadow: `0 0 8px ${selectedOp?.color}88`
          }}
        />
      </div>
    </div>
  );
}

