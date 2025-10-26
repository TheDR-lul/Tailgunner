import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Radio, Smartphone, Vibrate, Users, RefreshCw } from 'lucide-react';
import { api } from '../../api';
import type { DeviceInfo } from '../../types';

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
  const [availableDevices, setAvailableDevices] = useState<DeviceInfo[]>([]);
  const [isLoadingDevices, setIsLoadingDevices] = useState(false);
  
  useEffect(() => {
    data.deviceMode = deviceMode;
    data.deviceType = deviceType;
    data.selectedDevices = selectedDevices;
  }, [deviceMode, deviceType, selectedDevices, data]);
  
  // Load devices when switching to 'specific' mode
  useEffect(() => {
    if (deviceMode === 'specific') {
      loadDevices();
    }
  }, [deviceMode]);
  
  const loadDevices = async () => {
    setIsLoadingDevices(true);
    try {
      const devices = await api.getDevices();
      setAvailableDevices(devices);
    } catch (error) {
      console.error('Failed to load devices:', error);
      setAvailableDevices([]);
    } finally {
      setIsLoadingDevices(false);
    }
  };
  
  const toggleDevice = (deviceId: string) => {
    setSelectedDevices(prev => {
      if (prev.includes(deviceId)) {
        return prev.filter(id => id !== deviceId);
      } else {
        return [...prev, deviceId];
      }
    });
  };
  
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
            <div>
              <div style={{ 
                display: 'flex', 
                justifyContent: 'space-between', 
                alignItems: 'center',
                marginBottom: '6px'
              }}>
                <label style={{ fontSize: '10px', color: '#94a3b8' }}>
                  Select Devices:
                </label>
                <button 
                  onClick={(e) => { e.stopPropagation(); loadDevices(); }}
                  style={{
                    background: 'rgba(255, 153, 51, 0.2)',
                    border: '1px solid rgba(255, 153, 51, 0.4)',
                    color: '#ff9933',
                    padding: '2px 6px',
                    borderRadius: '3px',
                    fontSize: '9px',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '3px'
                  }}
                  disabled={isLoadingDevices}
                >
                  <RefreshCw size={10} className={isLoadingDevices ? 'spin' : ''} />
                  Refresh
                </button>
              </div>
              
              {isLoadingDevices ? (
                <div style={{
                  padding: '8px',
                  textAlign: 'center',
                  fontSize: '9px',
                  color: '#94a3b8'
                }}>
                  Loading devices...
                </div>
              ) : availableDevices.length === 0 ? (
                <div style={{
                  padding: '8px',
                  background: 'rgba(255, 153, 51, 0.1)',
                  borderRadius: '4px',
                  fontSize: '9px',
                  color: '#ff9933',
                  textAlign: 'center'
                }}>
                  No devices connected
                </div>
              ) : (
                <div style={{ 
                  display: 'flex', 
                  flexDirection: 'column', 
                  gap: '4px',
                  maxHeight: '120px',
                  overflowY: 'auto',
                  padding: '4px',
                  background: 'rgba(0, 0, 0, 0.2)',
                  borderRadius: '4px'
                }}>
                  {availableDevices.map(device => (
                    <label 
                      key={device.id}
                      style={{ 
                        display: 'flex', 
                        alignItems: 'center', 
                        gap: '6px',
                        fontSize: '10px',
                        color: '#94a3b8',
                        cursor: 'pointer',
                        padding: '4px 6px',
                        background: selectedDevices.includes(device.id) ? 'rgba(255, 153, 51, 0.2)' : 'transparent',
                        borderRadius: '4px',
                        border: selectedDevices.includes(device.id) ? '1px solid rgba(255, 153, 51, 0.5)' : '1px solid transparent'
                      }}
                    >
                      <input
                        type="checkbox"
                        checked={selectedDevices.includes(device.id)}
                        onChange={() => toggleDevice(device.id)}
                        onClick={(e) => e.stopPropagation()}
                        style={{ accentColor: '#ff9933' }}
                      />
                      <div style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {device.name}
                      </div>
                      {device.device_type && (
                        <span style={{ fontSize: '8px', opacity: 0.6 }}>
                          {device.device_type}
                        </span>
                      )}
                    </label>
                  ))}
                </div>
              )}
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

