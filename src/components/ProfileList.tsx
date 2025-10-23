import { useState, useEffect } from 'react';
import { Layers, Gamepad2, ChevronDown, ChevronUp, Power } from 'lucide-react';
import { api } from '../api';
import type { Profile } from '../types';

export function ProfileList() {
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

  const modeLabels: Record<string, string> = {
    Arcade: 'Аркада',
    Realistic: 'Реалистика',
    Simulator: 'Симулятор',
    Any: 'Все режимы',
  };

  const eventLabels: Record<string, string> = {
    Hit: 'Попадание',
    CriticalHit: 'Критическое попадание',
    PenetrationHit: 'Пробитие',
    Overspeed: 'Превышение скорости',
    OverG: 'G-перегрузка',
    HighAOA: 'Высокий угол атаки',
    CriticalAOA: 'Критический угол атаки',
    Mach1: 'Mach 1.0',
    LowFuel: 'Мало топлива',
    CriticalFuel: 'Критически мало топлива',
    LowAmmo: 'Мало боезапаса',
    LowAltitude: 'Низкая высота',
    EngineDamaged: 'Повреждение двигателя',
    EngineDestroyed: 'Уничтожен двигатель',
    TrackBroken: 'Сломана гусеница',
    Shooting: 'Выстрел',
    CannonFiring: 'Стрельба из пушки',
  };

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="card-title">
          <Layers size={20} />
          Профили
        </h3>
        <p className="card-description">
          Автоматические пресеты для разных типов техники
        </p>
      </div>

      <div className="card-content">
        {profiles.length === 0 ? (
          <div className="empty-state">
            <div className="empty-state-icon">
              <Gamepad2 size={48} />
            </div>
            <p>Загрузка профилей...</p>
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
                        {profile.enabled ? 'ВКЛ' : 'ВЫКЛ'}
                      </button>
                    </div>
                  </div>
                  
                  <div className="profile-description">
                    {modeLabels[profile.game_mode]} • {patterns.length} паттернов
                  </div>
                  
                  {isExpanded && (
                    <div className="profile-patterns">
                      {patterns.map(([eventName, pattern]) => (
                        <div key={eventName} className="pattern-chip">
                          <span className="pattern-name">{eventLabels[eventName] || eventName}</span>
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

