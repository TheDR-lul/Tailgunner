import { useCallback, useState } from 'react';
import ReactFlow, {
  Node,
  Edge,
  Controls,
  Background,
  BackgroundVariant,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
  MiniMap,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { useTranslation } from 'react-i18next';

import { InputNode } from './nodes/InputNode';
import { ConditionNode } from './nodes/ConditionNode';
import { VibrationNode } from './nodes/VibrationNode';
import { OutputNode } from './nodes/OutputNode';

const nodeTypes = {
  input: InputNode,
  condition: ConditionNode,
  vibration: VibrationNode,
  output: OutputNode,
};

const initialNodes: Node[] = [
  {
    id: 'speed-input',
    type: 'input',
    position: { x: 50, y: 100 },
    data: { label: 'Скорость', indicator: 'IAS (км/ч)', value: 0 },
  },
  {
    id: 'condition-1',
    type: 'condition',
    position: { x: 300, y: 100 },
    data: { operator: '>', threshold: 600 },
  },
  {
    id: 'vibration-1',
    type: 'vibration',
    position: { x: 550, y: 100 },
    data: {
      duration: 1.5,
      curve: [
        { x: 0, y: 0 },
        { x: 0.2, y: 1.0 },
        { x: 0.8, y: 0.5 },
        { x: 1.0, y: 0 },
      ],
    },
  },
  {
    id: 'output-1',
    type: 'output',
    position: { x: 850, y: 100 },
    data: {},
  },
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: 'speed-input', target: 'condition-1', sourceHandle: 'value', targetHandle: 'input' },
  { id: 'e2-3', source: 'condition-1', target: 'vibration-1', sourceHandle: 'true', targetHandle: 'trigger' },
  { id: 'e3-4', source: 'vibration-1', target: 'output-1', sourceHandle: 'output', targetHandle: 'vibration' },
];

export function NodeEditor() {
  const { t } = useTranslation();
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const [selectedNodeType, setSelectedNodeType] = useState<string>('input');

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const addNode = (type: string) => {
    const newNode: Node = {
      id: `${type}-${Date.now()}`,
      type,
      position: { x: Math.random() * 400, y: Math.random() * 400 },
      data: getDefaultNodeData(type),
    };
    setNodes((nds) => [...nds, newNode]);
  };

  const getDefaultNodeData = (type: string) => {
    switch (type) {
      case 'input':
        return { label: 'Индикатор', indicator: 'Скорость', value: 0 };
      case 'condition':
        return { operator: '>', threshold: 100 };
      case 'vibration':
        return {
          duration: 1.0,
          curve: [
            { x: 0, y: 0 },
            { x: 0.5, y: 1.0 },
            { x: 1.0, y: 0 },
          ],
        };
      case 'output':
        return {};
      default:
        return {};
    }
  };

  const clearAll = () => {
    setNodes([]);
    setEdges([]);
  };

  const exportConfig = () => {
    const config = { nodes, edges };
    const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'trigger-config.json';
    a.click();
  };

  return (
    <div className="card" style={{ height: '600px', position: 'relative' }}>
      <div className="card-header">
        <div>
          <h2>🎨 Нодовый редактор триггеров</h2>
          <p>Собирайте триггеры как в Blender</p>
        </div>
      </div>

      {/* Toolbar */}
      <div className="node-toolbar">
        <div className="node-buttons">
          <button className="btn btn-primary" onClick={() => addNode('input')}>
            ➕ Индикатор
          </button>
          <button className="btn btn-primary" onClick={() => addNode('condition')}>
            ➕ Условие
          </button>
          <button className="btn btn-primary" onClick={() => addNode('vibration')}>
            ➕ Вибрация
          </button>
          <button className="btn btn-primary" onClick={() => addNode('output')}>
            ➕ Устройство
          </button>
        </div>
        
        <div className="node-actions">
          <button className="btn btn-secondary" onClick={exportConfig}>
            💾 Экспорт
          </button>
          <button className="btn btn-danger" onClick={clearAll}>
            🗑️ Очистить
          </button>
        </div>
      </div>

      {/* React Flow Canvas */}
      <div style={{ height: 'calc(100% - 120px)' }}>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          nodeTypes={nodeTypes}
          fitView
        >
          <Background variant={BackgroundVariant.Dots} gap={16} size={1} />
          <Controls />
          <MiniMap
            nodeColor={(node) => {
              switch (node.type) {
                case 'input': return '#3b82f6';
                case 'condition': return '#8b5cf6';
                case 'vibration': return '#ec4899';
                case 'output': return '#10b981';
                default: return '#6b7280';
              }
            }}
            maskColor="rgba(0, 0, 0, 0.2)"
          />
        </ReactFlow>
      </div>
    </div>
  );
}

