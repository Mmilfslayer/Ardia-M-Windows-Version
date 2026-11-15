import React, { createContext, useContext, useState, useCallback, useEffect, ReactNode } from 'react';
import { X } from 'lucide-react';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: ReactNode;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  showCloseButton?: boolean;
  closeOnOverlayClick?: boolean;
  closeOnEscape?: boolean;
}

export function Modal({
  isOpen,
  onClose,
  title,
  children,
  size = 'md',
  showCloseButton = true,
  closeOnOverlayClick = true,
  closeOnEscape = true,
}: ModalProps) {
  const [isExiting, setIsExiting] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setIsExiting(false);
      document.body.style.overflow = 'hidden';
    } else if (!isExiting) {
      document.body.style.overflow = '';
    }
  }, [isOpen]);

  const handleClose = useCallback(() => {
    setIsExiting(true);
    setTimeout(() => {
      onClose();
      document.body.style.overflow = '';
    }, 150);
  }, [onClose]);

  useEffect(() => {
    if (!closeOnEscape) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        handleClose();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, closeOnEscape, handleClose]);

  if (!isOpen && !isExiting) return null;

  const sizeClasses = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-lg',
    xl: 'max-w-xl',
    full: 'max-w-4xl',
  };

  return (
    <div
      className={`fixed inset-0 z-[var(--z-modal)] flex items-center justify-center p-4 ${isExiting ? 'animate-fade-out' : 'animate-fade-in'}`}
      onClick={closeOnOverlayClick ? handleClose : undefined}
      role="dialog"
      aria-modal="true"
      aria-labelledby={title ? 'modal-title' : undefined}
    >
      <div
        className="fixed inset-0 bg-black/50 backdrop-blur-sm transition-opacity"
        aria-hidden="true"
      />
      <div
        className={`relative w-full ${sizeClasses[size]} bg-bg-modal border border-border rounded-xl shadow-xl overflow-hidden ${isExiting ? 'animate-scale-out' : 'animate-scale-in'}`}
        onClick={e => e.stopPropagation()}
      >
        {(title || showCloseButton) && (
          <div className="flex items-center justify-between p-4 border-b border-border">
            {title && (
              <h2 id="modal-title" className="text-lg font-semibold">
                {title}
              </h2>
            )}
            {showCloseButton && (
              <button
                onClick={handleClose}
                className="p-1.5 text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded-lg transition-fast"
                aria-label="Close"
              >
                <X className="w-5 h-5" />
              </button>
            )}
          </div>
        )}
        <div className="p-4">
          {children}
        </div>
      </div>
    </div>
  );
}

interface ConfirmModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: 'default' | 'danger';
  loading?: boolean;
}

export function ConfirmModal({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  variant = 'default',
  loading = false,
}: ConfirmModalProps) {
  return (
    <Modal isOpen={isOpen} onClose={onClose} title={title} size="sm">
      <div className="text-fg-secondary mb-6">{message}</div>
      <div className="flex justify-end gap-3">
        <button
          onClick={onClose}
          disabled={loading}
          className="px-4 py-2 text-sm font-medium text-fg-secondary hover:text-fg-primary hover:bg-bg-hover rounded-lg transition-fast"
        >
          {cancelLabel}
        </button>
        <button
          onClick={onConfirm}
          disabled={loading}
          className={`px-4 py-2 text-sm font-medium rounded-lg transition-fast ${
            variant === 'danger'
              ? 'bg-error text-white hover:bg-error/90'
              : 'bg-accent text-white hover:bg-accent-hover'
          }`}
        >
          {loading ? 'Please wait...' : confirmLabel}
        </button>
      </div>
    </Modal>
  );
}

interface PromptModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (value: string) => void;
  title: string;
  message?: string;
  placeholder?: string;
  defaultValue?: string;
  inputType?: 'text' | 'password' | 'url';
  submitLabel?: string;
  cancelLabel?: string;
  validation?: (value: string) => string | null;
  loading?: boolean;
}

export function PromptModal({
  isOpen,
  onClose,
  onSubmit,
  title,
  message,
  placeholder = '',
  defaultValue = '',
  inputType = 'text',
  submitLabel = 'Submit',
  cancelLabel = 'Cancel',
  validation,
  loading = false,
}: PromptModalProps) {
  const [value, setValue] = useState(defaultValue);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen) {
      setValue(defaultValue);
      setError(null);
      setTimeout(() => inputRef.current?.focus(), 100);
    }
  }, [isOpen, defaultValue]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validation) {
      const err = validation(value);
      if (err) {
        setError(err);
        return;
      }
    }
    onSubmit(value);
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={title} size="sm">
      <form onSubmit={handleSubmit}>
        {message && <div className="text-fg-secondary mb-4">{message}</div>}
        <div className="mb-4">
          <input
            ref={inputRef}
            type={inputType}
            value={value}
            onChange={e => {
              setValue(e.target.value);
              if (error) setError(null);
            }}
            onKeyDown={e => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSubmit(e);
              }
            }}
            placeholder={placeholder}
            className={`w-full px-3 py-2 bg-bg-primary border rounded-lg text-fg-primary placeholder-fg-muted focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent-alpha transition-fast ${
              error ? 'border-error focus:border-error' : 'border-border'
            }`}
            disabled={loading}
            autoComplete="off"
          />
          {error && <div className="text-error text-sm mt-1">{error}</div>}
        </div>
        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={onClose}
            disabled={loading}
            className="px-4 py-2 text-sm font-medium text-fg-secondary hover:text-fg-primary hover:bg-bg-hover rounded-lg transition-fast"
          >
            {cancelLabel}
          </button>
          <button
            type="submit"
            disabled={loading || !value.trim()}
            className={`px-4 py-2 text-sm font-medium rounded-lg transition-fast ${
              !value.trim() || loading
                ? 'bg-border text-fg-muted cursor-not-allowed'
                : 'bg-accent text-white hover:bg-accent-hover'
            }`}
          >
            {loading ? 'Please wait...' : submitLabel}
          </button>
        </div>
      </form>
    </Modal>
  );
}

import { useRef } from 'react';

// ModalContext for global modal management
interface ModalContextType {
  modals: Array<{
    id: string;
    component: ReactNode;
  }>;
  openModal: (component: ReactNode) => string;
  closeModal: (id: string) => void;
  closeAllModals: () => void;
}

const ModalContext = createContext<ModalContextType | null>(null);

export function ModalProvider({ children }: { children: ReactNode }) {
  const [modals, setModals] = useState<Array<{ id: string; component: ReactNode }>>([]);

  const openModal = useCallback((component: ReactNode) => {
    const id = crypto.randomUUID();
    setModals(prev => [...prev, { id, component }]);
    return id;
  }, []);

  const closeModal = useCallback((id: string) => {
    setModals(prev => prev.filter(m => m.id !== id));
  }, []);

  const closeAllModals = useCallback(() => {
    setModals([]);
  }, []);

  const value: ModalContextType = {
    modals,
    openModal,
    closeModal,
    closeAllModals,
  };

  return (
    <ModalContext.Provider value={value}>
      {children}
      {modals.map(({ id, component }) => (
        <Modal key={id} isOpen onClose={() => closeModal(id)} size="lg">
          {component}
        </Modal>
      ))}
    </ModalContext.Provider>
  );
}

export function useModal() {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error('useModal must be used within a ModalProvider');
  }
  return context;
}