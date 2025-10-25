import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Zap, Target, AlertTriangle, Flame, Droplet, Wind, Skull, TrendingUp, MessageSquare, Award, Heart } from 'lucide-react';

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
    // === COMBAT EVENTS ===
    { id: 'Hit', icon: Target, color: '#f59e0b', category: 'Combat' },
    { id: 'CriticalHit', icon: Flame, color: '#ef4444', category: 'Combat' },
    { id: 'PenetrationHit', icon: Zap, color: '#dc2626', category: 'Combat' },
    { id: 'TargetDestroyed', icon: Skull, color: '#22c55e', category: 'Combat' },
    { id: 'EnemySetAfire', icon: Flame, color: '#fb923c', category: 'Combat' },
    { id: 'TakingDamage', icon: Heart, color: '#ef4444', category: 'Combat' },
    { id: 'SeverelyDamaged', icon: AlertTriangle, color: '#dc2626', category: 'Combat' },
    { id: 'ShotDown', icon: Target, color: '#991b1b', category: 'Combat' },
    
    // === FLIGHT WARNINGS ===
    { id: 'Overspeed', icon: Wind, color: '#8b5cf6', category: 'Flight' },
    { id: 'OverG', icon: AlertTriangle, color: '#ef4444', category: 'Flight' },
    { id: 'HighAOA', icon: Wind, color: '#f59e0b', category: 'Flight' },
    { id: 'CriticalAOA', icon: AlertTriangle, color: '#dc2626', category: 'Flight' },
    { id: 'Mach1', icon: Zap, color: '#6366f1', category: 'Flight' },
    { id: 'LowAltitude', icon: Wind, color: '#dc2626', category: 'Flight' },
    { id: 'Crashed', icon: AlertTriangle, color: '#991b1b', category: 'Flight' },
    
    // === RESOURCES ===
    { id: 'LowFuel', icon: Droplet, color: '#eab308', category: 'Resources' },
    { id: 'CriticalFuel', icon: AlertTriangle, color: '#ef4444', category: 'Resources' },
    { id: 'LowAmmo', icon: Target, color: '#f97316', category: 'Resources' },
    
    // === ENGINE ===
    { id: 'EngineDamaged', icon: Flame, color: '#f59e0b', category: 'Engine' },
    { id: 'EngineDestroyed', icon: AlertTriangle, color: '#dc2626', category: 'Engine' },
    { id: 'EngineOverheat', icon: Flame, color: '#ef4444', category: 'Engine' },
    { id: 'OilOverheated', icon: Droplet, color: '#f97316', category: 'Engine' },
    
    // === MULTIPLAYER ===
    { id: 'Achievement', icon: Award, color: '#fbbf24', category: 'Multiplayer' },
    { id: 'ChatMessage', icon: MessageSquare, color: '#6366f1', category: 'Multiplayer' },
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
            {/* Group events by category */}
            {Object.entries(
              EVENTS.reduce((acc, evt) => {
                if (!acc[evt.category]) acc[evt.category] = [];
                acc[evt.category].push(evt);
                return acc;
              }, {} as Record<string, typeof EVENTS>)
            ).map(([category, events]) => (
              <optgroup key={category} label={`━━ ${category} ━━`}>
                {events.map(evt => (
                  <option key={evt.id} value={evt.id}>
                    {t(`game_events.${evt.id}`, evt.id)}
                  </option>
                ))}
              </optgroup>
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

