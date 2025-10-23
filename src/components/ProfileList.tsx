import { useState, useEffect } from 'react';
import { Layers, Gamepad2, ChevronDown, ChevronUp, Power } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { api } from '../api';
import type { Profile } from '../types';

export function ProfileList() {
  const { t } = useTranslation();
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [expandedProfiles, setExpandedProfiles] = useState<Set<string>>(new Set());

  useEffect(() => {
    loadProfiles();
  }, []);

  async function loadProfiles() {
    try {
      const profileList = await api.getProfiles();
      setProfiles(profileList);
    } catch (error) {
      console.error('Failed to load profiles:', error);
    }
  }

  const toggleProfile = (profileId: string) => {
    setExpandedProfiles(prev => {
      const newSet = new Set(prev);
      if (newSet.has(profileId)) {
        newSet.delete(profileId);
      } else {
        newSet.add(profileId);
      }
      return newSet;
    });
  };

  const vehicleIcons: Record<string, string> = {
    Tank: '🛡️',
    Aircraft: '✈️',
    Helicopter: '🚁',
    Ship: '⚓',
    Unknown: '🎮',
  };

  const getModeLabel = (mode: string) => t(`profiles.game_mode_${mode.toLowerCase()}`);
  const getEventLabel = (event: string) => t(`events.${event}`, event);

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="card-title">
          <Layers size={20} />
          {t('profiles.title')}
        </h3>
        <p className="card-description">
          {t('profiles.description')}
        </p>
      </div>

      <div className="card-content">
        {profiles.length === 0 ? (
          <div className="empty-state">
            <div className="empty-state-icon">
              <Gamepad2 size={48} />
            </div>
            <p>{t('profiles.loading')}</p>
          </div>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {profiles.map((profile) => {
              const isExpanded = expandedProfiles.has(profile.id);
              const patterns = Object.entries(profile.event_mappings);
              
              return (
                <div
                  key={profile.id}
                  className={`profile-item ${profile.enabled ? 'active' : ''}`}
                >
                  <div 
                    className="profile-header"
                    onClick={() => toggleProfile(profile.id)}
                    style={{ cursor: 'pointer' }}
                  >
                    <div className="profile-name">
                      <span style={{ fontSize: '18px' }}>{vehicleIcons[profile.vehicle_type]}</span>
                      <span>{profile.name}</span>
                      {isExpanded ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                      <button
                        className={`btn-toggle ${profile.enabled ? 'active' : ''}`}
                        onClick={(e) => {
                          e.stopPropagation();
                          // TODO: Toggle profile
                        }}
                        style={{ fontSize: '11px', padding: '4px 8px' }}
                      >
                        <Power size={12} />
                        {profile.enabled ? t('common.on') : t('common.off')}
                      </button>
                    </div>
                  </div>
                  
                  <div className="profile-description">
                    {getModeLabel(profile.game_mode)} • {patterns.length} {t('profiles.patterns_count')}
                  </div>
                  
                  {isExpanded && (
                    <div className="profile-patterns">
                      {patterns.map(([eventName, pattern]) => (
                        <div key={eventName} className="pattern-chip">
                          <span className="pattern-name">{getEventLabel(eventName)}</span>
                          <span className="pattern-duration">
                            {((pattern.attack.duration_ms + pattern.hold.duration_ms + pattern.decay.duration_ms) / 1000).toFixed(1)}s
                          </span>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}

