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

// Aircraft info card - Dashboard Style
  const AircraftCard: React.FC<{ data: AircraftLimits }> = ({ data }) => {
    const StatItem = ({ icon, label, value, desc, color }: { 
      icon: React.ReactNode, 
      label: string, 
      value: string, 
      desc?: string,
      color: string 
    }) => (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '8px',
        padding: '8px',
              background: 'var(--bg-secondary)',
        borderRadius: '6px',
      }}>
        <div style={{ color, flexShrink: 0 }}>{icon}</div>
        <div style={{ flex: 1, minWidth: 0 }}>
          <div style={{ fontSize: '11px', color: 'var(--text-muted)', lineHeight: 1.2, marginBottom: '3px' }}>
            {label}
                </div>
          <div style={{ fontSize: '14px', fontWeight: 'bold', color, lineHeight: 1.2 }}>
            {value}
                </div>
          {desc && (
            <div style={{ fontSize: '9px', color: 'var(--text-muted)', lineHeight: 1.2, marginTop: '2px', opacity: 0.8 }}>
              {desc}
            </div>
          )}
        </div>
      </div>
    );
    
    return (
      <div className="card" style={{ maxWidth: '480px' }}>
        <div className="card-header">
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '12px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <div style={{ 
                fontSize: '20px', 
                lineHeight: 1,
                filter: 'drop-shadow(0 2px 4px rgba(99, 102, 241, 0.3))'
              }}>
                ✈️
              </div>
              <h2 style={{ margin: 0, fontSize: '15px', fontWeight: 600 }}>{data.display_name}</h2>
            </div>
            <span style={{ 
              fontSize: '9px', 
              padding: '3px 7px', 
              background: 'rgba(99, 102, 241, 0.12)', 
              border: '1px solid rgba(99, 102, 241, 0.25)',
              borderRadius: '4px',
              color: '#818cf8',
              fontWeight: 600,
              letterSpacing: '0.5px',
              lineHeight: 1
            }}>
              AIRCRAFT
            </span>
                </div>
                </div>
        <div className="card-content" style={{ padding: '16px' }}>
          <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: '8px' }}>
            <StatItem 
              icon={<CircleAlert size={16} />} 
              label="Vne (Wing Rip Speed)" 
              value={
                data.vne_kmh_max 
                  ? `${Math.round(data.vne_kmh)} — ${Math.round(data.vne_kmh_max)} km/h`
                  : `${Math.round(data.vne_kmh)} km/h`
              }
              desc={
                data.vne_kmh_max 
                  ? "Swept-wing range: varies with wing sweep angle"
                  : "Exceeding this speed will tear wings off"
              }
              color="#ef4444"
            />
            <StatItem 
              icon={<Gauge size={16} />} 
              label="G-Load Limits" 
              value={
                data.max_positive_g != null && data.max_negative_g != null
                  ? `+${data.max_positive_g.toFixed(1)}G / ${data.max_negative_g.toFixed(1)}G`
                  : "N/A"
              }
              desc={
                data.max_positive_g != null 
                  ? "Maximum overload before structural failure"
                  : "G-load data not available for this aircraft"
              }
              color="#f59e0b"
            />
            {data.flutter_speed && (
              <StatItem 
                icon={<Wind size={16} />} 
                label="Flutter Speed" 
                value={`${Math.round(data.flutter_speed)} km/h`}
                desc="Speed at which wing flutter begins"
                color="#f97316"
              />
            )}
            <StatItem 
              icon={<Shield size={16} />} 
              label="Empty Mass" 
              value={`${(data.mass_kg / 1000).toFixed(1)} t`}
              desc="Aircraft weight without fuel"
              color="#a855f7"
            />
            <StatItem 
              icon={<Wind size={16} />} 
              label="Stall Speed" 
              value={`${Math.round(data.stall_speed)} km/h`}
              desc="Minimum speed to maintain flight"
              color="#06b6d4"
            />
            {data.gear_max_speed_kmh && data.gear_max_speed_kmh > 0 && (
              <StatItem 
                icon={<Shield size={16} />} 
                label="Gear Speed Limit (IAS)" 
                value={`${Math.round(data.gear_max_speed_kmh)} km/h`}
                desc="Maximum speed for gear operation"
                color="#8b5cf6"
              />
            )}
            {data.flaps_speeds_kmh && data.flaps_speeds_kmh.length > 0 && (
              <StatItem 
                icon={<Wind size={16} />} 
                label="Flap Speed Limits (IAS)" 
                value={
                  (() => {
                    // Normalize to 3 positions: L / T / C
                    const positions = ['L', 'T', 'C'];
                    const speeds = data.flaps_speeds_kmh;
                    
                    if (speeds.length === 1) {
                      return `${Math.round(speeds[0])} km/h`;
                    } else if (speeds.length === 2) {
                      return `${Math.round(speeds[0])} / — / ${Math.round(speeds[1])} km/h`;
                    } else if (speeds.length >= 3) {
                      return speeds.slice(0, 3).map(s => Math.round(s)).join(' / ') + ' km/h';
                    }
                    return speeds.map(s => Math.round(s)).join(' / ') + ' km/h';
                  })()
                }
                desc={
                  data.flaps_speeds_kmh.length > 1
                    ? `Different positions: L / T / C (telemetry shows current position)`
                    : "Maximum speed for flaps extension"
                }
                color="#14b8a6"
              />
            )}
            {data.horse_power && (
              <StatItem 
                icon={<Zap size={16} />} 
                label="Engine Power" 
                value={`${Math.round(data.horse_power)} HP`}
                desc="Total engine horsepower"
                color="#10b981"
              />
            )}
              </div>
          
          {/* Data source indicator */}
          {data.data_source && data.data_source !== 'datamine' && (
            <div style={{ 
              marginTop: '8px', 
              padding: '4px 8px', 
              fontSize: '9px', 
              color: 'var(--text-muted)', 
              background: 'rgba(20, 184, 166, 0.1)',
              borderRadius: '3px',
              textAlign: 'center'
            }}>
              🌐 Enhanced with Wiki data
            </div>
          )}
        </div>
      </div>
    );
};

// Ground vehicle info card - COMPACT VERSION
const GroundCard: React.FC<{ data: GroundLimits }> = ({ data }) => {
  const StatItem = ({ icon, label, value, desc, color }: { icon: React.ReactNode, label: string, value: string, desc?: string, color: string }) => (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
      gap: '6px',
      padding: '6px 8px',
              background: 'var(--bg-secondary)',
      borderRadius: '4px',
    }}>
      <div style={{ color, flexShrink: 0 }}>{icon}</div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ fontSize: '9px', color: 'var(--text-muted)', lineHeight: 1, marginBottom: '2px' }}>
          {label}
        </div>
        <div style={{ fontSize: '14px', fontWeight: 600, color, lineHeight: 1 }}>
          {value}
        </div>
        {desc && (
          <div style={{ fontSize: '9px', color: 'var(--text-muted)', lineHeight: 1.2, marginTop: '2px', opacity: 0.7 }}>
            {desc}
                </div>
        )}
                </div>
              </div>
  );
  
  return (
    <div className="card" style={{ maxWidth: '480px' }}>
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <div style={{ 
              fontSize: '20px', 
              lineHeight: 1,
              filter: 'drop-shadow(0 2px 4px rgba(34, 197, 94, 0.3))'
            }}>
              🪖
            </div>
            <h2 style={{ margin: 0, fontSize: '15px', fontWeight: 600 }}>{data.display_name}</h2>
          </div>
          <span style={{ 
            fontSize: '9px', 
            padding: '3px 7px', 
            background: 'rgba(34, 197, 94, 0.12)', 
            border: '1px solid rgba(34, 197, 94, 0.25)',
            borderRadius: '4px',
            color: '#4ade80',
            fontWeight: 600,
            letterSpacing: '0.5px',
            lineHeight: 1
          }}>
            GROUND
          </span>
        </div>
      </div>
      <div className="card-content" style={{ padding: '8px' }}>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: '6px' }}>
          {data.crew_count && (
            <StatItem 
              icon={<span style={{fontSize: '16px'}}>👥</span>} 
              label="Crew" 
              value={`${data.crew_count} members`}
              desc="Number of crew members"
              color="#8b5cf6"
            />
          )}
          {data.mass_kg && (
            <StatItem 
              icon={<Shield size={16} />} 
              label="Base Weight" 
              value={`${(data.mass_kg / 1000).toFixed(1)} t`}
              desc="Empty weight (without fuel and ammo)"
              color="#a855f7"
            />
          )}
          {data.horse_power && (
            <StatItem 
              icon={<Zap size={16} />} 
              label="Engine Power" 
              value={`${Math.round(data.horse_power)} HP`}
              desc={`${data.min_rpm ? Math.round(data.min_rpm) : '?'}-${data.max_rpm ? Math.round(data.max_rpm) : '?'} RPM`}
              color="#10b981"
            />
          )}
          {data.max_speed_kmh && (
            <StatItem 
              icon={<Wind size={16} />} 
              label="Max Speed" 
              value={`${Math.round(data.max_speed_kmh)} km/h`}
              desc={data.max_reverse_speed_kmh ? `Reverse: ${Math.round(data.max_reverse_speed_kmh)} km/h` : undefined}
              color="#06b6d4"
            />
          )}
          {data.main_gun_caliber_mm && (
            <StatItem 
              icon={<span style={{fontSize: '16px'}}>🎯</span>} 
              label="Main Gun" 
              value={`${Math.round(data.main_gun_caliber_mm)} mm`}
              desc={data.main_gun_fire_rate ? `${(data.main_gun_fire_rate * 60).toFixed(1)} rounds/min` : undefined}
              color="#f59e0b"
            />
          )}
          {data.ammo_count && (
            <StatItem 
              icon={<span style={{fontSize: '16px'}}>📦</span>} 
              label="Ammo" 
              value={`${data.ammo_count} rounds`}
              desc="Total ammunition"
              color="#f97316"
            />
          )}
          {data.forward_gears && (
            <StatItem 
              icon={<span style={{fontSize: '16px'}}>⚙️</span>} 
              label="Transmission" 
              value={`${data.reverse_gears || 0}R / ${data.forward_gears}F`}
              desc="Reverse / Forward gears"
              color="#14b8a6"
            />
          )}
          {data.crew_hp && (
            <StatItem 
              icon={<Gauge size={16} />} 
              label="Crew HP" 
              value={`${Math.round(data.crew_hp)}`}
              desc="Crew durability"
              color="#ef4444"
            />
          )}
        </div>
        {/* Data source indicator */}
        {data.data_source && data.data_source !== 'datamine' && (
          <div style={{ 
            marginTop: '8px', 
            padding: '4px 8px', 
            fontSize: '9px', 
            color: 'var(--text-muted)', 
            background: 'rgba(20, 184, 166, 0.1)',
            borderRadius: '3px',
            textAlign: 'center'
          }}>
            🌐 Enhanced with Wiki data
          </div>
        )}
      </div>
    </div>
  );
};

// Ship info card - COMPACT VERSION
const ShipCard: React.FC<{ data: ShipLimits }> = ({ data }) => {
  const criticalCompartments = data.compartments.filter(c => c.critical);
  const totalHp = data.compartments.reduce((sum, c) => sum + c.hp, 0);
  
  const StatItem = ({ icon, label, value, color }: { icon: React.ReactNode, label: string, value: string, color: string }) => (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
      gap: '6px',
      padding: '6px 8px',
              background: 'var(--bg-secondary)',
      borderRadius: '4px',
    }}>
      <div style={{ color, flexShrink: 0 }}>{icon}</div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ fontSize: '9px', color: 'var(--text-muted)', lineHeight: 1, marginBottom: '2px' }}>
          {label}
                </div>
        <div style={{ fontSize: '12px', fontWeight: 600, color, lineHeight: 1 }}>
          {value}
                </div>
              </div>
            </div>
  );
  
  return (
    <div className="card" style={{ maxWidth: '480px' }}>
      <div className="card-header">
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '12px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <div style={{ 
              fontSize: '20px', 
              lineHeight: 1,
              filter: 'drop-shadow(0 2px 4px rgba(14, 165, 233, 0.3))'
            }}>
              ⚓
            </div>
            <h2 style={{ margin: 0, fontSize: '15px', fontWeight: 600 }}>{data.display_name}</h2>
          </div>
          <span style={{ 
            fontSize: '9px', 
            padding: '3px 7px', 
            background: 'rgba(14, 165, 233, 0.12)', 
            border: '1px solid rgba(14, 165, 233, 0.25)',
            borderRadius: '4px',
            color: '#38bdf8',
            fontWeight: 600,
            letterSpacing: '0.5px',
            lineHeight: 1
          }}>
            {data.ship_class}
          </span>
        </div>
      </div>
      <div className="card-content" style={{ padding: '12px' }}>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>
          <StatItem 
            icon={<Anchor size={14} />} 
            label="Max Speed" 
            value={`${Math.round(data.max_speed_knots)} knots`}
            color="#06b6d4"
          />
          <StatItem 
            icon={<Shield size={14} />} 
            label="Compartments" 
            value={`${data.compartments.length}`}
            color="#a855f7"
          />
          <StatItem 
            icon={<Gauge size={14} />} 
            label="Total HP" 
            value={`${Math.round(totalHp)}`}
            color="#10b981"
          />
          <StatItem 
            icon={<CircleAlert size={14} />} 
            label="Critical Modules" 
            value={`${criticalCompartments.length}`}
            color="#ef4444"
          />
        </div>

        {criticalCompartments.length > 0 && (
          <div style={{ marginTop: '10px', paddingTop: '8px', borderTop: '1px solid var(--border)' }}>
            <div style={{ fontSize: '9px', color: 'var(--text-muted)', marginBottom: '4px', fontWeight: 500 }}>
              Critical Modules
            </div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px' }}>
              {criticalCompartments.slice(0, 5).map((comp, idx) => (
                <span 
                  key={idx}
                  style={{ 
                    fontSize: '9px', 
                    padding: '2px 5px', 
                    background: 'rgba(239, 68, 68, 0.1)', 
                    borderRadius: '3px',
                    color: 'var(--text-secondary)',
                    lineHeight: 1
                  }}
                >
                  {comp.name.replace('_dm', '')} ({Math.round(comp.hp)})
                </span>
              ))}
              {criticalCompartments.length > 5 && (
                <span style={{ fontSize: '9px', color: 'var(--text-muted)' }}>
                  +{criticalCompartments.length - 5} more
                </span>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
