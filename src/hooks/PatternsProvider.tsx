import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { Node, Edge } from 'reactflow';
import { invoke } from '@tauri-apps/api/core';

export interface Pattern {
  id: string;
  name: string;
  enabled: boolean;
  nodes: Node[];
  edges: Edge[];
  createdAt: Date;
}

interface PatternsContextType {
  patterns: Pattern[];
  loading: boolean;
  addPattern: (name: string, nodes: Node[], edges: Edge[]) => Promise<void>;
  addFullPattern: (pattern: Pattern) => Promise<void>;
  updatePattern: (id: string, updates: Partial<Pattern>) => Promise<void>;
  deletePattern: (id: string) => void;
  togglePattern: (id: string) => Promise<void>;
}

const PatternsContext = createContext<PatternsContextType | undefined>(undefined);

const STORAGE_KEY = 'haptic_patterns';

export function PatternsProvider({ children }: { children: ReactNode }) {
  const [patterns, setPatterns] = useState<Pattern[]>([]);
  const [loading, setLoading] = useState(true);

  // Load patterns from localStorage
  useEffect(() => {
    const savedPatterns = localStorage.getItem(STORAGE_KEY);
    if (savedPatterns) {
      try {
        const parsed = JSON.parse(savedPatterns);
        // Convert date strings back to Date objects and validate structure
        const patternsWithDates = parsed
          .map((p: any) => ({
            id: String(p.id || ''),
            name: typeof p.name === 'string' ? p.name : String(p.name || 'Unnamed'),
            // Ensure nodes and edges exist
            nodes: Array.isArray(p.nodes) ? p.nodes : [],
            edges: Array.isArray(p.edges) ? p.edges : [],
            enabled: p.enabled ?? true,
            createdAt: p.createdAt ? new Date(p.createdAt) : new Date(),
          }))
          .filter((p: Pattern) => p.id && typeof p.name === 'string'); // Filter out invalid patterns
        setPatterns(patternsWithDates);
        if ((window as any).debugLog) {
          (window as any).debugLog('info', `Loaded ${patternsWithDates.length} patterns from localStorage`);
        }
        
        // Sync all patterns with Rust engine
        (async () => {
          for (const pattern of patternsWithDates) {
            console.log(`[Patterns] Syncing '${pattern.name}' (enabled: ${pattern.enabled}, nodes: ${pattern.nodes?.length || 0})`);
            try {
              await invoke('add_pattern', { pattern });
              console.log(`[Patterns] ✅ Synced '${pattern.name}' to Rust`);
            } catch (error) {
              console.error(`[Patterns] ❌ Failed to sync '${pattern.name}':`, error);
            }
          }
        })();
        
      } catch (error) {
        console.error('Failed to load patterns:', error);
        // Clear corrupted data
        localStorage.removeItem(STORAGE_KEY);
        if ((window as any).debugLog) {
          (window as any).debugLog('error', 'Failed to load patterns from localStorage. Data cleared.');
        }
      }
    }
    setLoading(false);
  }, []);

  // Save patterns to localStorage
  const savePatterns = useCallback((newPatterns: Pattern[]) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(newPatterns));
    if ((window as any).debugLog) {
      (window as any).debugLog('info', `Saved ${newPatterns.length} patterns`);
    }
  }, []);

  const addPattern = useCallback(async (name: string, nodes: Node[], edges: Edge[]) => {
    const newPattern: Pattern = {
      id: Date.now().toString(),
      name,
      enabled: true,
      nodes,
      edges,
      createdAt: new Date(),
    };
    
    // Save to localStorage
    setPatterns(prev => {
      const updated = [...prev, newPattern];
      savePatterns(updated);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `Pattern created: ${name} (total: ${updated.length})`);
      }
      
      return updated;
    });
    
    // Sync with Rust engine
    try {
      await invoke('add_pattern', { pattern: newPattern });
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `✅ Pattern '${name}' synced to Rust engine`);
      }
    } catch (error) {
      console.error('Failed to sync pattern with Rust:', error);
      
      // Check if pattern is too complex for engine sync
      const hasComplexNodes = nodes.some(n => 
        ['condition', 'logic', 'multiCondition'].includes(n.type || '')
      );
      
      if ((window as any).debugLog) {
        if (hasComplexNodes) {
          (window as any).debugLog('warn', `⚠️ Pattern '${name}' uses advanced nodes (Condition/Logic). Pattern saved but won't trigger automatically. For auto-triggers, use simple Input→Vibration→Output flow.`);
        } else {
          (window as any).debugLog('error', `❌ Failed to sync pattern: ${error}`);
        }
      }
    }
  }, [savePatterns]);

  const addFullPattern = useCallback(async (pattern: Pattern) => {
    // Ensure the pattern has all required fields with strict validation
    const newPattern: Pattern = {
      id: pattern.id || Date.now().toString(),
      name: typeof pattern.name === 'string' ? pattern.name : 'Unnamed Pattern',
      nodes: Array.isArray(pattern.nodes) ? pattern.nodes : [],
      edges: Array.isArray(pattern.edges) ? pattern.edges : [],
      enabled: pattern.enabled ?? true,
      createdAt: pattern.createdAt || new Date(),
    };
    
    console.log('[PatternsProvider] Adding pattern:', newPattern.name, 'ID:', newPattern.id);
    
    // Save to localStorage
    setPatterns(prev => {
      const updated = [...prev, newPattern];
      savePatterns(updated);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `Pattern added: ${newPattern.name} (total: ${updated.length})`);
      }
      
      return updated;
    });
    
    // Sync with Rust engine
    try {
      await invoke('add_pattern', { pattern: newPattern });
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `✅ Pattern '${newPattern.name}' synced to Rust engine`);
      }
    } catch (error) {
      console.error('Failed to sync pattern with Rust:', error);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('warn', `⚠️ Pattern '${newPattern.name}' saved locally but not synced to engine`);
      }
    }
  }, [savePatterns]);

  const updatePattern = useCallback(async (id: string, updates: Partial<Pattern>) => {
    let updatedPattern: Pattern | undefined;
    
    setPatterns(prev => {
      const updated = prev.map(p => {
        if (p.id === id) {
          updatedPattern = { ...p, ...updates };
          return updatedPattern;
        }
        return p;
      });
      savePatterns(updated);
      return updated;
    });
    
    // Try to sync with Rust engine
    if (updatedPattern) {
      try {
        await invoke('add_pattern', { pattern: updatedPattern });
        
        if ((window as any).debugLog) {
          (window as any).debugLog('success', `✅ Pattern '${updatedPattern.name}' updated & synced`);
        }
      } catch (error) {
        console.error('Failed to sync updated pattern with Rust:', error);
        
        // Check if pattern is too complex
        const hasComplexNodes = updatedPattern.nodes?.some(n => 
          ['condition', 'logic', 'multiCondition'].includes(n.type || '')
        );
        
        if ((window as any).debugLog) {
          if (hasComplexNodes) {
            (window as any).debugLog('warn', `⚠️ Pattern '${updatedPattern.name}' uses advanced nodes. Saved locally but won't auto-trigger.`);
          } else {
            (window as any).debugLog('error', `❌ Failed to sync pattern: ${error}`);
          }
        }
      }
    }
  }, [savePatterns]);

  const deletePattern = useCallback((id: string) => {
    setPatterns(prev => {
      const pattern = prev.find(p => p.id === id);
      const updated = prev.filter(p => p.id !== id);
      savePatterns(updated);
      
      if ((window as any).debugLog && pattern) {
        (window as any).debugLog('warn', `Pattern deleted: ${pattern.name}`);
      }
      return updated;
    });
  }, [savePatterns]);

  const togglePattern = useCallback(async (id: string) => {
    // Get current state and toggle
    const pattern = patterns.find(p => p.id === id);
    if (!pattern) return;
    
    const newEnabled = !pattern.enabled;
    
    // Update in localStorage
    setPatterns(prev => {
      const updated = prev.map(p => 
        p.id === id ? { ...p, enabled: newEnabled } : p
      );
      savePatterns(updated);
      return updated;
    });
    
    // Sync with Rust engine
    try {
      await invoke('toggle_pattern', { id, enabled: newEnabled });
      
      if ((window as any).debugLog) {
        (window as any).debugLog(
          newEnabled ? 'success' : 'warn', 
          `${newEnabled ? '✅' : '⏸'} Pattern '${pattern.name}' ${newEnabled ? 'enabled' : 'disabled'}`
        );
      }
    } catch (error) {
      console.error('Failed to toggle pattern in Rust:', error);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Failed to toggle pattern: ${error}`);
      }
    }
  }, [patterns, savePatterns]);

  return (
    <PatternsContext.Provider value={{
      patterns,
      loading,
      addPattern,
      addFullPattern,
      updatePattern,
      deletePattern,
      togglePattern,
    }}>
      {children}
    </PatternsContext.Provider>
  );
}

export function usePatterns() {
  const context = useContext(PatternsContext);
  if (!context) {
    throw new Error('usePatterns must be used within PatternsProvider');
  }
  return context;
}

