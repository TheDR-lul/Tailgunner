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
  const logsEndRef = useRef<HTMLDivElement>(null);

  const addLog = (level: LogEntry['level'], message: string) => {
    const newLog: LogEntry = {
      id: Date.now().toString(),
      timestamp: new Date(),
      level,
      message,
    };
    setLogs(prev => [...prev, newLog]);
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

  // Expose addLog globally for debugging
  useEffect(() => {
    (window as any).debugLog = addLog;
    
    // Initial log
    addLog('info', t('debug.initialized'));
    
    return () => {
      delete (window as any).debugLog;
    };
  }, [t]);

  // Периодически получаем debug info из Rust
  useEffect(() => {
    const fetchDebugInfo = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const info = await invoke('get_debug_info');
        // Логи из Rust можно выводить в терминале
        // console.log('[Rust Debug]', info);
      } catch (error) {
        // Игнорируем ошибки если движок не запущен
      }
    };

    const interval = setInterval(fetchDebugInfo, 2000);
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
          <div className="log-entry log-info" style={{ borderBottom: '1px solid var(--border)', marginBottom: '8px', paddingBottom: '8px' }}>
            <span className="log-message">
              💡 <strong>Rust логи (триггеры, парсер)</strong> → смотри в терминале!  
              Для детальных логов запусти: <code style={{background: 'rgba(0,0,0,0.3)', padding: '2px 6px', borderRadius: '4px'}}>RUST_LOG=debug npm run tauri dev</code>
            </span>
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

