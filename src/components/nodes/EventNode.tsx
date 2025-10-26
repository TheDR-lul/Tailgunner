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
    // === COMBAT EVENTS (HUD-based detection) ===
    { id: 'Hit', icon: Target, color: '#f59e0b', category: 'Combat' },
    { id: 'CriticalHit', icon: Flame, color: '#ef4444', category: 'Combat' },
    { id: 'TargetHit', icon: Target, color: '#f97316', category: 'Combat' },
    { id: 'TargetDestroyed', icon: Skull, color: '#22c55e', category: 'Combat' },
    { id: 'TargetSetOnFire', icon: Flame, color: '#fb923c', category: 'Combat' },
    { id: 'TargetSeverelyDamaged', icon: AlertTriangle, color: '#dc2626', category: 'Combat' },
    { id: 'AircraftDestroyed', icon: Target, color: '#991b1b', category: 'Combat' },
    { id: 'ShipDestroyed', icon: Skull, color: '#0891b2', category: 'Combat' },
    { id: 'TankDestroyed', icon: Skull, color: '#65a30d', category: 'Combat' },
    { id: 'VehicleDestroyed', icon: Skull, color: '#71717a', category: 'Combat' },
    
    // === FLIGHT (HUD events) ===
    { id: 'Crashed', icon: AlertTriangle, color: '#991b1b', category: 'Flight' },
    
    // === ENGINE (HUD + indicators) ===
    { id: 'EngineRunning', icon: Zap, color: '#22c55e', category: 'Engine' },
    { id: 'EngineOverheat', icon: Flame, color: '#ef4444', category: 'Engine' },
    { id: 'OilOverheated', icon: Droplet, color: '#f97316', category: 'Engine' },
    { id: 'Shooting', icon: Target, color: '#fbbf24', category: 'Engine' },
    
    // === CREW (indicators) ===
    { id: 'CrewKnocked', icon: Heart, color: '#dc2626', category: 'Crew' },
    
    // === MISSION (/mission.json) ===
    { id: 'MissionObjectiveCompleted', icon: Award, color: '#22c55e', category: 'Mission' },
    { id: 'MissionFailed', icon: AlertTriangle, color: '#dc2626', category: 'Mission' },
    { id: 'MissionSuccess', icon: Award, color: '#10b981', category: 'Mission' },
    
    // === MULTIPLAYER (HUD + /gamechat) ===
    { id: 'Achievement', icon: Award, color: '#fbbf24', category: 'Multiplayer' },
  { id: 'ChatMessage', icon: MessageSquare, color: '#6366f1', category: 'Multiplayer' },
  { id: 'TeamChatMessage', icon: MessageSquare, color: '#22c55e', category: 'Multiplayer' },
  { id: 'AllChatMessage', icon: MessageSquare, color: '#f97316', category: 'Multiplayer' },
  { id: 'SquadChatMessage', icon: MessageSquare, color: '#8b5cf6', category: 'Multiplayer' },
  { id: 'EnemyChatMessage', icon: MessageSquare, color: '#ef4444', category: 'Multiplayer' },
  { id: 'FirstStrike', icon: Zap, color: '#8b5cf6', category: 'Multiplayer' },
    { id: 'ShipRescuer', icon: Heart, color: '#0891b2', category: 'Multiplayer' },
    { id: 'Assist', icon: Target, color: '#eab308', category: 'Multiplayer' },
    { id: 'BaseCapture', icon: Flame, color: '#22c55e', category: 'Multiplayer' },
    { id: 'TeamKill', icon: AlertTriangle, color: '#dc2626', category: 'Multiplayer' },
    { id: 'PlayerDisconnected', icon: AlertTriangle, color: '#64748b', category: 'Multiplayer' },
  ];
  
  const selectedEvent = EVENTS.find(e => e.id === event) || EVENTS[0];
  const EventIcon = selectedEvent.icon;
  
  // Determine if this event needs filtering
  const needsTextFilter = ['ChatMessage'].includes(event);
  const needsPlayerFilter = [
    'TargetDestroyed', 'TargetSetOnFire', 'TargetSeverelyDamaged', 'AircraftDestroyed',
    'ShipDestroyed', 'TankDestroyed', 'VehicleDestroyed',
    'Hit', 'CriticalHit', 'Achievement'
  ].includes(event);
  const showFilter = needsTextFilter || needsPlayerFilter;
  
  // Context-dependent filter labels
  const isKillEvent = [
    'TargetDestroyed', 'TargetSetOnFire', 'TargetSeverelyDamaged', 
    'AircraftDestroyed', 'ShipDestroyed', 'TankDestroyed', 'VehicleDestroyed'
  ].includes(event);
  const isDamageEvent = ['Hit', 'CriticalHit'].includes(event);
  
  // Get filter description based on event type
  const getFilterLabel = (filterValue: string) => {
    if (isKillEvent) {
      // For kill events: YOU are the attacker, entity_name is victim
      switch (filterValue) {
        case 'any': return '🌍 Any kill (anyone)';
        case 'my_players': return '✅ I killed someone';
        case 'my_clans': return '🏷️ My clan killed someone';
        case 'enemy_players': return '🎯 I killed tracked enemy';
        case 'enemy_clans': return '☠️ I killed enemy clan member';
        default: return filterValue;
      }
    } else if (isDamageEvent) {
      // For damage events: entity_name is the attacker, YOU are victim
      switch (filterValue) {
        case 'any': return '🌍 Anyone damaged me';
        case 'my_players': return '👤 I damaged myself?'; // Edge case
        case 'my_clans': return '🏷️ My clan damaged me?'; // Edge case
        case 'enemy_players': return '🎯 Tracked enemy damaged me';
        case 'enemy_clans': return '☠️ Enemy clan damaged me';
        default: return filterValue;
      }
    } else {
      // Generic labels
      return filterValue;
    }
  };
  
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
                {isKillEvent && (
                  <div style={{ fontSize: '7px', color: '#94a3b8', marginTop: '2px' }}>
                    💡 To trigger when <strong>YOU are killed</strong>, use "ShotDown" or "TakingDamage" event instead
                  </div>
                )}
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
          <option value="any">{getFilterLabel('any')}</option>
          {needsTextFilter && (
            <option value="text_contains">💬 Contains text</option>
          )}
          {needsPlayerFilter && (
            <>
              <option value="my_players">{getFilterLabel('my_players')}</option>
              <option value="my_clans">{getFilterLabel('my_clans')}</option>
              <option value="enemy_players">{getFilterLabel('enemy_players')}</option>
              <option value="enemy_clans">{getFilterLabel('enemy_clans')}</option>
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
            {isKillEvent ? (
              <div>
                💡 <strong>How it works:</strong><br/>
                • Set your nickname in <strong>Player Identity</strong><br/>
                • Fires when <strong>YOU</strong> kill anyone
              </div>
            ) : isDamageEvent ? (
              <div>
                💡 <strong>How it works:</strong><br/>
                • Set your nickname in <strong>Player Identity</strong><br/>
                • Fires when attacker matches your name (rare)
              </div>
            ) : (
              '💡 Uses names from Player Identity'
            )}
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
            {isKillEvent ? (
              <div>
                🎯 <strong>How it works:</strong><br/>
                • Add enemies in <strong>Player Identity → Enemy List</strong><br/>
                • Fires when <strong>YOU</strong> kill tracked enemy
              </div>
            ) : isDamageEvent ? (
              <div>
                🎯 <strong>How it works:</strong><br/>
                • Add enemies in <strong>Player Identity → Enemy List</strong><br/>
                • Fires when tracked enemy damages <strong>YOU</strong>
              </div>
            ) : (
              '🎯 Uses names from Enemy List'
            )}
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
                `🎯 ${getFilterLabel(filterType).replace(/^[🌍✅🎯☠️🏷️👤] /, '')}` : 
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

