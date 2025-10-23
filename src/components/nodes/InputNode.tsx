import { useState } from 'react';
import { Handle, Position } from 'reactflow';
import { useTranslation } from 'react-i18next';

interface InputNodeData {
  label: string;
  indicator: string;
  value?: number;
}

const INDICATORS = [
  { id: 'speed', label: 'Скорость (IAS)', unit: 'км/ч' },
  { id: 'altitude', label: 'Высота', unit: 'м' },
  { id: 'g_load', label: 'G-перегрузка', unit: 'G' },
  { id: 'aoa', label: 'Угол атаки', unit: '°' },
  { id: 'engine_rpm', label: 'Обороты двигателя', unit: 'RPM' },
  { id: 'engine_temp', label: 'Температура двигателя', unit: '°C' },
  { id: 'fuel', label: 'Топливо', unit: 'кг' },
  { id: 'ammo_count', label: 'Боезапас', unit: 'шт' },
  { id: 'mach', label: 'Число Маха', unit: 'M' },
  { id: 'tas', label: 'TAS', unit: 'км/ч' },
];

export function InputNode({ data, id }: { data: InputNodeData; id: string }) {
  const { t } = useTranslation();
  const [indicator, setIndicator] = useState(data.indicator || 'speed');
  
  const selectedIndicator = INDICATORS.find(i => i.id === indicator) || INDICATORS[0];
  
  return (
    <div className="node-input" onClick={(e) => e.stopPropagation()}>
      <div className="node-header">📊 Индикатор</div>
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

