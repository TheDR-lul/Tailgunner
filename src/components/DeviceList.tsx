import { useState } from 'react';
import { Wifi, RefreshCw, Bluetooth } from 'lucide-react';
import { api } from '../api';
import type { DeviceInfo } from '../types';

export function DeviceList() {
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [initialized, setInitialized] = useState(false);

  async function loadDevices() {
    try {
      const deviceList = await api.getDevices();
      setDevices(deviceList);
    } catch (error) {
      console.error('Failed to load devices:', error);
    }
  }

  async function handleInit() {
    setLoading(true);
    try {
      await api.initDevices();
      setInitialized(true);
      await new Promise(resolve => setTimeout(resolve, 3000));
      await loadDevices();
    } catch (error) {
      console.error('Failed to init devices:', error);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="card-title">
          <Bluetooth size={20} />
          Устройства
        </h3>
        <p className="card-description">
          Подключенные вибро-устройства
        </p>
      </div>

      <div className="card-content">
        {!initialized ? (
          <div className="empty-state">
            <div className="empty-state-icon">
              <Wifi size={48} />
            </div>
            <p>Сначала инициализируйте Buttplug/Intiface</p>
            <button
              className="btn btn-primary"
              onClick={handleInit}
              disabled={loading}
              style={{ marginTop: '1rem' }}
            >
              <RefreshCw size={16} className={loading ? 'spin' : ''} />
              {loading ? 'Инициализация...' : 'Инициализировать'}
            </button>
          </div>
        ) : devices.length === 0 ? (
          <div className="empty-state">
            <div className="empty-state-icon">
              <Bluetooth size={48} />
            </div>
            <p>Устройства не найдены</p>
            <p style={{ fontSize: '0.75rem', marginTop: '0.5rem' }}>
              Убедитесь, что Intiface Desktop запущен
            </p>
            <button
              className="btn btn-secondary"
              onClick={loadDevices}
              style={{ marginTop: '1rem' }}
            >
              <RefreshCw size={16} />
              Обновить
            </button>
          </div>
        ) : (
          <>
            {devices.map((device) => (
              <div key={device.id} className="device-item">
                <div className="device-icon">
                  <Bluetooth size={20} />
                </div>
                <div className="device-info">
                  <h4>{device.name}</h4>
                  <p>{device.device_type}</p>
                </div>
              </div>
            ))}
            <button
              className="btn btn-secondary"
              onClick={loadDevices}
              style={{ marginTop: '1rem' }}
            >
              <RefreshCw size={16} />
              Обновить список
            </button>
          </>
        )}
      </div>
    </div>
  );
}

