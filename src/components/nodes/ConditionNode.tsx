import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { GitBranch, Check, X } from 'lucide-react';

interface ConditionNodeData {
  operator: '>' | '<' | '=' | '>=' | '<=' | '!=' | 'between' | 'outside';
  threshold: number;
  threshold2?: number;
}

export function ConditionNode({ data, id, selected }: { data: ConditionNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [operator, setOperator] = useState(data.operator || '>');
  const [threshold, setThreshold] = useState(data.threshold || 0);
  const [threshold2, setThreshold2] = useState(data.threshold2 || 0);
  
  const needsTwoValues = operator === 'between' || operator === 'outside';
  
  useEffect(() => {
    data.operator = operator;
    data.threshold = threshold;
    if (needsTwoValues) {
      data.threshold2 = threshold2;
    }
  }, [operator, threshold, threshold2, needsTwoValues, data]);
  
  const OPERATORS = [
    { value: '>', label: 'Greater than (>)', symbol: '>' },
    { value: '<', label: 'Less than (<)', symbol: '<' },
    { value: '=', label: 'Equal (=)', symbol: '=' },
    { value: '>=', label: 'Greater or equal (≥)', symbol: '≥' },
    { value: '<=', label: 'Less or equal (≤)', symbol: '≤' },
    { value: '!=', label: 'Not equal (≠)', symbol: '≠' },
    { value: 'between', label: 'Between', symbol: '↔' },
    { value: 'outside', label: 'Outside', symbol: '↮' },
  ];
  
  const selectedOp = OPERATORS.find(op => op.value === operator);
  
  return (
    <div 
      className="custom-node condition-node" 
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
        minHeight={140}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(93, 138, 168, 0.2)' }}>
        <GitBranch size={16} color="#5d8aa8" />
        <span style={{ color: '#5d8aa8' }}>{t('nodes.condition.title')}</span>
      </div>
      <div className="node-body">
        <Handle 
          type="target" 
          position={Position.Left} 
          id="input"
          style={{ 
            background: '#ff9933', 
            width: 12, 
            height: 12, 
            border: 'none',
            boxShadow: '0 0 8px rgba(255, 153, 51, 0.6)'
          }}
        />
        
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', padding: '4px' }}>
          <select 
            value={operator} 
            onChange={(e) => setOperator(e.target.value as any)}
            className="node-select"
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: '1px solid rgba(93, 138, 168, 0.5)',
              color: '#94a3b8',
              fontSize: '12px'
            }}
          >
            {OPERATORS.map(op => (
              <option key={op.value} value={op.value}>
                {op.symbol} {op.label}
              </option>
            ))}
          </select>
          
          <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
            <input
              type="number"
              value={threshold}
              onChange={(e) => setThreshold(parseFloat(e.target.value))}
              className="node-input-field"
              step="0.1"
              placeholder="Value"
              onClick={(e) => e.stopPropagation()}
              onMouseDown={(e) => e.stopPropagation()}
              style={{
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid rgba(93, 138, 168, 0.5)',
                color: '#e2e8f0',
                fontSize: '11px',
                padding: '4px 6px',
                borderRadius: '4px',
                flex: 1
              }}
            />
            
            {needsTwoValues && (
              <>
                <span style={{ color: '#5d8aa8', fontSize: '10px' }}>↔</span>
                <input
                  type="number"
                  value={threshold2}
                  onChange={(e) => setThreshold2(parseFloat(e.target.value))}
                  className="node-input-field"
                  step="0.1"
                  placeholder="Value 2"
                  onClick={(e) => e.stopPropagation()}
                  onMouseDown={(e) => e.stopPropagation()}
                  style={{
                    background: 'rgba(0, 0, 0, 0.3)',
                    border: '1px solid rgba(93, 138, 168, 0.5)',
                    color: '#e2e8f0',
                    fontSize: '11px',
                    padding: '4px 6px',
                    borderRadius: '4px',
                    flex: 1
                  }}
                />
              </>
            )}
          </div>
          
          <div style={{ 
            fontSize: '10px', 
            color: '#5d8aa8', 
            textAlign: 'center',
            padding: '4px',
            background: 'rgba(93, 138, 168, 0.1)',
            borderRadius: '4px'
          }}>
            {selectedOp?.symbol} {threshold.toFixed(1)}{needsTwoValues && ` ↔ ${threshold2.toFixed(1)}`}
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="true" 
        style={{ 
          top: '40%', 
          background: '#4ade80', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(74, 222, 128, 0.6)'
        }}
      />
      <span style={{
        position: 'absolute',
        right: '18px',
        top: 'calc(40% - 8px)',
        fontSize: '10px',
        color: '#4ade80',
        fontWeight: 'bold',
        pointerEvents: 'none'
      }}>
        <Check size={12} />
      </span>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="false" 
        style={{ 
          top: '70%', 
          background: '#f87171', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(248, 113, 113, 0.6)'
        }}
      />
      <span style={{
        position: 'absolute',
        right: '18px',
        top: 'calc(70% - 8px)',
        fontSize: '10px',
        color: '#f87171',
        fontWeight: 'bold',
        pointerEvents: 'none'
      }}>
        <X size={12} />
      </span>
    </div>
  );
}
