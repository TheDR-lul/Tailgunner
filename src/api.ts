// API for working with Tauri backend

import { invoke } from '@tauri-apps/api/core';
import type { DeviceInfo, Profile, VibrationPattern, GameStatusInfo } from './types';

export const api = {
  // Initialize devices
  async initDevices(): Promise<string> {
    return invoke<string>('init_devices');
  },

  // Start engine
  async startEngine(): Promise<string> {
    return invoke<string>('start_engine');
  },

  // Stop engine
  async stopEngine(): Promise<string> {
    return invoke<string>('stop_engine');
  },

  // Check status
  async isRunning(): Promise<boolean> {
    try {
      return await invoke<boolean>('is_running');
    } catch (error) {
      console.error('Failed to check running status:', error);
      return false;
    }
  },

  // Player identity management
  async getPlayerNames(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_player_names');
    } catch (error) {
      console.error('Failed to get player names:', error);
      return [];
    }
  },

  async setPlayerNames(names: string[]): Promise<void> {
    try {
      await invoke('set_player_names', { names });
    } catch (error) {
      console.error('Failed to set player names:', error);
    }
  },

  async getClanTags(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_clan_tags');
    } catch (error) {
      console.error('Failed to get clan tags:', error);
      return [];
    }
  },

  async setClanTags(tags: string[]): Promise<void> {
    try {
      await invoke('set_clan_tags', { tags });
    } catch (error) {
      console.error('Failed to set clan tags:', error);
    }
  },

  async getEnemyNames(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_enemy_names');
    } catch (error) {
      console.error('Failed to get enemy names:', error);
      return [];
    }
  },

  async setEnemyNames(names: string[]): Promise<void> {
    try {
      await invoke('set_enemy_names', { names });
    } catch (error) {
      console.error('Failed to set enemy names:', error);
    }
  },

  async getEnemyClans(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_enemy_clans');
    } catch (error) {
      console.error('Failed to get enemy clans:', error);
      return [];
    }
  },

  async setEnemyClans(clans: string[]): Promise<void> {
    try {
      await invoke('set_enemy_clans', { clans });
    } catch (error) {
      console.error('Failed to set enemy clans:', error);
    }
  },

  // Get devices
  async getDevices(): Promise<DeviceInfo[]> {
    try {
      return await invoke<DeviceInfo[]>('get_devices');
    } catch (error) {
      console.error('Failed to get devices:', error);
      return [];
    }
  },

  // Scan for devices
  async scanDevices(): Promise<string> {
    return invoke<string>('scan_devices');
  },

  // Get profiles
  async getProfiles(): Promise<Profile[]> {
    try {
      return await invoke<Profile[]>('get_profiles');
    } catch (error) {
      console.error('Failed to get profiles:', error);
      return [];
    }
  },

  // Test vibration
  async testVibration(intensity: number, durationMs: number): Promise<string> {
    return invoke<string>('test_vibration', { 
      params: { intensity, durationMs }
    });
  },

  // Get preset patterns
  async getPresetPatterns(): Promise<VibrationPattern[]> {
    return invoke<VibrationPattern[]>('get_preset_patterns');
  },

  // Get game status
  async getGameStatus(): Promise<GameStatusInfo> {
    try {
      return await invoke<GameStatusInfo>('get_game_status');
    } catch (error) {
      console.error('Failed to get game status:', error);
      return {
        connected: false,
        vehicle_name: 'N/A',
        speed_kmh: 0,
        altitude_m: 0,
        g_load: 0,
        engine_rpm: 0,
        fuel_percent: 0,
      };
    }
  },

  // Datamine methods
  async datamineAutoInit(): Promise<string> {
    return invoke<string>('datamine_auto_init');
  },

  async datamineRebuild(): Promise<{ aircraft_count: number; ground_count: number; ships_count: number }> {
    return invoke('datamine_rebuild');
  },

  async datamineFindGame(): Promise<string> {
    return invoke<string>('datamine_find_game');
  },

  async datamineParse(gamePath: string): Promise<{
    aircraft_count: number;
    ground_count: number;
    ships_count: number;
  }> {
    return invoke('datamine_parse', { gamePath });
  },

  async datamineGetStats(): Promise<[number, number, number]> {
    return invoke('datamine_get_stats');
  },

  // File dialog
  async selectFolder(): Promise<string | null> {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select War Thunder installation folder',
    });
    
    if (selected && typeof selected === 'string') {
      return selected;
    }
    return null;
  },
};

