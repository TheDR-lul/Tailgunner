import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Scale } from 'lucide-react';

interface ConditionNodeData {
  param: 'speed' | 'altitude' | 'g_load' | 'fuel' | 'rpm';
  operator: '>' | '<' | '=' | '>=' | '<=';
  value: number;
  description?: string;
}

export function ConditionNode({ data, id, selected }: { data: ConditionNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [param, setParam] = useState(data.param || 'speed');
  const [operator, setOperator] = useState(data.operator || '>');
  const [value, setValue] = useState(data.value || 100);
  
  useEffect(() => {
    data.param = param;
    data.operator = operator;
    data.value = value;
  }, [param, operator, value, data]);
  
  const PARAMS = [
    { value: 'speed', label: 'Speed', unit: 'km/h', color: '#06b6d4' },
    { value: 'altitude', label: 'Altitude', unit: 'm', color: '#10b981' },
    { value: 'g_load', label: 'G-Load', unit: 'G', color: '#f59e0b' },
    { value: 'fuel', label: 'Fuel', unit: '%', color: '#ef4444' },
    { value: 'rpm', label: 'RPM', unit: '', color: '#a855f7' },
  ];
  
  const OPERATORS = [
    { value: '>', symbol: '>', label: 'Greater than', color: '#4ade80' },
    { value: '<', symbol: '<', label: 'Less than', color: '#f87171' },
    { value: '>=', symbol: '≥', label: 'Greater or equal', color: '#22c55e' },
    { value: '<=', symbol: '≤', label: 'Less or equal', color: '#fb923c' },
    { value: '=', symbol: '=', label: 'Equal', color: '#a855f7' },
  ];
  
  const selectedParam = PARAMS.find(p => p.value === param);
  const selectedOp = OPERATORS.find(op => op.value === operator);
  
  return (
    <div 
      className="custom-node condition-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: `2px solid ${selectedParam?.color || 'rgba(93, 138, 168, 0.4)'}`,
        minWidth: '200px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={200} 
        minHeight={160}
        color={selectedParam?.color || 'rgba(93, 138, 168, 0.8)'}
      />
      
      {/* Input */}
      <Handle 
        type="target" 
        position={Position.Left} 
        style={{ 
          background: '#ff9933', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(255, 153, 51, 0.6)'
        }}
      />
      
      <div className="node-header" style={{ background: `${selectedParam?.color}20` }}>
        <Scale size={16} style={{ color: selectedParam?.color }} />
        <span style={{ color: selectedParam?.color }}>CONDITION</span>
      </div>
      
      <div className="node-body">
        {/* Parameter selector */}
        <div style={{ marginBottom: '8px' }}>
          <label style={{ fontSize: '10px', color: 'var(--text-muted)', marginBottom: '4px', display: 'block' }}>
            Parameter
          </label>
          <select 
            value={param} 
            onChange={(e) => setParam(e.target.value as any)}
            className="node-select"
            style={{ width: '100%', padding: '4px 8px', fontSize: '12px' }}
          >
            {PARAMS.map((p) => (
              <option key={p.value} value={p.value}>
                {p.label} {p.unit && `(${p.unit})`}
              </option>
            ))}
          </select>
        </div>
        
        {/* Operator selector */}
        <div style={{ marginBottom: '8px' }}>
          <label style={{ fontSize: '10px', color: 'var(--text-muted)', marginBottom: '4px', display: 'block' }}>
            Operator
          </label>
          <select 
            value={operator} 
            onChange={(e) => setOperator(e.target.value as any)}
            className="node-select"
            style={{ width: '100%', padding: '4px 8px', fontSize: '12px' }}
          >
            {OPERATORS.map((op) => (
              <option key={op.value} value={op.value}>
                {op.symbol} {op.label}
              </option>
            ))}
          </select>
        </div>
        
        {/* Value input */}
        <div>
          <label style={{ fontSize: '10px', color: 'var(--text-muted)', marginBottom: '4px', display: 'block' }}>
            Value
          </label>
          <input 
            type="number"
            value={value}
            onChange={(e) => setValue(parseFloat(e.target.value) || 0)}
            className="node-input"
            style={{ width: '100%', padding: '4px 8px', fontSize: '12px' }}
            step={param === 'g_load' ? 0.1 : 1}
          />
        </div>
        
        {/* Formula preview */}
        <div style={{ 
          marginTop: '10px', 
          padding: '8px', 
          background: 'rgba(0, 0, 0, 0.3)', 
          borderRadius: '4px',
          fontSize: '11px',
          color: selectedOp?.color,
          fontFamily: 'monospace',
          textAlign: 'center'
        }}>
          {selectedParam?.label} {selectedOp?.symbol} {value}{selectedParam?.unit}
        </div>
      </div>
      
      {/* Output */}
      <Handle 
        type="source" 
        position={Position.Right} 
        style={{ 
          background: '#00ff88', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(0, 255, 136, 0.6)'
        }}
      />
    </div>
  );
}
