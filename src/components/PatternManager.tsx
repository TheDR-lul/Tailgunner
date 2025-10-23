import { useTranslation } from 'react-i18next';
import { Plus, Edit, Trash2, Power } from 'lucide-react';
import { usePatterns, Pattern } from '../hooks/usePatterns';

export function PatternManager({ onEditPattern }: { onEditPattern: (pattern?: Pattern) => void }) {
  const { t } = useTranslation();
  const { patterns, togglePattern, deletePattern } = usePatterns();

  const handleDelete = (id: string) => {
    if (confirm(t('pattern_manager.delete_confirm'))) {
      deletePattern(id);
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        <div>
          <h2>{t('pattern_manager.title')}</h2>
          <p>{t('pattern_manager.description')}</p>
        </div>
        <button className="btn btn-primary" onClick={() => onEditPattern()}>
          <Plus size={18} /> {t('pattern_manager.create_new')}
        </button>
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
          patterns.map((pattern) => (
            <div key={pattern.id} className={`pattern-item ${pattern.enabled ? 'enabled' : 'disabled'}`}>
              <div className="pattern-info">
                <h3>{pattern.name}</h3>
                <p>{t('pattern_manager.nodes_count', { count: pattern.nodes.length })}</p>
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
          ))
        )}
      </div>
    </div>
  );
}

