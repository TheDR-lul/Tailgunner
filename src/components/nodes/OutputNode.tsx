import { Handle, Position } from 'reactflow';

export function OutputNode() {
  return (
    <div className="node-output">
      <Handle type="target" position={Position.Left} id="vibration" />
      
      <div className="node-header">📳 Устройство</div>
      <div className="node-body">
        <div className="output-icon">🎯</div>
        <div className="output-text">Отправить на устройство</div>
      </div>
    </div>
  );
}

