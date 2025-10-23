// TypeScript типы для работы с Rust backend

export interface DeviceInfo {
  id: number;
  name: string;
  device_type: 'Buttplug' | 'Lovense';
  connected: boolean;
}

export interface VibrationPattern {
  name: string;
  attack: EnvelopeStage;
  hold: EnvelopeStage;
  decay: EnvelopeStage;
  burst: BurstConfig;
}

export interface EnvelopeStage {
  duration_ms: number;
  start_intensity: number;
  end_intensity: number;
  curve: 'Linear' | 'EaseIn' | 'EaseOut' | 'EaseInOut';
}

export interface BurstConfig {
  repeat_count: number;
  pause_between_ms: number;
}

export interface Profile {
  id: string;
  name: string;
  vehicle_type: VehicleType;
  game_mode: GameMode;
  event_mappings: Record<string, VibrationPattern>;
  enabled: boolean;
}

export type VehicleType = 'Tank' | 'Aircraft' | 'Helicopter' | 'Ship' | 'Unknown';
export type GameMode = 'Arcade' | 'Realistic' | 'Simulator' | 'Any';

