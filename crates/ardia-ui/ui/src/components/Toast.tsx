import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api';
import { X, CheckCircle, AlertCircle, Info, Loader2 } from 'lucide-react';

interface Toast {
  id: string;
  type: 'success' | 'error' | 'info' | 'loading';
  title: string;
  message?: string;
  duration?: number;
  action?: { label: string; onClick: () => void };
}

interface ToastContextType {
  toasts: Toast[];
  showToast: (toast: Omit<Toast, 'id'>) => string;
  hideToast: (id: string) => void;
  success: (title: string, message?: string, duration?: number) => string;
  error: (title: string, message?: string, duration?: number) => string;
  info: (title: string, message?: string, duration?: number) => string;
  loading: (title: string, message?: string) => string;
  dismissLoading: (id: string) => void;
}

const ToastContext = createContext<ToastContextType | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const showToast = useCallback((toast: Omit<Toast, 'id'>) => {
    const id = crypto.randomUUID();
    const newToast = { ...toast, id };
    setToasts(prev => [...prev, newToast]);

    // Auto-dismiss
    if (toast.duration !== 0 && toast.type !== 'loading') {
      setTimeout(() => hideToast(id), toast.duration || 5000);
    }

    return id;
  }, []);

  const hideToast = useCallback((id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const success = useCallback((title: string, message?: string, duration?: number) => {
    return showToast({ type: 'success', title, message, duration });
  }, [showToast]);

  const error = useCallback((title: string, message?: string, duration?: number) => {
    return showToast({ type: 'error', title, message, duration: duration || 8000 });
  }, [showToast]);

  const info = useCallback((title: string, message?: string, duration?: number) => {
    return showToast({ type: 'info', title, message, duration });
  }, [showToast]);

  const loading = useCallback((title: string, message?: string) => {
    return showToast({ type: 'loading', title, message, duration: 0 });
  }, [showToast]);

  const dismissLoading = useCallback((id: string) => {
    hideToast(id);
  }, [hideToast]);

  const value: ToastContextType = {
    toasts,
    showToast,
    hideToast,
    success,
    error,
    info,
    loading,
    dismissLoading,
  };

  return (
    <ToastContext.Provider value={value}>
      {children}
      <ToastContainer toasts={toasts} onClose={hideToast} />
    </ToastContext.Provider>
  );
}

export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
}

function ToastContainer({ toasts, onClose }: { toasts: Toast[]; onClose: (id: string) => void }) {
  const icons = {
    success: CheckCircle,
    error: AlertCircle,
    info: Info,
    loading: Loader2,
  };

  const colors = {
    success: 'bg-success-light border-success text-success',
    error: 'bg-error-light border-error text-error',
    info: 'bg-accent-light border-accent text-accent',
    loading: 'bg-accent-light border-accent text-accent',
  };

  return (
    <div className="fixed bottom-4 right-4 z-[var(--z-toast)] flex flex-col gap-2 pointer-events-none">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`flex items-start gap-3 p-4 rounded-lg border shadow-lg pointer-events-auto animate-slide-in-right min-w-[300px] max-w-md ${colors[toast.type]}`}
          role="alert"
          aria-live={toast.type === 'error' ? 'assertive' : 'polite'}
        >
          <icons[toast.type] className="w-5 h-5 flex-shrink-0 mt-0.5" />
          <div className="flex-1 min-w-0">
            <div className="font-medium">{toast.title}</div>
            {toast.message && (
              <div className="text-sm opacity-90 mt-1">{toast.message}</div>
            )}
            {toast.action && (
              <button
                onClick={() => {
                  toast.action!.onClick();
                  onClose(toast.id);
                }}
                className="mt-2 text-sm font-medium underline hover:no-underline"
              >
                {toast.action.label}
              </button>
            )}
          </div>
          <button
            onClick={() => onClose(toast.id)}
            className="p-1 opacity-50 hover:opacity-100 transition-opacity flex-shrink-0"
            aria-label="Dismiss"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      ))}
    </div>
  );
}