import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Info, Gauge, Shield, Zap, Wind, Anchor, CircleAlert } from 'lucide-react';
import type { VehicleLimits, AircraftLimits, GroundLimits, ShipLimits } from '../types';

export const VehicleInfoCard: React.FC = () => {
  const { t } = useTranslation();
  const [vehicleData, setVehicleData] = useState<VehicleLimits | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchVehicleInfo = async () => {
      try {
        const data = await invoke<VehicleLimits | null>('get_vehicle_info');
        
        if (data) {
          setVehicleData(data);
          setError(null);
          
          if ((window as any).debugLog) {
            const name = 'Aircraft' in data ? data.Aircraft.display_name :
                        'Ground' in data ? data.Ground.display_name :
                        'Ship' in data ? data.Ship.display_name : 'Unknown';
            (window as any).debugLog('success', `✅ Vehicle data loaded: ${name}`);
          }
        } else {
          setVehicleData(null);
          setError(null);
        }
      } catch (err) {
        const errorMsg = err as string;
        setError(errorMsg);
        setVehicleData(null);
        
        if ((window as any).debugLog) {
          (window as any).debugLog('error', `❌ Vehicle data error: ${errorMsg}`);
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

  // Always show card with placeholder when no data
  if (error) {
  return (
    <div className="card">
      <div className="card-header">
        <h2>{t('vehicle_info.title', 'Vehicle Information')}</h2>
      </div>
      <div className="card-content">
        <div style={{ textAlign: 'center', color: '#ef4444', padding: '12px', fontSize: '13px' }}>
          {error}
        </div>
      </div>
    </div>
  );
  }

  if (!vehicleData) {
    return (
      <div className="card">
        <div className="card-header">
          <h2>{t('vehicle_info.title', 'Vehicle Information')}</h2>
        </div>
        <div className="card-content">
          <div style={{ textAlign: 'center', color: 'var(--text-muted)', padding: '20px', fontSize: '13px' }}>
            <div style={{ marginBottom: '8px', fontSize: '32px', opacity: 0.5 }}>🎮</div>
            <div>Waiting for vehicle data...</div>
            <div style={{ fontSize: '11px', marginTop: '4px' }}>
              Start a battle or ensure War Thunder telemetry is enabled
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Render based on vehicle type
  if ('Aircraft' in vehicleData) {
    return <AircraftCard data={vehicleData.Aircraft} />;
  } else if ('Ground' in vehicleData) {
    return <GroundCard data={vehicleData.Ground} />;
  } else if ('Ship' in vehicleData) {
    return <ShipCard data={vehicleData.Ship} />;
  }

  return null;
};

// Aircraft info card
const AircraftCard: React.FC<{ data: AircraftLimits }> = ({ data }) => {
  const { t } = useTranslation();
  
  return (
    <div className="card">
      <div className="card-header">
        <div>
          <h2>{data.display_name}</h2>
          <div style={{ display: 'flex', gap: '8px', marginTop: '4px' }}>
            <span style={{ 
              fontSize: '11px', 
              padding: '4px 8px', 
              background: 'rgba(99, 102, 241, 0.2)', 
              borderRadius: 'var(--radius-sm)',
              color: 'var(--primary)'
            }}>
              ✈️ Aircraft
            </span>
          </div>
        </div>
      </div>
      <div className="card-content">
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '12px' }}>
          {/* Vne (Wing Rip Speed) */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <CircleAlert size={18} style={{ color: '#ef4444', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Vne (Wing Rip)
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#ef4444' }}>
                {Math.round(data.vne_kmh)} km/h
              </div>
            </div>
          </div>

          {/* Flutter Speed */}
          {data.flutter_speed && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Wind size={18} style={{ color: '#f97316', flexShrink: 0 }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  Flutter Speed
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#f97316' }}>
                  {Math.round(data.flutter_speed)} km/h
                </div>
              </div>
            </div>
          )}

          {/* Max G-Load */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Gauge size={18} style={{ color: '#f59e0b', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                G-Load Limits
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#f59e0b' }}>
                +{data.max_positive_g.toFixed(1)}G / {data.max_negative_g.toFixed(1)}G
              </div>
            </div>
          </div>

          {/* Engine Power */}
          {data.horse_power && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Zap size={18} style={{ color: '#10b981', flexShrink: 0 }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  Engine Power
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#10b981' }}>
                  {Math.round(data.horse_power)} HP
                </div>
              </div>
            </div>
          )}

          {/* Mass */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Shield size={18} style={{ color: '#a855f7', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Mass
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#a855f7' }}>
                {(data.mass_kg / 1000).toFixed(1)} t
              </div>
            </div>
          </div>

          {/* Stall Speed */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Wind size={18} style={{ color: '#06b6d4', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Stall Speed
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#06b6d4' }}>
                {Math.round(data.stall_speed)} km/h
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

// Ground vehicle info card
const GroundCard: React.FC<{ data: GroundLimits }> = ({ data }) => {
  const { t } = useTranslation();
  
  return (
    <div className="card">
      <div className="card-header">
        <Info size={20} />
        <span>{data.display_name}</span>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: '8px' }}>
          <span style={{ 
            fontSize: '11px', 
            padding: '4px 8px', 
            background: 'rgba(34, 197, 94, 0.2)', 
            borderRadius: 'var(--radius-sm)',
            color: '#22c55e'
          }}>
            🚜 {data.vehicle_type}
          </span>
        </div>
      </div>
      <div className="card-content">
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '12px' }}>
          {/* Max Speed */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Wind size={18} style={{ color: '#06b6d4', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Max Speed
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#06b6d4' }}>
                {Math.round(data.max_speed_kmh)} km/h
              </div>
            </div>
          </div>

          {/* Engine Power */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Zap size={18} style={{ color: '#10b981', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Engine Power
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#10b981' }}>
                {Math.round(data.horse_power)} HP
              </div>
            </div>
          </div>

          {/* Mass */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Shield size={18} style={{ color: '#a855f7', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Mass
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#a855f7' }}>
                {(data.mass_kg / 1000).toFixed(1)} t
              </div>
            </div>
          </div>

          {/* Armor */}
          {data.armor_thickness_mm && (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
              padding: '10px',
              background: 'var(--bg-secondary)',
              borderRadius: 'var(--radius-sm)',
              border: '1px solid var(--border)'
            }}>
              <Shield size={18} style={{ color: '#f59e0b', flexShrink: 0 }} />
              <div>
                <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                  Armor
                </div>
                <div style={{ fontSize: '14px', fontWeight: 600, color: '#f59e0b' }}>
                  {Math.round(data.armor_thickness_mm)} mm
                </div>
              </div>
            </div>
          )}

          {/* Hull HP */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Gauge size={18} style={{ color: '#ef4444', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Hull HP
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#ef4444' }}>
                {Math.round(data.hull_hp)}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

// Ship info card
const ShipCard: React.FC<{ data: ShipLimits }> = ({ data }) => {
  const { t } = useTranslation();
  
  const criticalCompartments = data.compartments.filter(c => c.critical);
  const totalHp = data.compartments.reduce((sum, c) => sum + c.hp, 0);
  
  return (
    <div className="card">
      <div className="card-header">
        <Info size={20} />
        <span>{data.display_name}</span>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: '8px' }}>
          <span style={{ 
            fontSize: '11px', 
            padding: '4px 8px', 
            background: 'rgba(59, 130, 246, 0.2)', 
            borderRadius: 'var(--radius-sm)',
            color: '#3b82f6'
          }}>
            ⚓ {data.ship_class}
          </span>
        </div>
      </div>
      <div className="card-content">
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))', gap: '12px' }}>
          {/* Max Speed */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Anchor size={18} style={{ color: '#06b6d4', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Max Speed
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#06b6d4' }}>
                {Math.round(data.max_speed_knots)} knots
              </div>
            </div>
          </div>

          {/* Compartments */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Shield size={18} style={{ color: '#a855f7', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Compartments
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#a855f7' }}>
                {data.compartments.length}
              </div>
            </div>
          </div>

          {/* Total HP */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <Gauge size={18} style={{ color: '#10b981', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Total HP
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#10b981' }}>
                {Math.round(totalHp)}
              </div>
            </div>
          </div>

          {/* Critical Modules */}
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            gap: '8px',
            padding: '10px',
            background: 'var(--bg-secondary)',
            borderRadius: 'var(--radius-sm)',
            border: '1px solid var(--border)'
          }}>
            <CircleAlert size={18} style={{ color: '#ef4444', flexShrink: 0 }} />
            <div>
              <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '2px' }}>
                Critical Modules
              </div>
              <div style={{ fontSize: '14px', fontWeight: 600, color: '#ef4444' }}>
                {criticalCompartments.length}
              </div>
            </div>
          </div>
        </div>

        {/* Critical compartments list */}
        {criticalCompartments.length > 0 && (
          <div style={{ marginTop: '12px', paddingTop: '12px', borderTop: '1px solid var(--border)' }}>
            <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginBottom: '6px' }}>
              Critical Modules
            </div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
              {criticalCompartments.map((comp, idx) => (
                <span 
                  key={idx}
                  style={{ 
                    fontSize: '10px', 
                    padding: '2px 6px', 
                    background: 'rgba(239, 68, 68, 0.1)', 
                    borderRadius: '4px',
                    color: 'var(--text-secondary)'
                  }}
                >
                  {comp.name.replace('_dm', '')} ({Math.round(comp.hp)} HP)
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
