import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Info, Gauge, Shield, Zap, Wind } from 'lucide-react';

interface VehicleData {
  identifier: string;
  wikiname: string;
  display_name: string;
  vehicle_type: string;
  country: string;
  rank: number;
  battle_rating: Record<string, number>;
  max_speed_kmh?: number;
  max_altitude_meters?: number;
  engine_power_hp?: number;
  max_positive_g?: number;
  max_negative_g?: number;
  fuel_capacity_kg?: number;
  weapons?: string[];
}

export const VehicleInfoCard: React.FC = () => {
  const { t } = useTranslation();
  const [vehicleData, setVehicleData] = useState<VehicleData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchVehicleInfo = async () => {
      try {
        const data = await invoke<VehicleData>('get_vehicle_info');
        setVehicleData(data);
        setError(null);
        
        if ((window as any).debugLog) {
          (window as any).debugLog('success', `✅ Vehicle data loaded: ${data.display_name}`);
        }
      } catch (err) {
        const errorMsg = err as string;
        
        // Don't show error if it's just "No vehicle connected"
        if (errorMsg.includes('No vehicle connected')) {
          setError(null);
          setVehicleData(null);
        } else {
          setError(errorMsg);
          setVehicleData(null);
          
          if ((window as any).debugLog) {
            (window as any).debugLog('error', `❌ Vehicle data error: ${errorMsg}`);
          }
        }
      } finally {
        setLoading(false);
      }
    };

    // Initial fetch
    setLoading(true);
    fetchVehicleInfo();
    
    // Update every 5s
    const interval = setInterval(fetchVehicleInfo, 5000);

    return () => clearInterval(interval);
  }, []);

  // Don't render anything if no vehicle data
  if (!vehicleData) {
    return null;
  }

  if (error) {
    return (
      <div className="card">
        <div className="card-header">
          <Info size={20} />
          <span>{t('vehicle_info.title', 'Vehicle Information')}</span>
        </div>
        <div className="card-content">
          <div style={{ textAlign: 'center', color: '#ef4444', padding: '12px', fontSize: '11px' }}>
            {error}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="card-header">
        <Info size={20} />
        <span>{vehicleData.display_name}</span>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: '8px' }}>
          <span style={{ 
            fontSize: '11px', 
            padding: '4px 8px', 
            background: 'rgba(99, 102, 241, 0.2)', 
            borderRadius: 'var(--radius-sm)',
            color: 'var(--primary)'
          }}>
            {vehicleData.vehicle_type}
          </span>
          <span style={{ 
            fontSize: '11px', 
            padding: '4px 8px', 
            background: 'rgba(249, 115, 22, 0.2)', 
            borderRadius: 'var(--radius-sm)',
            color: '#f97316'
          }}>
            BR {Object.values(vehicleData.battle_rating)[0] || 'N/A'}
          </span>
        </div>
      </div>
      <div className="card-content">
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '12px' }}>
          {/* Max Speed */}
          {vehicleData.max_speed_kmh && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Wind size={16} style={{ color: '#06b6d4' }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  {t('vehicle_info.max_speed', 'Max Speed')}
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#06b6d4' }}>
                  {vehicleData.max_speed_kmh} km/h
                </div>
              </div>
            </div>
          )}

          {/* Max G-Load */}
          {vehicleData.max_positive_g && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Gauge size={16} style={{ color: '#f59e0b' }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  {t('vehicle_info.max_g', 'Max G-Load')}
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#f59e0b' }}>
                  +{vehicleData.max_positive_g}G / {vehicleData.max_negative_g}G
                </div>
              </div>
            </div>
          )}

          {/* Engine Power */}
          {vehicleData.engine_power_hp && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Zap size={16} style={{ color: '#10b981' }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  {t('vehicle_info.engine_power', 'Engine Power')}
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#10b981' }}>
                  {vehicleData.engine_power_hp} HP
                </div>
              </div>
            </div>
          )}

          {/* Max Altitude */}
          {vehicleData.max_altitude_meters && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Shield size={16} style={{ color: '#a855f7' }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  {t('vehicle_info.max_altitude', 'Max Altitude')}
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#a855f7' }}>
                  {(vehicleData.max_altitude_meters / 1000).toFixed(1)} km
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Weapons */}
        {vehicleData.weapons && vehicleData.weapons.length > 0 && (
          <div style={{ marginTop: '12px', paddingTop: '12px', borderTop: '1px solid var(--border)' }}>
            <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '6px' }}>
              {t('vehicle_info.weapons', 'Weapons')}
            </div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
              {vehicleData.weapons.slice(0, 5).map((weapon, idx) => (
                <span 
                  key={idx}
                  style={{ 
                    fontSize: '10px', 
                    padding: '2px 6px', 
                    background: 'rgba(249, 115, 22, 0.1)', 
                    borderRadius: '4px',
                    color: 'var(--text-secondary)'
                  }}
                >
                  {weapon}
                </span>
              ))}
              {vehicleData.weapons.length > 5 && (
                <span style={{ fontSize: '10px', color: 'var(--text-muted)' }}>
                  +{vehicleData.weapons.length - 5} more
                </span>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

