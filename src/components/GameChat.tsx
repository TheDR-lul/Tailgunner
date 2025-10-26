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
  const [updateInterval, setUpdateInterval] = useState<number>(() => 
    parseInt(localStorage.getItem('feedUpdateInterval') || '500')
  );
  const messagesContainerRef = useRef<HTMLDivElement>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Listen for localStorage changes from DebugConsole
  useEffect(() => {
    const handleStorageChange = () => {
      const newInterval = parseInt(localStorage.getItem('feedUpdateInterval') || '500');
      setUpdateInterval(newInterval);
    };
    
    window.addEventListener('localStorageChange', handleStorageChange);
    return () => window.removeEventListener('localStorageChange', handleStorageChange);
  }, []);

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
            console.log(`[GameChat] New battle detected (${lastBattleId} → ${mapInfo.info.map_generation}), clearing feed`);
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
          }).catch((e) => {
            console.log('[GameChat] get_game_chat error:', e);
            return [];
          }),
          invoke<{ events: ChatMessage[], damage: ChatMessage[] }>('get_hud_messages', {
            lastEvtId: lastEvtId > 0 ? lastEvtId : null,
            lastDmgId: lastDmgId > 0 ? lastDmgId : null,
          }).catch((e) => {
            console.log('[GameChat] get_hud_messages error:', e);
            return { events: [], damage: [] };
          })
        ]);
        
        const newMessages: ChatMessage[] = [];
        
        // Add chat messages
        if (chat && Array.isArray(chat) && chat.length > 0) {
          console.log(`[GameChat] Received ${chat.length} new chat messages`);
          newMessages.push(...chat.map(msg => ({ ...msg, type: 'chat' as const })));
          setLastChatId(chat[chat.length - 1].id);
        }
        
        // Add HUD event messages
        if (hud.events && hud.events.length > 0) {
          console.log(`[GameChat] Received ${hud.events.length} new event messages`);
          newMessages.push(...hud.events.map(msg => ({ ...msg, type: 'event' as const })));
          setLastEvtId(hud.events[hud.events.length - 1].id);
        }
        
        // Add HUD damage messages (kills, hits, etc)
        if (hud.damage && hud.damage.length > 0) {
          console.log(`[GameChat] Received ${hud.damage.length} new damage messages`);
          newMessages.push(...hud.damage.map(msg => ({ ...msg, type: 'damage' as const })));
          setLastDmgId(hud.damage[hud.damage.length - 1].id);
        }
        
        if (newMessages.length > 0) {
          console.log(`[GameChat] Adding ${newMessages.length} new messages to feed`);
          setMessages(prev => {
            // Check if battle restarted: new message time < last message time
            if (prev.length > 0 && newMessages.length > 0) {
              const lastOldTime = prev[prev.length - 1].time;
              const firstNewTime = Math.min(...newMessages.map(m => m.time));
              
              if (firstNewTime < lastOldTime) {
                console.log(`[GameChat] Battle restart detected! Time reset: ${lastOldTime} → ${firstNewTime}. Clearing feed.`);
                return newMessages.slice().sort((a, b) => a.time - b.time).slice(-100);
              }
            }
            
            // Normal case: combine and sort by time
            const combined = [...prev, ...newMessages];
            combined.sort((a, b) => a.time - b.time);
            const result = combined.slice(-100); // Keep last 100 messages
            console.log(`[GameChat] Total messages in feed: ${result.length}`);
            return result;
          });
          // Scroll only the messages container to the bottom
          setTimeout(() => {
            if (messagesContainerRef.current) {
              messagesContainerRef.current.scrollTop = messagesContainerRef.current.scrollHeight;
            }
          }, 0);
        }
      } catch (err) {
        // Silently ignore errors (game not running)
      }
    }, updateInterval); // Use configurable update interval

    return () => clearInterval(interval);
  }, [isEnabled, lastChatId, lastEvtId, lastDmgId, lastBattleId, updateInterval]);

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
        <div className="game-chat-messages" ref={messagesContainerRef}>
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

