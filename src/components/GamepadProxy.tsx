import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Gamepad2, Activity, Settings as SettingsIcon } from 'lucide-react';
import { api } from '../api';
import { usePolling } from '../hooks/usePolling';
import type { GamepadProxyConfig, RumbleState } from '../types';
import './GamepadProxy.css';

export function GamepadProxy() {
  const { t } = useTranslation();
  const [config, setConfig] = useState<GamepadProxyConfig | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  
  // Poll rumble state when enabled
  const { data: rumbleState } = usePolling<RumbleState>(
    () => api.gamepadGetRumbleState(),
    100,
    { enabled: config?.enabled || false }
  );

  const { data: intensity } = usePolling<number>(
    () => api.gamepadGetIntensity(),
    100,
    { enabled: config?.enabled || false }
  );

  // Load config on mount
  useEffect(() => {
    const loadConfig = async () => {
      try {
        const cfg = await api.gamepadGetConfig();
        setConfig(cfg);
      } catch (error) {
        console.error('Failed to load gamepad config:', error);
      }
    };
    
    loadConfig();
  }, []);

  const handleToggle = async () => {
    if (!config) return;
    
    try {
      const newEnabled = !config.enabled;
      await api.gamepadSetEnabled(newEnabled);
      setConfig({ ...config, enabled: newEnabled });
    } catch (error) {
      console.error('Failed to toggle gamepad proxy:', error);
    }
  };

  const handleConfigChange = async (updates: Partial<GamepadProxyConfig>) => {
    if (!config) return;
    
    const newConfig = { ...config, ...updates };
    setConfig(newConfig);
    
    try {
      await api.gamepadSetConfig(newConfig);
    } catch (error) {
      console.error('Failed to update gamepad config:', error);
    }
  };

  const handleTest = async (left: number, right: number) => {
    try {
      await api.gamepadSetRumble(left, right);
      setTimeout(() => api.gamepadSetRumble(0, 0), 500);
    } catch (error) {
      console.error('Failed to test rumble:', error);
    }
  };

  if (!config) return null;

  return (
    <div className="card gamepad-proxy-card">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <Gamepad2 size={20} />
          <h2>Gamepad Rumble Proxy</h2>
        </div>
        <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
          <button
            className="btn btn-secondary btn-sm"
            onClick={() => setShowSettings(!showSettings)}
            title="Settings"
          >
            <SettingsIcon size={16} />
          </button>
          <button
            className={`btn btn-${config.enabled ? 'primary' : 'secondary'}`}
            onClick={handleToggle}
          >
            {config.enabled ? 'Enabled' : 'Disabled'}
          </button>
        </div>
      </div>

      <div className="card-body">
        {/* Description */}
        <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Translates gamepad vibration from War Thunder to haptic devices
        </p>

        {/* Status */}
        {config.enabled && rumbleState && (
          <div className="rumble-status">
            <div className="rumble-indicator">
              <div className="rumble-label">
                <Activity size={14} />
                <span>Low Frequency</span>
              </div>
              <div className="rumble-bar-wrapper">
                <div 
                  className="rumble-bar left-motor"
                  style={{ width: `${rumbleState.left_motor * 100}%` }}
                />
              </div>
              <span className="rumble-value">{(rumbleState.left_motor * 100).toFixed(0)}%</span>
            </div>

            <div className="rumble-indicator">
              <div className="rumble-label">
                <Activity size={14} />
                <span>High Frequency</span>
              </div>
              <div className="rumble-bar-wrapper">
                <div 
                  className="rumble-bar right-motor"
                  style={{ width: `${rumbleState.right_motor * 100}%` }}
                />
              </div>
              <span className="rumble-value">{(rumbleState.right_motor * 100).toFixed(0)}%</span>
            </div>

            <div className="combined-intensity">
              <strong>Combined Intensity:</strong>
              <span>{((intensity || 0) * 100).toFixed(0)}%</span>
            </div>
          </div>
        )}

        {/* Settings Panel */}
        {showSettings && (
          <div className="gamepad-settings">
            <div className="setting-item">
              <label>
                <input
                  type="checkbox"
                  checked={config.proxy_to_devices}
                  onChange={(e) => handleConfigChange({ proxy_to_devices: e.target.checked })}
                />
                <span>Proxy to haptic devices</span>
              </label>
            </div>

            <div className="setting-item">
              <label>
                Sensitivity: {config.sensitivity.toFixed(2)}x
              </label>
              <input
                type="range"
                min="0.1"
                max="2.0"
                step="0.1"
                value={config.sensitivity}
                onChange={(e) => handleConfigChange({ sensitivity: parseFloat(e.target.value) })}
              />
            </div>

            <div className="setting-item">
              <label>
                Deadzone: {(config.deadzone * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                min="0"
                max="0.3"
                step="0.01"
                value={config.deadzone}
                onChange={(e) => handleConfigChange({ deadzone: parseFloat(e.target.value) })}
              />
            </div>

            <div className="setting-item">
              <label>
                Low Freq Weight: {(config.left_motor_weight * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={config.left_motor_weight}
                onChange={(e) => handleConfigChange({ left_motor_weight: parseFloat(e.target.value) })}
              />
            </div>

            <div className="setting-item">
              <label>
                High Freq Weight: {(config.right_motor_weight * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={config.right_motor_weight}
                onChange={(e) => handleConfigChange({ right_motor_weight: parseFloat(e.target.value) })}
              />
            </div>

            {/* Test Buttons */}
            <div className="test-buttons">
              <button
                className="btn btn-secondary btn-sm"
                onClick={() => handleTest(1.0, 0)}
              >
                Test Low Freq
              </button>
              <button
                className="btn btn-secondary btn-sm"
                onClick={() => handleTest(0, 1.0)}
              >
                Test High Freq
              </button>
              <button
                className="btn btn-secondary btn-sm"
                onClick={() => handleTest(1.0, 1.0)}
              >
                Test Both
              </button>
            </div>
          </div>
        )}

        {/* Info Badge */}
        <div className="gamepad-info-badge">
          <span>ℹ️</span>
          <div>
            <strong>How it works:</strong>
            <small>
              Monitors gamepad rumble commands from the game and translates them to your haptic devices in real-time.
            </small>
          </div>
        </div>
      </div>
    </div>
  );
}

