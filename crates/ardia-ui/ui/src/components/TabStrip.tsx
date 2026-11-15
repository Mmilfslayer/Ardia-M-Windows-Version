import React, { useRef, useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { useStore } from '../state/store';
import { X, ChevronDown, Menu, Lock, Volume2, VolumeX, Pin, PinOff } from 'lucide-react';

interface TabStripProps {
  windowId: string;
  tabs: any[];
  activeTabId: string | null;
  onSetActiveTab: (tabId: string) => void;
  onCloseTab: (tabId: string) => void;
  onReorderTabs: (tabIds: string[]) => void;
  onPinTab: (tabId: string, pinned: boolean) => void;
  onMuteTab: (tabId: string, muted: boolean) => void;
  onNewTab: () => void;
}

export function TabStrip({
  windowId,
  tabs,
  activeTabId,
  onSetActiveTab,
  onCloseTab,
  onReorderTabs,
  onPinTab,
  onMuteTab,
  onNewTab,
}: TabStripProps) {
  const [draggedTabId, setDraggedTabId] = useState<string | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const tabRefs = useRef<Map<string, HTMLDivElement>>(new Map());

  const handleTabClick = useCallback((tabId: string) => {
    onSetActiveTab(tabId);
  }, [onSetActiveTab]);

  const handleTabDoubleClick = useCallback((e: React.MouseEvent, tabId: string) => {
    e.stopPropagation();
    // Double-click to rename or something
  }, []);

  const handleCloseClick = useCallback((e: React.MouseEvent, tabId: string) => {
    e.stopPropagation();
    onCloseTab(tabId);
  }, [onCloseTab]);

  const handlePinClick = useCallback((e: React.MouseEvent, tabId: string, pinned: boolean) => {
    e.stopPropagation();
    onPinTab(tabId, !pinned);
  }, [onPinTab]);

  const handleMuteClick = useCallback((e: React.MouseEvent, tabId: string, muted: boolean) => {
    e.stopPropagation();
    onMuteTab(tabId, !muted);
  }, [onMuteTab]);

  const handleContextMenu = useCallback((e: React.MouseEvent, tabId: string) => {
    e.preventDefault();
    // TODO: Show context menu
    console.log('Context menu for tab:', tabId);
  }, []);

  const handleDragStart = useCallback((e: React.DragEvent, tabId: string, index: number) => {
    setDraggedTabId(tabId);
    e.dataTransfer.setData('text/plain', tabId);
    e.dataTransfer.effectAllowed = 'move';
    e.currentTarget.classList.add('dragging');
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent, index: number) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    if (draggedTabId) {
      setDragOverIndex(index);
    }
  }, [draggedTabId]);

  const handleDragLeave = useCallback(() => {
    setDragOverIndex(null);
  }, []);

  const handleDrop = useCallback((e: React.DragEvent, targetIndex: number) => {
    e.preventDefault();
    const draggedTabId = e.dataTransfer.getData('text/plain');
    const draggedIndex = tabs.findIndex(t => t.id === draggedTabId);
    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newTabs = [...tabs];
    const [draggedTab] = newTabs.splice(draggedIndex, 1);
    newTabs.splice(targetIndex, 0, draggedTab);
    onReorderTabs(newTabs.map(t => t.id));
    setDraggedTabId(null);
    setDragOverIndex(null);
  }, [tabs, onReorderTabs]);

  const handleDragEnd = useCallback((e: React.DragEvent) => {
    e.currentTarget.classList.remove('dragging');
    setDraggedTabId(null);
    setDragOverIndex(null);
  }, []);

  const handleKeyDown = useCallback((e: React.KeyboardEvent, tabId: string, index: number) => {
    switch (e.key) {
      case 'Enter':
      case ' ':
        e.preventDefault();
        onSetActiveTab(tabId);
        break;
      case 'Delete':
      case 'Backspace':
        if (e.ctrlKey || e.metaKey) {
          e.preventDefault();
          onCloseTab(tabId);
        }
        break;
      case 'ArrowLeft':
        e.preventDefault();
        const prevTab = tabs[index - 1];
        if (prevTab) onSetActiveTab(prevTab.id);
        break;
      case 'ArrowRight':
        e.preventDefault();
        const nextTab = tabs[index + 1];
        if (nextTab) onSetActiveTab(nextTab.id);
        break;
      case 'Home':
        e.preventDefault();
        if (tabs[0]) onSetActiveTab(tabs[0].id);
        break;
      case 'End':
        e.preventDefault();
        if (tabs[tabs.length - 1]) onSetActiveTab(tabs[tabs.length - 1].id);
        break;
    }
  }, [tabs, onSetActiveTab, onCloseTab]);

  // Separate pinned and unpinned tabs
  const pinnedTabs = tabs.filter(t => t.pinned);
  const unpinnedTabs = tabs.filter(t => !t.pinned);

  const renderTab = useCallback((tab: any, index: number, isPinned: boolean) => (
    <div
      key={tab.id}
      ref={(el) => {
        if (el) tabRefs.current.set(tab.id, el);
        else tabRefs.current.delete(tab.id);
      }}
      className={`tab flex items-center gap-2 h-[var(--tab-height)] px-3 min-w-[var(--tab-min-width)] max-w-[var(--tab-max-width)] rounded-md transition-fast ${
        tab.id === activeTabId ? 'bg-bg-hover' : 'hover:bg-bg-hover'
      } ${isPinned ? 'border-l-2 border-accent' : ''} ${draggedTabId === tab.id ? 'opacity-50' : ''} ${dragOverIndex === index ? 'border-t-2 border-accent' : ''}`}
      onClick={() => handleTabClick(tab.id)}
      onDoubleClick={(e) => handleTabDoubleClick(e, tab.id)}
      onContextMenu={(e) => handleContextMenu(e, tab.id)}
      onDragStart={(e) => handleDragStart(e, tab.id, index)}
      onDragOver={(e) => handleDragOver(e, index)}
      onDragLeave={handleDragLeave}
      onDrop={(e) => handleDrop(e, index)}
      onDragEnd={handleDragEnd}
      onKeyDown={(e) => handleKeyDown(e, tab.id, index)}
      draggable={true}
      tabIndex={0}
      role="tab"
      aria-selected={tab.id === activeTabId}
      aria-label={tab.title}
      data-tab-id={tab.id}
      style={{ width: isPinned ? 'auto' : undefined }}
    >
      {/* Favicon */}
      {tab.favicon && (
        <img
          src={tab.favicon}
          alt=""
          className="w-16 h-16 rounded flex-shrink-0"
          loading="lazy"
        />
      )}

      {/* Loading indicator */}
      {tab.loading && !tab.favicon && (
        <div className="w-16 h-16 rounded flex-shrink-0 animate-pulse bg-accent-alpha" />
      )}

      {/* Security icon for https */}
      {!tab.favicon && !tab.loading && (
        <span className="w-16 h-16 flex-shrink-0 flex items-center justify-center text-xs text-fg-muted">🔒</span>
      )}

      {/* Title */}
      <span className="flex-1 truncate text-sm font-medium ${tab.id === activeTabId ? 'text-fg-primary' : 'text-fg-secondary'}">
        {tab.title || 'New Tab'}
      </span>

      {/* Audio indicator */}
      {tab.muted ? (
        <button
          onClick={(e) => handleMuteClick(e, tab.id, tab.muted)}
          className="p-1 text-warning hover:bg-bg-hover rounded flex-shrink-0"
          aria-label="Unmute tab"
        >
          <VolumeX className="w-4 h-4" />
        </button>
      ) : tab.title?.includes('🔊') || tab.title?.includes('🎵') ? (
        <span className="text-green-500 flex-shrink-0">
          <Volume2 className="w-4 h-4" />
        </span>
      ) : null}

      {/* Pin button */}
      <button
        onClick={(e) => handlePinClick(e, tab.id, tab.pinned)}
        className={`p-1 flex-shrink-0 transition-fast ${tab.pinned ? 'text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
        aria-label={tab.pinned ? 'Unpin tab' : 'Pin tab'}
      >
        {tab.pinned ? <PinOff className="w-4 h-4" /> : <Pin className="w-4 h-4" />}
      </button>

      {/* Close button */}
      {!isPinned && (
        <button
          onClick={(e) => handleCloseClick(e, tab.id)}
          className="p-1 flex-shrink-0 text-fg-muted hover:text-fg-primary hover:bg-bg-active rounded transition-fast"
          aria-label="Close tab"
        >
          <X className="w-4 h-4" />
        </button>
      )}
    </div>
  ), [activeTabId, draggedTabId, dragOverIndex, handleTabClick, handleTabDoubleClick, handleContextMenu, handleDragStart, handleDragOver, handleDragLeave, handleDrop, handleDragEnd, handleKeyDown, handleCloseClick, handlePinClick, handleMuteClick]);

  return (
    <div className="tab-strip flex items-center h-[var(--tab-height)] px-1 gap-1 overflow-x-auto scrollbar-hide" role="tablist">
      {/* Pinned tabs */}
      {pinnedTabs.length > 0 && (
        <div className="flex items-center gap-1 pr-2 border-r border-border">
          {pinnedTabs.map((tab, index) => renderTab(tab, index, true))}
        </div>
      )}

      {/* Regular tabs */}
      <div className="flex-1 flex items-center gap-1 min-w-0">
        {unpinnedTabs.map((tab, index) => renderTab(tab, index + pinnedTabs.length, false))}
      </div>

      {/* New tab button */}
      <button
        onClick={onNewTab}
        className="tab flex items-center justify-center w-8 h-[var(--tab-height)] text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded-md transition-fast flex-shrink-0"
        aria-label="New tab"
        data-tooltip="New Tab (Ctrl+T)"
      >
        <svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <line x1="12" y1="5" x2="12" y2="19" />
          <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
      </button>

      {/* Tab overflow menu */}
      {tabs.length > 10 && (
        <button className="tab flex items-center justify-center w-8 h-[var(--tab-height)] text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded-md transition-fast flex-shrink-0">
          <ChevronDown className="w-5 h-5" />
        </button>
      )}
    </div>
  );
}

// CSS for scrollbar hiding
const style = document.createElement('style');
style.textContent = `
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
  .tab.dragging {
    opacity: 0.5;
    transform: scale(1.02);
    z-index: 10;
    box-shadow: var(--shadow-lg);
  }
`;
document.head.appendChild(style);