import { useState, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { Terminal, Trash2, Download } from 'lucide-react';

interface LogEntry {
  id: string;
  timestamp: Date;
  level: 'info' | 'warn' | 'error' | 'success';
  message: string;
}

export function DebugConsole() {
  const { t } = useTranslation();
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isExpanded, setIsExpanded] = useState(false);
  const [updateInterval, setUpdateInterval] = useState(() => 
    parseInt(localStorage.getItem('gameStatusUpdateInterval') || '200')
  );
  const logsEndRef = useRef<HTMLDivElement>(null);
  const logTimeoutRef = useRef<number | null>(null);
  const lastLoggedTriggers = useRef<Set<string>>(new Set());

  const addLog = (level: LogEntry['level'], message: string) => {
    const newLog: LogEntry = {
      id: Date.now().toString(),
      timestamp: new Date(),
      level,
      message,
    };
    setLogs(prev => [...prev, newLog]);
  };

  const handleIntervalChange = (value: number) => {
    setUpdateInterval(value);
    localStorage.setItem('gameStatusUpdateInterval', value.toString());
    
    // Debounce logging - only log after user stops dragging
    if (logTimeoutRef.current) {
      clearTimeout(logTimeoutRef.current);
    }
    logTimeoutRef.current = setTimeout(() => {
      addLog('info', `⚙️ Update rate: ${value}ms (${(1000/value).toFixed(1)} Hz)`);
    }, 500); // Log only after 500ms of no changes
  };

  const clearLogs = () => {
    setLogs([]);
  };

  const exportLogs = () => {
    const logsText = logs.map(log => 
      `[${log.timestamp.toLocaleTimeString()}] [${log.level.toUpperCase()}] ${log.message}`
    ).join('\n');
    
    const blob = new Blob([logsText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `debug-log-${Date.now()}.txt`;
    a.click();
  };

  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (logTimeoutRef.current) {
        clearTimeout(logTimeoutRef.current);
      }
    };
  }, []);

  // Expose addLog globally for debugging
  useEffect(() => {
    (window as any).debugLog = addLog;
    
    // Initial log
    addLog('info', t('debug.initialized'));
    
    return () => {
      delete (window as any).debugLog;
    };
  }, [t]);

  // Periodically fetch debug info from Rust
  useEffect(() => {
    const fetchDebugInfo = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const info: any = await invoke('get_debug_info');
        
        // Log recent trigger events
        if (info.recent_triggers && info.recent_triggers.length > 0) {
          const currentBatch = new Set<string>();
          
          info.recent_triggers.forEach((trigger: any) => {
            const triggerId = `${trigger.trigger_name}-${trigger.timestamp}-${trigger.entity}`;
            currentBatch.add(triggerId);
            
            // Only log if not already logged
            if (!lastLoggedTriggers.current.has(triggerId)) {
              const logMessage = `🎯 ${trigger.trigger_name}: ${trigger.event_type}${trigger.entity ? ` (${trigger.entity})` : ''}`;
              addLog('success', logMessage);
              lastLoggedTriggers.current.add(triggerId);
            }
          });
          
          // Clean up old entries (keep only last 20)
          if (lastLoggedTriggers.current.size > 20) {
            const entries = Array.from(lastLoggedTriggers.current);
            lastLoggedTriggers.current = new Set(entries.slice(-20));
          }
        }
      } catch (error) {
        // Ignore errors if engine is not running
      }
    };

    const interval = setInterval(fetchDebugInfo, 1000); // Check every second
    return () => clearInterval(interval);
  }, []);

  return (
    <div className={`debug-console ${isExpanded ? 'expanded' : ''}`}>
      <div className="debug-header" onClick={() => setIsExpanded(!isExpanded)}>
        <div className="debug-title">
          <Terminal size={18} />
          <span>{t('debug.title')}</span>
          <span className="log-count">({logs.length})</span>
        </div>
        <div className="debug-actions">
          <button className="btn-icon" onClick={(e) => { e.stopPropagation(); exportLogs(); }}>
            <Download size={16} />
          </button>
          <button className="btn-icon" onClick={(e) => { e.stopPropagation(); clearLogs(); }}>
            <Trash2 size={16} />
          </button>
        </div>
      </div>

      {isExpanded && (
        <div className="debug-body">
          {/* Settings Section */}
          <div style={{ 
            borderBottom: '1px solid var(--border)', 
            marginBottom: '12px', 
            paddingBottom: '12px',
            background: 'rgba(255, 153, 51, 0.05)',
            padding: '10px',
            borderRadius: '4px'
          }}>
            <div style={{ 
              fontSize: '11px', 
              color: '#ff9933', 
              fontWeight: 'bold', 
              marginBottom: '8px',
              display: 'flex',
              alignItems: 'center',
              gap: '6px'
            }}>
              ⚙️ Update Rate
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
              <input
                type="range"
                min="50"
                max="1000"
                step="50"
                value={updateInterval}
                onChange={(e) => handleIntervalChange(parseInt(e.target.value))}
                style={{ flex: 1 }}
              />
              <span style={{ 
                fontSize: '11px', 
                color: 'var(--text-secondary)', 
                minWidth: '100px',
                textAlign: 'right'
              }}>
                {updateInterval}ms ({(1000/updateInterval).toFixed(1)} Hz)
              </span>
            </div>
            <div style={{ 
              fontSize: '10px', 
              color: 'var(--text-muted)', 
              marginTop: '6px'
            }}>
              Lower = faster updates, Higher = less CPU usage
            </div>
          </div>

          <div className="log-entry log-info" style={{ borderBottom: '1px solid var(--border)', marginBottom: '8px', paddingBottom: '8px' }}>
            <span 
              className="log-message"
              dangerouslySetInnerHTML={{ __html: t('debug.hint') }}
            />
          </div>
          {logs.map((log) => (
            <div key={log.id} className={`log-entry log-${log.level}`}>
              <span className="log-time">{log.timestamp.toLocaleTimeString()}</span>
              <span className="log-level">[{log.level.toUpperCase()}]</span>
              <span className="log-message">{log.message}</span>
            </div>
          ))}
          <div ref={logsEndRef} />
        </div>
      )}
    </div>
  );
}

