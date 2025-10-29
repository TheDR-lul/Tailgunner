import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Ship, Plane, Square, Wifi, WifiOff, Radio } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import './VehicleModeCard.css';

interface VehicleModeInfo {
  vehicle_type: 'Ship' | 'Aircraft' | 'Tank' | 'Helicopter' | 'Unknown';
  mode: 'HudOnly' | 'FullTelemetry' | 'Disconnected';
  available_data: string[];
}

export function VehicleModeCard() {
  const { t } = useTranslation();
  const [modeInfo, setModeInfo] = useState<VehicleModeInfo | null>(null);

  useEffect(() => {
    const fetchModeInfo = async () => {
      try {
        const info = await invoke<VehicleModeInfo>('get_vehicle_mode');
        setModeInfo(info);
      } catch (error) {
        // Silently fail if not connected
      }
    };

    fetchModeInfo();
    const interval = setInterval(fetchModeInfo, 2000);
    return () => clearInterval(interval);
  }, []);

  if (!modeInfo || modeInfo.mode === 'Disconnected') {
    return (
      <div className="card vehicle-mode-card disconnected">
        <div className="mode-header">
          <WifiOff size={20} />
          <h3>{t('vehicle_mode.disconnected')}</h3>
        </div>
        <p className="mode-desc">{t('vehicle_mode.waiting')}</p>
      </div>
    );
  }

  const getVehicleIcon = () => {
    switch (modeInfo.vehicle_type) {
      case 'Ship':
        return <Ship size={24} />;
      case 'Aircraft':
        return <Plane size={24} />;
      case 'Tank':
        return <Square size={24} />;
      case 'Helicopter':
        return <Plane size={24} />;
      default:
        return <Radio size={24} />;
    }
  };

  const getModeClass = () => {
    return modeInfo.mode === 'HudOnly' ? 'hud-only' : 'full-telemetry';
  };

  const getModeIcon = () => {
    return modeInfo.mode === 'HudOnly' ? <Radio size={18} /> : <Wifi size={18} />;
  };

  return (
    <div className={`card vehicle-mode-card ${getModeClass()}`}>
      <div className="mode-header">
        <div className="vehicle-icon">
          {getVehicleIcon()}
        </div>
        <div className="mode-info">
          <h3>{t(`vehicle_mode.type_${modeInfo.vehicle_type.toLowerCase()}`)}</h3>
          <div className="mode-badge">
            {getModeIcon()}
            <span>
              {modeInfo.mode === 'HudOnly' 
                ? t('vehicle_mode.hud_only') 
                : t('vehicle_mode.full_telemetry')}
            </span>
          </div>
        </div>
      </div>

      <div className="mode-description">
        {modeInfo.mode === 'HudOnly' ? (
          <>
            <div className="mode-warning">
              <span>⚠️</span>
              <p>{t('vehicle_mode.hud_only_warning')}</p>
            </div>
            <div className="available-data">
              <strong>{t('vehicle_mode.available')}:</strong>
              <ul>
                {modeInfo.available_data.map((item, i) => (
                  <li key={i}>✓ {item}</li>
                ))}
              </ul>
            </div>
          </>
        ) : (
          <div className="mode-success">
            <span>✅</span>
            <p>{t('vehicle_mode.full_telemetry_desc')}</p>
          </div>
        )}
      </div>
    </div>
  );
}


