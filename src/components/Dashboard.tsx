import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Play, Square, Zap, Shield, Database, RefreshCw, FolderOpen } from 'lucide-react';
import { api } from '../api';

export function Dashboard() {
  const { t } = useTranslation();
  const [isRunning, setIsRunning] = useState(false);
  const [dbStats, setDbStats] = useState<[number, number, number] | null>(null);
  const [isParsingDb, setIsParsingDb] = useState(false);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        const running = await api.isRunning();
        setIsRunning(running);
      } catch (error) {
        // Silently ignore
      }
    };
    
    const checkDbStats = async () => {
      try {
        const stats = await api.datamineGetStats();
        setDbStats(stats);
      } catch (error) {
        // Silently ignore
      }
    };
    
    checkStatus();
    checkDbStats();
    const interval = setInterval(() => {
      checkStatus();
      checkDbStats();
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleStart = async () => {
    try {
      await api.startEngine();
      setIsRunning(true);
      if ((window as any).debugLog) {
        (window as any).debugLog('success', '✅ System started');
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Start error: ${error.message}`);
      }
    }
  };

  const handleStop = async () => {
    try {
      await api.stopEngine();
      setIsRunning(false);
      if ((window as any).debugLog) {
        (window as any).debugLog('info', '⏹️ System stopped');
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Stop error: ${error.message}`);
      }
    }
  };

  const handleTest = async () => {
    try {
      const result = await api.testVibration(0.8, 500);
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `⚡ ${result}`);
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        const errorMsg = typeof error === 'string' ? error : (error?.message || JSON.stringify(error));
        (window as any).debugLog('error', `❌ Test error: ${errorMsg}`);
      }
    }
  };

  const handleInitDatabase = async () => {
    try {
      setIsParsingDb(true);
      if ((window as any).debugLog) {
        (window as any).debugLog('info', '🔄 Building vehicle database...');
      }
      
      const result = await api.datamineAutoInit();
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `✅ ${result}`);
      }
      
      // Refresh stats
      const stats = await api.datamineGetStats();
      setDbStats(stats);
    } catch (error: any) {
      if ((window as any).debugLog) {
        const errorMsg = typeof error === 'string' ? error : (error?.message || JSON.stringify(error));
        (window as any).debugLog('error', `❌ Auto-detect failed: ${errorMsg}`);
        (window as any).debugLog('info', '💡 Try manual selection below');
      }
    } finally {
      setIsParsingDb(false);
    }
  };

  const handleRebuildDatabase = async () => {
    try {
      setIsParsingDb(true);
      if ((window as any).debugLog) {
        (window as any).debugLog('info', '🔄 Force rebuilding database...');
      }
      
      const stats = await api.datamineRebuild();
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', 
          `✅ Database rebuilt: ${stats.aircraft_count} aircraft, ${stats.ground_count} ground, ${stats.ships_count} ships`);
      }
      
      // Refresh stats
      setDbStats([stats.aircraft_count, stats.ground_count, stats.ships_count]);
    } catch (error: any) {
      if ((window as any).debugLog) {
        const errorMsg = typeof error === 'string' ? error : (error?.message || JSON.stringify(error));
        (window as any).debugLog('error', `❌ Rebuild failed: ${errorMsg}`);
      }
    } finally {
      setIsParsingDb(false);
    }
  };

  const handleManualSelect = async () => {
    try {
      if ((window as any).debugLog) {
        (window as any).debugLog('info', '📁 Select War Thunder folder...');
      }
      
      const selectedPath = await api.selectFolder();
      
      if (!selectedPath) {
        if ((window as any).debugLog) {
          (window as any).debugLog('warn', '⚠️ No folder selected');
        }
        return;
      }
      
      setIsParsingDb(true);
      if ((window as any).debugLog) {
        (window as any).debugLog('info', `🔄 Parsing from: ${selectedPath}`);
      }
      
      const stats = await api.datamineParse(selectedPath);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', 
          `✅ Database built: ${stats.aircraft_count} aircraft, ${stats.ground_count} ground, ${stats.ships_count} ships`);
      }
      
      // Refresh stats
      const newStats = await api.datamineGetStats();
      setDbStats(newStats);
    } catch (error: any) {
      if ((window as any).debugLog) {
        const errorMsg = typeof error === 'string' ? error : (error?.message || JSON.stringify(error));
        (window as any).debugLog('error', `❌ Parse error: ${errorMsg}`);
        (window as any).debugLog('info', '💡 Make sure you selected the War Thunder root folder (contains aces.exe)');
      }
    } finally {
      setIsParsingDb(false);
    }
  };

  return (
    <div className="card dashboard-card">
      <div className="card-header">
        <h2>⚡ {t('dashboard.title')}</h2>
        <p>{t('dashboard.description')}</p>
      </div>

      <div className="card-body">
        <div className="control-buttons">
          {!isRunning ? (
            <button className="btn btn-primary btn-lg" onClick={handleStart}>
              <Play size={20} />
              {t('dashboard.btn_start')}
            </button>
          ) : (
            <button className="btn btn-danger btn-lg" onClick={handleStop}>
              <Square size={20} />
              {t('dashboard.btn_stop')}
            </button>
          )}
          
          <button className="btn btn-secondary" onClick={handleTest}>
            <Zap size={18} />
            {t('dashboard.btn_test')}
          </button>
        </div>

        <div className="safety-badge">
          <Shield size={20} className="safety-icon" />
          <div className="safety-text">
            <strong>{t('dashboard.eac_safe_title')}</strong>
            <small>{t('dashboard.eac_safe_desc')}</small>
          </div>
        </div>

        {/* Vehicle Database Section */}
        <div style={{ 
          marginTop: '20px', 
          padding: '16px', 
          background: 'var(--bg-secondary)', 
          borderRadius: 'var(--radius)',
          border: '1px solid var(--border)'
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
            <Database size={18} style={{ color: 'var(--primary)' }} />
            <strong style={{ fontSize: '14px' }}>Vehicle Database</strong>
          </div>
          
          {dbStats && (dbStats[0] > 0 || dbStats[1] > 0 || dbStats[2] > 0) ? (
            <div>
              <div style={{ 
                display: 'grid', 
                gridTemplateColumns: 'repeat(3, 1fr)', 
                gap: '12px',
                marginBottom: '12px'
              }}>
                <div style={{ textAlign: 'center', padding: '8px', background: 'rgba(99, 102, 241, 0.1)', borderRadius: '6px' }}>
                  <div style={{ fontSize: '20px', fontWeight: 'bold', color: 'var(--primary)' }}>
                    {dbStats[0]}
                  </div>
                  <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>✈️ Aircraft</div>
                </div>
                <div style={{ textAlign: 'center', padding: '8px', background: 'rgba(34, 197, 94, 0.1)', borderRadius: '6px' }}>
                  <div style={{ fontSize: '20px', fontWeight: 'bold', color: '#22c55e' }}>
                    {dbStats[1]}
                  </div>
                  <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>🚜 Ground</div>
                </div>
                <div style={{ textAlign: 'center', padding: '8px', background: 'rgba(59, 130, 246, 0.1)', borderRadius: '6px' }}>
                  <div style={{ fontSize: '20px', fontWeight: 'bold', color: '#3b82f6' }}>
                    {dbStats[2]}
                  </div>
                  <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>⚓ Ships</div>
                </div>
              </div>
              
              {/* Info about lazy loading */}
              <div style={{ 
                marginBottom: '12px', 
                padding: '8px 12px', 
                background: 'rgba(20, 184, 166, 0.1)', 
                borderRadius: '6px',
                border: '1px solid rgba(20, 184, 166, 0.2)',
                fontSize: '11px',
                color: 'var(--text-muted)',
                lineHeight: 1.4
              }}>
                <span style={{ color: '#14b8a6', fontWeight: 600 }}>🌐 Smart Loading:</span> Missing data (like tank speed) will be fetched from Wiki automatically when you select a vehicle in-game and cached for future use.
              </div>
              
              <button 
                className="btn btn-secondary btn-sm" 
                onClick={handleRebuildDatabase}
                disabled={isParsingDb}
                style={{ width: '100%' }}
              >
                <RefreshCw size={16} className={isParsingDb ? 'spin' : ''} />
                {isParsingDb ? 'Rebuilding...' : 'Rebuild Database'}
              </button>
            </div>
          ) : (
            <div>
              <div style={{ 
                textAlign: 'center', 
                padding: '20px', 
                color: 'var(--text-muted)',
                fontSize: '13px'
              }}>
                <Database size={48} style={{ opacity: 0.3, marginBottom: '8px' }} />
                <div>Database is empty</div>
                <div style={{ fontSize: '11px', marginTop: '4px' }}>
                  Build from War Thunder files to show vehicle data
                </div>
              </div>
              
              <button 
                className="btn btn-primary btn-sm" 
                onClick={handleInitDatabase}
                disabled={isParsingDb}
                style={{ width: '100%', marginBottom: '8px' }}
              >
                <Database size={16} className={isParsingDb ? 'spin' : ''} />
                {isParsingDb ? 'Building Database...' : 'Auto-Build Database'}
              </button>
              <button 
                className="btn btn-secondary btn-sm" 
                onClick={handleManualSelect}
                disabled={isParsingDb}
                style={{ width: '100%' }}
              >
                <FolderOpen size={16} />
                Select Folder Manually
              </button>
              <div style={{ 
                marginTop: '8px', 
                fontSize: '10px', 
                color: 'var(--text-muted)',
                textAlign: 'center'
              }}>
                If auto-detect fails, use manual selection
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
