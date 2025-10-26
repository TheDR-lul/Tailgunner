import { useState, useEffect } from 'react';
import { Handle, Position, NodeResizer } from 'reactflow';
import { useTranslation } from 'react-i18next';
import { Zap, Target, AlertTriangle, Flame, Droplet, Wind, Skull, TrendingUp, MessageSquare, Award, Heart } from 'lucide-react';

interface EventNodeData {
  event: string;
  filter_type?: string;  // 'any', 'my_players', 'my_clans', 'text_contains'
  filter_text?: string;  // For ChatMessage text filtering
}

export function EventNode({ data, id, selected }: { data: EventNodeData; id: string; selected?: boolean }) {
  const { t } = useTranslation();
  const [event, setEvent] = useState(data.event || 'Hit');
  const [filterType, setFilterType] = useState(data.filter_type || 'any');
  const [filterText, setFilterText] = useState(data.filter_text || '');
  
  useEffect(() => {
    data.event = event;
    data.filter_type = filterType;
    data.filter_text = filterText;
  }, [event, filterType, filterText, data]);
  
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
  
  // Determine if this event needs filtering
  const needsTextFilter = ['ChatMessage'].includes(event);
  const needsPlayerFilter = ['TargetDestroyed', 'EnemySetAfire', 'TakingDamage', 'SeverelyDamaged', 'ShotDown', 'Achievement'].includes(event);
  const showFilter = needsTextFilter || needsPlayerFilter;
  
  return (
    <div 
      className="custom-node event-node" 
      onClick={(e) => e.stopPropagation()}
      style={{
        background: 'linear-gradient(135deg, #1a1f29 0%, #252b3a 100%)',
        border: '2px solid rgba(255, 153, 51, 0.3)',
        minWidth: '200px'
      }}
    >
      <NodeResizer 
        isVisible={selected} 
        minWidth={200} 
        minHeight={showFilter ? 180 : 100}
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
          
          {/* Filter Section */}
          {showFilter && (
            <div style={{ marginTop: '10px', paddingTop: '8px', borderTop: '1px solid rgba(255, 255, 255, 0.1)' }}>
              <div style={{ fontSize: '9px', color: '#64748b', marginBottom: '4px' }}>
                🎯 Filter:
              </div>
        <select
          value={filterType}
          onChange={(e) => setFilterType(e.target.value)}
          className="node-select"
          style={{
            width: '100%',
            background: 'rgba(0, 0, 0, 0.3)',
            border: '1px solid #6366f1',
            color: '#6366f1',
            fontSize: '10px'
          }}
        >
          <option value="any">🌍 Any {needsTextFilter ? 'message' : 'victim'}</option>
          {needsTextFilter && (
            <option value="text_contains">💬 Contains text</option>
          )}
          {needsPlayerFilter && (
            <>
              <option value="my_players">👤 My players (attacker)</option>
              <option value="my_clans">🏷️ My clans (attacker)</option>
              <option value="enemy_players">🎯 Enemy players (victim)</option>
              <option value="enemy_clans">☠️ Enemy clans (victim)</option>
            </>
          )}
        </select>
              
              {/* Text input for ChatMessage */}
              {needsTextFilter && filterType === 'text_contains' && (
                <input
                  type="text"
                  value={filterText}
                  onChange={(e) => setFilterText(e.target.value)}
                  placeholder="e.g. gg wp"
                  onClick={(e) => e.stopPropagation()}
                  onMouseDown={(e) => e.stopPropagation()}
                  style={{
                    width: '100%',
                    marginTop: '6px',
                    padding: '5px 8px',
                    background: 'rgba(0, 0, 0, 0.4)',
                    border: '1px solid #6366f1',
                    borderRadius: '4px',
                    color: '#fff',
                    fontSize: '11px',
                    outline: 'none'
                  }}
                />
              )}
              
        {(filterType === 'my_players' || filterType === 'my_clans') && (
          <div style={{ 
            fontSize: '8px', 
            color: '#64748b', 
            marginTop: '4px',
            padding: '4px',
            background: 'rgba(99, 102, 241, 0.1)',
            borderRadius: '3px'
          }}>
            💡 Uses names from Player Identity
          </div>
        )}
        
        {(filterType === 'enemy_players' || filterType === 'enemy_clans') && (
          <div style={{ 
            fontSize: '8px', 
            color: '#ef4444', 
            marginTop: '4px',
            padding: '4px',
            background: 'rgba(239, 68, 68, 0.1)',
            borderRadius: '3px'
          }}>
            🎯 Uses names from Enemy List
          </div>
        )}
            </div>
          )}
          
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
            {showFilter && filterType === 'text_contains' && filterText ? 
              `🎯 "${filterText}"` :
              showFilter && filterType !== 'any' ? 
                `🎯 ${filterType === 'my_players' ? 'My players' : 'My clans'}` : 
                `Triggers on: ${t(`game_events.${event}`, event)}`
            }
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

