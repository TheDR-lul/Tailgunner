import { useState, useEffect, useCallback } from 'react';
import { Node, Edge } from 'reactflow';

export interface Pattern {
  id: string;
  name: string;
  enabled: boolean;
  nodes: Node[];
  edges: Edge[];
  createdAt: Date;
}

const STORAGE_KEY = 'haptic_patterns';

export function usePatterns() {
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
    const updated = [...patterns, newPattern];
    setPatterns(updated);
    savePatterns(updated);
    
    if ((window as any).debugLog) {
      (window as any).debugLog('success', `Pattern created: ${name}`);
    }
  }, [patterns, savePatterns]);

  const updatePattern = useCallback((id: string, updates: Partial<Pattern>) => {
    const updated = patterns.map(p => p.id === id ? { ...p, ...updates } : p);
    setPatterns(updated);
    savePatterns(updated);
    
    if ((window as any).debugLog) {
      (window as any).debugLog('info', `Pattern updated: ${id}`);
    }
  }, [patterns, savePatterns]);

  const deletePattern = useCallback((id: string) => {
    const pattern = patterns.find(p => p.id === id);
    const updated = patterns.filter(p => p.id !== id);
    setPatterns(updated);
    savePatterns(updated);
    
    if ((window as any).debugLog && pattern) {
      (window as any).debugLog('warn', `Pattern deleted: ${pattern.name}`);
    }
  }, [patterns, savePatterns]);

  const togglePattern = useCallback((id: string) => {
    const updated = patterns.map(p => 
      p.id === id ? { ...p, enabled: !p.enabled } : p
    );
    setPatterns(updated);
    savePatterns(updated);
  }, [patterns, savePatterns]);

  return {
    patterns,
    loading,
    addPattern,
    updatePattern,
    deletePattern,
    togglePattern,
  };
}

