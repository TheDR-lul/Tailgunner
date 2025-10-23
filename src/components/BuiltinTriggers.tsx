import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';
import { Zap, Power, Info } from 'lucide-react';

interface EventTrigger {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  is_builtin: boolean;
  cooldown_ms: number;
}

export function BuiltinTriggers() {
  const { t } = useTranslation();
  const [triggers, setTriggers] = useState<EventTrigger[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadTriggers();
  }, []);

  const loadTriggers = async () => {
    try {
      const result = await invoke<EventTrigger[]>('get_triggers');
      setTriggers(result);
      setLoading(false);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('info', `Загружено ${result.length} встроенных триггеров`);
      }
    } catch (error) {
      console.error('Failed to load triggers:', error);
      setLoading(false);
    }
  };

  const toggleTrigger = async (id: string, enabled: boolean) => {
    try {
      await invoke('toggle_trigger', { id, enabled });
      setTriggers(prev => prev.map(t => 
        t.id === id ? { ...t, enabled } : t
      ));
      
      if ((window as any).debugLog) {
        const trigger = triggers.find(t => t.id === id);
        (window as any).debugLog(
          enabled ? 'success' : 'warn', 
          `${trigger?.name}: ${enabled ? 'включен' : 'выключен'}`
        );
      }
    } catch (error) {
      console.error('Failed to toggle trigger:', error);
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        <div>
          <h2><Zap size={20} style={{display: 'inline', marginRight: '8px'}} />Встроенные триггеры</h2>
          <p>Автоматические события на основе показателей игры</p>
        </div>
      </div>

      <div className="trigger-list">
        {loading ? (
          <div className="empty-state">
            <p>Загрузка...</p>
          </div>
        ) : triggers.length === 0 ? (
          <div className="empty-state">
            <p>Нет триггеров</p>
          </div>
        ) : (
          triggers.map((trigger) => (
            <div 
              key={trigger.id} 
              className={`trigger-item ${trigger.enabled ? 'enabled' : 'disabled'}`}
            >
              <div className="trigger-info">
                <div className="trigger-header">
                  <h3>{trigger.name}</h3>
                  <span className="trigger-cooldown">
                    {(trigger.cooldown_ms / 1000).toFixed(0)}s cooldown
                  </span>
                </div>
                <p className="trigger-description">
                  <Info size={14} style={{display: 'inline', marginRight: '4px'}} />
                  {trigger.description}
                </p>
              </div>
              
              <button
                className={`btn-toggle ${trigger.enabled ? 'active' : ''}`}
                onClick={() => toggleTrigger(trigger.id, !trigger.enabled)}
                title={trigger.enabled ? 'Выключить' : 'Включить'}
              >
                <Power size={18} />
                {trigger.enabled ? 'ВКЛ' : 'ВЫКЛ'}
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  );
}


