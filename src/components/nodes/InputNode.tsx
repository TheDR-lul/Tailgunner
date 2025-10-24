import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Activity, Gauge, Zap, Wind, Droplet, Crosshair, Fuel, Thermometer, Settings, Users, Navigation } from 'lucide-react';

interface InputNodeData {
  label?: string;
  indicator: string;
  operator?: string;
  value?: number;
}

export function InputNode({ data, id, selected }: { data: InputNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [indicator, setIndicator] = useState(data.indicator || 'speed');
  const [operator, setOperator] = useState(data.operator || '>');
  const [value, setValue] = useState(data.value || 0);
  
  const INDICATORS = [
    // Flight Parameters
    { id: 'speed', label: 'IAS', unit: 'km/h', icon: Activity, color: '#3b82f6', category: 'Flight' },
    { id: 'tas', label: 'TAS', unit: 'km/h', icon: Activity, color: '#6366f1', category: 'Flight' },
    { id: 'altitude', label: 'Altitude', unit: 'm', icon: Wind, color: '#06b6d4', category: 'Flight' },
    { id: 'mach', label: 'Mach', unit: 'M', icon: Zap, color: '#8b5cf6', category: 'Flight' },
    { id: 'aoa', label: 'AoA', unit: '°', icon: Activity, color: '#f97316', category: 'Flight' },
    { id: 'g_load', label: 'G-Load (Ny)', unit: 'G', icon: Zap, color: '#eab308', category: 'Flight' },
    
    // Controls
    { id: 'aileron', label: 'Aileron', unit: '%', icon: Settings, color: '#3b82f6', category: 'Controls' },
    { id: 'elevator', label: 'Elevator', unit: '%', icon: Settings, color: '#6366f1', category: 'Controls' },
    { id: 'rudder', label: 'Rudder', unit: '%', icon: Settings, color: '#8b5cf6', category: 'Controls' },
    { id: 'flaps', label: 'Flaps', unit: '%', icon: Settings, color: '#a855f7', category: 'Controls' },
    { id: 'gear', label: 'Landing Gear', unit: '%', icon: Settings, color: '#c084fc', category: 'Controls' },
    { id: 'airbrake', label: 'Airbrake', unit: '%', icon: Settings, color: '#e879f9', category: 'Controls' },
    
    // Stick/Pedals (Raw Input)
    { id: 'stick_elevator', label: 'Stick Elevator', unit: '', icon: Navigation, color: '#3b82f6', category: 'Raw Input' },
    { id: 'stick_aileron', label: 'Stick Aileron', unit: '', icon: Navigation, color: '#6366f1', category: 'Raw Input' },
    { id: 'pedals', label: 'Rudder Pedals', unit: '', icon: Navigation, color: '#8b5cf6', category: 'Raw Input' },
    
    // Engine
    { id: 'rpm', label: 'RPM', unit: 'RPM', icon: Gauge, color: '#ef4444', category: 'Engine' },
    { id: 'engine_temp', label: 'Engine Temp', unit: '°C', icon: Thermometer, color: '#dc2626', category: 'Engine' },
    { id: 'oil_temp', label: 'Oil Temp', unit: '°C', icon: Thermometer, color: '#f87171', category: 'Engine' },
    { id: 'water_temp', label: 'Water Temp', unit: '°C', icon: Thermometer, color: '#fca5a5', category: 'Engine' },
    { id: 'manifold_pressure', label: 'Manifold Pressure', unit: 'atm', icon: Gauge, color: '#fb923c', category: 'Engine' },
    { id: 'throttle', label: 'Throttle', unit: '%', icon: Gauge, color: '#f59e0b', category: 'Engine' },
    
    // Weapons
    { id: 'ammo', label: 'Ammo Count', unit: 'pcs', icon: Crosshair, color: '#fbbf24', category: 'Weapons' },
    { id: 'cannon_ready', label: 'Cannon Ready', unit: '', icon: Crosshair, color: '#ef4444', category: 'Weapons' },
    
    // Resources
    { id: 'fuel', label: 'Fuel', unit: 'kg', icon: Fuel, color: '#10b981', category: 'Resources' },
    { id: 'fuel_percent', label: 'Fuel %', unit: '%', icon: Fuel, color: '#34d399', category: 'Resources' },
    
    // Tank Specific
    { id: 'stabilizer', label: 'Stabilizer', unit: '', icon: Settings, color: '#8b5cf6', category: 'Tank' },
    { id: 'gear_ratio', label: 'Gear Ratio', unit: '', icon: Gauge, color: '#a855f7', category: 'Tank' },
    { id: 'has_speed_warning', label: 'Speed Warning', unit: '', icon: Zap, color: '#ef4444', category: 'Tank' },
    { id: 'cruise_control', label: 'Cruise Control', unit: '', icon: Settings, color: '#10b981', category: 'Tank' },
    { id: 'driving_direction_mode', label: 'Drive Direction', unit: '', icon: Navigation, color: '#6366f1', category: 'Tank' },
    
    // Crew
    { id: 'crew_total', label: 'Crew Total', unit: '', icon: Users, color: '#10b981', category: 'Crew' },
    { id: 'crew_current', label: 'Crew Alive', unit: '', icon: Users, color: '#34d399', category: 'Crew' },
    { id: 'crew_distance', label: 'Crew Distance', unit: 'm', icon: Users, color: '#6ee7b7', category: 'Crew' },
    { id: 'gunner_state', label: 'Gunner State', unit: '', icon: Users, color: '#a7f3d0', category: 'Crew' },
    { id: 'driver_state', label: 'Driver State', unit: '', icon: Users, color: '#d1fae5', category: 'Crew' },
    
    // Advanced Aerodynamics
    { id: 'blister1', label: 'Blister 1', unit: '', icon: Wind, color: '#3b82f6', category: 'Advanced' },
    { id: 'blister2', label: 'Blister 2', unit: '', icon: Wind, color: '#6366f1', category: 'Advanced' },
    { id: 'blister3', label: 'Blister 3', unit: '', icon: Wind, color: '#8b5cf6', category: 'Advanced' },
    { id: 'blister4', label: 'Blister 4', unit: '', icon: Wind, color: '#a855f7', category: 'Advanced' },
    { id: 'gear_lamp_up', label: 'Gear Lamp Up', unit: '', icon: Settings, color: '#10b981', category: 'Advanced' },
    { id: 'gear_lamp_down', label: 'Gear Lamp Down', unit: '', icon: Settings, color: '#ef4444', category: 'Advanced' },
    { id: 'gear_lamp_off', label: 'Gear Lamp Off', unit: '', icon: Settings, color: '#6b7280', category: 'Advanced' },
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
    data.operator = operator;
    data.value = value;
  }, [indicator, operator, value, data]);
  
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
        
        <div style={{ display: 'flex', gap: '4px', marginTop: '8px', alignItems: 'center' }}>
          <select 
            value={operator}
            onChange={(e) => setOperator(e.target.value)}
            className="node-select"
            onClick={(e) => e.stopPropagation()}
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: '1px solid rgba(255, 153, 51, 0.5)',
              color: '#94a3b8',
              flex: '0 0 45px'
            }}
          >
            <option value=">">{'>'}</option>
            <option value="<">{'<'}</option>
            <option value=">=">{'≥'}</option>
            <option value="<=">{'≤'}</option>
            <option value="==">{'='}</option>
          </select>
          
          <input
            type="number"
            value={value}
            onChange={(e) => setValue(parseFloat(e.target.value) || 0)}
            className="node-input-field"
            onClick={(e) => e.stopPropagation()}
            onMouseDown={(e) => e.stopPropagation()}
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: '1px solid rgba(255, 153, 51, 0.5)',
              color: '#ff9933',
              padding: '4px 8px',
              borderRadius: '4px',
              fontSize: '12px',
              flex: 1
            }}
          />
        </div>
        
        <div style={{
          marginTop: '6px',
          fontSize: '9px',
          color: '#64748b',
          textAlign: 'center'
        }}>
          {selectedIndicator.category}
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
