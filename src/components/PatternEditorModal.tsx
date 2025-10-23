import { useState, useCallback } from 'react';
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
import { X, Save, Download, Upload } from 'lucide-react';

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

interface PatternEditorModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (name: string, nodes: Node[], edges: Edge[]) => void;
  initialData?: {
    name: string;
    nodes: Node[];
    edges: Edge[];
  };
}

export function PatternEditorModal({ isOpen, onClose, onSave, initialData }: PatternEditorModalProps) {
  const { t } = useTranslation();
  const [patternName, setPatternName] = useState(initialData?.name || '');
  const [nodes, setNodes, onNodesChange] = useNodesState(initialData?.nodes || []);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialData?.edges || []);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const addNode = (type: string) => {
    const newNode: Node = {
      id: `${type}-${Date.now()}`,
      type,
      position: { x: Math.random() * 400 + 100, y: Math.random() * 300 + 100 },
      data: getDefaultNodeData(type),
    };
    setNodes((nds) => [...nds, newNode]);
  };

  const getDefaultNodeData = (type: string) => {
    switch (type) {
      case 'input':
        return { label: t('nodes.input.default_label'), indicator: 'speed', value: 0 };
      case 'condition':
        return { operator: '>', threshold: 100 };
      case 'vibration':
        return {
          duration: 1.0,
          mode: 'once',
          repeatCount: 3,
          curve: [
            { x: 0.4, y: 0.6 },
            { x: 0.6, y: 0.8 },
          ],
        };
      case 'output':
        return {};
      default:
        return {};
    }
  };

  const handleSave = () => {
    if (!patternName.trim()) {
      alert(t('pattern_editor.name_required'));
      return;
    }
    onSave(patternName, nodes, edges);
    onClose();
  };

  const exportPattern = () => {
    const config = { name: patternName, nodes, edges };
    const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${patternName || 'pattern'}.json`;
    a.click();
  };

  const importPattern = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = (e: any) => {
      const file = e.target.files[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          try {
            const config = JSON.parse(event.target?.result as string);
            setPatternName(config.name || '');
            setNodes(config.nodes || []);
            setEdges(config.edges || []);
          } catch (error) {
            alert(t('pattern_editor.import_error'));
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  };

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <div>
            <h2>{initialData ? t('pattern_editor.edit_title') : t('pattern_editor.create_title')}</h2>
            <input
              type="text"
              value={patternName}
              onChange={(e) => setPatternName(e.target.value)}
              placeholder={t('pattern_editor.name_placeholder')}
              className="pattern-name-input"
            />
          </div>
          <button className="btn-icon" onClick={onClose}>
            <X size={24} />
          </button>
        </div>

        <div className="modal-toolbar">
          <div className="node-buttons">
            <button className="btn btn-sm" onClick={() => addNode('input')}>
              ➕ {t('nodes.input.title')}
            </button>
            <button className="btn btn-sm" onClick={() => addNode('condition')}>
              ➕ {t('nodes.condition.title')}
            </button>
            <button className="btn btn-sm" onClick={() => addNode('vibration')}>
              ➕ {t('nodes.vibration.title')}
            </button>
            <button className="btn btn-sm" onClick={() => addNode('output')}>
              ➕ {t('nodes.output.title')}
            </button>
          </div>
          
          <div className="node-actions">
            <button className="btn btn-secondary btn-sm" onClick={importPattern}>
              <Upload size={16} /> {t('pattern_editor.import')}
            </button>
            <button className="btn btn-secondary btn-sm" onClick={exportPattern}>
              <Download size={16} /> {t('pattern_editor.export')}
            </button>
          </div>
        </div>

        <div className="modal-body">
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

        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose}>
            {t('common.cancel')}
          </button>
          <button className="btn btn-primary" onClick={handleSave}>
            <Save size={16} /> {t('common.save')}
          </button>
        </div>
      </div>
    </div>
  );
}

