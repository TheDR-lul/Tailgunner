import { Handle, Position } from 'reactflow';

export function OutputNode() {
  return (
    <div className="node-output">
      <Handle type="target" position={Position.Left} id="vibration" />
      
      <div className="node-header">📳 Output</div>
      <div className="node-body">
        <div className="output-icon">🎯</div>
        <div className="output-text">Send to device</div>
      </div>
    </div>
  );
}

