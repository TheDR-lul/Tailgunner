import { useState, useEffect } from 'react';
import { Activity, Download } from 'lucide-react';
import { api } from '../api';
import type { VibrationPattern } from '../types';

export function PatternEditor() {
  const [patterns, setPatterns] = useState<VibrationPattern[]>([]);
  const [selectedPattern, setSelectedPattern] = useState<VibrationPattern | null>(null);

  useEffect(() => {
    loadPatterns();
  }, []);

  async function loadPatterns() {
    try {
      const patternList = await api.getPresetPatterns();
      setPatterns(patternList);
      if (patternList.length > 0) {
        setSelectedPattern(patternList[0]);
      }
    } catch (error) {
      console.error('Failed to load patterns:', error);
    }
  }

  const curveLabels: Record<string, string> = {
    Linear: 'Линейная',
    EaseIn: 'Плавный вход',
    EaseOut: 'Плавный выход',
    EaseInOut: 'S-образная',
  };

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="card-title">
          <Activity size={20} />
          Визуальный Редактор Паттернов
        </h3>
        <p className="card-description">
          "Синтезатор Ощущений" - ADSR конструктор
        </p>
      </div>

      <div className="card-content">
        {/* Список пресетов */}
        <div style={{ marginBottom: '1.5rem' }}>
          <div className="slider-label">
            <span>Выберите пресет:</span>
          </div>
          <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
            {patterns.map((pattern, idx) => (
              <button
                key={idx}
                className={`btn ${selectedPattern?.name === pattern.name ? 'btn-primary' : 'btn-secondary'}`}
                onClick={() => setSelectedPattern(pattern)}
                style={{ flex: '1 1 auto', minWidth: '120px' }}
              >
                {pattern.name}
              </button>
            ))}
          </div>
        </div>

        {selectedPattern && (
          <>
            {/* Визуализация паттерна */}
            <div className="pattern-visualizer">
              <div className="pattern-grid"></div>
              <svg className="pattern-canvas" viewBox="0 0 400 100" preserveAspectRatio="none">
                <PatternVisualization pattern={selectedPattern} />
              </svg>
            </div>

            {/* Параметры ADSR */}
            <div style={{ display: 'grid', gap: '1rem' }}>
              <div>
                <h4 style={{ fontSize: '0.875rem', marginBottom: '0.75rem', color: 'var(--accent)' }}>
                  ⚡ Attack (Атака)
                </h4>
                <div className="slider-label">
                  <span>Длительность</span>
                  <span className="slider-value">{selectedPattern.attack.duration_ms}ms</span>
                </div>
                <div className="slider-label">
                  <span>Интенсивность</span>
                  <span className="slider-value">
                    {selectedPattern.attack.start_intensity.toFixed(2)} → {selectedPattern.attack.end_intensity.toFixed(2)}
                  </span>
                </div>
                <div className="slider-label">
                  <span>Кривая</span>
                  <span className="slider-value">{curveLabels[selectedPattern.attack.curve]}</span>
                </div>
              </div>

              <div>
                <h4 style={{ fontSize: '0.875rem', marginBottom: '0.75rem', color: 'var(--accent)' }}>
                  🔥 Hold (Удержание)
                </h4>
                <div className="slider-label">
                  <span>Длительность</span>
                  <span className="slider-value">{selectedPattern.hold.duration_ms}ms</span>
                </div>
                <div className="slider-label">
                  <span>Интенсивность</span>
                  <span className="slider-value">{selectedPattern.hold.start_intensity.toFixed(2)}</span>
                </div>
              </div>

              <div>
                <h4 style={{ fontSize: '0.875rem', marginBottom: '0.75rem', color: 'var(--accent)' }}>
                  💨 Decay (Затухание)
                </h4>
                <div className="slider-label">
                  <span>Длительность</span>
                  <span className="slider-value">{selectedPattern.decay.duration_ms}ms</span>
                </div>
                <div className="slider-label">
                  <span>Кривая</span>
                  <span className="slider-value">{curveLabels[selectedPattern.decay.curve]}</span>
                </div>
              </div>

              <div>
                <h4 style={{ fontSize: '0.875rem', marginBottom: '0.75rem', color: 'var(--accent)' }}>
                  🔁 Burst (Повторы)
                </h4>
                <div className="slider-label">
                  <span>Количество</span>
                  <span className="slider-value">{selectedPattern.burst.repeat_count + 1}x</span>
                </div>
                <div className="slider-label">
                  <span>Пауза</span>
                  <span className="slider-value">{selectedPattern.burst.pause_between_ms}ms</span>
                </div>
              </div>
            </div>

            <button className="btn btn-secondary" style={{ marginTop: '1.5rem' }}>
              <Download size={16} />
              Экспортировать паттерн
            </button>
          </>
        )}
      </div>
    </div>
  );
}

function PatternVisualization({ pattern }: { pattern: VibrationPattern }) {
  const totalDuration = pattern.attack.duration_ms + pattern.hold.duration_ms + pattern.decay.duration_ms;
  
  let points: Array<[number, number]> = [];
  
  // Attack
  const attackEnd = (pattern.attack.duration_ms / totalDuration) * 400;
  points.push([0, 100 - pattern.attack.start_intensity * 100]);
  points.push([attackEnd, 100 - pattern.attack.end_intensity * 100]);
  
  // Hold
  const holdEnd = attackEnd + (pattern.hold.duration_ms / totalDuration) * 400;
  points.push([holdEnd, 100 - pattern.hold.end_intensity * 100]);
  
  // Decay
  const decayEnd = holdEnd + (pattern.decay.duration_ms / totalDuration) * 400;
  points.push([decayEnd, 100 - pattern.decay.end_intensity * 100]);
  
  const pathData = points.map((p, i) => 
    i === 0 ? `M ${p[0]} ${p[1]}` : `L ${p[0]} ${p[1]}`
  ).join(' ');

  return (
    <>
      <path
        d={pathData}
        fill="none"
        stroke="url(#gradient)"
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <defs>
        <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor="var(--accent)" />
          <stop offset="100%" stopColor="#8b5cf6" />
        </linearGradient>
      </defs>
    </>
  );
}

