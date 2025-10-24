import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Radio, Smartphone, Vibrate, Users } from 'lucide-react';

interface OutputNodeData {
  deviceMode: 'all' | 'specific' | 'type';
  deviceType?: 'vibrator' | 'linear' | 'rotator';
  selectedDevices?: string[];
}

export function OutputNode({ data, id, selected }: { data: OutputNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [deviceMode, setDeviceMode] = useState<'all' | 'specific' | 'type'>(data.deviceMode || 'all');
  const [deviceType, setDeviceType] = useState(data.deviceType || 'vibrator');
  const [selectedDevices, setSelectedDevices] = useState<string[]>(data.selectedDevices || []);
  
  useEffect(() => {
    data.deviceMode = deviceMode;
    data.deviceType = deviceType;
    data.selectedDevices = selectedDevices;
  }, [deviceMode, deviceType, selectedDevices, data]);
  
  const DEVICE_MODES = [
    { value: 'all', label: 'All Devices', icon: Users, desc: 'Send to all connected devices' },
    { value: 'type', label: 'By Type', icon: Vibrate, desc: 'Filter by device type' },
    { value: 'specific', label: 'Specific', icon: Smartphone, desc: 'Select individual devices' },
  ];
  
  const DEVICE_TYPES = [
    { value: 'vibrator', label: '💥 Vibrators', color: '#ec4899' },
    { value: 'linear', label: '📏 Linear', color: '#10b981' },
    { value: 'rotator', label: '🔄 Rotators', color: '#8b5cf6' },
  ];
  
  const selectedMode = DEVICE_MODES.find(m => m.value === deviceMode) || DEVICE_MODES[0];
  const ModeIcon = selectedMode.icon;
  
  return (
    <div 
      className="custom-node output-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '1px solid rgba(255, 153, 51, 0.2)',
        outline: 'none',
        boxShadow: '0 4px 12px rgba(0, 0, 0, 0.5)',
        minWidth: '200px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={200} 
        minHeight={150}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(255, 153, 51, 0.15)' }}>
        <Radio size={16} color="#ff9933" />
        <span style={{ color: '#ff9933' }}>{t('nodes.output.title')}</span>
      </div>
      <div className="node-body">
        <Handle 
          type="target" 
          position={Position.Left} 
          id="trigger"
          style={{ 
            background: '#5d8aa8', 
            width: 12, 
            height: 12, 
            border: 'none',
            boxShadow: '0 0 8px rgba(93, 138, 168, 0.6)',
            top: '30%' 
          }}
        />
        <Handle 
          type="target" 
          position={Position.Left} 
          id="vibration"
          style={{ 
            background: '#5d8aa8', 
            width: 12, 
            height: 12, 
            border: 'none',
            boxShadow: '0 0 8px rgba(93, 138, 168, 0.6)',
            top: '50%' 
          }}
        />
        <Handle 
          type="target" 
          position={Position.Left} 
          id="linear"
          style={{ 
            background: '#5d8aa8', 
            width: 12, 
            height: 12, 
            border: 'none',
            boxShadow: '0 0 8px rgba(93, 138, 168, 0.6)',
            top: '70%' 
          }}
        />
        
        <div style={{ padding: '8px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div>
            <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
              Device Mode:
            </label>
            <select 
              value={deviceMode}
              onChange={(e) => setDeviceMode(e.target.value as any)}
              className="node-select"
              style={{
                background: 'rgba(0, 0, 0, 0.3)',
                border: '1px solid rgba(255, 153, 51, 0.5)',
                color: '#94a3b8',
                fontSize: '11px'
              }}
            >
              {DEVICE_MODES.map(mode => (
                <option key={mode.value} value={mode.value}>
                  {mode.label}
                </option>
              ))}
            </select>
          </div>
          
          {deviceMode === 'type' && (
            <div>
              <label style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '4px', display: 'block' }}>
                Device Type:
              </label>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                {DEVICE_TYPES.map(type => (
                  <label 
                    key={type.value}
                    style={{ 
                      display: 'flex', 
                      alignItems: 'center', 
                      gap: '6px',
                      fontSize: '10px',
                      color: '#94a3b8',
                      cursor: 'pointer',
                      padding: '4px 6px',
                      background: deviceType === type.value ? 'rgba(255, 153, 51, 0.2)' : 'transparent',
                      borderRadius: '4px',
                      border: deviceType === type.value ? '1px solid rgba(255, 153, 51, 0.5)' : '1px solid transparent'
                    }}
                  >
                    <input
                      type="radio"
                      name={`device-type-${id}`}
                      checked={deviceType === type.value}
                      onChange={() => setDeviceType(type.value as any)}
                      onClick={(e) => e.stopPropagation()}
                      style={{ accentColor: type.color }}
                    />
                    {type.label}
                  </label>
                ))}
              </div>
            </div>
          )}
          
          {deviceMode === 'specific' && (
            <div style={{
              padding: '6px',
              background: 'rgba(255, 153, 51, 0.1)',
              borderRadius: '4px',
              fontSize: '9px',
              color: '#ff9933',
              textAlign: 'center'
            }}>
              ⚠️ Device list available at runtime
            </div>
          )}
          
          <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '4px',
            padding: '6px',
            background: 'rgba(255, 153, 51, 0.1)',
            borderRadius: '4px',
            fontSize: '9px',
            color: '#94a3b8'
          }}>
            <ModeIcon size={12} />
            <span style={{ lineHeight: '1.3' }}>{selectedMode.desc}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
