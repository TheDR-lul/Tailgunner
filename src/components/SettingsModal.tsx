import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { X, Globe, Gauge } from 'lucide-react';

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsModal({ isOpen, onClose }: SettingsModalProps) {
  const { t, i18n } = useTranslation();
  
  // Update intervals state
  const [updateInterval, setUpdateInterval] = useState(() => 
    parseInt(localStorage.getItem('gameStatusUpdateInterval') || '200')
  );
  const [mapUpdateInterval, setMapUpdateInterval] = useState(() => 
    parseInt(localStorage.getItem('mapUpdateInterval') || '200')
  );
  const [feedUpdateInterval, setFeedUpdateInterval] = useState(() => 
    parseInt(localStorage.getItem('feedUpdateInterval') || '500')
  );

  // Listen for external changes
  useEffect(() => {
    const handleStorageChange = () => {
      setUpdateInterval(parseInt(localStorage.getItem('gameStatusUpdateInterval') || '200'));
      setMapUpdateInterval(parseInt(localStorage.getItem('mapUpdateInterval') || '200'));
      setFeedUpdateInterval(parseInt(localStorage.getItem('feedUpdateInterval') || '500'));
    };
    
    window.addEventListener('localStorageChange', handleStorageChange);
    return () => window.removeEventListener('localStorageChange', handleStorageChange);
  }, []);

  if (!isOpen) return null;

  const handleLanguageChange = (lang: string) => {
    i18n.changeLanguage(lang);
    localStorage.setItem('language', lang);
  };

  const handleIntervalChange = (value: number) => {
    setUpdateInterval(value);
    localStorage.setItem('gameStatusUpdateInterval', value.toString());
    window.dispatchEvent(new Event('localStorageChange'));
  };

  const handleMapIntervalChange = (value: number) => {
    setMapUpdateInterval(value);
    localStorage.setItem('mapUpdateInterval', value.toString());
    window.dispatchEvent(new Event('localStorageChange'));
  };

  const handleFeedIntervalChange = (value: number) => {
    setFeedUpdateInterval(value);
    localStorage.setItem('feedUpdateInterval', value.toString());
    window.dispatchEvent(new Event('localStorageChange'));
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()} style={{ maxWidth: '550px' }}>
        {/* Header */}
        <div className="modal-header">
          <h2 style={{ display: 'flex', alignItems: 'center', gap: '8px', margin: 0 }}>
            ⚙️ {t('settings.title', 'Settings')}
          </h2>
          <button 
            onClick={onClose} 
            className="settings-close-btn"
            title="Close"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="modal-body" style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '24px' }}>
          
          {/* Language Section */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <Globe size={20} style={{ color: 'var(--primary)' }} />
              <h3 style={{ margin: 0, fontSize: '16px' }}>{t('settings.language', 'Language')}</h3>
            </div>
            
            <div style={{ display: 'flex', gap: '12px' }}>
              <button
                onClick={() => handleLanguageChange('en')}
                className={`btn ${i18n.language === 'en' ? 'btn-primary' : 'btn-secondary'}`}
                style={{ flex: 1, padding: '12px' }}
              >
                🇬🇧 English
              </button>
              <button
                onClick={() => handleLanguageChange('ru')}
                className={`btn ${i18n.language === 'ru' ? 'btn-primary' : 'btn-secondary'}`}
                style={{ flex: 1, padding: '12px' }}
              >
                🇷🇺 Русский
              </button>
            </div>
          </div>

          {/* Divider */}
          <div style={{ height: '1px', background: 'var(--border)', opacity: 0.3 }}></div>

          {/* Update Rates Section */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <Gauge size={20} style={{ color: 'var(--primary)' }} />
              <h3 style={{ margin: 0, fontSize: '16px' }}>{t('settings.update_rates', 'Update Rates')}</h3>
            </div>
            
            <p style={{ margin: 0, fontSize: '13px', color: 'var(--text-muted)', lineHeight: 1.5 }}>
              {t('settings.update_rates_desc', 'Control how often components fetch data. Lower = faster updates, Higher = less CPU usage.')}
            </p>
            
            {/* Game Status Slider */}
            <div>
              <div style={{ fontSize: '13px', color: 'var(--text-primary)', marginBottom: '8px', fontWeight: 500 }}>
                🎮 Game Status
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
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
                  fontSize: '12px', 
                  color: 'var(--text-secondary)', 
                  minWidth: '120px',
                  textAlign: 'right',
                  fontFamily: 'monospace'
                }}>
                  {updateInterval}ms ({(1000/updateInterval).toFixed(1)} Hz)
                </span>
              </div>
            </div>

            {/* Map Slider */}
            <div>
              <div style={{ fontSize: '13px', color: 'var(--text-primary)', marginBottom: '8px', fontWeight: 500 }}>
                🗺️ Map
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                <input
                  type="range"
                  min="50"
                  max="1000"
                  step="50"
                  value={mapUpdateInterval}
                  onChange={(e) => handleMapIntervalChange(parseInt(e.target.value))}
                  style={{ flex: 1 }}
                />
                <span style={{ 
                  fontSize: '12px', 
                  color: 'var(--text-secondary)', 
                  minWidth: '120px',
                  textAlign: 'right',
                  fontFamily: 'monospace'
                }}>
                  {mapUpdateInterval}ms ({(1000/mapUpdateInterval).toFixed(1)} Hz)
                </span>
              </div>
            </div>

            {/* Feed / Mission Slider */}
            <div>
              <div style={{ fontSize: '13px', color: 'var(--text-primary)', marginBottom: '8px', fontWeight: 500 }}>
                💬 Feed / Mission
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                <input
                  type="range"
                  min="100"
                  max="2000"
                  step="100"
                  value={feedUpdateInterval}
                  onChange={(e) => handleFeedIntervalChange(parseInt(e.target.value))}
                  style={{ flex: 1 }}
                />
                <span style={{ 
                  fontSize: '12px', 
                  color: 'var(--text-secondary)', 
                  minWidth: '120px',
                  textAlign: 'right',
                  fontFamily: 'monospace'
                }}>
                  {feedUpdateInterval}ms ({(1000/feedUpdateInterval).toFixed(1)} Hz)
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
