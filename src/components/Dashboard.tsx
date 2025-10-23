import { useState, useEffect } from 'react';
import { Play, Square, Zap } from 'lucide-react';
import { api } from '../api';

export function Dashboard() {
  const [isRunning, setIsRunning] = useState(false);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  useEffect(() => {
    checkStatus();
    const interval = setInterval(checkStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  async function checkStatus() {
    try {
      const running = await api.isRunning();
      setIsRunning(running);
    } catch (error) {
      console.error('Failed to check status:', error);
    }
  }

  async function handleStart() {
    setLoading(true);
    try {
      const result = await api.startEngine();
      setMessage(result);
      setIsRunning(true);
    } catch (error) {
      setMessage(`Ошибка: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleStop() {
    setLoading(true);
    try {
      const result = await api.stopEngine();
      setMessage(result);
      setIsRunning(false);
    } catch (error) {
      setMessage(`Ошибка: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleTest() {
    setLoading(true);
    try {
      await api.testVibration(0.5, 500);
      setMessage('Тестовая вибрация отправлена');
    } catch (error) {
      setMessage(`Ошибка: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="card-title">
          <Zap size={20} />
          Управление
        </h3>
        <p className="card-description">
          Запуск и остановка системы
        </p>
      </div>

      <div className="card-content">
        <div className="control-panel">
          <div className={`status-badge ${isRunning ? 'active' : 'inactive'}`}>
            <span className="status-dot"></span>
            {isRunning ? 'Система активна' : 'Система остановлена'}
          </div>

          <div className="control-buttons">
            {!isRunning ? (
              <button
                className="btn btn-primary"
                onClick={handleStart}
                disabled={loading}
              >
                <Play size={18} />
                Запустить
              </button>
            ) : (
              <button
                className="btn btn-danger"
                onClick={handleStop}
                disabled={loading}
              >
                <Square size={18} />
                Остановить
              </button>
            )}

            <button
              className="btn btn-secondary"
              onClick={handleTest}
              disabled={loading}
            >
              Тест вибрации
            </button>
          </div>

          {message && (
            <div style={{
              padding: '1rem',
              background: 'var(--bg-hover)',
              borderRadius: '0.5rem',
              fontSize: '0.875rem',
              color: 'var(--text-dim)',
            }}>
              {message}
            </div>
          )}

          <div style={{
            marginTop: '2rem',
            padding: '1rem',
            background: 'rgba(0, 212, 255, 0.05)',
            border: '1px solid var(--accent)',
            borderRadius: '0.75rem',
            fontSize: '0.875rem',
          }}>
            <h4 style={{ marginBottom: '0.5rem', color: 'var(--accent)' }}>
              ✓ EAC-Safe
            </h4>
            <p style={{ color: 'var(--text-dim)', fontSize: '0.8rem' }}>
              Приложение читает только публичные данные с localhost:8111.
              Никакой инъекции в память игры. Риск бана — ноль.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

