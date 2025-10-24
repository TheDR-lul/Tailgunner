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
  const [expandedEvent, setExpandedEvent] = useState<string | null>(null);
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

  // Get triggers for a specific event
  const getTriggersForEvent = (eventName: string): EventTrigger[] => {
    return triggers.filter(trigger => trigger.event === eventName);
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
            {t('events.title')}
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
            {t('events.title')}
          </h2>
          <p>{t('events.description')}</p>
        </div>
      </div>

      <div className="card-content">
        <div style={{ marginBottom: '24px' }}>
          <h3 style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '12px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            <Layers size={16} style={{ display: 'inline', marginRight: '6px' }} />
            {t('events.active_profiles')}
          </h3>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {profiles.map((profile) => {
              const isExpanded = expandedProfile === profile.id;
              const patterns = Object.entries(profile.event_mappings);
              
              return (
                <div
                  key={profile.id}
                  className={`profile-item ${profile.enabled ? 'active' : ''}`}
                  style={{ cursor: 'pointer' }}
                >
                  <div 
                    className="profile-header"
                    onClick={() => setExpandedProfile(isExpanded ? null : profile.id)}
                  >
                    <div className="profile-name">
                      <span style={{ fontSize: '18px' }}>{vehicleIcons[profile.vehicle_type]}</span>
                      <span>{t(`profiles.${profile.id}`)}</span>
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
                      <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginBottom: '12px', fontWeight: 600 }}>
                        {t('events.assigned_events')}:
                      </div>
                      
                      {/* Event cards with triggers */}
                      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        {patterns.map(([eventName]) => {
                          const eventTriggers = getTriggersForEvent(eventName);
                          const isEventExpanded = expandedEvent === `${profile.id}_${eventName}`;
                          const eventKey = `${profile.id}_${eventName}`;
                          
                          return (
                            <div
                              key={eventName}
                              style={{
                                background: 'var(--bg-tertiary)',
                                border: '1px solid var(--border)',
                                borderRadius: 'var(--radius-sm)',
                                overflow: 'hidden'
                              }}
                            >
                              <div
                                onClick={(e) => {
                                  e.stopPropagation();
                                  setExpandedEvent(isEventExpanded ? null : eventKey);
                                }}
                                style={{
                                  padding: '8px 12px',
                                  cursor: 'pointer',
                                  display: 'flex',
                                  justifyContent: 'space-between',
                                  alignItems: 'center',
                                  background: eventTriggers.length > 0 ? 'var(--bg-secondary)' : 'transparent'
                                }}
                              >
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                  <span style={{ fontSize: '12px', fontWeight: 600 }}>
                                    {t(`game_events.${eventName}`, eventName)}
                                  </span>
                                  {eventTriggers.length > 0 && (
                                    <span style={{
                                      fontSize: '10px',
                                      padding: '2px 6px',
                                      background: 'var(--primary)',
                                      color: 'white',
                                      borderRadius: '10px'
                                    }}>
                                      {eventTriggers.length} {eventTriggers.length === 1 ? 'trigger' : 'triggers'}
                                    </span>
                                  )}
                                </div>
                                {eventTriggers.length > 0 && (
                                  isEventExpanded ? <ChevronUp size={14} /> : <ChevronDown size={14} />
                                )}
                              </div>
                              
                              {/* Triggers for this event */}
                              {isEventExpanded && eventTriggers.length > 0 && (
                                <div style={{ padding: '8px', borderTop: '1px solid var(--border)' }}>
                                  {eventTriggers.map((trigger) => (
                                    <div
                                      key={trigger.id}
                                      style={{
                                        display: 'flex',
                                        justifyContent: 'space-between',
                                        alignItems: 'center',
                                        padding: '6px 8px',
                                        marginBottom: '4px',
                                        background: trigger.enabled ? 'rgba(99, 102, 241, 0.1)' : 'var(--bg-primary)',
                                        border: `1px solid ${trigger.enabled ? 'var(--primary)' : 'var(--border)'}`,
                                        borderRadius: 'var(--radius-sm)',
                                      }}
                                      onClick={(e) => e.stopPropagation()}
                                    >
                                      <div style={{ flex: 1 }}>
                                        <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '2px' }}>
                                          {trigger.name}
                                        </div>
                                        <div style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                                          {trigger.description}
                                        </div>
                                        <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginTop: '2px' }}>
                                          {t('triggers.cooldown', { time: (trigger.cooldown_ms / 1000).toFixed(0) })}
                                        </div>
                                      </div>
                                      
                                      <button
                                        className={`btn-toggle ${trigger.enabled ? 'active' : ''}`}
                                        onClick={() => toggleTrigger(trigger.id, !trigger.enabled)}
                                        title={trigger.enabled ? t('common.disable') : t('common.enable')}
                                        style={{
                                          padding: '4px 8px',
                                          fontSize: '10px',
                                          minWidth: '50px'
                                        }}
                                      >
                                        <Power size={12} />
                                        {trigger.enabled ? t('common.on') : t('common.off')}
                                      </button>
                                    </div>
                                  ))}
                                </div>
                              )}
                            </div>
                          );
                        })}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
