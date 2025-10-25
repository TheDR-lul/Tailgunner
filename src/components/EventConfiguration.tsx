import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';
import { Zap, Power, ChevronDown, ChevronUp, Layers } from 'lucide-react';
import type { Profile } from '../types';
import { VibrationCurveEditor } from './VibrationCurveEditor';
import { EditableNumberInput } from './EditableNumberInput';

interface CurvePoint {
  x: number;
  y: number;
}

interface EventTrigger {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  is_builtin: boolean;
  cooldown_ms: number;
  event: string;
  pattern?: {
    intensity?: number;
    duration_ms?: number;
  };
  curve_points?: CurvePoint[];
}

export function EventConfiguration() {
  const { t } = useTranslation();
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [triggers, setTriggers] = useState<EventTrigger[]>([]);
  const [expandedProfile, setExpandedProfile] = useState<string | null>(null);
  const [expandedEvent, setExpandedEvent] = useState<string | null>(null);
  const [expandedTrigger, setExpandedTrigger] = useState<string | null>(null);
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

  const updateTrigger = async (id: string, cooldown_ms: number | null, pattern: any) => {
    try {
      // Update backend
      await invoke('update_trigger', { id, cooldown_ms, pattern });
      
      // Update local state immediately without full reload
      setTriggers(prev => prev.map(t => {
        if (t.id !== id) return t;
        
        const updated = { ...t };
        
        if (cooldown_ms !== null) {
          updated.cooldown_ms = cooldown_ms;
        }
        
        if (pattern !== null) {
          updated.pattern = updated.pattern || {};
          updated.pattern.duration_ms = pattern.duration_ms || updated.pattern.duration_ms || 500;
          updated.pattern.intensity = pattern.intensity || updated.pattern.intensity || 1.0;
          updated.curve_points = pattern.curve || updated.curve_points;
        }
        
        return updated;
      }));
    } catch (error) {
      console.error('Failed to update trigger:', error);
      // Reload on error to ensure consistency
      loadData();
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
        {/* All Triggers Section */}
        <div style={{ marginBottom: '32px' }}>
          <h3 style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '12px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            <Zap size={16} style={{ display: 'inline', marginRight: '6px' }} />
            All Triggers ({triggers.length})
          </h3>
          
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '8px' }}>
            {triggers.map((trigger) => {
              const isExpanded = expandedTrigger === trigger.id;
              return (
                <div
                  key={trigger.id}
                  style={{
                    display: 'flex',
                    flexDirection: 'column',
                    padding: '10px 12px',
                    background: trigger.enabled ? 'rgba(99, 102, 241, 0.1)' : 'var(--bg-secondary)',
                    border: `1px solid ${trigger.enabled ? 'var(--primary)' : 'var(--border)'}`,
                    borderRadius: 'var(--radius-sm)',
                  }}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <div style={{ flex: 1, minWidth: 0 }}>
                      <div style={{ fontSize: '12px', fontWeight: 600, marginBottom: '4px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                        {trigger.is_builtin ? (
                          <span style={{ fontSize: '10px', padding: '2px 4px', background: 'rgba(99, 102, 241, 0.2)', borderRadius: '4px', color: 'var(--primary)' }}>
                            Built-in
                          </span>
                        ) : (
                          <span style={{ fontSize: '10px', padding: '2px 4px', background: 'rgba(16, 185, 129, 0.2)', borderRadius: '4px', color: '#10b981' }}>
                            Dynamic
                          </span>
                        )}
                        <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                          {trigger.name}
                        </span>
                      </div>
                      <div style={{ fontSize: '10px', color: 'var(--text-muted)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {trigger.description}
                      </div>
                      <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginTop: '2px' }}>
                        Event: {t(`game_events.${trigger.event}`, trigger.event)}
                      </div>
                      <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginTop: '2px' }}>
                        Cooldown: {(trigger.cooldown_ms / 1000).toFixed(1)}s | 
                        Duration: {((trigger.pattern?.duration_ms || 500) / 1000).toFixed(1)}s | 
                        Points: {trigger.curve_points?.length || 2}
                      </div>
                    </div>
                    
                    <div style={{ display: 'flex', gap: '4px', marginLeft: '8px', flexShrink: 0 }}>
                      <button
                        onClick={() => setExpandedTrigger(isExpanded ? null : trigger.id)}
                        style={{
                          padding: '4px 6px',
                          fontSize: '10px',
                          background: 'var(--bg-secondary)',
                          border: '1px solid var(--border)',
                          borderRadius: 'var(--radius-sm)',
                          cursor: 'pointer',
                          color: 'var(--text-secondary)'
                        }}
                        title={t('common.settings', 'Settings')}
                      >
                        {isExpanded ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
                      </button>
                      <button
                        className={`btn-toggle ${trigger.enabled ? 'active' : ''}`}
                        onClick={() => toggleTrigger(trigger.id, !trigger.enabled)}
                        title={trigger.enabled ? t('common.disable') : t('common.enable')}
                        style={{
                          padding: '4px 10px',
                          fontSize: '10px',
                          minWidth: '55px',
                        }}
                      >
                        <Power size={12} />
                        {trigger.enabled ? t('common.on') : t('common.off')}
                      </button>
                    </div>
                  </div>

                  {/* Expanded Settings */}
                  {isExpanded && (
                    <div style={{ 
                      marginTop: '12px', 
                      paddingTop: '12px', 
                      borderTop: '1px solid var(--border)',
                      display: 'flex',
                      flexDirection: 'column',
                      gap: '12px'
                    }}>
                      {/* Cooldown */}
                      <div>
                        <label style={{ fontSize: '10px', color: 'var(--text-muted)', marginBottom: '4px', display: 'block' }}>
                          {t('trigger_settings.cooldown', 'Cooldown (seconds)')}
                        </label>
                        <EditableNumberInput
                          value={trigger.cooldown_ms / 1000}
                          onChange={(newValue) => {
                            updateTrigger(trigger.id, newValue * 1000, null);
                          }}
                          min={0.1}
                          max={300}
                          step={0.1}
                          decimals={1}
                          suffix="s"
                          style={{
                            width: '100%',
                            padding: '4px 8px',
                            fontSize: '11px',
                            background: 'var(--bg-tertiary)',
                            border: '1px solid var(--border)',
                            borderRadius: 'var(--radius-sm)',
                            color: 'var(--text-primary)'
                          }}
                        />
                      </div>

                      {/* Vibration Curve Editor */}
                      <VibrationCurveEditor
                        duration={(trigger.pattern?.duration_ms || 500) / 1000}
                        curve={trigger.curve_points || [
                          { x: 0.0, y: 1.0 },
                          { x: 1.0, y: 0.0 }
                        ]}
                        onDurationChange={(newDuration) => {
                          updateTrigger(trigger.id, null, {
                            intensity: trigger.pattern?.intensity || 1.0,
                            duration_ms: Math.round(newDuration * 1000),
                            curve: trigger.curve_points
                          });
                        }}
                        onCurveChange={(newCurve) => {
                          updateTrigger(trigger.id, null, {
                            intensity: trigger.pattern?.intensity || 1.0,
                            duration_ms: trigger.pattern?.duration_ms || 500,
                            curve: newCurve
                          });
                        }}
                      />

                      <div style={{ fontSize: '9px', color: 'var(--text-muted)', fontStyle: 'italic' }}>
                        {t('trigger_settings.hint', 'These settings will be saved automatically')}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
        
        {/* Profiles Section */}
        <div style={{ marginBottom: '24px' }}>
          <h3 style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '12px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            <Layers size={16} style={{ display: 'inline', marginRight: '6px' }} />
            {t('events.active_profiles')}
          </h3>
          
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {profiles.map((profile) => {
              const isExpanded = expandedProfile === profile.id;
              // Only show events that have at least one trigger
              const patterns = profile.event_mappings 
                ? Object.entries(profile.event_mappings).filter(([eventName]) => 
                    getTriggersForEvent(eventName).length > 0
                  )
                : [];
              
              // Skip profiles with no events that have triggers
              if (patterns.length === 0) return null;
              
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
                      <span style={{ fontSize: '18px' }}>{profile.vehicle_type ? vehicleIcons[profile.vehicle_type] : '🎮'}</span>
                      <span>{t(`profiles.${profile.id}`)}</span>
                      {isExpanded ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                      <span className="profile-badge">
                        {getModeLabel(profile.game_mode || 'any')}
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
                                  background: 'var(--bg-secondary)'
                                }}
                              >
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                  <span style={{ fontSize: '12px', fontWeight: 600 }}>
                                    {t(`game_events.${eventName}`, eventName)}
                                  </span>
                                  <span style={{
                                    fontSize: '10px',
                                    padding: '2px 6px',
                                    background: 'var(--primary)',
                                    color: 'white',
                                    borderRadius: '10px'
                                  }}>
                                    {eventTriggers.length} {eventTriggers.length === 1 ? 'trigger' : 'triggers'}
                                  </span>
                                </div>
                                {isEventExpanded ? <ChevronUp size={14} /> : <ChevronDown size={14} />}
                              </div>
                              
                              {/* Triggers for this event */}
                              {isEventExpanded && (
                                <div style={{ padding: '8px', borderTop: '1px solid var(--border)' }}>
                                  {eventTriggers.map((trigger) => {
                                    const triggerExpanded = expandedTrigger === `${eventKey}_${trigger.id}`;
                                    return (
                                      <div
                                        key={trigger.id}
                                        style={{
                                          marginBottom: '8px',
                                          background: trigger.enabled ? 'rgba(99, 102, 241, 0.1)' : 'var(--bg-primary)',
                                          border: `1px solid ${trigger.enabled ? 'var(--primary)' : 'var(--border)'}`,
                                          borderRadius: 'var(--radius-sm)',
                                        }}
                                        onClick={(e) => e.stopPropagation()}
                                      >
                                        <div style={{
                                          display: 'flex',
                                          justifyContent: 'space-between',
                                          alignItems: 'center',
                                          padding: '8px 10px',
                                        }}>
                                          <div style={{ flex: 1, minWidth: 0 }}>
                                            <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '2px' }}>
                                              {trigger.name}
                                            </div>
                                            <div style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                                              {trigger.description}
                                            </div>
                                            <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginTop: '2px' }}>
                                              Cooldown: {(trigger.cooldown_ms / 1000).toFixed(1)}s | 
                                              Duration: {((trigger.pattern?.duration_ms || 500) / 1000).toFixed(1)}s | 
                                              Points: {trigger.curve_points?.length || 2}
                                            </div>
                                          </div>
                                          
                                          <div style={{ display: 'flex', gap: '4px', marginLeft: '8px', flexShrink: 0 }}>
                                            <button
                                              onClick={() => setExpandedTrigger(triggerExpanded ? null : `${eventKey}_${trigger.id}`)}
                                              style={{
                                                padding: '4px 6px',
                                                fontSize: '10px',
                                                background: 'var(--bg-secondary)',
                                                border: '1px solid var(--border)',
                                                borderRadius: 'var(--radius-sm)',
                                                cursor: 'pointer',
                                                color: 'var(--text-secondary)'
                                              }}
                                              title={t('common.settings', 'Settings')}
                                            >
                                              {triggerExpanded ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
                                            </button>
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
                                        </div>

                                        {/* Expanded Settings for Profile Trigger */}
                                        {triggerExpanded && (
                                          <div style={{ 
                                            padding: '12px', 
                                            borderTop: '1px solid var(--border)',
                                            display: 'flex',
                                            flexDirection: 'column',
                                            gap: '12px'
                                          }}>
                                            {/* Cooldown */}
                                            <div>
                                              <label style={{ fontSize: '10px', color: 'var(--text-muted)', marginBottom: '4px', display: 'block' }}>
                                                {t('trigger_settings.cooldown', 'Cooldown (seconds)')}
                                              </label>
                                              <EditableNumberInput
                                                value={trigger.cooldown_ms / 1000}
                                                onChange={(newValue) => {
                                                  updateTrigger(trigger.id, newValue * 1000, null);
                                                }}
                                                min={0.1}
                                                max={300}
                                                step={0.1}
                                                decimals={1}
                                                suffix="s"
                                                style={{
                                                  width: '100%',
                                                  padding: '4px 8px',
                                                  fontSize: '11px',
                                                  background: 'var(--bg-tertiary)',
                                                  border: '1px solid var(--border)',
                                                  borderRadius: 'var(--radius-sm)',
                                                  color: 'var(--text-primary)'
                                                }}
                                              />
                                            </div>

                                            {/* Vibration Curve Editor */}
                                            <VibrationCurveEditor
                                              duration={(trigger.pattern?.duration_ms || 500) / 1000}
                                              curve={trigger.curve_points || [
                                                { x: 0.0, y: 1.0 },
                                                { x: 1.0, y: 0.0 }
                                              ]}
                                              onDurationChange={(newDuration) => {
                                                updateTrigger(trigger.id, null, {
                                                  intensity: trigger.pattern?.intensity || 1.0,
                                                  duration_ms: Math.round(newDuration * 1000),
                                                  curve: trigger.curve_points
                                                });
                                              }}
                                              onCurveChange={(newCurve) => {
                                                updateTrigger(trigger.id, null, {
                                                  intensity: trigger.pattern?.intensity || 1.0,
                                                  duration_ms: trigger.pattern?.duration_ms || 500,
                                                  curve: newCurve
                                                });
                                              }}
                                            />

                                            <div style={{ fontSize: '9px', color: 'var(--text-muted)', fontStyle: 'italic' }}>
                                              {t('trigger_settings.hint', 'These settings will be saved automatically')}
                                            </div>
                                          </div>
                                        )}
                                      </div>
                                    );
                                  })}
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
