import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Play, Pause, Zap, Target, MessageSquare, Flame, Skull } from 'lucide-react';
import { api } from '../api';
import './APIEmulator.css';

interface EmulatorState {
  enabled: boolean;
  vehicle_type: 'Tank' | 'Aircraft' | 'Ship';
  speed: number;
  altitude: number;
  heading: number;
  position: [number, number];
  ammo: number;
  hp: number;
  engine_running: boolean;
  in_battle: boolean;
}

export function APIEmulator() {
  const { t } = useTranslation();
  const [state, setState] = useState<EmulatorState>({
    enabled: false,
    vehicle_type: 'Tank',
    speed: 0,
    altitude: 100,
    heading: 0,
    position: [0.5, 0.5],
    ammo: 50,
    hp: 100,
    engine_running: true,
    in_battle: false,
  });
  const [expanded, setExpanded] = useState(false);
  const [chatMessage, setChatMessage] = useState('');
  const [chatMode, setChatMode] = useState('Team');

  useEffect(() => {
    loadState();
    const interval = setInterval(loadState, 1000);
    return () => clearInterval(interval);
  }, []);

  const loadState = async () => {
    try {
      const newState = await api.emulatorGetState();
      setState(newState);
    } catch (err) {
      console.error('[APIEmulator] Failed to load state:', err);
    }
  };

  const toggleEnabled = async () => {
    try {
      await api.emulatorSetEnabled(!state.enabled);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to toggle:', err);
    }
  };

  const setVehicleType = async (vehicleType: string) => {
    try {
      await api.emulatorSetVehicleType(vehicleType);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set vehicle type:', err);
    }
  };

  const setSpeed = async (speed: number) => {
    try {
      await api.emulatorSetSpeed(speed);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set speed:', err);
    }
  };

  const setAltitude = async (altitude: number) => {
    try {
      await api.emulatorSetAltitude(altitude);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set altitude:', err);
    }
  };

  const setHeading = async (heading: number) => {
    try {
      await api.emulatorSetHeading(heading);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set heading:', err);
    }
  };

  const setAmmo = async (ammo: number) => {
    try {
      await api.emulatorSetAmmo(ammo);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set ammo:', err);
    }
  };

  const setHP = async (hp: number) => {
    try {
      await api.emulatorSetHp(hp);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to set HP:', err);
    }
  };

  const toggleInBattle = async () => {
    try {
      await api.emulatorSetInBattle(!state.in_battle);
      await loadState();
    } catch (err) {
      console.error('[APIEmulator] Failed to toggle battle:', err);
    }
  };

  const triggerEvent = async (eventType: string) => {
    try {
      await api.emulatorTriggerEvent(eventType);
      console.log('[APIEmulator] Triggered event:', eventType);
    } catch (err) {
      console.error('[APIEmulator] Failed to trigger event:', err);
    }
  };

  const sendChat = async () => {
    if (!chatMessage.trim()) return;
    
    try {
      await api.emulatorSendChat(chatMessage, chatMode);
      console.log('[APIEmulator] Chat sent:', chatMessage);
      setChatMessage('');
    } catch (err) {
      console.error('[APIEmulator] Failed to send chat:', err);
    }
  };

  return (
    <div className="api-emulator-container">
      <div className="api-emulator-header" onClick={() => setExpanded(!expanded)}>
        <div className="api-emulator-title">
          <Zap size={18} />
          <h3>🧪 API Test Mode</h3>
        </div>
        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
          <span style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
            {state.enabled ? '● ACTIVE' : '○ INACTIVE'}
          </span>
          <button 
            className={`btn btn-${state.enabled ? 'primary' : 'secondary'}`}
            onClick={(e) => {
              e.stopPropagation();
              toggleEnabled();
            }}
            style={{ fontSize: '11px', padding: '4px 12px' }}
          >
            {state.enabled ? <Pause size={12} /> : <Play size={12} />}
            {state.enabled ? 'Stop' : 'Start'}
          </button>
        </div>
      </div>

      {expanded && (
        <div className="api-emulator-content">
          {/* Vehicle Type Selection */}
          <div className="emulator-section">
            <h4>🚗 Vehicle Type</h4>
            <div style={{ display: 'flex', gap: '8px' }}>
              {['Tank', 'Aircraft', 'Ship'].map((type) => (
                <button
                  key={type}
                  className={`btn btn-${state.vehicle_type === type ? 'primary' : 'secondary'}`}
                  onClick={() => setVehicleType(type)}
                  disabled={!state.enabled}
                  style={{ fontSize: '11px', flex: 1 }}
                >
                  {type}
                </button>
              ))}
            </div>
          </div>

          {/* Battle State */}
          <div className="emulator-section">
            <h4>⚔️ Battle State</h4>
            <button
              className={`btn btn-${state.in_battle ? 'primary' : 'secondary'}`}
              onClick={toggleInBattle}
              disabled={!state.enabled}
              style={{ fontSize: '11px', width: '100%' }}
            >
              {state.in_battle ? '✓ In Battle' : '✗ Not in Battle'}
            </button>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
            {/* Vehicle Parameters */}
            <div className="emulator-section">
              <h4>📊 Parameters</h4>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <div>
                  <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                    Speed: {state.speed.toFixed(0)} km/h
                  </label>
                  <input
                    type="range"
                    min="0"
                    max={state.vehicle_type === 'Aircraft' ? '800' : '60'}
                    value={state.speed}
                    onChange={(e) => setSpeed(parseFloat(e.target.value))}
                    disabled={!state.enabled}
                    style={{ width: '100%' }}
                  />
                </div>

                {state.vehicle_type === 'Aircraft' && (
                  <div>
                    <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                      Altitude: {state.altitude.toFixed(0)} m
                    </label>
                    <input
                      type="range"
                      min="0"
                      max="10000"
                      value={state.altitude}
                      onChange={(e) => setAltitude(parseFloat(e.target.value))}
                      disabled={!state.enabled}
                      style={{ width: '100%' }}
                    />
                  </div>
                )}

                <div>
                  <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                    Heading: {state.heading.toFixed(0)}°
                  </label>
                  <input
                    type="range"
                    min="0"
                    max="360"
                    value={state.heading}
                    onChange={(e) => setHeading(parseFloat(e.target.value))}
                    disabled={!state.enabled}
                    style={{ width: '100%' }}
                  />
                </div>

                <div>
                  <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                    Ammo: {state.ammo}
                  </label>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={state.ammo}
                    onChange={(e) => setAmmo(parseInt(e.target.value))}
                    disabled={!state.enabled}
                    style={{ width: '100%' }}
                  />
                </div>

                <div>
                  <label style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                    HP: {state.hp.toFixed(0)}%
                  </label>
                  <input
                    type="range"
                    min="0"
                    max="100"
                    value={state.hp}
                    onChange={(e) => setHP(parseFloat(e.target.value))}
                    disabled={!state.enabled}
                    style={{ width: '100%' }}
                  />
                </div>
              </div>
            </div>

            {/* Event Triggers */}
            <div className="emulator-section">
              <h4>⚡ Trigger Events</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('Hit')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <Target size={12} />
                  Hit
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('CriticalHit')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <Flame size={12} />
                  Crit Hit
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('TargetHit')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <Target size={12} />
                  Target Hit
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('TargetDestroyed')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <Skull size={12} />
                  Kill
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('TargetSetOnFire')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <Flame size={12} />
                  Fire
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('EngineOverheat')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  🔥 Overheat
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('ChatMessage')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  <MessageSquare size={12} />
                  Chat
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => triggerEvent('Achievement')}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', padding: '6px 8px' }}
                >
                  🏆 Award
                </button>
              </div>
            </div>
          </div>

          {/* Chat Emulator */}
          <div className="emulator-section" style={{ gridColumn: '1 / -1' }}>
            <h4>💬 Chat Emulator</h4>
            <div style={{ display: 'flex', gap: '8px', marginBottom: '8px' }}>
              {['Team', 'All', 'Squad'].map((mode) => (
                <button
                  key={mode}
                  className={`btn btn-${chatMode === mode ? 'primary' : 'secondary'}`}
                  onClick={() => setChatMode(mode)}
                  disabled={!state.enabled || !state.in_battle}
                  style={{ fontSize: '10px', flex: 1 }}
                >
                  {mode}
                </button>
              ))}
            </div>
            <div style={{ display: 'flex', gap: '8px' }}>
              <input
                type="text"
                value={chatMessage}
                onChange={(e) => setChatMessage(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && sendChat()}
                placeholder="Type a test message..."
                disabled={!state.enabled || !state.in_battle}
                style={{
                  flex: 1,
                  padding: '8px 12px',
                  background: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid var(--border)',
                  borderRadius: '4px',
                  color: 'var(--text-primary)',
                  fontSize: '12px',
                }}
              />
              <button
                className="btn btn-primary"
                onClick={sendChat}
                disabled={!state.enabled || !state.in_battle || !chatMessage.trim()}
                style={{ fontSize: '11px', padding: '8px 16px' }}
              >
                Send
              </button>
            </div>
            <div style={{ fontSize: '10px', color: 'var(--text-muted)', marginTop: '4px', fontStyle: 'italic' }}>
              Message will appear in Game Feed component
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

