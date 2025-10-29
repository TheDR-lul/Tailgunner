import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Activity, Gauge, Zap, Wind, Droplet, Crosshair, Fuel, Thermometer, Settings, Users, Navigation } from 'lucide-react';

interface InputNodeData {
  label?: string;
  indicator: string;
}

export function InputNode({ data, id, selected }: { data: InputNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [indicator, setIndicator] = useState(data.indicator || 'speed');
  
  const INDICATORS = [
    // === AIRCRAFT: Flight Parameters (from /state) ===
    { id: 'ias', label: 'IAS', unit: 'km/h', icon: Activity, color: '#3b82f6', category: 'Aircraft: Flight' },
    { id: 'tas', label: 'TAS', unit: 'km/h', icon: Activity, color: '#6366f1', category: 'Aircraft: Flight' },
    { id: 'altitude', label: 'Altitude', unit: 'm', icon: Wind, color: '#06b6d4', category: 'Aircraft: Flight' },
    { id: 'mach', label: 'Mach', unit: 'M', icon: Zap, color: '#8b5cf6', category: 'Aircraft: Flight' },
    { id: 'aoa', label: 'AoA', unit: '°', icon: Activity, color: '#f97316', category: 'Aircraft: Flight' },
    { id: 'g_load', label: 'G-Load (Ny)', unit: 'G', icon: Zap, color: '#eab308', category: 'Aircraft: Flight' },
    
    // === AIRCRAFT: Controls (from /state) ===
    { id: 'aileron', label: 'Aileron', unit: '%', icon: Settings, color: '#3b82f6', category: 'Aircraft: Controls' },
    { id: 'elevator', label: 'Elevator', unit: '%', icon: Settings, color: '#6366f1', category: 'Aircraft: Controls' },
    { id: 'rudder', label: 'Rudder', unit: '%', icon: Settings, color: '#8b5cf6', category: 'Aircraft: Controls' },
    { id: 'flaps', label: 'Flaps', unit: '%', icon: Settings, color: '#a855f7', category: 'Aircraft: Controls' },
    { id: 'gear', label: 'Landing Gear', unit: '%', icon: Settings, color: '#c084fc', category: 'Aircraft: Controls' },
    { id: 'airbrake', label: 'Airbrake', unit: '%', icon: Settings, color: '#e879f9', category: 'Aircraft: Controls' },
    
    // === AIRCRAFT: Stick/Pedals (from /indicators) ===
    { id: 'stick_elevator', label: 'Stick Elevator', unit: '', icon: Navigation, color: '#3b82f6', category: 'Aircraft: Raw Input' },
    { id: 'stick_ailerons', label: 'Stick Aileron', unit: '', icon: Navigation, color: '#6366f1', category: 'Aircraft: Raw Input' },
    { id: 'pedals', label: 'Rudder Pedals', unit: '', icon: Navigation, color: '#8b5cf6', category: 'Aircraft: Raw Input' },
    
    // === AIRCRAFT: Engine (from /state + /indicators) ===
    { id: 'throttle', label: 'Throttle', unit: '%', icon: Gauge, color: '#f59e0b', category: 'Aircraft: Engine' },
    { id: 'rpm', label: 'RPM', unit: 'RPM', icon: Gauge, color: '#ef4444', category: 'Aircraft: Engine' },
    { id: 'manifold_pressure', label: 'Manifold Pressure', unit: 'atm', icon: Gauge, color: '#fb923c', category: 'Aircraft: Engine' },
    { id: 'oil_temp', label: 'Oil Temp', unit: '°C', icon: Thermometer, color: '#f87171', category: 'Aircraft: Engine' },
    { id: 'water_temp', label: 'Water Temp', unit: '°C', icon: Thermometer, color: '#fca5a5', category: 'Aircraft: Engine' },
    
    // === AIRCRAFT: Resources (from /state) ===
    { id: 'fuel', label: 'Fuel', unit: 'kg', icon: Fuel, color: '#10b981', category: 'Aircraft: Resources' },
    { id: 'fuel_percent', label: 'Fuel %', unit: '%', icon: Fuel, color: '#34d399', category: 'Aircraft: Resources' },
    
    // === AIRCRAFT: Advanced (from /indicators) ===
    { id: 'blister1', label: 'Blister 1', unit: '', icon: Wind, color: '#3b82f6', category: 'Aircraft: Advanced' },
    { id: 'blister2', label: 'Blister 2', unit: '', icon: Wind, color: '#6366f1', category: 'Aircraft: Advanced' },
    { id: 'gear_lamp_down', label: 'Gear Lamp Down', unit: '', icon: Settings, color: '#ef4444', category: 'Aircraft: Advanced' },
    { id: 'gear_lamp_up', label: 'Gear Lamp Up', unit: '', icon: Settings, color: '#10b981', category: 'Aircraft: Advanced' },
    
    // === TANK/SHIP: Movement (from /indicators) ===
    { id: 'speed', label: 'Speed', unit: 'km/h', icon: Activity, color: '#3b82f6', category: 'Tank: Movement' },
    
    // === TANK: Controls (from /indicators) ===
    { id: 'tank_gear', label: 'Gear', unit: '', icon: Settings, color: '#a855f7', category: 'Tank: Controls' },
    { id: 'stabilizer', label: 'Stabilizer', unit: '', icon: Settings, color: '#8b5cf6', category: 'Tank: Controls' },
    { id: 'has_speed_warning', label: 'Speed Warning', unit: '', icon: Zap, color: '#ef4444', category: 'Tank: Controls' },
    { id: 'cruise_control', label: 'Cruise Control', unit: '', icon: Settings, color: '#10b981', category: 'Tank: Controls' },
    { id: 'driving_direction_mode', label: 'Drive Direction', unit: '', icon: Navigation, color: '#6366f1', category: 'Tank: Controls' },
    
    // === TANK: Weapons (from /indicators) ===
    { id: 'first_stage_ammo', label: 'Ready Ammo', unit: 'rounds', icon: Crosshair, color: '#fbbf24', category: 'Tank: Weapons' },
    
    // === TANK: Crew (from /indicators) ===
    { id: 'crew_total', label: 'Crew Total', unit: '', icon: Users, color: '#10b981', category: 'Tank: Crew' },
    { id: 'crew_current', label: 'Crew Alive', unit: '', icon: Users, color: '#34d399', category: 'Tank: Crew' },
    { id: 'crew_distance', label: 'Crew Distance', unit: 'm', icon: Users, color: '#6ee7b7', category: 'Tank: Crew' },
    { id: 'gunner_state', label: 'Gunner State', unit: '', icon: Users, color: '#a7f3d0', category: 'Tank: Crew' },
    { id: 'driver_state', label: 'Driver State', unit: '', icon: Users, color: '#d1fae5', category: 'Tank: Crew' },
    
    // === TANK: Defense (from /indicators) ===
    { id: 'lws', label: 'LWS (Laser Warning)', unit: '', icon: Zap, color: '#f59e0b', category: 'Tank: Defense' },
    { id: 'ircm', label: 'IRCM (IR Countermeasure)', unit: '', icon: Zap, color: '#eab308', category: 'Tank: Defense' },
  ];
  
  const selectedIndicator = INDICATORS.find(i => i.id === indicator) || INDICATORS[0];
  const Icon = selectedIndicator.icon;
  
  // Group indicators by category
  const categorizedIndicators = INDICATORS.reduce((acc, ind) => {
    if (!acc[ind.category]) acc[ind.category] = [];
    acc[ind.category].push(ind);
    return acc;
  }, {} as Record<string, typeof INDICATORS>);
  
  useEffect(() => {
    data.indicator = indicator;
  }, [indicator, data]);
  
  return (
    <div 
      className="custom-node input-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '1px solid rgba(255, 153, 51, 0.2)',
        outline: 'none',
        boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={180} 
        minHeight={120}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(255, 153, 51, 0.15)' }}>
        <Icon size={16} color="#ff9933" />
        <span style={{ color: '#ff9933' }}>{t('nodes.input.title')}</span>
      </div>
      <div className="node-body">
        <select 
          value={indicator}
          onChange={(e) => setIndicator(e.target.value)}
          className="node-select"
          onClick={(e) => e.stopPropagation()}
          style={{
            background: 'rgba(0, 0, 0, 0.3)',
            border: '1px solid rgba(255, 153, 51, 0.5)',
            color: '#94a3b8'
          }}
        >
          {Object.entries(categorizedIndicators).map(([category, indicators]) => (
            <optgroup key={category} label={`━━ ${category} ━━`}>
              {indicators.map(ind => (
                <option key={ind.id} value={ind.id}>
                  {ind.label} {ind.unit && `(${ind.unit})`}
                </option>
              ))}
            </optgroup>
          ))}
        </select>
        
        <div style={{
          marginTop: '8px',
          padding: '8px',
          background: 'rgba(255, 153, 51, 0.1)',
          borderRadius: '6px',
          border: '1px solid rgba(255, 153, 51, 0.3)'
        }}>
          <div style={{
            fontSize: '10px',
            color: '#94a3b8',
            textAlign: 'center',
            marginBottom: '4px'
          }}>
            📊 Current Value
          </div>
          <div style={{
            fontSize: '16px',
            fontWeight: 'bold',
            color: selectedIndicator.color,
            textAlign: 'center',
            textShadow: `0 0 10px ${selectedIndicator.color}88`
          }}>
            {selectedIndicator.label}
          </div>
          <div style={{
            fontSize: '9px',
            color: '#64748b',
            textAlign: 'center',
            marginTop: '2px'
          }}>
            {selectedIndicator.category} • {selectedIndicator.unit}
          </div>
        </div>
        
        <div style={{
          marginTop: '6px',
          fontSize: '8px',
          color: '#10b981',
          textAlign: 'center',
          background: 'rgba(16, 185, 129, 0.1)',
          padding: '4px',
          borderRadius: '4px'
        }}>
          ✓ Connect to Condition node
        </div>
        
        {/* API Limitation Notice */}
        <div style={{
          marginTop: '6px',
          fontSize: '7px',
          color: '#64748b',
          textAlign: 'center',
          background: 'rgba(100, 116, 139, 0.1)',
          padding: '4px',
          borderRadius: '4px',
          borderLeft: '2px solid #64748b'
        }}>
          ⚠️ Aircraft: Full telemetry available<br/>
          ⚠️ Tanks/Ships: Limited to /indicators only
        </div>
      </div>
      <Handle 
        type="source" 
        position={Position.Right} 
        id="value"
        style={{ 
          background: '#ff9933', 
          width: 12, 
          height: 12, 
          border: 'none',
          boxShadow: '0 0 8px rgba(255, 153, 51, 0.6)'
        }}
      />
    </div>
  );
}
