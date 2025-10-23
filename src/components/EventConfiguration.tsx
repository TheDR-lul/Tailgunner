import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';
import { Zap, Power, ChevronDown, ChevronUp, Layers } from 'lucide-react';
import type { Profile } from '../types';

interface EventTrigger {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  is_builtin: boolean;
  cooldown_ms: number;
  event: string;
}

export function EventConfiguration() {
  const { t } = useTranslation();
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [triggers, setTriggers] = useState<EventTrigger[]>([]);
  const [expandedProfile, setExpandedProfile] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [profileList, triggerList] = await Promise.all([
        invoke<Profile[]>('get_profiles'),
        invoke<EventTrigger[]>('get_triggers')
      ]);
      setProfiles(profileList);
      setTriggers(triggerList);
      setLoading(false);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('info', `Loaded ${profileList.length} profiles, ${triggerList.length} triggers`);
      }
    } catch (error) {
      console.error('Failed to load event configuration:', error);
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
          `${trigger?.name}: ${enabled ? 'enabled' : 'disabled'}`
        );
      }
    } catch (error) {
      console.error('Failed to toggle trigger:', error);
    }
  };

  const vehicleIcons: Record<string, string> = {
    Tank: '🛡️',
    Aircraft: '✈️',
    Helicopter: '🚁',
    Ship: '⚓',
    Unknown: '🎮',
  };

  const getModeLabel = (mode: string) => t(`profiles.game_mode_${mode.toLowerCase()}`);

  if (loading) {
    return (
      <div className="card">
        <div className="card-header">
          <h3 className="card-title">
            <Zap size={20} />
            {t('dashboard.title')}
          </h3>
        </div>
        <div className="empty-state">
          <p>{t('common.loading')}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="card-header">
        <div>
          <h2 style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <Zap size={20} />
            Game Events
          </h2>
          <p>Profiles and triggers configuration</p>
        </div>
      </div>

      <div className="card-content">
        {/* Profiles List */}
        <div style={{ marginBottom: '24px' }}>
          <h3 style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '12px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            <Layers size={16} style={{ display: 'inline', marginRight: '6px' }} />
            Active Profiles
          </h3>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {profiles.map((profile) => {
              const isExpanded = expandedProfile === profile.id;
              const patterns = Object.entries(profile.event_mappings);
              
              return (
                <div
                  key={profile.id}
                  className={`profile-item ${profile.enabled ? 'active' : ''}`}
                  onClick={() => setExpandedProfile(isExpanded ? null : profile.id)}
                  style={{ cursor: 'pointer' }}
                >
                  <div className="profile-header">
                    <div className="profile-name">
                      <span style={{ fontSize: '18px' }}>{vehicleIcons[profile.vehicle_type]}</span>
                      <span>{profile.name}</span>
                      {isExpanded ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                      <span className="profile-badge">
                        {getModeLabel(profile.game_mode)}
                      </span>
                      <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
                        {patterns.length} {t('profiles.patterns_count')}
                      </span>
                    </div>
                  </div>
                  
                  {isExpanded && (
                    <div style={{ marginTop: '12px', paddingTop: '12px', borderTop: '1px solid var(--border)' }}>
                      <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginBottom: '8px' }}>
                        Assigned events for this profile:
                      </div>
                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                        {patterns.map(([eventName]) => (
                          <div
                            key={eventName}
                            style={{
                              padding: '4px 8px',
                              background: 'var(--bg-tertiary)',
                              border: '1px solid var(--border)',
                              borderRadius: 'var(--radius-sm)',
                              fontSize: '11px',
                              color: 'var(--text-secondary)'
                            }}
                          >
                            {t(`events.${eventName}`, eventName)}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Built-in Triggers */}
        <div>
          <h3 style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '12px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            <Zap size={16} style={{ display: 'inline', marginRight: '6px' }} />
            Event Triggers
          </h3>

          {triggers.length === 0 ? (
            <div className="empty-state">
              <p>{t('triggers.no_triggers')}</p>
            </div>
          ) : (
            <div className="trigger-list">
              {triggers.map((trigger) => (
                <div 
                  key={trigger.id} 
                  className={`trigger-item ${trigger.enabled ? 'enabled' : 'disabled'}`}
                >
                  <div className="trigger-info">
                    <div className="trigger-header">
                      <h3>{t(`events.${trigger.event}`, trigger.name)}</h3>
                      <span className="trigger-cooldown">
                        {t('triggers.cooldown', { time: (trigger.cooldown_ms / 1000).toFixed(0) })}
                      </span>
                    </div>
                    <p className="trigger-description">
                      {trigger.description}
                    </p>
                  </div>
                  
                  <button
                    className={`btn-toggle ${trigger.enabled ? 'active' : ''}`}
                    onClick={() => toggleTrigger(trigger.id, !trigger.enabled)}
                    title={trigger.enabled ? t('common.disable') : t('common.enable')}
                  >
                    <Power size={18} />
                    {trigger.enabled ? t('common.on') : t('common.off')}
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

