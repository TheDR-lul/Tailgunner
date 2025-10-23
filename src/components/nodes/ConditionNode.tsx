import { useState } from 'react';
import { Handle, Position } from 'reactflow';

interface ConditionNodeData {
  operator: '>' | '<' | '=' | '>=' | '<=';
  threshold: number;
}

export function ConditionNode({ data, id }: { data: ConditionNodeData; id: string }) {
  const [operator, setOperator] = useState(data.operator || '>');
  const [threshold, setThreshold] = useState(data.threshold || 0);
  
  return (
    <div className="node-condition" onClick={(e) => e.stopPropagation()}>
      <div className="node-header">🔍 Условие</div>
      <div className="node-body">
        <Handle type="target" position={Position.Left} id="input" />
        
        <div className="condition-controls">
          <select 
            value={operator} 
            onChange={(e) => setOperator(e.target.value as any)}
            className="node-select"
          >
            <option value=">">{'>'}</option>
            <option value="<">{'<'}</option>
            <option value="=">{'='}</option>
            <option value=">=">{'>='}</option>
            <option value="<=">{'<='}</option>
          </select>
          
          <input
            type="number"
            value={threshold}
            onChange={(e) => setThreshold(parseFloat(e.target.value))}
            className="node-input-field"
            step="0.1"
            onClick={(e) => e.stopPropagation()}
            onMouseDown={(e) => e.stopPropagation()}
          />
        </div>
      </div>
      <Handle type="source" position={Position.Right} id="true" style={{ top: '30%' }} />
      <Handle type="source" position={Position.Right} id="false" style={{ top: '70%' }} />
    </div>
  );
}

