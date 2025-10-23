import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Play, Square, Zap, Shield } from 'lucide-react';
import { api } from '../api';

export function Dashboard() {
  const { t } = useTranslation();
  const [isRunning, setIsRunning] = useState(false);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        const running = await api.isRunning();
        setIsRunning(running);
      } catch (error) {
        // Тихо игнорируем
      }
    };
    
    checkStatus();
    const interval = setInterval(checkStatus, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleStart = async () => {
    try {
      await api.startEngine();
      setIsRunning(true);
      if ((window as any).debugLog) {
        (window as any).debugLog('success', '✅ Система запущена');
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Ошибка запуска: ${error.message}`);
      }
    }
  };

  const handleStop = async () => {
    try {
      await api.stopEngine();
      setIsRunning(false);
      if ((window as any).debugLog) {
        (window as any).debugLog('info', '⏹️ Система остановлена');
      }
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Ошибка остановки: ${error.message}`);
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
        (window as any).debugLog('error', `❌ Ошибка теста: ${errorMsg}`);
      }
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
      </div>
    </div>
  );
}
