import { useState, useEffect } from 'react';
import { Layers, Gamepad2 } from 'lucide-react';
import { api } from '../api';
import type { Profile } from '../types';

export function ProfileList() {
  const [profiles, setProfiles] = useState<Profile[]>([]);

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

  const vehicleIcons: Record<string, string> = {
    Tank: '🛡️',
    Aircraft: '✈️',
    Helicopter: '🚁',
    Ship: '⚓',
    Unknown: '🎮',
  };

  const modeLabels: Record<string, string> = {
    Arcade: 'AB',
    Realistic: 'RB',
    Simulator: 'SB',
    Any: '∀',
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
            <p>Профили загружаются...</p>
          </div>
        ) : (
          profiles.map((profile) => (
            <div
              key={profile.id}
              className={`profile-item ${profile.enabled ? 'active' : ''}`}
            >
              <div className="profile-header">
                <div className="profile-name">
                  {vehicleIcons[profile.vehicle_type]} {profile.name}
                </div>
                <div className="profile-badge">
                  {modeLabels[profile.game_mode]}
                </div>
              </div>
              <div className="profile-description">
                {Object.keys(profile.event_mappings).length} паттернов
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

