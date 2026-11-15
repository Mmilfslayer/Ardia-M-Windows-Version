import React, { useState, useRef, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';
import { useStore } from '../state/store';
import {
  ChevronLeft, ChevronRight, RefreshCw, Home, Plus, ChevronDown,
  Menu, X, Mic, Sparkles, LayoutDashboard, Bookmark, History,
  Extensions, Download, Settings, Sidebar, Volume2, VolumeX,
  Maximize, Minimize, Minus, Search
} from 'lucide-react';

interface ToolbarProps {
  windowId: string;
  activeTab: any;
  tabs: any[];
  onNavigate: (url: string) => void;
  onNewTab: () => void;
  onGoBack: () => void;
  onGoForward: () => void;
  onReload: () => void;
  onCloseTab: (tabId: string) => void;
  onSetActiveTab: (tabId: string) => void;
  onReorderTabs: (tabIds: string[]) => void;
  onPinTab: (tabId: string, pinned: boolean) => void;
  onMuteTab: (tabId: string, muted: boolean) => void;
  onEnterPip: () => void;
  onToggleSidebar: () => void;
  sidebarOpen: boolean;
  sidebarPanel: string;
  onSetSidebarPanel: (panel: string) => void;
}

export function Toolbar({
  windowId,
  activeTab,
  tabs,
  onNavigate,
  onNewTab,
  onGoBack,
  onGoForward,
  onReload,
  onCloseTab,
  onSetActiveTab,
  onReorderTabs,
  onPinTab,
  onMuteTab,
  onEnterPip,
  onToggleSidebar,
  sidebarOpen,
  sidebarPanel,
  onSetSidebarPanel,
}: ToolbarProps) {
  const [addressValue, setAddressValue] = useState('');
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [suggestions, setSuggestions] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const addressBarRef = useRef<HTMLInputElement>(null);
  const suggestionsRef = useRef<HTMLDivElement>(null);
  const { autocompleteResults, autocompleteLoading, setAutocompleteResults, setAutocompleteLoading, theme } = useStore();

  const canGoBack = activeTab?.canGoBack || false;
  const canGoForward = activeTab?.canGoForward || false;
  const isLoading = activeTab?.loading || false;
  const loadProgress = activeTab?.loadProgress || 0;
  const favicon = activeTab?.favicon;
  const title = activeTab?.title || 'New Tab';
  const url = activeTab?.url || '';
  const muted = activeTab?.muted || false;
  const pinned = activeTab?.pinned || false;

  // Sync address bar with active tab
  useEffect(() => {
    if (activeTab && document.activeElement !== addressBarRef.current) {
      setAddressValue(activeTab.url === 'about:blank' ? '' : activeTab.url);
    }
  }, [activeTab]);

  // Update loading state
  useEffect(() => {
    setLoading(isLoading);
  }, [isLoading]);

  const handleAddressChange = useCallback(async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setAddressValue(value);

    if (value.trim().length > 0) {
      setAutocompleteLoading(true);
      try {
        const results = await invoke<any[]>('autocomplete_suggest', {
          request: { input: value, profile_id: 'default' },
        });
        setSuggestions(results);
        setAutocompleteResults(results);
        setShowSuggestions(true);
      } catch (error) {
        console.error('Autocomplete error:', error);
        setShowSuggestions(false);
      } finally {
        setAutocompleteLoading(false);
      }
    } else {
      setShowSuggestions(false);
      setSuggestions([]);
    }
  }, [setAutocompleteResults, setAutocompleteLoading]);

  const handleAddressSubmit = useCallback((e: React.FormEvent) => {
    e.preventDefault();
    const value = addressValue.trim();
    if (value) {
      onNavigate(value);
      setShowSuggestions(false);
      addressBarRef.current?.blur();
    }
  }, [addressValue, onNavigate]);

  const handleSuggestionClick = useCallback((suggestion: any) => {
    onNavigate(suggestion.url);
    setShowSuggestions(false);
    setAddressValue('');
    addressBarRef.current?.blur();
  }, [onNavigate]);

  const handleFocus = useCallback(() => {
    if (addressValue.trim().length > 0) {
      setShowSuggestions(true);
    }
    addressBarRef.current?.select();
  }, [addressValue]);

  const handleBlur = useCallback(() => {
    // Delay to allow click on suggestions
    setTimeout(() => setShowSuggestions(false), 200);
  }, []);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      setShowSuggestions(false);
      addressBarRef.current?.blur();
    }
  }, []);

  const handleNewTabClick = useCallback(() => {
    onNewTab();
  }, [onNewTab]);

  const handleHomeClick = useCallback(() => {
    onNavigate('https://www.google.com');
  }, [onNavigate]);

  const handleGoBackClick = useCallback(() => {
    onGoBack();
  }, [onGoBack]);

  const handleGoForwardClick = useCallback(() => {
    onGoForward();
  }, [onGoForward]);

  const handleReloadClick = useCallback(() => {
    onReload();
  }, [onReload]);

  const handleEnterPipClick = useCallback(() => {
    onEnterPip();
  }, [onEnterPip]);

  const handleMuteClick = useCallback(() => {
    if (activeTab) {
      onMuteTab(activeTab.id, !muted);
    }
  }, [activeTab, muted, onMuteTab]);

  const handlePinClick = useCallback(() => {
    if (activeTab) {
      onPinTab(activeTab.id, !pinned);
    }
  }, [activeTab, pinned, onPinTab]);

  const handleCloseTabClick = useCallback((e: React.MouseEvent, tabId: string) => {
    e.stopPropagation();
    onCloseTab(tabId);
  }, [onCloseTab]);

  const handleTabClick = useCallback((tabId: string) => {
    onSetActiveTab(tabId);
  }, [onSetActiveTab]);

  const handleTabDragStart = useCallback((e: React.DragEvent, tabId: string, index: number) => {
    e.dataTransfer.setData('text/plain', tabId);
    e.dataTransfer.effectAllowed = 'move';
  }, []);

  const handleTabDragOver = useCallback((e: React.DragEvent, index: number) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  }, []);

  const handleTabDrop = useCallback((e: React.DragEvent, targetIndex: number) => {
    e.preventDefault();
    const draggedTabId = e.dataTransfer.getData('text/plain');
    const draggedIndex = tabs.findIndex(t => t.id === draggedTabId);
    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newTabs = [...tabs];
    const [draggedTab] = newTabs.splice(draggedIndex, 1);
    newTabs.splice(targetIndex, 0, draggedTab);
    onReorderTabs(newTabs.map(t => t.id));
  }, [tabs, onReorderTabs]);

  const handleTabDragEnd = useCallback(() => {
    // Clean up drag state if needed
  }, []);

  return (
    <div className="toolbar flex items-center h-[var(--toolbar-height)] px-2 bg-bg-secondary border-b border-border">
      {/* Sidebar toggle */}
      <button
        className={`toolbar-button ${sidebarOpen ? 'bg-bg-hover' : ''}`}
        onClick={onToggleSidebar}
        aria-label="Toggle sidebar"
        data-tooltip="Toggle Sidebar (Ctrl+Shift+B)"
      >
        <Sidebar className="w-5 h-5" />
      </button>

      <div className="divider" />

      {/* Navigation buttons */}
      <button
        className="toolbar-button"
        onClick={handleGoBackClick}
        disabled={!canGoBack}
        aria-label="Back"
        data-tooltip="Back (Alt+Left)"
      >
        <ChevronLeft className="w-5 h-5" />
      </button>

      <button
        className="toolbar-button"
        onClick={handleGoForwardClick}
        disabled={!canGoForward}
        aria-label="Forward"
        data-tooltip="Forward (Alt+Right)"
      >
        <ChevronRight className="w-5 h-5" />
      </button>

      <button
        className={`toolbar-button ${isLoading ? 'animate-spin' : ''}`}
        onClick={handleReloadClick}
        aria-label={isLoading ? 'Stop' : 'Reload'}
        data-tooltip={isLoading ? 'Stop (Esc)' : 'Reload (F5/Ctrl+R)'}
      >
        {isLoading ? (
          <X className="w-5 h-5" />
        ) : (
          <RefreshCw className="w-5 h-5" />
        )}
      </button>

      <button
        className="toolbar-button"
        onClick={handleHomeClick}
        aria-label="Home"
        data-tooltip="Home"
      >
        <Home className="w-5 h-5" />
      </button>

      <div className="divider" />

      {/* Address bar */}
      <form onSubmit={handleAddressSubmit} className="flex-1 flex items-center max-w-2xl mx-4 relative">
        <div className="relative flex-1">
          <input
            ref={addressBarRef}
            type="text"
            value={addressValue}
            onChange={handleAddressChange}
            onFocus={handleFocus}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            placeholder="Search or enter address"
            className="w-full h-32 px-4 py-2 bg-bg-primary border border-border rounded-md text-fg-primary placeholder-fg-muted transition-fast"
            style={{
              borderColor: document.activeElement === addressBarRef.current ? 'var(--border-focus)' : 'var(--border)',
              boxShadow: document.activeElement === addressBarRef.current ? '0 0 0 2px var(--accent-alpha)' : 'none',
            }}
            data-address-bar="true"
            aria-label="Address bar"
            autoComplete="off"
            spellCheck={false}
          />

          {/* Loading progress bar */}
          {isLoading && loadProgress > 0 && (
            <div
              className="absolute bottom-0 left-0 h-0.5 bg-accent transition-fast"
              style={{ width: `${loadProgress}%` }}
            />
          )}

          {/* Favicon */}
          {favicon && !isLoading && (
            <img
              src={favicon}
              alt=""
              className="absolute left-3 top-1/2 -translate-y-1/2 w-16 h-16 rounded"
              loading="lazy"
            />
          )}

          {/* Security indicator */}
          {!favicon && !isLoading && (
            <div className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 flex items-center justify-center">
              <span className="text-xs text-fg-muted">🔒</span>
            </div>
          )}

          {/* Mute button */}
          {muted && (
            <button
              className="absolute right-8 top-1/2 -translate-y-1/2 toolbar-button text-warning"
              onClick={handleMuteClick}
              aria-label="Unmute tab"
              data-tooltip="Tab muted (click to unmute)"
            >
              <VolumeX className="w-4 h-4" />
            </button>
          )}

          {/* PiP button */}
          {!muted && (
            <button
              className="absolute right-8 top-1/2 -translate-y-1/2 toolbar-button"
              onClick={handleEnterPipClick}
              aria-label="Picture in Picture"
              data-tooltip="Picture in Picture"
            >
              <Maximize className="w-4 h-4" />
            </button>
          )}
        </div>

        {/* Suggestions dropdown */}
        {showSuggestions && suggestions.length > 0 && (
          <div
            ref={suggestionsRef}
            className="absolute top-full left-0 right-0 mt-1 bg-bg-modal border border-border rounded-md shadow-lg overflow-hidden z-[var(--z-dropdown)] animate-slide-in"
          >
            {suggestions.map((suggestion, index) => (
              <button
                key={suggestion.url}
                onClick={() => handleSuggestionClick(suggestion)}
                className="w-full flex items-center gap-3 px-3 py-2 text-left hover:bg-bg-hover transition-fast"
              >
                {suggestion.icon_url && (
                  <img src={suggestion.icon_url} alt="" className="w-4 h-4 rounded" />
                )}
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-fg-primary truncate">{suggestion.title}</div>
                  {suggestion.description && (
                    <div className="text-xs text-fg-muted truncate">{suggestion.description}</div>
                  )}
                </div>
                <span className="text-xs text-fg-muted capitalize">{suggestion.resultType}</span>
              </button>
            ))}
          </div>
        )}
      </form>

      <div className="divider" />

      {/* New tab button */}
      <button
        className="toolbar-button"
        onClick={handleNewTabClick}
        aria-label="New tab"
        data-tooltip="New Tab (Ctrl+T)"
      >
        <Plus className="w-5 h-5" />
      </button>

      {/* Tab strip will be rendered here by TabStrip component */}
      <div className="flex-1 min-w-0" />

      {/* Window controls (handled by OS on Windows) */}
      <div className="flex items-center gap-1 ml-2">
        <button
          className="toolbar-button"
          onClick={onToggleSidebar}
          aria-label={sidebarOpen ? 'Collapse sidebar' : 'Expand sidebar'}
          data-tooltip={sidebarOpen ? 'Collapse Sidebar' : 'Expand Sidebar'}
        >
          {sidebarOpen ? <ChevronLeft className="w-5 h-5" /> : <ChevronRight className="w-5 h-5" />}
        </button>
      </div>
    </div>
  );
}