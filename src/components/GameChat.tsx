import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import './GameChat.css';

interface ChatMessage {
  id: number;
  msg: string;
  sender: string;
  enemy: boolean;
  mode: string;
  time: number;
  type?: 'chat' | 'event' | 'damage'; // Added type to distinguish message sources
}

export function GameChat() {
  const { t } = useTranslation();
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [lastChatId, setLastChatId] = useState<number>(0);
  const [lastEvtId, setLastEvtId] = useState<number>(0);
  const [lastDmgId, setLastDmgId] = useState<number>(0);
  const [isEnabled, setIsEnabled] = useState(false);
  const [lastBattleId, setLastBattleId] = useState<number>(0); // Track battle changes
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isEnabled) return;

    const interval = setInterval(async () => {
      try {
        // Check if we're in a new battle (map_generation changed)
        const mapInfo = await invoke<{info: {map_generation: number}}>('get_map_data')
          .catch(() => null);
        
        if (mapInfo && mapInfo.info.map_generation !== lastBattleId) {
          // New battle detected - clear old messages
          if (lastBattleId !== 0) {
            console.log('[GameChat] New battle detected, clearing feed');
            setMessages([]);
            setLastChatId(0);
            setLastEvtId(0);
            setLastDmgId(0);
          }
          setLastBattleId(mapInfo.info.map_generation);
        }
        
        // Fetch both chat and HUD messages
        const [chat, hud] = await Promise.all([
          invoke<ChatMessage[]>('get_game_chat', { 
            lastId: lastChatId > 0 ? lastChatId : null 
          }).catch(() => []),
          invoke<{ events: ChatMessage[], damage: ChatMessage[] }>('get_hud_messages', {
            lastEvtId: lastEvtId > 0 ? lastEvtId : null,
            lastDmgId: lastDmgId > 0 ? lastDmgId : null,
          }).catch(() => ({ events: [], damage: [] }))
        ]);
        
        const newMessages: ChatMessage[] = [];
        
        // Add chat messages
        if (chat && Array.isArray(chat) && chat.length > 0) {
          newMessages.push(...chat.map(msg => ({ ...msg, type: 'chat' as const })));
          setLastChatId(chat[chat.length - 1].id);
        }
        
        // Add HUD event messages
        if (hud.events && hud.events.length > 0) {
          newMessages.push(...hud.events.map(msg => ({ ...msg, type: 'event' as const })));
          setLastEvtId(hud.events[hud.events.length - 1].id);
        }
        
        // Add HUD damage messages (kills, hits, etc)
        if (hud.damage && hud.damage.length > 0) {
          newMessages.push(...hud.damage.map(msg => ({ ...msg, type: 'damage' as const })));
          setLastDmgId(hud.damage[hud.damage.length - 1].id);
        }
        
        if (newMessages.length > 0) {
          setMessages(prev => {
            // Combine and sort by time
            const combined = [...prev, ...newMessages];
            combined.sort((a, b) => a.time - b.time);
            return combined.slice(-100); // Keep last 100 messages
          });
          messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
        }
      } catch (err) {
        // Silently ignore errors (game not running)
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [isEnabled, lastChatId, lastEvtId, lastDmgId, lastBattleId]);

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="game-chat-container">
      <div className="game-chat-header">
        <h3>{t('chat.title')}</h3>
        <button 
          className={`btn btn-${isEnabled ? 'primary' : 'secondary'}`}
          onClick={() => {
            setIsEnabled(!isEnabled);
            if (!isEnabled) {
              setMessages([]);
              setLastChatId(0);
              setLastEvtId(0);
              setLastDmgId(0);
            }
          }}
        >
          {isEnabled ? t('chat.enabled') : t('chat.disabled')}
        </button>
      </div>
      {isEnabled && (
        <div className="game-chat-messages">
          {messages.length === 0 ? (
            <div className="game-chat-empty">{t('chat.waiting')}</div>
          ) : (
            messages.map((msg, index) => (
              <div 
                key={`${msg.type}-${msg.id}-${index}`}
                className={`chat-message ${msg.type || 'system'} ${msg.enemy ? 'enemy' : 'ally'} ${msg.sender ? 'player' : 'system'}`}
              >
                <span className="chat-time">{formatTime(msg.time)}</span>
                {msg.mode && <span className="chat-mode">[{msg.mode}]</span>}
                {msg.sender && <span className="chat-sender">{msg.sender}:</span>}
                <span className="chat-text">{msg.msg}</span>
              </div>
            ))
          )}
          <div ref={messagesEndRef} />
        </div>
      )}
    </div>
  );
}

