// API для работы с Tauri backend

import { invoke } from '@tauri-apps/api/core';
import type { DeviceInfo, Profile, VibrationPattern } from './types';

export const api = {
  // Инициализация устройств
  async initDevices(): Promise<string> {
    return invoke<string>('init_devices');
  },

  // Запуск движка
  async startEngine(): Promise<string> {
    return invoke<string>('start_engine');
  },

  // Остановка движка
  async stopEngine(): Promise<string> {
    return invoke<string>('stop_engine');
  },

  // Проверка статуса
  async isRunning(): Promise<boolean> {
    try {
      return await invoke<boolean>('is_running');
    } catch (error) {
      console.error('Failed to check running status:', error);
      return false;
    }
  },

  // Получение устройств
  async getDevices(): Promise<DeviceInfo[]> {
    try {
      return await invoke<DeviceInfo[]>('get_devices');
    } catch (error) {
      console.error('Failed to get devices:', error);
      return [];
    }
  },

  // Получение профилей
  async getProfiles(): Promise<Profile[]> {
    try {
      return await invoke<Profile[]>('get_profiles');
    } catch (error) {
      console.error('Failed to get profiles:', error);
      return [];
    }
  },

  // Тестовая вибрация
  async testVibration(intensity: number, durationMs: number): Promise<string> {
    return invoke<string>('test_vibration', { 
      intensity, 
      durationMs  // Tauri expects camelCase!
    });
  },

  // Получение пресетов паттернов
  async getPresetPatterns(): Promise<VibrationPattern[]> {
    return invoke<VibrationPattern[]>('get_preset_patterns');
  },
};

