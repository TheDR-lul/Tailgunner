import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Target, CheckCircle, XCircle, Circle } from 'lucide-react';
import './MissionInfo.css';

interface MissionObjective {
  primary: boolean;
  status: 'in_progress' | 'completed' | 'failed' | 'undefined';
  text: string;
}

interface MissionData {
  objectives?: MissionObjective[];
  status: 'running' | 'not_started' | 'completed' | 'failed';
}

export function MissionInfo() {
  const { t } = useTranslation();
  const [mission, setMission] = useState<MissionData | null>(null);
  const [isEnabled, setIsEnabled] = useState(false);

  useEffect(() => {
    if (!isEnabled) return;

    const interval = setInterval(async () => {
      try {
        const data = await invoke<MissionData>('get_mission_info');
        setMission(data);
      } catch (err) {
        // Silently ignore errors (game not running)
        setMission(null);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [isEnabled]);

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle size={14} className="status-icon completed" />;
      case 'failed':
        return <XCircle size={14} className="status-icon failed" />;
      case 'in_progress':
        return <Circle size={14} className="status-icon in-progress" />;
      default:
        return null;
    }
  };

  const primaryObjectives = mission?.objectives?.filter(o => o.primary) || [];
  const secondaryObjectives = mission?.objectives?.filter(o => !o.primary) || [];

  return (
    <div className="mission-info-container">
      <div className="mission-info-header">
        <div className="mission-info-title">
          <Target size={18} />
          <h3>{t('mission.title')}</h3>
        </div>
        <button 
          className={`btn btn-${isEnabled ? 'primary' : 'secondary'}`}
          onClick={() => setIsEnabled(!isEnabled)}
        >
          {isEnabled ? t('mission.enabled') : t('mission.disabled')}
        </button>
      </div>
      {isEnabled && mission && (
        <div className="mission-info-content">
          {primaryObjectives.length > 0 && (
            <div className="objectives-section">
              <div className="objectives-label">{t('mission.primary')}</div>
              {primaryObjectives.map((obj, idx) => (
                <div key={idx} className={`objective ${obj.status}`}>
                  {getStatusIcon(obj.status)}
                  <span>{obj.text}</span>
                </div>
              ))}
            </div>
          )}
          {secondaryObjectives.length > 0 && (
            <div className="objectives-section">
              <div className="objectives-label">{t('mission.secondary')}</div>
              {secondaryObjectives.map((obj, idx) => (
                <div key={idx} className={`objective ${obj.status}`}>
                  {getStatusIcon(obj.status)}
                  <span>{obj.text}</span>
                </div>
              ))}
            </div>
          )}
          {(!mission.objectives || mission.objectives.length === 0) && (
            <div className="no-objectives">{t('mission.no_objectives')}</div>
          )}
        </div>
      )}
      {isEnabled && !mission && (
        <div className="mission-info-empty">{t('mission.waiting')}</div>
      )}
    </div>
  );
}

