import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { BarChart3, TrendingUp, Activity, Clock, RotateCcw } from 'lucide-react';
import { useAppStore } from '../store/useAppStore';
import { usePatterns } from '../hooks/PatternsProvider';
import './AnalyticsDashboard.css';

export function AnalyticsDashboard() {
  const { t } = useTranslation();
  const stats = useAppStore((state) => state.stats);
  const resetStats = useAppStore((state) => state.resetStats);
  const { patterns } = usePatterns();
  const [sessionDuration, setSessionDuration] = useState(0);

  // Update session duration every second
  useEffect(() => {
    const interval = setInterval(() => {
      const duration = Math.floor((Date.now() - stats.sessionStart) / 1000);
      setSessionDuration(duration);
    }, 1000);

    return () => clearInterval(interval);
  }, [stats.sessionStart]);

  const formatDuration = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) {
      return `${hours}h ${minutes}m ${secs}s`;
    } else if (minutes > 0) {
      return `${minutes}m ${secs}s`;
    } else {
      return `${secs}s`;
    }
  };

  const formatVibrationDuration = (ms: number): string => {
    if (ms < 1000) {
      return `${ms}ms`;
    }
    const seconds = Math.floor(ms / 1000);
    if (seconds < 60) {
      return `${seconds}s`;
    }
    const minutes = Math.floor(seconds / 60);
    return `${minutes}m ${seconds % 60}s`;
  };

  // Get top used patterns
  const topPatterns = Object.entries(stats.patternUsage)
    .sort(([, a], [, b]) => b - a)
    .slice(0, 5)
    .map(([id, count]) => {
      const pattern = patterns.find(p => p.id === id);
      return {
        id,
        name: pattern?.name || 'Unknown',
        count,
      };
    });

  const avgVibrationDuration = stats.totalVibrations > 0
    ? stats.totalDuration / stats.totalVibrations
    : 0;

  return (
    <div className="card analytics-dashboard">
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <BarChart3 size={20} />
          <h2>Analytics</h2>
        </div>
        <button
          className="btn btn-secondary btn-sm"
          onClick={resetStats}
          title="Reset Statistics"
        >
          <RotateCcw size={16} />
        </button>
      </div>

      <div className="card-body">
        {/* Session Stats */}
        <div className="stats-grid">
          <div className="stat-card">
            <div className="stat-icon" style={{ background: 'rgba(59, 130, 246, 0.1)' }}>
              <Clock size={20} style={{ color: '#3b82f6' }} />
            </div>
            <div className="stat-content">
              <div className="stat-label">Session Time</div>
              <div className="stat-value">{formatDuration(sessionDuration)}</div>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon" style={{ background: 'rgba(16, 185, 129, 0.1)' }}>
              <Activity size={20} style={{ color: '#10b981' }} />
            </div>
            <div className="stat-content">
              <div className="stat-label">Total Vibrations</div>
              <div className="stat-value">{stats.totalVibrations.toLocaleString()}</div>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon" style={{ background: 'rgba(139, 92, 246, 0.1)' }}>
              <TrendingUp size={20} style={{ color: '#8b5cf6' }} />
            </div>
            <div className="stat-content">
              <div className="stat-label">Avg Duration</div>
              <div className="stat-value">
                {formatVibrationDuration(avgVibrationDuration)}
              </div>
            </div>
          </div>

          <div className="stat-card">
            <div className="stat-icon" style={{ background: 'rgba(236, 72, 153, 0.1)' }}>
              <Activity size={20} style={{ color: '#ec4899' }} />
            </div>
            <div className="stat-content">
              <div className="stat-label">Total Duration</div>
              <div className="stat-value">
                {formatVibrationDuration(stats.totalDuration)}
              </div>
            </div>
          </div>
        </div>

        {/* Top Patterns */}
        {topPatterns.length > 0 && (
          <div className="top-patterns">
            <h3>Most Used Patterns</h3>
            <div className="pattern-usage-list">
              {topPatterns.map((pattern, index) => {
                const maxCount = topPatterns[0].count;
                const percentage = (pattern.count / maxCount) * 100;

                return (
                  <div key={pattern.id} className="pattern-usage-item">
                    <div className="pattern-rank">#{index + 1}</div>
                    <div className="pattern-info">
                      <div className="pattern-name">{pattern.name}</div>
                      <div className="pattern-bar-wrapper">
                        <div 
                          className="pattern-bar"
                          style={{ width: `${percentage}%` }}
                        />
                      </div>
                    </div>
                    <div className="pattern-count">{pattern.count}x</div>
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {stats.totalVibrations === 0 && (
          <div className="analytics-empty">
            <p>No data yet. Start using patterns to see analytics!</p>
          </div>
        )}
      </div>
    </div>
  );
}


