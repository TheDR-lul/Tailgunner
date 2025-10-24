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
import { LogicNode } from './nodes/LogicNode';
import { EventNode } from './nodes/EventNode';
import { LinearNode } from './nodes/LinearNode';
import { RotateNode } from './nodes/RotateNode';
import { MultiConditionNode } from './nodes/MultiConditionNode';

const nodeTypes = {
  input: InputNode,
  condition: ConditionNode,
  multiCondition: MultiConditionNode,
  vibration: VibrationNode,
  linear: LinearNode,
  rotate: RotateNode,
  output: OutputNode,
  logic: LogicNode,
  event: EventNode,
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
      case 'multiCondition':
        return { logic: 'AND', conditions: [{ id: '1', operator: '>', value: 100 }] };
      case 'logic':
        return { operation: 'AND' };
      case 'event':
        return { event: 'Hit' };
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
      case 'linear':
        return { duration: 1.0, position: 0.5, mode: 'once' };
      case 'rotate':
        return { duration: 1.0, speed: 0.5, clockwise: true, mode: 'once' };
      case 'output':
        return { deviceMode: 'all', deviceType: 'vibrator', selectedDevices: [] };
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
            <div style={{ display: 'flex', gap: '6px', flexWrap: 'wrap' }}>
              {/* Inputs */}
              <button 
                className="btn btn-sm node-btn-input" 
                onClick={() => addNode('input')}
                title="Add input sensor (Speed, G-Load, Fuel, etc.)"
              >
                📊 Input
              </button>
              <button 
                className="btn btn-sm node-btn-event" 
                onClick={() => addNode('event')}
                title="Add game event trigger (Hit, Overspeed, etc.)"
              >
                ⚡ Event
              </button>
              
              {/* Conditions */}
              <button 
                className="btn btn-sm node-btn-condition" 
                onClick={() => addNode('condition')}
                title="Add single condition (>, <, =, between)"
              >
                🔍 Condition
              </button>
              <button 
                className="btn btn-sm" 
                onClick={() => addNode('multiCondition')}
                title="Add multiple conditions (AND/OR)"
                style={{
                  background: 'linear-gradient(135deg, #7c2d12 0%, #c2410c 100%)',
                  border: '1px solid #f97316',
                  color: '#fff'
                }}
              >
                🎯 Multi
              </button>
              <button 
                className="btn btn-sm node-btn-logic" 
                onClick={() => addNode('logic')}
                title="Add logic gate (AND/OR/NOT/XOR)"
              >
                ⚙️ Logic
              </button>
              
              {/* Outputs */}
              <button 
                className="btn btn-sm node-btn-vibration" 
                onClick={() => addNode('vibration')}
                title="Add vibration pattern"
              >
                💥 Vibro
              </button>
              <button 
                className="btn btn-sm" 
                onClick={() => addNode('linear')}
                title="Add linear motion (strokers, thrusters)"
                style={{
                  background: 'linear-gradient(135deg, #065f46 0%, #059669 100%)',
                  border: '1px solid #10b981',
                  color: '#fff'
                }}
              >
                📏 Linear
              </button>
              <button 
                className="btn btn-sm" 
                onClick={() => addNode('rotate')}
                title="Add rotation (rotating toys)"
                style={{
                  background: 'linear-gradient(135deg, #4c1d95 0%, #6d28d9 100%)',
                  border: '1px solid #8b5cf6',
                  color: '#fff'
                }}
              >
                🔄 Rotate
              </button>
              <button 
                className="btn btn-sm node-btn-output" 
                onClick={() => addNode('output')}
                title="Add output to device"
              >
                📡 Output
              </button>
            </div>
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
                  case 'event': return '#f59e0b';
                  case 'condition': return '#a855f7';
                  case 'multiCondition': return '#f97316';
                  case 'logic': return '#6366f1';
                  case 'vibration': return '#ec4899';
                  case 'linear': return '#10b981';
                  case 'rotate': return '#8b5cf6';
                  case 'output': return '#10b981';
                  default: return '#6b7280';
                }
              }}
              maskColor="rgba(0, 0, 0, 0.3)"
              style={{
                background: 'rgba(0, 0, 0, 0.4)',
                border: '1px solid rgba(255, 153, 51, 0.3)'
              }}
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

