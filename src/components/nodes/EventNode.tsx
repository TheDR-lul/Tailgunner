import { useState, useEffect } from 'react';
import { Handle, Position } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Zap, Target, AlertTriangle, Flame, Droplet, Wind } from 'lucide-react';

interface EventNodeData {
  event: string;
}

export function EventNode({ data, id }: { data: EventNodeData; id: string }) {
  const { t } = useTranslation();
  const [event, setEvent] = useState(data.event || 'Hit');
  
  useEffect(() => {
    data.event = event;
  }, [event, data]);
  
  const EVENTS = [
    { id: 'Hit', icon: Target, color: '#f59e0b' },
    { id: 'CriticalHit', icon: Flame, color: '#ef4444' },
    { id: 'PenetrationHit', icon: Zap, color: '#dc2626' },
    { id: 'Overspeed', icon: Wind, color: '#8b5cf6' },
    { id: 'OverG', icon: AlertTriangle, color: '#ef4444' },
    { id: 'HighAOA', icon: Wind, color: '#f59e0b' },
    { id: 'CriticalAOA', icon: AlertTriangle, color: '#dc2626' },
    { id: 'Mach1', icon: Zap, color: '#6366f1' },
    { id: 'LowFuel', icon: Droplet, color: '#eab308' },
    { id: 'CriticalFuel', icon: AlertTriangle, color: '#ef4444' },
    { id: 'LowAmmo', icon: Target, color: '#f97316' },
    { id: 'LowAltitude', icon: Wind, color: '#dc2626' },
    { id: 'EngineDamaged', icon: Flame, color: '#f59e0b' },
    { id: 'EngineDestroyed', icon: AlertTriangle, color: '#dc2626' },
  ];
  
  const selectedEvent = EVENTS.find(e => e.id === event) || EVENTS[0];
  const EventIcon = selectedEvent.icon;
  
  return (
    <div 
      className="custom-node event-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: `linear-gradient(135deg, ${selectedEvent.color}22 0%, ${selectedEvent.color}44 100%)`,
        border: `2px solid ${selectedEvent.color}`,
        minWidth: '180px'
      }}
    >
      <div className="node-header" style={{ background: `${selectedEvent.color}33` }}>
        <EventIcon size={16} color={selectedEvent.color} />
        <span style={{ color: selectedEvent.color }}>Game Event</span>
      </div>
      <div className="node-body">
        <div style={{ padding: '8px' }}>
          <select 
            value={event} 
            onChange={(e) => setEvent(e.target.value)}
            className="node-select"
            style={{
              background: 'rgba(0, 0, 0, 0.3)',
              border: `1px solid ${selectedEvent.color}`,
              color: selectedEvent.color,
              fontSize: '11px',
              width: '100%'
            }}
          >
            {EVENTS.map(evt => (
              <option key={evt.id} value={evt.id}>
                {t(`game_events.${evt.id}`, evt.id)}
              </option>
            ))}
          </select>
          
          <div style={{
            marginTop: '8px',
            padding: '6px',
            background: `${selectedEvent.color}22`,
            borderRadius: '4px',
            fontSize: '9px',
            color: selectedEvent.color,
            textAlign: 'center',
            fontWeight: 600
          }}>
            Triggers on: {t(`game_events.${event}`, event)}
          </div>
        </div>
      </div>
      
      <Handle 
        type="source" 
        position={Position.Right} 
        id="triggered"
        style={{ 
          background: selectedEvent.color, 
          width: 12, 
          height: 12, 
          border: `2px solid ${selectedEvent.color}`,
          boxShadow: `0 0 8px ${selectedEvent.color}88`
        }}
      />
    </div>
  );
}

