import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Filter, Plus, X } from 'lucide-react';

interface Condition {
  id: string;
  operator: '>' | '<' | '=' | '>=' | '<=';
  value: number;
}

interface MultiConditionNodeData {
  logic: 'AND' | 'OR';
  conditions: Condition[];
}

export function MultiConditionNode({ data, id, selected }: { data: MultiConditionNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [logic, setLogic] = useState(data.logic || 'AND');
  const [conditions, setConditions] = useState<Condition[]>(data.conditions || [
    { id: '1', operator: '>', value: 100 }
  ]);
  
  useEffect(() => {
    data.logic = logic;
    data.conditions = conditions;
  }, [logic, conditions, data]);
  
  const addCondition = () => {
    setConditions([...conditions, { 
      id: Date.now().toString(), 
      operator: '>', 
      value: 0 
    }]);
  };
  
  const removeCondition = (condId: string) => {
    if (conditions.length > 1) {
      setConditions(conditions.filter(c => c.id !== condId));
    }
  };
  
  const updateCondition = (condId: string, field: keyof Condition, value: any) => {
    setConditions(conditions.map(c => 
      c.id === condId ? { ...c, [field]: value } : c
    ));
  };
  
  return (
    <div 
      className="custom-node multi-condition-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '2px solid rgba(93, 138, 168, 0.4)',
        minWidth: '220px',
        maxWidth: '280px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={220} 
        minHeight={160}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(93, 138, 168, 0.2)' }}>
        <Filter size={16} color="#5d8aa8" />
        <span style={{ color: '#5d8aa8' }}>Multi Condition</span>
      </div>
      <div className="node-body">
        {/* Multiple input handles */}
        {conditions.map((cond, idx) => (
          <Handle 
            key={cond.id}
            type="target" 
            position={Position.Left} 
            id={`input-${cond.id}`}
            style={{ 
              top: `${30 + (idx * 60)}px`,
              background: '#3b82f6', 
              width: 10, 
              height: 10, 
              border: '2px solid #3b82f6',
              boxShadow: '0 0 6px #3b82f688'
            }}
          />
        ))}
        
        <div style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
            <select 
              value={logic}
              onChange={(e) => setLogic(e.target.value as any)}
              className="node-select"
              style={{
                flex: 1,
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid #f97316',
                color: '#f97316',
                fontSize: '12px',
                fontWeight: 'bold'
              }}
            >
              <option value="AND">ALL (AND)</option>
              <option value="OR">ANY (OR)</option>
            </select>
            
            <button
              onClick={(e) => { e.stopPropagation(); addCondition(); }}
              style={{
                background: 'rgba(16, 185, 129, 0.3)',
                border: '1px solid #10b981',
                color: '#10b981',
                padding: '4px 6px',
                borderRadius: '4px',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center'
              }}
              title="Add condition"
            >
              <Plus size={14} />
            </button>
          </div>
          
          {conditions.map((cond, idx) => (
            <div 
              key={cond.id}
              style={{
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid rgba(249, 115, 22, 0.3)',
                borderRadius: '4px',
                padding: '6px',
                display: 'flex',
                gap: '4px',
                alignItems: 'center'
              }}
            >
              <span style={{ fontSize: '9px', color: '#f97316' }}>
                #{idx + 1}
              </span>
              
              <select 
                value={cond.operator}
                onChange={(e) => updateCondition(cond.id, 'operator', e.target.value)}
                className="node-select"
                style={{
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid #f97316',
                  color: '#f97316',
                  fontSize: '10px',
                  flex: '0 0 45px'
                }}
              >
                <option value=">">{'>'}</option>
                <option value="<">{'<'}</option>
                <option value="=">{'='}</option>
                <option value=">=">{'>='}</option>
                <option value="<=">{'<='}</option>
              </select>
              
              <input
                type="number"
                value={cond.value}
                onChange={(e) => updateCondition(cond.id, 'value', parseFloat(e.target.value))}
                className="node-input-field"
                step="0.1"
                onClick={(e) => e.stopPropagation()}
                onMouseDown={(e) => e.stopPropagation()}
                style={{
                  flex: 1,
                  background: 'rgba(0, 0, 0, 0.3)',
                  border: '1px solid #f97316',
                  color: '#fff',
                  fontSize: '10px',
                  padding: '3px 5px',
                  borderRadius: '3px'
                }}
              />
              
              {conditions.length > 1 && (
                <button
                  onClick={(e) => { e.stopPropagation(); removeCondition(cond.id); }}
                  style={{
                    background: 'rgba(239, 68, 68, 0.3)',
                    border: '1px solid #ef4444',
                    color: '#ef4444',
                    padding: '2px 4px',
                    borderRadius: '3px',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center'
                  }}
                  title="Remove"
                >
                  <X size={12} />
                </button>
              )}
            </div>
          ))}
          
          <div style={{
            padding: '5px',
            background: 'rgba(249, 115, 22, 0.15)',
            borderRadius: '4px',
            fontSize: '8px',
            color: '#f97316',
            textAlign: 'center',
            lineHeight: '1.3'
          }}>
            {logic === 'AND' ? 'All conditions must be true' : 'Any condition can be true'}
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="output"
        style={{ 
          background: '#f97316', 
          width: 12, 
          height: 12, 
          border: '2px solid #f97316',
          boxShadow: '0 0 8px #f9731688'
        }}
      />
    </div>
  );
}

