import React, { useEffect, useRef, useState, useCallback } from 'react';
import { invoke, WebviewWindow } from '@tauri-apps/api';
import { useStore } from '../state/store';
import { Toolbar } from './Toolbar';
import { TabStrip } from './TabStrip';
import { Sidebar } from './Sidebar';
import { WebView } from './WebView';
import { DownloadShelf } from './DownloadShelf';
import { PipOverlay } from './PipOverlay';

export function BrowserWindow({ windowId }: { windowId: string }) {
  const {
    windows,
    activeWindowId,
    sidebar,
    pipWindows,
    downloads,
    showDownloadShelf,
    toggleDownloadShelf,
  } = useStore();

  const window = windows[windowId];
  const activeTab = window?.tabs.find(t => t.id === window.activeTabId);
  const isActive = activeWindowId === windowId;
  const webviewRef = useRef<HTMLDivElement>(null);

  const handleNavigate = useCallback(async (url: string) => {
    if (!activeTab) return;
    await invoke('tab_navigate', {
      request: { window_id: windowId, tab_id: activeTab.id, url },
    });
  }, [windowId, activeTab]);

  const handleNewTab = useCallback(async () => {
    await invoke('tab_create', {
      request: { window_id: windowId, url: 'about:blank', background: false },
    });
  }, [windowId]);

  const handleCloseTab = useCallback(async (tabId: string) => {
    await invoke('tab_close', { request: { window_id: windowId, tab_id: tabId } });
  }, [windowId]);

  const handleSetActiveTab = useCallback(async (tabId: string) => {
    await invoke('tab_set_active', { request: { window_id: windowId, tab_id: tabId } });
  }, [windowId]);

  const handleReorderTabs = useCallback(async (tabIds: string[]) => {
    await invoke('tab_reorder', { request: { window_id: windowId, tab_ids: tabIds } });
  }, [windowId]);

  const handlePinTab = useCallback(async (tabId: string, pinned: boolean) => {
    await invoke('tab_pin', { request: { window_id: windowId, tab_id: tabId, pinned } });
  }, [windowId]);

  const handleMuteTab = useCallback(async (tabId: string, muted: boolean) => {
    await invoke('tab_mute', { request: { window_id: windowId, tab_id: tabId, muted } });
  }, [windowId]);

  const handleGoBack = useCallback(async () => {
    if (activeTab) {
      await invoke('tab_go_back', { window_id: windowId, tab_id: activeTab.id });
    }
  }, [windowId, activeTab]);

  const handleGoForward = useCallback(async () => {
    if (activeTab) {
      await invoke('tab_go_forward', { window_id: windowId, tab_id: activeTab.id });
    }
  }, [windowId, activeTab]);

  const handleReload = useCallback(async () => {
    if (activeTab) {
      await invoke('tab_reload', { window_id: windowId, tab_id: activeTab.id });
    }
  }, [windowId, activeTab]);

  const handleCloseWindow = useCallback(async () => {
    await invoke('window_close', { window_id: windowId });
  }, [windowId]);

  const handleToggleSidebar = useCallback(async () => {
    const { toggleSidebar } = useStore.getState();
    toggleSidebar();
    await invoke('sidebar_set_open', { open: !sidebar.isOpen });
  }, [sidebar.isOpen]);

  const handleSetSidebarPanel = useCallback(async (panel: typeof sidebar.activePanel) => {
    const { setSidebarPanel } = useStore.getState();
    setSidebarPanel(panel);
    await invoke('sidebar_set_panel', { request: { panel } });
  }, [sidebar.activePanel]);

  const handleEnterPip = useCallback(async () => {
    if (activeTab) {
      await invoke('pip_enter', { request: { tab_id: activeTab.id, window_id: windowId } });
    }
  }, [windowId, activeTab]);

  const handleExitPip = useCallback(async (pipId: string) => {
    await invoke('pip_exit', { pip_id: pipId });
  }, []);

  const handleDownloadAction = useCallback(async (downloadId: string, action: 'pause' | 'resume' | 'cancel') => {
    await invoke(`download_${action}`, { download_id: downloadId });
  }, []);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isActive) return;

      // Ctrl/Cmd + T = New tab
      if ((e.ctrlKey || e.metaKey) && e.key === 't') {
        e.preventDefault();
        handleNewTab();
      }

      // Ctrl/Cmd + W = Close tab
      if ((e.ctrlKey || e.metaKey) && e.key === 'w') {
        e.preventDefault();
        if (activeTab) handleCloseTab(activeTab.id);
      }

      // Ctrl/Cmd + Shift + T = Reopen closed tab
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 't') {
        e.preventDefault();
        // TODO: Implement reopen closed tab
      }

      // Ctrl/Cmd + L = Focus address bar
      if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
        e.preventDefault();
        // Focus address bar
        document.querySelector('[data-address-bar]')?.focus();
      }

      // Ctrl/Cmd + 1-9 = Switch to tab
      if ((e.ctrlKey || e.metaKey) && e.key >= '1' && e.key <= '9') {
        e.preventDefault();
        const index = parseInt(e.key) - 1;
        const tab = window?.tabs[index];
        if (tab) handleSetActiveTab(tab.id);
      }

      // Ctrl/Cmd + Tab = Next tab
      if ((e.ctrlKey || e.metaKey) && e.key === 'Tab' && !e.shiftKey) {
        e.preventDefault();
        if (window?.tabs.length) {
          const currentIndex = window.tabs.findIndex(t => t.id === window.activeTabId);
          const nextIndex = (currentIndex + 1) % window.tabs.length;
          handleSetActiveTab(window.tabs[nextIndex].id);
        }
      }

      // Ctrl/Cmd + Shift + Tab = Previous tab
      if ((e.ctrlKey || e.metaKey) && e.key === 'Tab' && e.shiftKey) {
        e.preventDefault();
        if (window?.tabs.length) {
          const currentIndex = window.tabs.findIndex(t => t.id === window.activeTabId);
          const prevIndex = (currentIndex - 1 + window.tabs.length) % window.tabs.length;
          handleSetActiveTab(window.tabs[prevIndex].id);
        }
      }

      // F5 / Ctrl+R = Reload
      if (e.key === 'F5' || ((e.ctrlKey || e.metaKey) && e.key === 'r')) {
        e.preventDefault();
        handleReload();
      }

      // Escape = Stop loading / Close dialogs
      if (e.key === 'Escape') {
        // TODO: Stop loading, close dialogs
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isActive, window, activeTab, handleNewTab, handleCloseTab, handleSetActiveTab, handleReload]);

  if (!window) {
    return null;
  }

  const windowPipWindows = pipWindows.filter(p => p.windowId === windowId);

  return (
    <div className="flex h-full w-full flex-col" data-window-id={windowId}>
      {/* Title bar area (handled by Tauri) */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Toolbar */}
        <Toolbar
          windowId={windowId}
          activeTab={activeTab}
          tabs={window.tabs}
          onNavigate={handleNavigate}
          onNewTab={handleNewTab}
          onGoBack={handleGoBack}
          onGoForward={handleGoForward}
          onReload={handleReload}
          onCloseTab={handleCloseTab}
          onSetActiveTab={handleSetActiveTab}
          onReorderTabs={handleReorderTabs}
          onPinTab={handlePinTab}
          onMuteTab={handleMuteTab}
          onEnterPip={handleEnterPip}
          onToggleSidebar={handleToggleSidebar}
          sidebarOpen={sidebar.isOpen}
          sidebarPanel={sidebar.activePanel}
          onSetSidebarPanel={handleSetSidebarPanel}
        />

        {/* Main content area */}
        <div className="flex-1 flex overflow-hidden relative">
          {/* Sidebar */}
          <Sidebar
            windowId={windowId}
            isOpen={sidebar.isOpen}
            activePanel={sidebar.activePanel}
            width={sidebar.width}
            isCollapsed={sidebar.isCollapsed}
            tabs={window.tabs}
            activeTabId={window.activeTabId}
            onSetActiveTab={handleSetActiveTab}
            onCloseTab={handleCloseTab}
            onPinTab={handlePinTab}
            onSetPanel={handleSetSidebarPanel}
          />

          {/* Webview area */}
          <div
            className="flex-1 flex flex-col overflow-hidden"
            style={{
              marginLeft: sidebar.isOpen && !sidebar.isCollapsed ? sidebar.width : (sidebar.isOpen && sidebar.isCollapsed ? 48 : 0),
            }}
          >
            {activeTab && (
              <WebView
                ref={webviewRef}
                windowId={windowId}
                tabId={activeTab.id}
                url={activeTab.url}
                title={activeTab.title}
                favicon={activeTab.favicon}
                loading={activeTab.loading}
                loadProgress={activeTab.loadProgress}
                canGoBack={activeTab.canGoBack}
                canGoForward={activeTab.canGoForward}
                muted={activeTab.muted}
                pipWindows={windowPipWindows}
                onExitPip={handleExitPip}
              />
            )}
            {!activeTab && (
              <div className="flex-1 flex items-center justify-center bg-bg-primary">
                <div className="text-center">
                  <div className="text-xl font-medium text-secondary mb-2">New Tab</div>
                  <button
                    className="px-4 py-2 bg-accent text-white rounded-md hover:bg-accent-hover transition-fast"
                    onClick={handleNewTab}
                  >
                    Open a new tab
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Download shelf */}
        <DownloadShelf
          downloads={downloads.filter(d => d.status === 'downloading' || d.status === 'paused')}
          onAction={handleDownloadAction}
          onClose={toggleDownloadShelf}
          visible={showDownloadShelf}
        />

        {/* PiP Overlay */}
        {windowPipWindows.length > 0 && (
          <PipOverlay
            pipWindows={windowPipWindows}
            onExit={handleExitPip}
          />
        )}
      </div>
    </div>
  );
}