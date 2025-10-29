import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Plus, Edit, Trash2, Power, AlertCircle, AlertTriangle, Sparkles } from 'lucide-react';
import { usePatterns, Pattern } from '../hooks/usePatterns';
import { ConfirmDialog } from './ConfirmDialog';
import { PatternTemplatesModal } from './PatternTemplatesModal';
import { validatePattern } from '../utils/patternValidation';

export function PatternManager({ onEditPattern }: { onEditPattern: (pattern?: Pattern) => void }) {
  const { t } = useTranslation();
  const { patterns, togglePattern, deletePattern } = usePatterns();
  const [isTemplatesOpen, setIsTemplatesOpen] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<{ isOpen: boolean; patternId: string | null }>({
    isOpen: false,
    patternId: null
  });

  const handleDelete = (id: string) => {
    setDeleteConfirm({ isOpen: true, patternId: id });
  };

  const confirmDelete = () => {
    if (deleteConfirm.patternId) {
      deletePattern(deleteConfirm.patternId);
    }
    setDeleteConfirm({ isOpen: false, patternId: null });
  };

  const cancelDelete = () => {
    setDeleteConfirm({ isOpen: false, patternId: null });
  };

  const getPatternValidation = (pattern: Pattern) => {
    // Safety check for nodes and edges
    if (!pattern.nodes || !pattern.edges) {
      return { 
        errors: [{ type: 'error' as const, message: 'pattern_manager.invalid_pattern' }], 
        hasErrors: true, 
        hasWarnings: false 
      };
    }
    
    const errors = validatePattern(pattern.nodes, pattern.edges);
    const hasErrors = errors.some(e => e.type === 'error');
    const hasWarnings = errors.some(e => e.type === 'warning');
    return { errors, hasErrors, hasWarnings };
  };

  return (
    <>
      <div className="card">
        <div className="card-header">
          <div>
            <h2>{t('pattern_manager.title')}</h2>
            <p>{t('pattern_manager.description')}</p>
          </div>
          <div style={{ display: 'flex', gap: '0.5rem' }}>
            <button className="btn btn-secondary" onClick={() => setIsTemplatesOpen(true)}>
              <Sparkles size={18} /> From Template
            </button>
            <button className="btn btn-primary" onClick={() => onEditPattern()}>
              <Plus size={18} /> {t('pattern_manager.create_new')}
            </button>
          </div>
        </div>

        <div className="pattern-list">
          {patterns.length === 0 ? (
            <div className="empty-state">
              <p>{t('pattern_manager.empty')}</p>
              <button className="btn btn-primary" onClick={() => onEditPattern()}>
                {t('pattern_manager.create_first')}
              </button>
            </div>
          ) : (
            patterns.map((pattern) => {
              const validation = getPatternValidation(pattern);
              return (
                <div 
                  key={pattern.id} 
                  className={`pattern-item ${pattern.enabled ? 'enabled' : 'disabled'} ${validation.hasErrors ? 'has-error' : ''}`}
                >
                  <div className="pattern-info">
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                      <h3>{String(pattern.name || 'Unnamed Pattern')}</h3>
                      {validation.hasErrors && (
                        <span 
                          className="pattern-status error" 
                          title={validation.errors.filter(e => e.type === 'error').map(e => t(e.message)).join(', ')}
                        >
                          <AlertCircle size={16} />
                        </span>
                      )}
                      {validation.hasWarnings && !validation.hasErrors && (
                        <span 
                          className="pattern-status warning" 
                          title={validation.errors.filter(e => e.type === 'warning').map(e => t(e.message)).join(', ')}
                        >
                          <AlertTriangle size={16} />
                        </span>
                      )}
                    </div>
                    <p>{t('pattern_manager.nodes_count', { count: pattern.nodes?.length || 0 })}</p>
                  </div>
                  
                  <div className="pattern-actions">
                  <button
                    className={`btn-icon ${pattern.enabled ? 'active' : ''}`}
                    onClick={() => togglePattern(pattern.id)}
                    title={pattern.enabled ? t('common.disable') : t('common.enable')}
                  >
                    <Power size={18} />
                  </button>
                  <button
                    className="btn-icon"
                    onClick={() => onEditPattern(pattern)}
                    title={t('common.edit')}
                  >
                    <Edit size={18} />
                  </button>
                  <button
                    className="btn-icon danger"
                    onClick={() => handleDelete(pattern.id)}
                    title={t('common.delete')}
                  >
                    <Trash2 size={18} />
                  </button>
                </div>
              </div>
              );
            })
          )}
        </div>
      </div>

      <ConfirmDialog
        isOpen={deleteConfirm.isOpen}
        message={t('pattern_manager.delete_confirm')}
        onConfirm={confirmDelete}
        onCancel={cancelDelete}
      />

      <PatternTemplatesModal
        isOpen={isTemplatesOpen}
        onClose={() => setIsTemplatesOpen(false)}
      />
    </>
  );
}

