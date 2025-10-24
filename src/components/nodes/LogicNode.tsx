import { useState, useEffect } from 'react';
import { Handle, Position } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Layers } from 'lucide-react';

interface LogicNodeData {
  operation: 'AND' | 'OR' | 'NOT' | 'XOR';
}

export function LogicNode({ data, id }: { data: LogicNodeData; id: string }) {
  const { t } = useTranslation();
  const [operation, setOperation] = useState(data.operation || 'AND');
  
  useEffect(() => {
    data.operation = operation;
  }, [operation, data]);
  
  const OPERATIONS = [
    { value: 'AND', label: 'AND', symbol: '&', color: '#3b82f6', desc: 'All inputs must be true' },
    { value: 'OR', label: 'OR', symbol: '|', color: '#f59e0b', desc: 'Any input can be true' },
    { value: 'NOT', label: 'NOT', symbol: '!', color: '#ef4444', desc: 'Inverts input' },
    { value: 'XOR', label: 'XOR', symbol: '⊕', color: '#8b5cf6', desc: 'Exactly one must be true' },
  ];
  
  const selectedOp = OPERATIONS.find(op => op.value === operation) || OPERATIONS[0];
  const isNOT = operation === 'NOT';
  
  return (
    <div 
      className="custom-node logic-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: `linear-gradient(135deg, ${selectedOp.color}22 0%, ${selectedOp.color}44 100%)`,
        border: `2px solid ${selectedOp.color}`,
        minWidth: '160px'
      }}
    >
      <div className="node-header" style={{ background: `${selectedOp.color}33` }}>
        <Layers size={16} color={selectedOp.color} />
        <span style={{ color: selectedOp.color }}>Logic Gate</span>
      </div>
      <div className="node-body">
        {/* Input A */}
        <Handle 
          type="target" 
          position={Position.Left} 
          id="inputA"
          style={{ 
            top: isNOT ? '50%' : '35%',
            background: '#10b981', 
            width: 10, 
            height: 10, 
            border: '2px solid #10b981',
            boxShadow: '0 0 6px #10b98188'
          }}
        />
        {!isNOT && (
          <span style={{
            position: 'absolute',
            left: '12px',
            top: 'calc(35% - 8px)',
            fontSize: '9px',
            color: '#10b981',
            fontWeight: 'bold'
          }}>
            A
          </span>
        )}
        
        {/* Input B (not for NOT gate) */}
        {!isNOT && (
          <>
            <Handle 
              type="target" 
              position={Position.Left} 
              id="inputB"
              style={{ 
                top: '65%',
                background: '#10b981', 
                width: 10, 
                height: 10, 
                border: '2px solid #10b981',
                boxShadow: '0 0 6px #10b98188'
              }}
            />
            <span style={{
              position: 'absolute',
              left: '12px',
              top: 'calc(65% - 8px)',
              fontSize: '9px',
              color: '#10b981',
              fontWeight: 'bold'
            }}>
              B
            </span>
          </>
        )}
        
        <div style={{ padding: '8px', textAlign: 'center' }}>
          <select 
            value={operation} 
            onChange={(e) => setOperation(e.target.value as any)}
            className="node-select"
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: `1px solid ${selectedOp.color}`,
              color: selectedOp.color,
              fontSize: '12px',
              fontWeight: 'bold',
              textAlign: 'center'
            }}
          >
            {OPERATIONS.map(op => (
              <option key={op.value} value={op.value}>
                {op.symbol} {op.label}
              </option>
            ))}
          </select>
          
          <div style={{
            marginTop: '8px',
            fontSize: '9px',
            color: 'var(--text-muted)',
            lineHeight: '1.3',
            opacity: 0.8
          }}>
            {selectedOp.desc}
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="output"
        style={{ 
          background: selectedOp.color, 
          width: 12, 
          height: 12, 
          border: `2px solid ${selectedOp.color}`,
          boxShadow: `0 0 8px ${selectedOp.color}88`
        }}
      />
    </div>
  );
}

