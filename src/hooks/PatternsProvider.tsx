import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { Node, Edge } from 'reactflow';

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
  addPattern: (name: string, nodes: Node[], edges: Edge[]) => void;
  updatePattern: (id: string, updates: Partial<Pattern>) => void;
  deletePattern: (id: string) => void;
  togglePattern: (id: string) => void;
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
        // Convert date strings back to Date objects
        const patternsWithDates = parsed.map((p: any) => ({
          ...p,
          createdAt: new Date(p.createdAt),
        }));
        setPatterns(patternsWithDates);
        if ((window as any).debugLog) {
          (window as any).debugLog('info', `Loaded ${patternsWithDates.length} patterns`);
        }
      } catch (error) {
        console.error('Failed to load patterns:', error);
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

  const addPattern = useCallback((name: string, nodes: Node[], edges: Edge[]) => {
    const newPattern: Pattern = {
      id: Date.now().toString(),
      name,
      enabled: true,
      nodes,
      edges,
      createdAt: new Date(),
    };
    setPatterns(prev => {
      const updated = [...prev, newPattern];
      savePatterns(updated);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `Pattern created: ${name} (total: ${updated.length})`);
      }
      
      return updated;
    });
  }, [savePatterns]);

  const updatePattern = useCallback((id: string, updates: Partial<Pattern>) => {
    setPatterns(prev => {
      const updated = prev.map(p => p.id === id ? { ...p, ...updates } : p);
      savePatterns(updated);
      return updated;
    });
    
    if ((window as any).debugLog) {
      (window as any).debugLog('info', `Pattern updated: ${id}`);
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

  const togglePattern = useCallback((id: string) => {
    setPatterns(prev => {
      const updated = prev.map(p => 
        p.id === id ? { ...p, enabled: !p.enabled } : p
      );
      savePatterns(updated);
      return updated;
    });
  }, [savePatterns]);

  return (
    <PatternsContext.Provider value={{
      patterns,
      loading,
      addPattern,
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

