import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Bluetooth, RefreshCw, Zap } from 'lucide-react';
import { api } from '../api';
import type { DeviceInfo } from '../types';

export function DeviceList() {
  const { t } = useTranslation();
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const handleInit = async () => {
    setIsLoading(true);
    try {
      await api.initDevices();
      setIsInitialized(true);
      await handleRefresh();
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', '✅ Intiface connected');
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('warn', '⚠️ Failed to connect to Intiface');
        (window as any).debugLog('info', '💡 Download: https://intiface.com/central/');
      }
    } finally {
      setIsLoading(false);
    }
  };

  const handleRefresh = async () => {
    try {
      const deviceList = await api.getDevices();
      setDevices(deviceList);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('info', `📱 Found ${deviceList.length} device(s)`);
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', '❌ Failed to refresh devices');
      }
    }
  };

  return (
    <div className="card device-card">
      <div className="card-header">
        <h2>📱 {t('devices.title')}</h2>
        <p>{t('devices.description')}</p>
      </div>

      <div className="card-body">
        {!isInitialized ? (
          <div className="empty-state-compact">
            <Bluetooth size={48} className="empty-icon" />
            <p>{t('devices.empty_init')}</p>
            <button 
              className="btn btn-primary" 
              onClick={handleInit}
              disabled={isLoading}
            >
              {isLoading ? (
                <>
                  <RefreshCw size={16} className="spin" />
                  {t('devices.initializing')}
                </>
              ) : (
                <>
                  <Zap size={16} />
                  {t('devices.btn_init')}
                </>
              )}
            </button>
            <small className="hint">{t('devices.empty_hint')}</small>
          </div>
        ) : (
          <>
            <button className="btn btn-secondary btn-sm" onClick={handleRefresh}>
              <RefreshCw size={16} />
              {t('devices.btn_refresh')}
            </button>
            
            {devices.length === 0 ? (
              <div className="empty-state-compact">
                <p>{t('devices.empty_devices')}</p>
                <small className="hint">{t('devices.empty_hint')}</small>
              </div>
            ) : (
              <div className="device-list">
                {devices.map((device) => (
                  <div key={device.id} className="device-item">
                    <div className="device-icon">🎮</div>
                    <div className="device-info">
                      <strong>{device.name}</strong>
                      <span className="device-type">{device.device_type}</span>
                    </div>
                    <div className="device-status">●</div>
                  </div>
                ))}
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
