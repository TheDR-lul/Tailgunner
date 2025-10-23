import { useState } from 'react';
import { Handle, Position } from 'reactflow';
import { useTranslation } from 'react-i18next';

interface InputNodeData {
  label: string;
  indicator: string;
  value?: number;
}

export function InputNode({ data, id }: { data: InputNodeData; id: string }) {
  const { t } = useTranslation();
  const [indicator, setIndicator] = useState(data.indicator || 'speed');
  
  const INDICATORS = [
    { id: 'speed', label: t('indicators.ias'), unit: 'km/h' },
    { id: 'altitude', label: t('indicators.altitude'), unit: 'm' },
    { id: 'g_load', label: t('indicators.g_load'), unit: 'G' },
    { id: 'aoa', label: t('indicators.aoa'), unit: '°' },
    { id: 'engine_rpm', label: t('indicators.rpm'), unit: 'RPM' },
    { id: 'engine_temp', label: t('indicators.temperature'), unit: '°C' },
    { id: 'fuel', label: t('indicators.fuel'), unit: 'kg' },
    { id: 'ammo_count', label: t('indicators.ammo'), unit: 'pcs' },
    { id: 'mach', label: t('indicators.mach'), unit: 'M' },
    { id: 'tas', label: t('indicators.tas'), unit: 'km/h' },
  ];
  
  const selectedIndicator = INDICATORS.find(i => i.id === indicator) || INDICATORS[0];
  
  return (
    <div className="node-input" onClick={(e) => e.stopPropagation()}>
      <div className="node-header">📊 {t('nodes.input.title')}</div>
      <div className="node-body">
        <select 
          value={indicator}
          onChange={(e) => setIndicator(e.target.value)}
          className="node-select"
          onClick={(e) => e.stopPropagation()}
        >
          {INDICATORS.map(ind => (
            <option key={ind.id} value={ind.id}>{ind.label}</option>
          ))}
        </select>
        {data.value !== undefined && (
          <div className="node-value">{data.value.toFixed(1)} {selectedIndicator.unit}</div>
        )}
      </div>
      <Handle type="source" position={Position.Right} id="value" />
    </div>
  );
}

