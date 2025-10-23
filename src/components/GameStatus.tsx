import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Gamepad2, Gauge, Mountain, Droplet, Zap } from 'lucide-react';
import { api } from '../api';
import type { GameStatusInfo } from '../types';

export function GameStatus() {
  const { t } = useTranslation();
  const [status, setStatus] = useState<GameStatusInfo>({
    connected: false,
    vehicle_name: 'N/A',
    speed_kmh: 0,
    altitude_m: 0,
    g_load: 0,
    engine_rpm: 0,
    fuel_percent: 0,
  });

  useEffect(() => {
    const interval = setInterval(async () => {
      const gameStatus = await api.getGameStatus();
      setStatus(gameStatus);
      
      // Логируем только при подключении/отключении
      if (gameStatus.connected && !status.connected) {
        if ((window as any).debugLog) {
          (window as any).debugLog('success', `WT подключен: ${gameStatus.vehicle_name}`);
        }
      } else if (!gameStatus.connected && status.connected) {
        if ((window as any).debugLog) {
          (window as any).debugLog('warn', 'WT отключен');
        }
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [status.connected]);

  return (
    <div className="card game-status-card">
      <div className="card-header">
        <div>
          <h2>{t('game_status.title') || 'War Thunder'}</h2>
          <div className={`status-indicator ${status.connected ? 'connected' : 'disconnected'}`}>
            {status.connected ? '🟢 Подключено' : '🔴 Не подключено'}
          </div>
        </div>
      </div>

      {status.connected ? (
        <div className="game-stats">
          <div className="stat-item">
            <Gamepad2 size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">Техника</span>
              <span className="stat-value">{status.vehicle_name}</span>
            </div>
          </div>

          <div className="stat-item">
            <Gauge size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">Скорость</span>
              <span className="stat-value">{status.speed_kmh} км/ч</span>
            </div>
          </div>

          <div className="stat-item">
            <Mountain size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">Высота</span>
              <span className="stat-value">{status.altitude_m} м</span>
            </div>
          </div>

          <div className="stat-item">
            <Zap size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">Перегрузка</span>
              <span className={`stat-value ${Math.abs(status.g_load) > 8 ? 'text-danger' : ''}`}>
                {status.g_load.toFixed(1)} G
              </span>
            </div>
          </div>

          <div className="stat-item">
            <Droplet size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">Топливо</span>
              <span className={`stat-value ${status.fuel_percent < 20 ? 'text-warning' : ''}`}>
                {status.fuel_percent}%
              </span>
            </div>
          </div>
        </div>
      ) : (
        <div className="empty-state">
          <p style={{ margin: 0, fontSize: '14px', color: 'var(--text-secondary)' }}>
            Запустите War Thunder и зайдите в бой
          </p>
        </div>
      )}
    </div>
  );
}


