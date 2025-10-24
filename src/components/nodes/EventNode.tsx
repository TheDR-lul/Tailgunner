import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Zap, Target, AlertTriangle, Flame, Droplet, Wind } from 'lucide-react';

interface EventNodeData {
  event: string;
}

export function EventNode({ data, id, selected }: { data: EventNodeData; id: string; selected?: boolean }) {
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
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '2px solid rgba(255, 153, 51, 0.3)',
        minWidth: '180px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={180} 
        minHeight={100}
        color="rgba(255, 153, 51, 0.8)"
      />
      <div className="node-header" style={{ background: 'rgba(255, 153, 51, 0.15)' }}>
        <EventIcon size={16} color="#ff9933" />
        <span style={{ color: '#ff9933' }}>Game Event</span>
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

