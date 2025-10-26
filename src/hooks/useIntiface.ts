import { useState, useEffect } from 'react';
import { api } from '../api';

export function useIntiface(autoConnect: boolean = true) {
  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (autoConnect) {
      connectToIntiface();
    }
  }, [autoConnect]);

  const connectToIntiface = async () => {
    setIsConnecting(true);
    setError(null);
    
    try {
      await api.initDevices();
      setIsConnected(true);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('success', 'Connected to Intiface Central');
      }
    } catch (err: any) {
      const errorMessage = err.message || 'Failed to connect to Intiface';
      setError(errorMessage);
      setIsConnected(false);
      
      if ((window as any).debugLog) {
        (window as any).debugLog('warn', 'Intiface unavailable. Launch Intiface Central.');
        (window as any).debugLog('info', 'Download: https://intiface.com/central/');
      }
    } finally {
      setIsConnecting(false);
    }
  };

  return {
    isConnected,
    isConnecting,
    error,
    connect: connectToIntiface,
  };
}

