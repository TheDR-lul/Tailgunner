// Vehicle presets for Test Mode emulator
export interface VehiclePreset {
  name: string;           // Internal name (e.g. "f_16a")
  displayName: string;    // Display name (e.g. "F-16A")
  type: 'Tank' | 'Aircraft' | 'Ship';
  maxSpeed: number;       // km/h
  icon: string;           // Emoji or icon
}

export const VEHICLE_PRESETS: VehiclePreset[] = [
  // Aircraft - Fighters
  { name: 'f_16a', displayName: 'F-16A Fighting Falcon', type: 'Aircraft', maxSpeed: 2120, icon: '✈️' },
  { name: 'mig_29', displayName: 'MiG-29', type: 'Aircraft', maxSpeed: 2450, icon: '✈️' },
  { name: 'f_15e', displayName: 'F-15E Strike Eagle', type: 'Aircraft', maxSpeed: 2655, icon: '✈️' },
  { name: 'su_27', displayName: 'Su-27 Flanker', type: 'Aircraft', maxSpeed: 2500, icon: '✈️' },
  { name: 'jas39c', displayName: 'JAS 39C Gripen', type: 'Aircraft', maxSpeed: 2130, icon: '✈️' },
  { name: 'f_14b', displayName: 'F-14B Tomcat', type: 'Aircraft', maxSpeed: 2485, icon: '✈️' },
  { name: 'mirage_2000c', displayName: 'Mirage 2000C', type: 'Aircraft', maxSpeed: 2495, icon: '✈️' },
  
  // Aircraft - Attackers
  { name: 'a_10a', displayName: 'A-10A Thunderbolt II', type: 'Aircraft', maxSpeed: 706, icon: '🛩️' },
  { name: 'su_25', displayName: 'Su-25 Frogfoot', type: 'Aircraft', maxSpeed: 950, icon: '🛩️' },
  
  // Aircraft - Bombers
  { name: 'b_17g', displayName: 'B-17G Flying Fortress', type: 'Aircraft', maxSpeed: 460, icon: '🛩️' },
  { name: 'tu_95', displayName: 'Tu-95 Bear', type: 'Aircraft', maxSpeed: 815, icon: '🛩️' },
  
  // Tanks - Modern MBTs
  { name: 'm1a2_abrams', displayName: 'M1A2 Abrams', type: 'Tank', maxSpeed: 68, icon: '🛡️' },
  { name: 't_90a', displayName: 'T-90A', type: 'Tank', maxSpeed: 60, icon: '🛡️' },
  { name: 'leopard_2a6', displayName: 'Leopard 2A6', type: 'Tank', maxSpeed: 72, icon: '🛡️' },
  { name: 'challenger_2', displayName: 'Challenger 2', type: 'Tank', maxSpeed: 59, icon: '🛡️' },
  { name: 'type_90', displayName: 'Type 90', type: 'Tank', maxSpeed: 70, icon: '🛡️' },
  
  // Tanks - Light/Medium
  { name: 'm18_hellcat', displayName: 'M18 Hellcat', type: 'Tank', maxSpeed: 80, icon: '🚙' },
  { name: 'type_16', displayName: 'Type 16 (FPS)', type: 'Tank', maxSpeed: 100, icon: '🚙' },
  
  // Ships - Battleships
  { name: 'uss_missouri', displayName: 'USS Missouri', type: 'Ship', maxSpeed: 59, icon: '⚓' },
  { name: 'yamato', displayName: 'IJN Yamato', type: 'Ship', maxSpeed: 50, icon: '⚓' },
  { name: 'bismarck', displayName: 'Bismarck', type: 'Ship', maxSpeed: 56, icon: '⚓' },
  
  // Ships - Cruisers
  { name: 'baltimore', displayName: 'USS Baltimore', type: 'Ship', maxSpeed: 60, icon: '🚢' },
  { name: 'prinz_eugen', displayName: 'Prinz Eugen', type: 'Ship', maxSpeed: 64, icon: '🚢' },
  
  // Ships - Destroyers
  { name: 'fletcher', displayName: 'USS Fletcher', type: 'Ship', maxSpeed: 66, icon: '⛵' },
  { name: 'gearing', displayName: 'USS Gearing', type: 'Ship', maxSpeed: 69, icon: '⛵' },
];

export function getVehiclesByType(type: 'Tank' | 'Aircraft' | 'Ship'): VehiclePreset[] {
  return VEHICLE_PRESETS.filter(v => v.type === type);
}

export function getVehiclePreset(name: string): VehiclePreset | undefined {
  return VEHICLE_PRESETS.find(v => v.name === name);
}

