import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Info } from 'lucide-react';
import './MapInfoBox.css';

interface MapDatabaseInfo {
  name: string;
  localized_name: string;
  ground_grid_step: number | null;
  air_grid_step: number | null;
  naval_grid_step: number | null;
  map_size: [number, number];
  game_modes: string[];
}

interface Props {
  mapName: string | null;
  apiGridStep: [number, number];
  correctGridStep: [number, number] | null;
}

export function MapInfoBox({ mapName, apiGridStep, correctGridStep }: Props) {
  const [dbInfo, setDbInfo] = useState<MapDatabaseInfo | null>(null);
  const [isExpanded, setIsExpanded] = useState(false);

  useEffect(() => {
    if (!mapName) {
      setDbInfo(null);
      return;
    }

    // Try to get database info for this map
    const mapKey = mapName.toLowerCase().replace(/\s+/g, '_');
    invoke<MapDatabaseInfo>('get_map_database_info', { mapName: mapKey })
      .then(info => setDbInfo(info))
      .catch(() => setDbInfo(null));
  }, [mapName]);

  if (!mapName && !dbInfo) return null;

  const hasCorrection = correctGridStep && 
    (correctGridStep[0] !== apiGridStep[0] || correctGridStep[1] !== apiGridStep[1]);

  return (
    <div className="map-infobox">
      <button 
        className="map-infobox-toggle"
        onClick={() => setIsExpanded(!isExpanded)}
        title="Map Information"
      >
        <Info size={16} />
        {hasCorrection && <span className="correction-badge">!</span>}
      </button>

      {isExpanded && (
        <div className="map-infobox-content">
          <div className="map-infobox-header">
            <Info size={14} />
            <span>Информация о карте</span>
          </div>

          {mapName && (
            <div className="map-info-row">
              <span className="label">Карта:</span>
              <span className="value">{mapName}</span>
            </div>
          )}

          {dbInfo && (
            <>
              <div className="map-info-row">
                <span className="label">Размер:</span>
                <span className="value">
                  {(dbInfo.map_size[0] / 1000).toFixed(1)}km × {(dbInfo.map_size[1] / 1000).toFixed(1)}km
                </span>
              </div>

              <div className="map-info-section">
                <div className="section-title">Размеры сетки (из игры):</div>
                {dbInfo.ground_grid_step && (
                  <div className="map-info-row">
                    <span className="label">🚜 Танки:</span>
                    <span className="value">{dbInfo.ground_grid_step}m × {dbInfo.ground_grid_step}m</span>
                  </div>
                )}
                {dbInfo.air_grid_step && (
                  <div className="map-info-row">
                    <span className="label">✈️ Авиация:</span>
                    <span className="value">
                      {(dbInfo.air_grid_step / 1000).toFixed(1)}km × {(dbInfo.air_grid_step / 1000).toFixed(1)}km
                    </span>
                  </div>
                )}
                {dbInfo.naval_grid_step && (
                  <div className="map-info-row">
                    <span className="label">⚓ Корабли:</span>
                    <span className="value">{dbInfo.naval_grid_step}m × {dbInfo.naval_grid_step}m</span>
                  </div>
                )}
              </div>
            </>
          )}

          {hasCorrection && (
            <div className="map-info-warning">
              <div className="warning-title">⚠️ API неточен!</div>
              <div className="map-info-row">
                <span className="label">API даёт:</span>
                <span className="value error">{apiGridStep[0]}m × {apiGridStep[1]}m</span>
              </div>
              <div className="map-info-row">
                <span className="label">Реально:</span>
                <span className="value correct">
                  {correctGridStep![0]}m × {correctGridStep![1]}m ✓
                </span>
              </div>
              <div className="correction-note">
                Используем правильные размеры из базы данных
              </div>
            </div>
          )}

          {!dbInfo && mapName && (
            <div className="map-info-note">
              Карта не найдена в базе данных
              <br/>
              <small>Используем размеры из API (могут быть неточными)</small>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

