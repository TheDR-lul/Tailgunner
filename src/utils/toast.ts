import toast from 'react-hot-toast';

export const showToast = {
  success: (message: string) => {
    toast.success(message, {
      duration: 3000,
      position: 'bottom-right',
      style: {
        background: 'rgba(16, 185, 129, 0.1)',
        backdropFilter: 'blur(8px)',
        border: '1px solid rgba(16, 185, 129, 0.3)',
        color: '#10b981',
        fontWeight: 500,
      },
      iconTheme: {
        primary: '#10b981',
        secondary: 'rgba(16, 185, 129, 0.1)',
      },
    });
  },

  error: (message: string) => {
    toast.error(message, {
      duration: 4000,
      position: 'bottom-right',
      style: {
        background: 'rgba(239, 68, 68, 0.1)',
        backdropFilter: 'blur(8px)',
        border: '1px solid rgba(239, 68, 68, 0.3)',
        color: '#ef4444',
        fontWeight: 500,
      },
      iconTheme: {
        primary: '#ef4444',
        secondary: 'rgba(239, 68, 68, 0.1)',
      },
    });
  },

  warning: (message: string) => {
    toast(message, {
      duration: 3500,
      position: 'bottom-right',
      icon: '⚠️',
      style: {
        background: 'rgba(245, 158, 11, 0.1)',
        backdropFilter: 'blur(8px)',
        border: '1px solid rgba(245, 158, 11, 0.3)',
        color: '#f59e0b',
        fontWeight: 500,
      },
    });
  },

  info: (message: string) => {
    toast(message, {
      duration: 3000,
      position: 'bottom-right',
      icon: 'ℹ️',
      style: {
        background: 'rgba(59, 130, 246, 0.1)',
        backdropFilter: 'blur(8px)',
        border: '1px solid rgba(59, 130, 246, 0.3)',
        color: '#3b82f6',
        fontWeight: 500,
      },
    });
  },

  loading: (message: string) => {
    return toast.loading(message, {
      position: 'bottom-right',
      style: {
        background: 'rgba(107, 114, 128, 0.1)',
        backdropFilter: 'blur(8px)',
        border: '1px solid rgba(107, 114, 128, 0.3)',
        color: '#6b7280',
        fontWeight: 500,
      },
    });
  },

  promise: <T,>(
    promise: Promise<T>,
    messages: {
      loading: string;
      success: string | ((data: T) => string);
      error: string | ((err: any) => string);
    }
  ) => {
    return toast.promise(
      promise,
      {
        loading: messages.loading,
        success: messages.success,
        error: messages.error,
      },
      {
        position: 'bottom-right',
        style: {
          backdropFilter: 'blur(8px)',
          fontWeight: 500,
        },
      }
    );
  },
};


