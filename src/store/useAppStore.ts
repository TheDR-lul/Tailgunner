import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { DeviceInfo } from '../types';
import type { Pattern } from '../hooks/PatternsProvider';

interface AppState {
  // Engine state
  isRunning: boolean;
  testModeEnabled: boolean;
  
  // Devices
  devices: DeviceInfo[];
  devicesInitialized: boolean;
  
  // Patterns
  patterns: Pattern[];
  
  // UI state
  isEditorOpen: boolean;
  editingPattern: Pattern | undefined;
  isPlayerModalOpen: boolean;
  isSettingsModalOpen: boolean;
  
  // Analytics
  stats: {
    totalVibrations: number;
    totalDuration: number;
    patternUsage: Record<string, number>;
    sessionStart: number;
  };
  
  // Actions
  setRunning: (running: boolean) => void;
  setTestModeEnabled: (enabled: boolean) => void;
  setDevices: (devices: DeviceInfo[]) => void;
  setDevicesInitialized: (initialized: boolean) => void;
  
  setPatterns: (patterns: Pattern[]) => void;
  addPattern: (pattern: Pattern) => void;
  updatePattern: (id: string, updates: Partial<Pattern>) => void;
  removePattern: (id: string) => void;
  
  setEditorOpen: (open: boolean, pattern?: Pattern) => void;
  setPlayerModalOpen: (open: boolean) => void;
  setSettingsModalOpen: (open: boolean) => void;
  
  trackVibration: (patternId: string, duration: number) => void;
  resetStats: () => void;
}

const initialStats = {
  totalVibrations: 0,
  totalDuration: 0,
  patternUsage: {},
  sessionStart: Date.now(),
};

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      // Initial state
      isRunning: false,
      testModeEnabled: false,
      devices: [],
      devicesInitialized: false,
      patterns: [],
      isEditorOpen: false,
      editingPattern: undefined,
      isPlayerModalOpen: false,
      isSettingsModalOpen: false,
      stats: initialStats,
      
      // Actions
      setRunning: (running) => set({ isRunning: running }),
      
      setTestModeEnabled: (enabled) => set({ testModeEnabled: enabled }),
      
      setDevices: (devices) => set({ devices }),
      
      setDevicesInitialized: (initialized) => set({ devicesInitialized: initialized }),
      
      setPatterns: (patterns) => set({ patterns }),
      
      addPattern: (pattern) => set((state) => ({
        patterns: [...state.patterns, pattern],
      })),
      
      updatePattern: (id, updates) => set((state) => ({
        patterns: state.patterns.map((p) =>
          p.id === id ? { ...p, ...updates } : p
        ),
      })),
      
      removePattern: (id) => set((state) => ({
        patterns: state.patterns.filter((p) => p.id !== id),
      })),
      
      setEditorOpen: (open, pattern) => set({
        isEditorOpen: open,
        editingPattern: pattern,
      }),
      
      setPlayerModalOpen: (open) => set({ isPlayerModalOpen: open }),
      
      setSettingsModalOpen: (open) => set({ isSettingsModalOpen: open }),
      
      trackVibration: (patternId, duration) => set((state) => ({
        stats: {
          ...state.stats,
          totalVibrations: state.stats.totalVibrations + 1,
          totalDuration: state.stats.totalDuration + duration,
          patternUsage: {
            ...state.stats.patternUsage,
            [patternId]: (state.stats.patternUsage[patternId] || 0) + 1,
          },
        },
      })),
      
      resetStats: () => set({ stats: initialStats }),
    }),
    {
      name: 'tailgunner-storage',
      partialize: (state) => ({
        patterns: state.patterns,
        stats: state.stats,
      }),
    }
  )
);


