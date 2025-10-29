import { useEffect } from 'react';
import { useHotkeys } from 'react-hotkeys-hook';
import { showToast } from '../utils/toast';

export interface KeyboardShortcut {
  key: string;
  description: string;
  action: () => void;
  disabled?: boolean;
}

export interface KeyboardShortcutsConfig {
  // Engine control
  toggleEngine?: () => void;
  
  // Pattern management
  createPattern?: () => void;
  openTemplates?: () => void;
  
  // Modals
  togglePlayerModal?: () => void;
  toggleSettingsModal?: () => void;
  
  // UI
  toggleDebugConsole?: () => void;
  
  // General
  closeModal?: () => void;
  
  // Test mode
  toggleTestMode?: () => void;
}

export function useKeyboardShortcuts(config: KeyboardShortcutsConfig) {
  const {
    toggleEngine,
    createPattern,
    openTemplates,
    togglePlayerModal,
    toggleSettingsModal,
    toggleDebugConsole,
    closeModal,
    toggleTestMode,
  } = config;

  // Engine control
  useHotkeys('ctrl+e, cmd+e', (e) => {
    e.preventDefault();
    if (toggleEngine) {
      toggleEngine();
      showToast.info('Engine toggled');
    }
  }, { enableOnFormTags: false });

  // Pattern management
  useHotkeys('ctrl+n, cmd+n', (e) => {
    e.preventDefault();
    if (createPattern) {
      createPattern();
    }
  }, { enableOnFormTags: false });

  useHotkeys('ctrl+t, cmd+t', (e) => {
    e.preventDefault();
    if (openTemplates) {
      openTemplates();
    }
  }, { enableOnFormTags: false });

  // Modal controls
  useHotkeys('ctrl+p, cmd+p', (e) => {
    e.preventDefault();
    if (togglePlayerModal) {
      togglePlayerModal();
    }
  }, { enableOnFormTags: false });

  useHotkeys('ctrl+comma, cmd+comma', (e) => {
    e.preventDefault();
    if (toggleSettingsModal) {
      toggleSettingsModal();
    }
  }, { enableOnFormTags: false });

  // Debug console
  useHotkeys('ctrl+d, cmd+d', (e) => {
    e.preventDefault();
    if (toggleDebugConsole) {
      toggleDebugConsole();
    }
  }, { enableOnFormTags: false });

  // Test mode
  useHotkeys('ctrl+shift+t, cmd+shift+t', (e) => {
    e.preventDefault();
    if (toggleTestMode) {
      toggleTestMode();
      showToast.info('Test mode toggled');
    }
  }, { enableOnFormTags: false });

  // Close modal (ESC)
  useHotkeys('esc', () => {
    if (closeModal) {
      closeModal();
    }
  }, { enableOnFormTags: true });

  // Help overlay (Ctrl+?)
  useHotkeys('ctrl+shift+slash, cmd+shift+slash', (e) => {
    e.preventDefault();
    showKeyboardShortcutsHelp();
  }, { enableOnFormTags: false });
}

function showKeyboardShortcutsHelp() {
  const shortcuts = [
    { key: 'Ctrl/Cmd + E', desc: 'Toggle Engine' },
    { key: 'Ctrl/Cmd + N', desc: 'Create New Pattern' },
    { key: 'Ctrl/Cmd + T', desc: 'Open Pattern Templates' },
    { key: 'Ctrl/Cmd + P', desc: 'Open Player Identity' },
    { key: 'Ctrl/Cmd + ,', desc: 'Open Settings' },
    { key: 'Ctrl/Cmd + D', desc: 'Toggle Debug Console' },
    { key: 'Ctrl/Cmd + Shift + T', desc: 'Toggle Test Mode' },
    { key: 'Esc', desc: 'Close Modal' },
    { key: 'Ctrl/Cmd + Shift + ?', desc: 'Show Shortcuts' },
  ];

  const message = shortcuts
    .map(s => `${s.key}: ${s.desc}`)
    .join('\n');

  showToast.info('Keyboard Shortcuts:\n' + message);
}

export const keyboardShortcuts: KeyboardShortcut[] = [
  { key: 'Ctrl/Cmd + E', description: 'Toggle Engine', action: () => {} },
  { key: 'Ctrl/Cmd + N', description: 'Create New Pattern', action: () => {} },
  { key: 'Ctrl/Cmd + T', description: 'Open Pattern Templates', action: () => {} },
  { key: 'Ctrl/Cmd + P', description: 'Open Player Identity', action: () => {} },
  { key: 'Ctrl/Cmd + ,', description: 'Open Settings', action: () => {} },
  { key: 'Ctrl/Cmd + D', description: 'Toggle Debug Console', action: () => {} },
  { key: 'Ctrl/Cmd + Shift + T', description: 'Toggle Test Mode', action: () => {} },
  { key: 'Esc', description: 'Close Modal', action: () => {} },
  { key: 'Ctrl/Cmd + Shift + ?', description: 'Show Shortcuts', action: () => {} },
];


