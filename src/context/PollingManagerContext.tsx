import { createContext, useContext, useState, useCallback, ReactNode } from 'react';

interface PollingTask {
  id: string;
  name: string;
  interval: number;
  enabled: boolean;
  lastRun: number;
}

interface PollingManagerContextType {
  tasks: Map<string, PollingTask>;
  registerTask: (id: string, name: string, interval: number) => void;
  unregisterTask: (id: string) => void;
  enableTask: (id: string) => void;
  disableTask: (id: string) => void;
  enableAll: () => void;
  disableAll: () => void;
  updateTaskRun: (id: string) => void;
  globalEnabled: boolean;
  setGlobalEnabled: (enabled: boolean) => void;
}

const PollingManagerContext = createContext<PollingManagerContextType | undefined>(undefined);

export function PollingManagerProvider({ children }: { children: ReactNode }) {
  const [tasks, setTasks] = useState<Map<string, PollingTask>>(new Map());
  const [globalEnabled, setGlobalEnabled] = useState(true);

  const registerTask = useCallback((id: string, name: string, interval: number) => {
    setTasks((prev) => {
      const next = new Map(prev);
      next.set(id, {
        id,
        name,
        interval,
        enabled: true,
        lastRun: Date.now(),
      });
      return next;
    });
  }, []);

  const unregisterTask = useCallback((id: string) => {
    setTasks((prev) => {
      const next = new Map(prev);
      next.delete(id);
      return next;
    });
  }, []);

  const enableTask = useCallback((id: string) => {
    setTasks((prev) => {
      const next = new Map(prev);
      const task = next.get(id);
      if (task) {
        next.set(id, { ...task, enabled: true });
      }
      return next;
    });
  }, []);

  const disableTask = useCallback((id: string) => {
    setTasks((prev) => {
      const next = new Map(prev);
      const task = next.get(id);
      if (task) {
        next.set(id, { ...task, enabled: false });
      }
      return next;
    });
  }, []);

  const enableAll = useCallback(() => {
    setTasks((prev) => {
      const next = new Map(prev);
      next.forEach((task, id) => {
        next.set(id, { ...task, enabled: true });
      });
      return next;
    });
  }, []);

  const disableAll = useCallback(() => {
    setTasks((prev) => {
      const next = new Map(prev);
      next.forEach((task, id) => {
        next.set(id, { ...task, enabled: false });
      });
      return next;
    });
  }, []);

  const updateTaskRun = useCallback((id: string) => {
    setTasks((prev) => {
      const next = new Map(prev);
      const task = next.get(id);
      if (task) {
        next.set(id, { ...task, lastRun: Date.now() });
      }
      return next;
    });
  }, []);

  return (
    <PollingManagerContext.Provider
      value={{
        tasks,
        registerTask,
        unregisterTask,
        enableTask,
        disableTask,
        enableAll,
        disableAll,
        updateTaskRun,
        globalEnabled,
        setGlobalEnabled,
      }}
    >
      {children}
    </PollingManagerContext.Provider>
  );
}

export function usePollingManager() {
  const context = useContext(PollingManagerContext);
  if (!context) {
    throw new Error('usePollingManager must be used within PollingManagerProvider');
  }
  return context;
}

// Enhanced usePolling hook that integrates with PollingManager
import { useState, useEffect, useCallback, useRef } from 'react';

interface UseManagedPollingOptions {
  enabled?: boolean;
  onError?: (error: Error) => void;
  taskName?: string;
}

export function useManagedPolling<T>(
  fetcher: () => Promise<T>,
  interval: number,
  options: UseManagedPollingOptions = {}
): {
  data: T | null;
  error: Error | null;
  loading: boolean;
  refetch: () => Promise<void>;
} {
  const { enabled = true, onError, taskName = 'Unknown Task' } = options;
  const pollingManager = usePollingManager();
  
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [loading, setLoading] = useState(false);
  
  const taskId = useRef(`task-${Math.random().toString(36).substr(2, 9)}`);
  const fetcherRef = useRef(fetcher);
  const onErrorRef = useRef(onError);
  
  // Keep refs up to date
  useEffect(() => {
    fetcherRef.current = fetcher;
    onErrorRef.current = onError;
  }, [fetcher, onError]);

  // Register with PollingManager
  useEffect(() => {
    pollingManager.registerTask(taskId.current, taskName, interval);
    
    return () => {
      pollingManager.unregisterTask(taskId.current);
    };
  }, [pollingManager, taskName, interval]);

  const poll = useCallback(async () => {
    setLoading(true);
    try {
      const result = await fetcherRef.current();
      setData(result);
      setError(null);
      pollingManager.updateTaskRun(taskId.current);
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      onErrorRef.current?.(error);
    } finally {
      setLoading(false);
    }
  }, [pollingManager]);

  useEffect(() => {
    const isEnabled = enabled && pollingManager.globalEnabled;
    
    if (!isEnabled) {
      setData(null);
      setError(null);
      return;
    }

    poll(); // Initial fetch
    const id = setInterval(poll, interval);

    return () => clearInterval(id);
  }, [poll, interval, enabled, pollingManager.globalEnabled]);

  return { data, error, loading, refetch: poll };
}


