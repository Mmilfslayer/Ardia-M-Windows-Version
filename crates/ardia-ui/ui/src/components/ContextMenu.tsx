import React, { createContext, useContext, useState, useCallback, useEffect, ReactNode } from 'react';
import { X } from 'lucide-react';

interface ContextMenuItem {
  label: string;
  onClick: () => void;
  icon?: React.ReactNode;
  disabled?: boolean;
  divider?: boolean;
  submenu?: ContextMenuItem[];
  shortcut?: string;
  danger?: boolean;
}

interface ContextMenuState {
  x: number;
  y: number;
  items: ContextMenuItem[];
  visible: boolean;
}

interface ContextMenuContextType {
  openContextMenu: (x: number, y: number, items: ContextMenuItem[]) => void;
  closeContextMenu: () => void;
}

const ContextMenuContext = createContext<ContextMenuContextType | null>(null);

export function ContextMenuProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<ContextMenuState>({
    x: 0,
    y: 0,
    items: [],
    visible: false,
  });

  const openContextMenu = useCallback((x: number, y: number, items: ContextMenuItem[]) => {
    // Adjust position to keep menu within viewport
    const menuWidth = 240; // approximate
    const menuHeight = items.length * 36; // approximate
    const adjustedX = Math.min(x, window.innerWidth - menuWidth - 10);
    const adjustedY = Math.min(y, window.innerHeight - menuHeight - 10);

    setState({
      x: adjustedX,
      y: adjustedY,
      items,
      visible: true,
    });
  }, []);

  const closeContextMenu = useCallback(() => {
    setState(prev => ({ ...prev, visible: false }));
  }, []);

  // Close on outside click
  useEffect(() => {
    const handleClick = () => closeContextMenu();
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') closeContextMenu();
    };

    if (state.visible) {
      document.addEventListener('click', handleClick, true);
      document.addEventListener('keydown', handleKeyDown);
    }

    return () => {
      document.removeEventListener('click', handleClick, true);
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [state.visible, closeContextMenu]);

  const value: ContextMenuContextType = {
    openContextMenu,
    closeContextMenu,
  };

  return (
    <ContextMenuContext.Provider value={value}>
      {children}
      {state.visible && (
        <ContextMenuOverlay
          x={state.x}
          y={state.y}
          items={state.items}
          onClose={closeContextMenu}
        />
      )}
    </ContextMenuContext.Provider>
  );
}

export function useContextMenu() {
  const context = useContext(ContextMenuContext);
  if (!context) {
    throw new Error('useContextMenu must be used within a ContextMenuProvider');
  }
  return context;
}

function ContextMenuOverlay({
  x,
  y,
  items,
  onClose,
}: {
  x: number;
  y: number;
  items: ContextMenuItem[];
  onClose: () => void;
}) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Focus the menu for keyboard navigation
    menuRef.current?.focus();
  }, []);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    const enabledItems = items.filter(item => !item.disabled && !item.divider);
    const focusedIndex = enabledItems.findIndex(item => item === focusedItemRef.current);

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        if (focusedIndex < enabledItems.length - 1) {
          focusItem(enabledItems[focusedIndex + 1]);
        }
        break;
      case 'ArrowUp':
        e.preventDefault();
        if (focusedIndex > 0) {
          focusItem(enabledItems[focusedIndex - 1]);
        }
        break;
      case 'Enter':
      case ' ':
        e.preventDefault();
        if (focusedItemRef.current) {
          focusedItemRef.current.onClick();
          onClose();
        }
        break;
      case 'ArrowRight':
        e.preventDefault();
        if (focusedItemRef.current?.submenu) {
          openSubmenu(focusedItemRef.current.submenu);
        }
        break;
      case 'ArrowLeft':
        e.preventDefault();
        closeSubmenu();
        break;
      case 'Escape':
        onClose();
        break;
    }
  }, [items, onClose]);

  const focusedItemRef = useRef<ContextMenuItem | null>(null);
  const [submenu, setSubmenu] = useState<{ items: ContextMenuItem[]; x: number; y: number } | null>(null);

  const focusItem = (item: ContextMenuItem) => {
    focusedItemRef.current = item;
  };

  const openSubmenu = (submenuItems: ContextMenuItem[]) => {
    const rect = menuRef.current?.getBoundingClientRect();
    if (rect) {
      setSubmenu({
        items: submenuItems,
        x: rect.right,
        y: rect.top + (focusedItemRef.current ? items.indexOf(focusedItemRef.current) * 36 : 0),
      });
    }
  };

  const closeSubmenu = () => {
    setSubmenu(null);
  };

  const renderItems = (itemsToRender: ContextMenuItem[], isSubmenu = false) => (
    itemsToRender.map((item, index) => {
      if (item.divider) {
        return <div key={`divider-${index}`} className="divider-horizontal my-1" />;
      }

      const isFocused = focusedItemRef.current === item;

      return (
        <div
          key={item.label}
          className={`flex items-center gap-2 px-3 py-2 text-sm transition-fast ${
            item.disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:bg-bg-hover'
          } ${isFocused ? 'bg-bg-hover' : ''} ${item.danger ? 'text-error' : 'text-fg-primary'}`}
          onClick={() => {
            if (!item.disabled) {
              item.onClick();
              if (!item.submenu) onClose();
            }
          }}
          onMouseEnter={() => {
            if (!item.disabled) focusItem(item);
          }}
          tabIndex={-1}
          role="menuitem"
          aria-disabled={item.disabled}
        >
          {item.icon && <span className="w-5 h-5 flex-shrink-0">{item.icon}</span>}
          <span className="flex-1 truncate">{item.label}</span>
          {item.shortcut && <span className="text-xs text-fg-muted">{item.shortcut}</span>}
          {item.submenu && <ChevronRight className="w-4 h-4 flex-shrink-0" />}
        </div>
      )
    )
  );

  return (
    <>
      <div
        className="fixed inset-0 z-[var(--z-context-menu)]"
        onClick={onClose}
      />
      <div
        ref={menuRef}
        className={`fixed z-[var(--z-context-menu)] bg-bg-modal border border-border rounded-lg shadow-xl min-w-[200px] max-h-[400px] overflow-auto animate-slide-in`}
        style={{ left: x, top: y }}
        onKeyDown={handleKeyDown}
        onContextMenu={e => e.preventDefault()}
        role="menu"
      >
        {renderItems(items)}
      </div>
      {submenu && (
        <div
          className="fixed z-[var(--z-context-menu)] bg-bg-modal border border-border rounded-lg shadow-xl min-w-[200px] max-h-[400px] overflow-auto animate-slide-in"
          style={{ left: submenu.x, top: submenu.y }}
          role="menu"
        >
          {renderItems(submenu.items, true)}
        </div>
      )}
    </>
  );
}

// Import ChevronRight
import { ChevronRight } from 'lucide-react';
import { useRef } from 'react';