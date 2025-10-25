import { useState, useEffect, useRef } from 'react';
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

  // Use ref to track previous values without causing re-renders
  const prevStatusRef = useRef<GameStatusInfo>(status);

  useEffect(() => {
    // Get update interval from localStorage (default: 200ms = 5 times per second)
    const updateInterval = parseInt(localStorage.getItem('gameStatusUpdateInterval') || '200');
    
    const interval = setInterval(async () => {
      try {
        const gameStatus = await api.getGameStatus();
        const prevStatus = prevStatusRef.current;
        
        // Debug: Log raw data from backend
        if (gameStatus.connected && gameStatus.vehicle_name !== prevStatus.vehicle_name) {
          console.log('[GameStatus] 🔍 Raw vehicle_name from backend:', gameStatus.vehicle_name);
        }
        
        // Update state
        setStatus(gameStatus);
        
        // Log vehicle changes (skip unknown/N/A)
        if (gameStatus.connected && 
            gameStatus.vehicle_name !== prevStatus.vehicle_name && 
            gameStatus.vehicle_name !== 'N/A' &&
            gameStatus.vehicle_name !== 'unknown') {
          if ((window as any).debugLog) {
            (window as any).debugLog('info', `🚗 Vehicle detected: ${gameStatus.vehicle_name}`);
          }
        }
        
        // Log ONLY on connection state change (not every tick!)
        if (gameStatus.connected !== prevStatus.connected) {
          if (gameStatus.connected) {
            if ((window as any).debugLog) {
              (window as any).debugLog('success', `✅ WT connected${gameStatus.vehicle_name !== 'unknown' ? ': ' + gameStatus.vehicle_name : ''}`);
            }
          } else {
            if ((window as any).debugLog) {
              (window as any).debugLog('warn', '⚠️ WT disconnected');
            }
          }
        }
        
        // Update ref for next comparison
        prevStatusRef.current = gameStatus;
      } catch (error) {
        console.error('[GameStatus] Failed to get game status:', error);
      }
    }, updateInterval);

    return () => clearInterval(interval);
  }, []); // Empty deps - runs once, interval handles updates

  return (
    <div className="card game-status-card">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <div style={{ 
              fontSize: '20px', 
              lineHeight: 1,
              filter: status.connected 
                ? 'drop-shadow(0 2px 4px rgba(34, 197, 94, 0.4))' 
                : 'drop-shadow(0 2px 4px rgba(239, 68, 68, 0.4))'
            }}>
              {status.connected ? '🎮' : '🎮'}
            </div>
            <h2 style={{ margin: 0, fontSize: '15px', fontWeight: 600 }}>{t('game_status.title') || 'War Thunder'}</h2>
          </div>
          <span style={{ 
            fontSize: '9px', 
            padding: '3px 7px', 
            background: status.connected ? 'rgba(34, 197, 94, 0.12)' : 'rgba(239, 68, 68, 0.12)', 
            border: status.connected ? '1px solid rgba(34, 197, 94, 0.25)' : '1px solid rgba(239, 68, 68, 0.25)',
            borderRadius: '4px',
            color: status.connected ? '#4ade80' : '#f87171',
            fontWeight: 600,
            letterSpacing: '0.5px',
            lineHeight: 1,
            display: 'flex',
            alignItems: 'center',
            gap: '4px'
          }}>
            <span>{status.connected ? '●' : '●'}</span>
            {status.connected ? t('game_status.connected') || 'CONNECTED' : t('game_status.disconnected') || 'OFFLINE'}
          </span>
        </div>
      </div>

      {status.connected ? (
        <div className="game-stats">
          <div className="stat-item">
            <Gamepad2 size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">{t('game_status.vehicle')}</span>
              <span className="stat-value" style={{
                color: (status.vehicle_name === 'unknown' || status.vehicle_name === 'N/A') 
                  ? 'var(--text-muted)' 
                  : 'var(--text)',
                fontStyle: (status.vehicle_name === 'unknown' || status.vehicle_name === 'N/A')
                  ? 'italic'
                  : 'normal'
              }}>
                {(status.vehicle_name === 'unknown' || status.vehicle_name === 'N/A')
                  ? '—'
                  : status.vehicle_name}
              </span>
            </div>
          </div>

          <div className="stat-item">
            <Gauge size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">{t('game_status.speed')}</span>
              <span className="stat-value">{status.speed_kmh} km/h</span>
            </div>
          </div>

          <div className="stat-item">
            <Mountain size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">{t('game_status.altitude')}</span>
              <span className="stat-value">{status.altitude_m} m</span>
            </div>
          </div>

          <div className="stat-item">
            <Zap size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">{t('game_status.g_load')}</span>
              <span className={`stat-value ${Math.abs(status.g_load) > 8 ? 'text-danger' : ''}`}>
                {status.g_load.toFixed(1)} G
              </span>
            </div>
          </div>

          <div className="stat-item">
            <Droplet size={18} className="stat-icon" />
            <div className="stat-content">
              <span className="stat-label">{t('game_status.fuel')}</span>
              <span className={`stat-value ${status.fuel_percent < 20 ? 'text-warning' : ''}`}>
                {status.fuel_percent}%
              </span>
            </div>
          </div>
        </div>
      ) : (
        <div className="empty-state">
          <p style={{ margin: 0, fontSize: '14px', color: 'var(--text-secondary)' }}>
            Launch War Thunder and enter battle
          </p>
        </div>
      )}
    </div>
  );
}


