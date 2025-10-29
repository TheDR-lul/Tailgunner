import { useState, useEffect, useCallback, useRef } from 'react';

interface UsePollingOptions<T = any> {
  enabled?: boolean;
  onError?: (error: Error) => void;
  initialData?: T | null;
}

export function usePolling<T>(
  fetcher: () => Promise<T>,
  interval: number,
  options: UsePollingOptions<T> = {}
): {
  data: T | null;
  error: Error | null;
  loading: boolean;
  refetch: () => Promise<void>;
} {
  const { enabled = true, onError, initialData = null } = options;
  
  const [data, setData] = useState<T | null>(initialData);
  const [error, setError] = useState<Error | null>(null);
  const [loading, setLoading] = useState(false);
  
  const fetcherRef = useRef(fetcher);
  const onErrorRef = useRef(onError);
  
  // Keep refs up to date
  useEffect(() => {
    fetcherRef.current = fetcher;
    onErrorRef.current = onError;
  }, [fetcher, onError]);

  const poll = useCallback(async () => {
    setLoading(true);
    try {
      const result = await fetcherRef.current();
      setData(result);
      setError(null);
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error);
      onErrorRef.current?.(error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!enabled) {
      if (data !== initialData) setData(initialData);
      if (error !== null) setError(null);
      return;
    }

    poll(); // Initial fetch
    const id = setInterval(poll, interval);

    return () => clearInterval(id);
  }, [poll, interval, enabled, initialData, data, error]);

  return { data, error, loading, refetch: poll };
}


