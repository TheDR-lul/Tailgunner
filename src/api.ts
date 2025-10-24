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
};

