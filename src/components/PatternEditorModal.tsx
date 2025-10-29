import { useState, useCallback, useEffect, useMemo } from 'react';
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
import { 
  X, 
  Save, 
  Download, 
  Upload, 
  Gauge, 
  Zap, 
  Filter, 
  Vibrate, 
  Radio,
  Trash2,
  Grid3x3,
  GitBranch,
  Cable,
  CheckCircle,
  AlertCircle,
  RotateCcw,
  Layout
} from 'lucide-react';
import './PatternEditorModal.css';

import { InputNode } from './nodes/InputNode';
import { ConditionNode } from './nodes/ConditionNode';
import { MultiConditionNode } from './nodes/MultiConditionNode';
import { VibrationNode } from './nodes/VibrationNode';
import { OutputNode } from './nodes/OutputNode';
import { LogicNode } from './nodes/LogicNode';
import { EventNode } from './nodes/EventNode';
import { LinearNode } from './nodes/LinearNode';
import { RotateNode } from './nodes/RotateNode';

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
  onSave: (name: string, nodes: Node[], edges: Edge[], cooldownMs?: number) => void;
  initialData?: {
    name: string;
    nodes: Node[];
    edges: Edge[];
    cooldownMs?: number;
  };
}

export function PatternEditorModal({ isOpen, onClose, onSave, initialData }: PatternEditorModalProps) {
  const { t } = useTranslation();
  const [patternName, setPatternName] = useState(initialData?.name || '');
  const [nameError, setNameError] = useState(false);
  const [cooldownMs, setCooldownMs] = useState(initialData?.cooldownMs || 1000);
  const [nodes, setNodes, onNodesChange] = useNodesState(initialData?.nodes || []);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialData?.edges || []);
  const [snapToGrid, setSnapToGrid] = useState(true);

  // Update nodes and edges when initialData changes (e.g., when editing a pattern)
  // OR clear them when creating a new pattern (initialData === undefined)
  useEffect(() => {
    if (initialData) {
      setPatternName(initialData.name || '');
      setCooldownMs(initialData.cooldownMs || 1000);
      setNodes(initialData.nodes || []);
      setEdges(initialData.edges || []);
      console.log('[Pattern Editor] ✏️ Loaded pattern:', initialData.name, 'Cooldown:', initialData.cooldownMs, 'Nodes:', initialData.nodes?.length, 'Edges:', initialData.edges?.length);
    } else if (isOpen) {
      // Clear state when creating a new pattern
      setPatternName('');
      setCooldownMs(1000);
      setNodes([]);
      setEdges([]);
      console.log('[Pattern Editor] ✨ Creating new pattern');
    }
  }, [initialData, isOpen, setNodes, setEdges]);

  const handleSave = useCallback(() => {
    if (!patternName.trim()) {
      setNameError(true);
      return;
    }
    setNameError(false);
    onSave(patternName, nodes, edges, cooldownMs);
    onClose();
  }, [patternName, nodes, edges, cooldownMs, onSave, onClose]);

  // Keyboard shortcuts
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl/Cmd + S = Save
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, handleSave]);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  // Statistics
  const stats = useMemo(() => {
    const inputNodes = nodes.filter(n => n.type === 'input' || n.type === 'event').length;
    const conditionNodes = nodes.filter(n => n.type === 'condition' || n.type === 'multiCondition' || n.type === 'logic').length;
    const outputNodes = nodes.filter(n => n.type === 'output' || n.type === 'vibration' || n.type === 'linear' || n.type === 'rotate').length;
    
    return {
      totalNodes: nodes.length,
      totalEdges: edges.length,
      inputNodes,
      conditionNodes,
      outputNodes,
      isValid: inputNodes > 0 && outputNodes > 0,
    };
  }, [nodes, edges]);

  const clearAll = useCallback(() => {
    if (confirm('Clear all nodes and connections?')) {
      setNodes([]);
      setEdges([]);
    }
  }, [setNodes, setEdges]);

  const autoLayout = useCallback(() => {
    // Simple auto-layout: arrange nodes in columns by type
    const inputNodes = nodes.filter(n => n.type === 'input' || n.type === 'event');
    const conditionNodes = nodes.filter(n => n.type === 'condition' || n.type === 'multiCondition' || n.type === 'logic');
    const outputNodes = nodes.filter(n => n.type === 'vibration' || n.type === 'linear' || n.type === 'rotate' || n.type === 'output');
    
    const layoutNodes = [...inputNodes, ...conditionNodes, ...outputNodes];
    const nodeSpacing = 120;
    const columnSpacing = 300;
    
    const updatedNodes = layoutNodes.map((node, index) => {
      let column = 0;
      let row = index;
      
      if (conditionNodes.includes(node)) {
        column = 1;
        row = conditionNodes.indexOf(node);
      } else if (outputNodes.includes(node)) {
        column = 2;
        row = outputNodes.indexOf(node);
      } else {
        row = inputNodes.indexOf(node);
      }
      
      return {
        ...node,
        position: {
          x: 100 + column * columnSpacing,
          y: 100 + row * nodeSpacing,
        },
      };
    });
    
    setNodes(updatedNodes);
  }, [nodes, setNodes]);

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
        return { operator: '>', value: 100 };
      case 'multiCondition':
        return { logic: 'AND', conditions: [{ id: '1', operator: '>', value: 100 }] };
      case 'logic':
        return { operation: 'AND' };
      case 'event':
        return { event: 'Hit', filter_type: 'any' };
      case 'output':
        return { deviceMode: 'all' };
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

  const exportPattern = async () => {
    try {
      const { save } = await import('@tauri-apps/plugin-dialog');
      
      const config = { name: patternName, nodes, edges };
      const jsonContent = JSON.stringify(config, null, 2);
      
      // Open save dialog with suggested filename
      const filePath = await save({
        title: 'Export Pattern',
        defaultPath: `${patternName || 'pattern'}.json`,
        filters: [{
          name: 'Pattern File',
          extensions: ['json']
        }]
      });
      
      if (filePath) {
        const { writeTextFile } = await import('@tauri-apps/plugin-fs');
        await writeTextFile(filePath, jsonContent);
        
        if ((window as any).debugLog) {
          (window as any).debugLog('success', `✅ Pattern exported to: ${filePath}`);
        }
      }
    } catch (error) {
      console.error('Export failed:', error);
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Export failed: ${error}`);
      }
    }
  };

  const importPattern = async () => {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      
      // Open file dialog
      const filePath = await open({
        title: 'Import Pattern',
        filters: [{
          name: 'Pattern File',
          extensions: ['json']
        }],
        multiple: false
      });
      
      if (filePath && typeof filePath === 'string') {
        const { readTextFile } = await import('@tauri-apps/plugin-fs');
        const content = await readTextFile(filePath);
        
        const config = JSON.parse(content);
        setPatternName(config.name || '');
        setNodes(config.nodes || []);
        setEdges(config.edges || []);
        
        console.log('[Pattern Editor] ✅ Imported pattern:', config.name, 'from:', filePath);
        
        if ((window as any).debugLog) {
          (window as any).debugLog('success', `✅ Pattern imported: ${config.name}`);
        }
      }
    } catch (error) {
      console.error('[Pattern Editor] ❌ Import error:', error);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `Import failed: ${error}`);
      }
      
      console.error('[Pattern Editor] Import failed:', error);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-container pattern-editor-modal" onClick={(e) => e.stopPropagation()}>
        <div className="pattern-editor-container">
          {/* Header */}
          <div className="pattern-editor-header">
            <div className="pattern-editor-title-row">
              <div className="pattern-editor-title">
                <div className="pattern-editor-title-icon">
                  <GitBranch size={20} />
                </div>
                {initialData ? t('pattern_editor.edit_title') : t('pattern_editor.create_title')}
              </div>
              <button className="pattern-editor-close" onClick={onClose}>
                <X size={20} />
              </button>
            </div>

            <div className="pattern-editor-inputs">
              <div className="pattern-name-wrapper">
                <label className="pattern-name-label">Pattern Name</label>
                <input
                  type="text"
                  value={patternName}
                  onChange={(e) => {
                    setPatternName(e.target.value);
                    setNameError(false);
                  }}
                  placeholder={t('pattern_editor.name_placeholder')}
                  className={`pattern-name-input-field ${nameError ? 'error' : ''}`}
                />
              </div>

              <div className="pattern-cooldown-wrapper">
                <label className="pattern-name-label">Cooldown</label>
                <div className="pattern-cooldown-input">
                  <input
                    type="number"
                    value={cooldownMs}
                    onChange={(e) => setCooldownMs(Math.max(0, parseInt(e.target.value) || 0))}
                    min="0"
                    step="100"
                  />
                  <span>ms</span>
                </div>
              </div>
            </div>

            {/* Statistics */}
            <div className="pattern-editor-stats">
              <div className="pattern-stat">
                <GitBranch size={14} className="pattern-stat-icon" />
                <span>Nodes:</span>
                <span className="pattern-stat-value">{stats.totalNodes}</span>
              </div>
              <div className="pattern-stat">
                <Cable size={14} className="pattern-stat-icon" />
                <span>Connections:</span>
                <span className="pattern-stat-value">{stats.totalEdges}</span>
              </div>
              <div className="pattern-stat">
                {stats.isValid ? (
                  <CheckCircle size={14} style={{ color: '#10b981' }} />
                ) : (
                  <AlertCircle size={14} style={{ color: '#f59e0b' }} />
                )}
                <span>Status:</span>
                <span className="pattern-stat-value" style={{ color: stats.isValid ? '#10b981' : '#f59e0b' }}>
                  {stats.isValid ? 'Valid' : 'Incomplete'}
                </span>
              </div>
            </div>
          </div>

          {/* Content: Sidebar + Canvas */}
          <div className="pattern-editor-content">
            {/* Sidebar */}
            <div className="pattern-editor-sidebar">
              {/* Input Nodes */}
              <div className="pattern-editor-sidebar-section">
                <h3 className="pattern-editor-sidebar-title">Inputs</h3>
                <div className="pattern-node-buttons">
                  <button className="pattern-node-btn pattern-node-btn-input" onClick={() => addNode('input')}>
                    <div className="pattern-node-btn-icon">
                      <Gauge size={16} />
                    </div>
                    <span>Input Sensor</span>
                  </button>
                  <button className="pattern-node-btn pattern-node-btn-event" onClick={() => addNode('event')}>
                    <div className="pattern-node-btn-icon">
                      <Zap size={16} />
                    </div>
                    <span>Game Event</span>
                  </button>
                </div>
              </div>

              {/* Condition Nodes */}
              <div className="pattern-editor-sidebar-section">
                <h3 className="pattern-editor-sidebar-title">Logic</h3>
                <div className="pattern-node-buttons">
                  <button className="pattern-node-btn pattern-node-btn-condition" onClick={() => addNode('condition')}>
                    <div className="pattern-node-btn-icon">
                      <Filter size={16} />
                    </div>
                    <span>Condition</span>
                  </button>
                </div>
              </div>

              {/* Action Nodes */}
              <div className="pattern-editor-sidebar-section">
                <h3 className="pattern-editor-sidebar-title">Actions</h3>
                <div className="pattern-node-buttons">
                  <button className="pattern-node-btn pattern-node-btn-vibration" onClick={() => addNode('vibration')}>
                    <div className="pattern-node-btn-icon">
                      <Vibrate size={16} />
                    </div>
                    <span>Vibration</span>
                  </button>
                  <button className="pattern-node-btn" onClick={() => addNode('linear')}>
                    <div className="pattern-node-btn-icon" style={{ background: 'rgba(16, 185, 129, 0.2)', color: '#10b981' }}>
                      <RotateCcw size={16} style={{ transform: 'rotate(90deg)' }} />
                    </div>
                    <span>Linear</span>
                  </button>
                  <button className="pattern-node-btn" onClick={() => addNode('rotate')}>
                    <div className="pattern-node-btn-icon" style={{ background: 'rgba(139, 92, 246, 0.2)', color: '#8b5cf6' }}>
                      <RotateCcw size={16} />
                    </div>
                    <span>Rotate</span>
                  </button>
                </div>
              </div>

              {/* Output Nodes */}
              <div className="pattern-editor-sidebar-section">
                <h3 className="pattern-editor-sidebar-title">Output</h3>
                <div className="pattern-node-buttons">
                  <button className="pattern-node-btn pattern-node-btn-output" onClick={() => addNode('output')}>
                    <div className="pattern-node-btn-icon">
                      <Radio size={16} />
                    </div>
                    <span>Target Devices</span>
                  </button>
                </div>
              </div>

              {/* Editor Tools */}
              <div className="pattern-editor-sidebar-section">
                <h3 className="pattern-editor-sidebar-title">Tools</h3>
                <div className="pattern-editor-actions">
                  <button className="pattern-action-btn" onClick={autoLayout}>
                    <Layout size={14} />
                    Auto Layout
                  </button>
                  <button className="pattern-action-btn" onClick={() => setSnapToGrid(!snapToGrid)}>
                    <Grid3x3 size={14} />
                    Snap: {snapToGrid ? 'ON' : 'OFF'}
                  </button>
                  <button className="pattern-action-btn" onClick={importPattern}>
                    <Upload size={14} />
                    Import
                  </button>
                  <button className="pattern-action-btn" onClick={exportPattern}>
                    <Download size={14} />
                    Export
                  </button>
                  <button className="pattern-action-btn danger" onClick={clearAll}>
                    <Trash2 size={14} />
                    Clear All
                  </button>
                </div>
              </div>
            </div>

            {/* Canvas */}
            <div className="pattern-editor-canvas">
              <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                nodeTypes={nodeTypes}
                snapToGrid={snapToGrid}
                snapGrid={[15, 15]}
                fitView
                proOptions={{ hideAttribution: true }}
                onContextMenu={(e) => e.preventDefault()}
              >
                <Background 
                  variant={BackgroundVariant.Dots} 
                  gap={snapToGrid ? 15 : 16} 
                  size={1} 
                />
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
                    border: '1px solid rgba(139, 92, 246, 0.3)'
                  }}
                />
              </ReactFlow>
            </div>
          </div>

          {/* Footer */}
          <div className="pattern-editor-footer">
            <div className="pattern-editor-shortcuts">
              <div className="pattern-shortcut">
                <span className="pattern-shortcut-key">Del</span>
                <span>Delete</span>
              </div>
              <div className="pattern-shortcut">
                <span className="pattern-shortcut-key">Ctrl+S</span>
                <span>Save</span>
              </div>
              <div className="pattern-shortcut">
                <span className="pattern-shortcut-key">Scroll</span>
                <span>Zoom</span>
              </div>
            </div>
            <div className="pattern-editor-footer-actions">
              <button className="btn btn-secondary" onClick={onClose}>
                {t('common.cancel')}
              </button>
              <button className="btn btn-primary" onClick={handleSave}>
                <Save size={16} /> {t('common.save')}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

