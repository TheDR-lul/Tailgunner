import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Play, Pause, Zap, Target, MessageSquare, Flame, Skull } from 'lucide-react';
import { api } from '../api';
import { VEHICLE_PRESETS, getVehiclesByType, type VehiclePreset } from '../data/vehiclePresets';
import './APIEmulator.css';

interface EmulatorState {
  // Core
  enabled: boolean;
  vehicle_type: 'Tank' | 'Aircraft' | 'Ship';
  vehicle_name: string;
  vehicle_display_name: string;
  in_battle: boolean;
  
  // Movement
  speed: number;
  altitude: number;
  heading: number;
  position: [number, number];
  
  // Combat
  ammo: number;
  hp: number;
  engine_running: boolean;
  
  // Aircraft specific
  tas: number;
  ias: number;
  mach: number;
  aoa: number;
  aos: number;
  g_load: number;
  vertical_speed: number;
  roll_rate: number;
  
  // Fuel
  fuel_kg: number;
  fuel_max_kg: number;
  
  // Engine
  rpm: number;
  throttle: number;
  manifold_pressure: number;
  oil_temp: number;
  water_temp: number;
  thrust: number;
  
  // Controls
  stick_elevator: number;
  stick_ailerons: number;
  pedals: number;
  flaps: number;
  gear: number;
  
  // Orientation
  pitch: number;
  roll: number;
  compass: number;
}

export function APIEmulator() {
  const { t } = useTranslation();
  const [state, setState] = useState<EmulatorState>({
    enabled: false,
    vehicle_type: 'Aircraft',
    vehicle_name: 'f_16a',
    vehicle_display_name: 'F-16A',
    in_battle: false,
    speed: 0,
    altitude: 1000,
    heading: 0,
    position: [0.5, 0.5],
    ammo: 300,
    hp: 100,
    engine_running: true,
    tas: 0,
    ias: 0,
    mach: 0,
    aoa: 0,
    aos: 0,
    g_load: 1.0,
    vertical_speed: 0,
    roll_rate: 0,
    fuel_kg: 3000,
    fuel_max_kg: 5000,
    rpm: 0,
    throttle: 0,
    manifold_pressure: 1.0,
    oil_temp: 15,
    water_temp: 15,
    thrust: 0,
    stick_elevator: 0,
    stick_ailerons: 0,
    pedals: 0,
    flaps: 0,
    gear: 1.0,
    pitch: 0,
    roll: 0,
    compass: 0,
  });
  const [expanded, setExpanded] = useState(false);
  const [activeTab, setActiveTab] = useState('basic');
  const [chatMessage, setChatMessage] = useState('');
  const [chatMode, setChatMode] = useState('Team');
  const [chatSender, setChatSender] = useState('TestPlayer');
  const [chatIsEnemy, setChatIsEnemy] = useState(false);

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

  const setVehicle = async (preset: VehiclePreset) => {
    try {
      await api.emulatorSetVehicleType(preset.type);
      await api.emulatorSetVehicleName(preset.name, preset.displayName);
      await loadState();
      console.log('[APIEmulator] Vehicle set to:', preset.displayName);
    } catch (err) {
      console.error('[APIEmulator] Failed to set vehicle:', err);
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
      await api.emulatorSendChat(chatMessage, chatMode, chatSender, chatIsEnemy);
      console.log('[APIEmulator] Chat sent from', chatSender, ':', chatMessage);
      setChatMessage('');
    } catch (err) {
      console.error('[APIEmulator] Failed to send chat:', err);
    }
  };
  
  // Player presets
  const playerPresets = [
    { name: 'TestPlayer', enemy: false },
    { name: 'ButtThunder', enemy: false },
    { name: '[SQUAD] Wingman', enemy: false },
    { name: 'EnemyAce', enemy: true },
    { name: '[CLAN] Enemy1', enemy: true },
    { name: 'RandomEnemy', enemy: true },
  ];

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
          {/* Vehicle Selection */}
          <div className="emulator-section" style={{ gridColumn: '1 / -1' }}>
            <h4>🚗 Vehicle Selection</h4>
            <div style={{ marginBottom: '12px' }}>
              <div style={{ display: 'flex', gap: '8px', marginBottom: '8px' }}>
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
              
              {/* Specific Vehicle Selector */}
              <div>
                <label style={{ fontSize: '10px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>
                  Current: {state.vehicle_display_name}
                </label>
                <div style={{ display: 'flex', gap: '4px', flexWrap: 'wrap' }}>
                  {getVehiclesByType(state.vehicle_type).map((preset) => (
                    <button
                      key={preset.name}
                      className={`btn btn-${state.vehicle_name === preset.name ? 'primary' : 'secondary'}`}
                      onClick={() => setVehicle(preset)}
                      disabled={!state.enabled}
                      style={{ fontSize: '9px', padding: '4px 8px' }}
                      title={`Max speed: ${preset.maxSpeed} km/h`}
                    >
                      {preset.icon} {preset.displayName}
                    </button>
                  ))}
                </div>
              </div>
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
                    Speed: {state.speed.toFixed(0)} / {VEHICLE_PRESETS.find(v => v.name === state.vehicle_name)?.maxSpeed || 800} km/h
                  </label>
                  <input
                    type="range"
                    min="0"
                    max={VEHICLE_PRESETS.find(v => v.name === state.vehicle_name)?.maxSpeed || 800}
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
                    Damage: {(100 - state.hp).toFixed(0)}% (Integrity: {state.hp.toFixed(0)}%)
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

          {/* Computed Parameters (Aircraft) */}
          {state.vehicle_type === 'Aircraft' && (
            <div className="emulator-section" style={{ gridColumn: '1 / -1' }}>
              <h4>🧮 Auto-Computed Parameters (Read-only)</h4>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '12px', fontSize: '10px' }}>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>IAS</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.ias.toFixed(0)} km/h</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>TAS</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.tas.toFixed(0)} km/h</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Mach</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.mach.toFixed(3)}</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>RPM</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.rpm.toFixed(0)}</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Throttle</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.throttle.toFixed(1)}%</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Thrust</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.thrust.toFixed(0)} kgs</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Oil Temp</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.oil_temp.toFixed(1)}°C</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Water Temp</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.water_temp.toFixed(1)}°C</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Fuel</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.fuel_kg.toFixed(0)}/{state.fuel_max_kg.toFixed(0)} kg</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>G-load</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.g_load.toFixed(2)}G</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Compass</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.compass.toFixed(0)}°</div>
                </div>
                <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                  <div style={{ color: 'var(--text-muted)', marginBottom: '2px' }}>Gear</div>
                  <div style={{ color: 'var(--text-primary)', fontWeight: 'bold' }}>{state.gear > 0.9 ? '🟢 Down' : state.gear < 0.1 ? '🔴 Up' : '🟡 Moving'}</div>
                </div>
              </div>
              <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginTop: '8px', fontStyle: 'italic' }}>
                💡 These values are automatically calculated when you change Speed or Altitude
              </div>
            </div>
          )}

          {/* Chat Emulator */}
          <div className="emulator-section" style={{ gridColumn: '1 / -1' }}>
            <h4>💬 Chat Emulator</h4>
            
            {/* Player Selection */}
            <div style={{ marginBottom: '8px' }}>
              <label style={{ fontSize: '10px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>
                Player: {chatSender} {chatIsEnemy ? '(Enemy)' : '(Friendly)'}
              </label>
              <div style={{ display: 'flex', gap: '4px', flexWrap: 'wrap' }}>
                {playerPresets.map((preset) => (
                  <button
                    key={preset.name}
                    className={`btn btn-${chatSender === preset.name ? 'primary' : 'secondary'}`}
                    onClick={() => {
                      setChatSender(preset.name);
                      setChatIsEnemy(preset.enemy);
                    }}
                    disabled={!state.enabled || !state.in_battle}
                    style={{ 
                      fontSize: '9px', 
                      padding: '4px 8px',
                      background: preset.enemy 
                        ? 'rgba(250, 12, 0, 0.1)' 
                        : 'rgba(23, 77, 255, 0.1)',
                      borderColor: preset.enemy 
                        ? 'rgba(250, 12, 0, 0.3)' 
                        : 'rgba(23, 77, 255, 0.3)'
                    }}
                  >
                    {preset.enemy ? '🔴' : '🔵'} {preset.name}
                  </button>
                ))}
              </div>
            </div>
            
            {/* Chat Mode */}
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
            
            {/* Message Input */}
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
              Message from {chatSender} will appear in Game Feed component
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

