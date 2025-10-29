// Application types

export interface DeviceInfo {
  id: string;
  name: string;
  connected: boolean;
  device_type?: string;
}

// Gamepad Proxy types
export interface GamepadProxyConfig {
  enabled: boolean;
  proxy_to_devices: boolean;
  sensitivity: number;
  deadzone: number;
  left_motor_weight: number;
  right_motor_weight: number;
}

export interface RumbleState {
  left_motor: number;
  right_motor: number;
  timestamp: number;
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
  vne_kmh_max?: number;  // Max Vne for swept wing aircraft (range: vne_kmh - vne_kmh_max)
  max_speed_ground: number;
  stall_speed: number;
  flutter_speed?: number;
  gear_max_speed_kmh?: number;
  flaps_speeds_kmh: number[];  // All flap positions [L, T, C, ...] - can be empty
  mass_kg: number;
  wing_overload_pos_n?: number; // Optional - may be null/undefined if data not available
  wing_overload_neg_n?: number; // Optional - may be null/undefined if data not available
  max_positive_g?: number;      // Optional - will be null if CritOverload not available
  max_negative_g?: number;      // Optional - will be null if CritOverload not available
  max_rpm?: number;
  horse_power?: number;
  vehicle_type: string;
  data_source: string; // "datamine", "wiki", "datamine+wiki"
  last_updated: string;
}

export interface GroundLimits {
  identifier: string;
  display_name: string;
  max_speed_kmh?: number;
  max_reverse_speed_kmh?: number;
  mass_kg?: number;
  horse_power?: number;
  max_rpm?: number;
  min_rpm?: number;
  crew_hp?: number;
  crew_count?: number;
  main_gun_caliber_mm?: number;
  main_gun_fire_rate?: number;
  ammo_count?: number;
  forward_gears?: number;
  reverse_gears?: number;
  vehicle_type: string;
  data_source: string; // "datamine", "wiki", "datamine+wiki"
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
