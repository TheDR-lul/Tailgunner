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
  // Separate streams: chat and HUD
  const [chatMessages, setChatMessages] = useState<ChatMessage[]>([]);
  const [hudMessages, setHudMessages] = useState<ChatMessage[]>([]);
  const [lastChatId, setLastChatId] = useState<number>(0);
  const [lastEvtId, setLastEvtId] = useState<number>(0);
  const [lastDmgId, setLastDmgId] = useState<number>(0);
  const [isFeedEnabled, setIsFeedEnabled] = useState(false);
  const [isChatEnabled, setIsChatEnabled] = useState(false);
  const [lastBattleId, setLastBattleId] = useState<number>(0); // Track battle changes
  const [updateInterval, setUpdateInterval] = useState<number>(() => 
    parseInt(localStorage.getItem('feedUpdateInterval') || '500')
  );
  const chatContainerRef = useRef<HTMLDivElement>(null);
  const feedContainerRef = useRef<HTMLDivElement>(null);

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
    if (!isFeedEnabled && !isChatEnabled) return;

    const interval = setInterval(async () => {
      try {
        // Check if we're in a new battle (map_generation changed)
        const mapInfo = await invoke<{info: {map_generation: number}}>('get_map_data')
          .catch(() => null);
        
        if (mapInfo && mapInfo.info.map_generation !== lastBattleId) {
          // New battle detected - clear old messages
          console.log(`[GameChat] New battle detected (${lastBattleId} → ${mapInfo.info.map_generation}), clearing streams`);
          setChatMessages([]);
          setHudMessages([]);
          setLastChatId(0);
          setLastEvtId(0);
          setLastDmgId(0);
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
        
        // Process CHAT messages (separate stream)
        if (chat && Array.isArray(chat) && chat.length > 0) {
          console.log(`[GameChat] Received ${chat.length} new chat messages`);
          const newChat = chat.map(msg => ({ ...msg, type: 'chat' as const }));
          setChatMessages(prev => {
            // 🔍 FILTER OLD EVENTS: If receiving >5 messages at once after empty/first load → old events
            if (newChat.length > 5 && prev.length === 0) {
              console.warn(`[GameChat] ⏭️ Ignoring ${newChat.length} old chat messages (bulk load detected)`);
              // Update lastChatId but don't show messages
              setLastChatId(chat[chat.length - 1].id);
              return prev;
            }
            
            const combined = [...prev, ...newChat];
            combined.sort((a, b) => a.time - b.time);
            return combined.slice(-100); // Keep last 100
          });
          setLastChatId(chat[chat.length - 1].id);
          // Auto-scroll chat
          setTimeout(() => {
            if (chatContainerRef.current) {
              chatContainerRef.current.scrollTop = chatContainerRef.current.scrollHeight;
            }
          }, 0);
        }
        
        // Process HUD messages (separate stream)
        const newHud: ChatMessage[] = [];
        if (hud.events && hud.events.length > 0) {
          console.log(`[GameChat] Received ${hud.events.length} new event messages`);
          newHud.push(...hud.events.map(msg => ({ ...msg, type: 'event' as const })));
          setLastEvtId(hud.events[hud.events.length - 1].id);
        }
        if (hud.damage && hud.damage.length > 0) {
          console.log(`[GameChat] Received ${hud.damage.length} new damage messages`);
          newHud.push(...hud.damage.map(msg => ({ ...msg, type: 'damage' as const })));
          setLastDmgId(hud.damage[hud.damage.length - 1].id);
        }
        if (newHud.length > 0) {
          setHudMessages(prev => {
            // 🔍 FILTER OLD EVENTS: If receiving >5 messages at once after empty/first load → old events
            if (newHud.length > 5 && prev.length === 0) {
              console.warn(`[GameChat] ⏭️ Ignoring ${newHud.length} old HUD messages (bulk load detected)`);
              return prev; // Don't add them!
            }
            
            const combined = [...prev, ...newHud];
            combined.sort((a, b) => a.time - b.time);
            return combined.slice(-100); // Keep last 100
          });
          // Auto-scroll feed
          setTimeout(() => {
            if (feedContainerRef.current) {
              feedContainerRef.current.scrollTop = feedContainerRef.current.scrollHeight;
            }
          }, 0);
        }
      } catch (err) {
        // Silently ignore errors (game not running)
      }
    }, updateInterval); // Use configurable update interval

    return () => clearInterval(interval);
  }, [isFeedEnabled, isChatEnabled, lastChatId, lastEvtId, lastDmgId, lastBattleId, updateInterval]);

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <>
      {/* Game Feed (HUD Events) - Separate Block */}
      <div className="game-feed-container">
        <div className="game-chat-header">
          <h3>📢 Game Feed</h3>
          <button 
            className={`btn btn-${isFeedEnabled ? 'primary' : 'secondary'}`}
            onClick={() => {
              setIsFeedEnabled(!isFeedEnabled);
              if (!isFeedEnabled) {
                setHudMessages([]);
                setLastEvtId(0);
                setLastDmgId(0);
              }
            }}
          >
            {isFeedEnabled ? t('chat.enabled') : t('chat.disabled')}
          </button>
        </div>
        {isFeedEnabled && (
          <div className="game-chat-messages" ref={feedContainerRef}>
            {hudMessages.length === 0 ? (
              <div className="game-chat-empty">Waiting for messages...</div>
            ) : (
              hudMessages.map((msg, index) => (
                <div 
                  key={`hud-${msg.id}-${index}`}
                  className={`chat-message ${msg.type || 'system'} ${msg.enemy ? 'enemy' : 'ally'} ${msg.sender ? 'player' : 'system'}`}
                >
                  <span className="chat-time">{formatTime(msg.time)}</span>
                  <span className="chat-text">{msg.msg}</span>
                </div>
              ))
            )}
          </div>
        )}
      </div>
      
      {/* Game Chat (Chat Messages) - Separate Block */}
      <div className="game-chat-container">
        <div className="game-chat-header">
          <h3>💬 Game Chat</h3>
          <button 
            className={`btn btn-${isChatEnabled ? 'primary' : 'secondary'}`}
            onClick={() => {
              setIsChatEnabled(!isChatEnabled);
              if (!isChatEnabled) {
                setChatMessages([]);
                setLastChatId(0);
              }
            }}
          >
            {isChatEnabled ? t('chat.enabled') : t('chat.disabled')}
          </button>
        </div>
        {isChatEnabled && (
          <div className="game-chat-messages" ref={chatContainerRef}>
            {chatMessages.length === 0 ? (
              <div className="game-chat-empty">Waiting for messages...</div>
            ) : (
              chatMessages.map((msg, index) => (
                <div 
                  key={`chat-${msg.id}-${index}`}
                  className={`chat-message chat ${msg.enemy ? 'enemy' : 'ally'}`}
                >
                  <span className="chat-time">{formatTime(msg.time)}</span>
                  {msg.mode && <span className="chat-mode">[{msg.mode}]</span>}
                  {msg.sender && <span className="chat-sender">{msg.sender}:</span>}
                  <span className="chat-text">{msg.msg}</span>
                </div>
              ))
            )}
          </div>
        )}
      </div>
    </>
  );
}

