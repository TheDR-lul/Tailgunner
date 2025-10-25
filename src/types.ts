// Application types

export interface DeviceInfo {
  id: string;
  name: string;
  connected: boolean;
  device_type?: string;
}

export interface Profile {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  event_mappings?: Record<string, string>;
  vehicle_type?: string;
  game_mode?: string;
}

export interface VibrationPattern {
  id: string;
  name: string;
  nodes: any[];
  edges: any[];
  is_default: boolean;
}

export interface GameStatusInfo {
  connected: boolean;
  vehicle_name: string;
  speed_kmh: number;
  altitude_m: number;
  g_load: number;
  engine_rpm: number;
  fuel_percent: number;
}

// Datamine types
export interface AircraftLimits {
  identifier: string;
  display_name: string;
  vne_kmh: number;
  vne_mach?: number;
  max_speed_ground: number;
  stall_speed: number;
  flutter_speed?: number;
  mass_kg: number;
  wing_overload_pos_n: number;
  wing_overload_neg_n: number;
  max_positive_g: number;
  max_negative_g: number;
  max_rpm?: number;
  horse_power?: number;
  vehicle_type: string;
  last_updated: string;
}

export interface GroundLimits {
  identifier: string;
  display_name: string;
  max_speed_kmh: number;
  max_reverse_speed_kmh: number;
  mass_kg: number;
  horse_power: number;
  max_rpm: number;
  min_rpm: number;
  hull_hp: number;
  armor_thickness_mm?: number;
  vehicle_type: string;
  last_updated: string;
}

export interface ShipLimits {
  identifier: string;
  display_name: string;
  max_speed_knots: number;
  max_reverse_speed_knots: number;
  compartments: Array<{
    name: string;
    hp: number;
    critical: boolean;
  }>;
  ship_class: string;
  last_updated: string;
}

export type VehicleLimits = 
  | { Aircraft: AircraftLimits }
  | { Ground: GroundLimits }
  | { Ship: ShipLimits };
