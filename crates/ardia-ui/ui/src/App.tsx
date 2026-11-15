import React, { useEffect, useState, useCallback } from 'react';
import { invoke, listen, emit, eventChannel, WebviewWindow } from '@tauri-apps/api';
import { useStore } from './state/store';
import { BrowserWindow } from './components/BrowserWindow';
import { ToastProvider } from './components/Toast';
import { ContextMenuProvider } from './components/ContextMenu';
import { ModalProvider } from './components/Modal';

function App() {
  const [initialized, setInitialized] = useState(false);
  const { setWindows, setActiveWindow, setTheme, theme } = useStore();

  useEffect(() => {
    initializeApp();
  }, []);

  const initializeApp = async () => {
    try {
      // Listen for window events
      const unlistenWindowCreated = await listen('window-created', (event) => {
        const { window_id, profile_id } = event.payload as { window_id: string; profile_id: string };
        useStore.getState().addWindow(window_id, profile_id);
      });

      const unlistenWindowClosed = await listen('window-closed', (event) => {
        const { window_id } = event.payload as { window_id: string };
        useStore.getState().removeWindow(window_id);
      });

      const unlistenWindowFocused = await listen('window-focused', (event) => {
        const { window_id } = event.payload as { window_id: string };
        useStore.getState().setActiveWindow(window_id);
      });

      // Listen for tab events
      const unlistenTabCreated = await listen('tab-created', (event) => {
        const { window_id, tab_id, url } = event.payload as { window_id: string; tab_id: string; url: string };
        useStore.getState().addTab(window_id, { id: tab_id, url, title: 'New Tab', loading: true });
      });

      const unlistenTabClosed = await listen('tab-closed', (event) => {
        const { window_id, tab_id } = event.payload as { window_id: string; tab_id: string };
        useStore.getState().removeTab(window_id, tab_id);
      });

      const unlistenTabNavigated = await listen('tab-navigated', (event) => {
        const { window_id, tab_id, url, title } = event.payload as { window_id: string; tab_id: string; url: string; title: string };
        useStore.getState().updateTab(window_id, tab_id, { url, title, loading: false });
      });

      const unlistenTabTitleChanged = await listen('tab-title-changed', (event) => {
        const { window_id, tab_id, title } = event.payload as { window_id: string; tab_id: string; title: string };
        useStore.getState().updateTab(window_id, tab_id, { title });
      });

      const unlistenTabFaviconChanged = await listen('tab-favicon-changed', (event) => {
        const { window_id, tab_id, favicon_url } = event.payload as { window_id: string; tab_id: string; favicon_url: string };
        useStore.getState().updateTab(window_id, tab_id, { favicon: favicon_url });
      });

      const unlistenTabLoadingChanged = await listen('tab-loading-changed', (event) => {
        const { window_id, tab_id, loading, progress } = event.payload as { window_id: string; tab_id: string; loading: boolean; progress: number };
        useStore.getState().updateTab(window_id, tab_id, { loading, loadProgress: progress });
      });

      // Listen for download events
      const unlistenDownloadStarted = await listen('download-started', (event) => {
        const { download_id, url, filename } = event.payload as { download_id: string; url: string; filename: string };
        useStore.getState().addDownload({ id: download_id, url, filename, progress: 0, status: 'downloading' });
      });

      const unlistenDownloadProgress = await listen('download-progress', (event) => {
        const { download_id, progress, received_bytes, total_bytes } = event.payload as { download_id: string; progress: number; received_bytes: number; total_bytes: number };
        useStore.getState().updateDownload(download_id, { progress, receivedBytes: received_bytes, totalBytes: total_bytes });
      });

      const unlistenDownloadCompleted = await listen('download-completed', (event) => {
        const { download_id, path } = event.payload as { download_id: string; path: string };
        useStore.getState().updateDownload(download_id, { status: 'completed', path, progress: 100 });
      });

      const unlistenDownloadFailed = await listen('download-failed', (event) => {
        const { download_id, error } = event.payload as { download_id: string; error: string };
        useStore.getState().updateDownload(download_id, { status: 'failed', error, progress: 0 });
      });

      // Listen for theme changes
      const unlistenThemeChanged = await listen('theme-changed', (event) => {
        const { theme } = event.payload as { theme: string };
        setTheme(theme as 'light' | 'dark' | 'system');
      });

      // Load initial windows
      const windows = await invoke<{ id: string; profile_id: string }[]>('window_list');
      for (const win of windows) {
        useStore.getState().addWindow(win.id, win.profile_id);

        // Load tabs for each window
        const tabs = await invoke<{ id: string; url: string; title: string; loading: boolean; favicon?: string }[]>('tab_list', { window_id: win.id });
        for (const tab of tabs) {
          useStore.getState().addTab(win.id, tab);
        }

        // Set active tab
        const activeTab = tabs.find(t => !t.loading && t.url !== 'about:blank');
        if (activeTab) {
          useStore.getState().setActiveTab(win.id, activeTab.id);
        }
      }

      // Set active window
      if (windows.length > 0) {
        useStore.getState().setActiveWindow(windows[0].id);
      }

      // Cleanup listeners on unmount
      return () => {
        unlistenWindowCreated();
        unlistenWindowClosed();
        unlistenWindowFocused();
        unlistenTabCreated();
        unlistenTabClosed();
        unlistenTabNavigated();
        unlistenTabTitleChanged();
        unlistenTabFaviconChanged();
        unlistenTabLoadingChanged();
        unlistenDownloadStarted();
        unlistenDownloadProgress();
        unlistenDownloadCompleted();
        unlistenDownloadFailed();
        unlistenThemeChanged();
      };
    } catch (error) {
      console.error('Failed to initialize app:', error);
    } finally {
      setInitialized(true);
    }
  };

  if (!initialized) {
    return (
      <div className="flex items-center justify-center h-full w-full">
        <div className="animate-pulse text-secondary">Loading Ardia...</div>
      </div>
    );
  }

  const activeWindowId = useStore(state => state.activeWindowId);
  const windows = useStore(state => state.windows);

  return (
    <ContextMenuProvider>
      <ToastProvider>
        <ModalProvider>
          <div
            className="h-full w-full flex flex-col"
            data-theme={theme}
          >
            {activeWindowId && windows[activeWindowId] && (
              <BrowserWindow windowId={activeWindowId} />
            )}
            {!activeWindowId && (
              <div className="flex-1 flex items-center justify-center">
                <div className="text-center">
                  <div className="text-2xl font-semibold mb-2">Welcome to Ardia</div>
                  <div className="text-secondary mb-4">No windows open</div>
                  <button
                    className="px-4 py-2 bg-accent text-white rounded-md hover:bg-accent-hover transition-fast"
                    onClick={() => invoke('window_create', { request: { profile_id: 'default' } })}
                  >
                    New Window
                  </button>
                </div>
              </div>
            )}
          </div>
        </ModalProvider>
      </ToastProvider>
    </ContextMenuProvider>
  );
}

export default App;