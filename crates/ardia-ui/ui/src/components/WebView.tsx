import React, { useRef, useEffect, useState, useCallback, forwardRef, useImperativeHandle } from 'react';
import { invoke } from '@tauri-apps/api';
import { useStore } from '../state/store';
import { Volume2, VolumeX, Maximize, Minimize, Popcorn, RefreshCw, ChevronLeft, ChevronRight, Home, X } from 'lucide-react';

interface WebViewProps {
  windowId: string;
  tabId: string;
  url: string;
  title: string;
  favicon?: string;
  loading: boolean;
  loadProgress: number;
  canGoBack: boolean;
  canGoForward: boolean;
  muted: boolean;
  pipWindows: any[];
  onExitPip: (pipId: string) => void;
}

export const WebView = forwardRef<HTMLDivElement, WebViewProps>(
  ({ windowId, tabId, url, title, favicon, loading, loadProgress, canGoBack, canGoForward, muted, pipWindows, onExitPip }, ref) => {
    const webviewRef = useRef<HTMLDivElement>(null);
    const [showControls, setShowControls] = useState(false);
    const { blockerEnabled } = useStore();

    useImperativeHandle(ref, () => ({
      focus: () => webviewRef.current?.focus(),
      navigate: (newUrl: string) => invoke('tab_navigate', { request: { window_id: windowId, tab_id: tabId, url: newUrl } }),
      reload: () => invoke('tab_reload', { window_id: windowId, tab_id: tabId }),
      goBack: () => invoke('tab_go_back', { window_id: windowId, tab_id: tabId }),
      goForward: () => invoke('tab_go_forward', { window_id: windowId, tab_id: tabId }),
      executeScript: (script: string) => invoke('webview_execute_script', { window_id: windowId, tab_id: tabId, script }),
    }));

    // Create the WebView using Tauri's webview
    useEffect(() => {
      if (!webviewRef.current) return;

      // Initialize WebView
      const initWebView = async () => {
        try {
          // The actual WebView creation is handled by the Rust side
          // This is just a placeholder for the React component
          // In a real implementation, this would use @tauri-apps/api/webview
        } catch (error) {
          console.error('Failed to initialize WebView:', error);
        }
      };

      initWebView();

      // Cleanup
      return () => {
        // Cleanup WebView if needed
      };
    }, [windowId, tabId]);

    const handleContextMenu = useCallback((e: React.MouseEvent) => {
      e.preventDefault();
      // TODO: Show custom context menu
    }, []);

    const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
      // Handle keyboard shortcuts within the webview
      if (e.key === 'Escape' && loading) {
        invoke('tab_reload', { window_id: windowId, tab_id: tabId, bypass_cache: false });
      }
    }, [windowId, tabId, loading]);

    const handleLoadStart = useCallback(() => {
      // Loading started
    }, []);

    const handleLoadEnd = useCallback(() => {
      // Loading finished
    }, []);

    const handleTitleChange = useCallback((newTitle: string) => {
      // Title changed - handled by events
    }, []);

    const handleFaviconChange = useCallback((newFavicon: string) => {
      // Favicon changed - handled by events
    }, []);

    const handleNavigation = useCallback((newUrl: string) => {
      // Navigation happened - handled by events
    }, []);

    const handleConsoleMessage = useCallback((message: any) => {
      // Console message from webview
      console.log('[WebView]', message);
    }, []);

    // Since we can't embed a real WebView in React without Tauri's webview API,
    // this component renders a placeholder that will be replaced by the actual WebView
    // The actual WebView is created and managed by the Rust side

    return (
      <div
        ref={webviewRef}
        className="webview-container flex-1 flex flex-col overflow-hidden relative bg-bg-primary"
        onContextMenu={handleContextMenu}
        onKeyDown={handleKeyDown}
        data-window-id={windowId}
        data-tab-id={tabId}
      >
        {/* Loading indicator */}
        {loading && (
          <div className="absolute top-0 left-0 right-0 h-1 bg-accent animate-pulse z-10" style={{ width: `${loadProgress}%` }} />
        )}

        {/* Error state */}
        {url.startsWith('ardia-error://') && (
          <div className="flex-1 flex items-center justify-center p-8 text-center">
            <div className="max-w-md">
              <div className="text-6xl mb-4">⚠️</div>
              <h2 className="text-xl font-semibold mb-2">Unable to load page</h2>
              <p className="text-secondary mb-6">{url.replace('ardia-error://', '')}</p>
              <button
                onClick={() => invoke('tab_reload', { window_id: windowId, tab_id: tabId })}
                className="px-4 py-2 bg-accent text-white rounded-md hover:bg-accent-hover transition-fast"
              >
                Try Again
              </button>
            </div>
          </div>
        )}

        {/* Internal pages */}
        {url === 'ardia://newtab' && (
          <NewTabPage windowId={windowId} tabId={tabId} />
        )}

        {url === 'ardia://settings' && (
          <SettingsPage windowId={windowId} tabId={tabId} />
        )}

        {url === 'ardia://history' && (
          <HistoryPage windowId={windowId} tabId={tabId} />
        )}

        {url === 'ardia://bookmarks' && (
          <BookmarksPage windowId={windowId} tabId={tabId} />
        )}

        {url === 'ardia://extensions' && (
          <ExtensionsPage windowId={windowId} tabId={tabId} />
        )}

        {url === 'ardia://downloads' && (
          <DownloadsPage windowId={windowId} tabId={tabId} />
        )}

        {/* The actual WebView will be injected here by the Rust side */}
        <div
          className="webview-placeholder flex-1 w-full"
          id={`webview-${windowId}-${tabId}`}
          style={{ backgroundColor: 'var(--bg-primary)' }}
        >
          {/* This div will be replaced by the actual WebView via Tauri's webview API */}
        </div>

        {/* PiP indicator */}
        {pipWindows.length > 0 && (
          <div className="absolute top-2 right-2 z-20 flex items-center gap-2">
            {pipWindows.map((pip) => (
              <div key={pip.id} className="flex items-center gap-2 px-3 py-1 bg-bg-modal border border-border rounded-lg shadow-lg">
                <Popcorn className="w-4 h-4 text-accent" />
                <span className="text-sm font-medium">PiP Active</span>
                <button
                  onClick={() => onExitPip(pip.id)}
                  className="p-1 text-fg-muted hover:text-fg-primary"
                  aria-label="Exit Picture in Picture"
                >
                  <X className="w-3 h-3" />
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    );
  }
);

WebView.displayName = 'WebView';

// Internal page components
function NewTabPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  const [shortcuts, setShortcuts] = useState<any[]>([]);

  useEffect(() => {
    // Load most visited sites
    invoke<any[]>('history_most_visited', { profile_id: 'default', limit: 8 })
      .then(setShortcuts)
      .catch(console.error);
  }, []);

  const handleNavigate = (url: string) => {
    invoke('tab_navigate', { request: { window_id: windowId, tab_id: tabId, url } });
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center p-8">
      <div className="text-center mb-8">
        <div className="text-4xl font-light text-fg-primary mb-2">Ardia</div>
        <div className="text-secondary">Fast, private, and yours</div>
      </div>

      <div className="relative max-w-2xl w-full">
        <div className="relative">
          <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-fg-muted" />
          <input
            type="text"
            placeholder="Search or enter address"
            className="w-full px-12 py-4 bg-bg-secondary border border-border rounded-lg text-lg text-fg-primary placeholder-fg-muted focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent-alpha transition-fast"
            autoFocus
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleNavigate(e.currentTarget.value);
              }
            }}
          />
        </div>
      </div>

      {shortcuts.length > 0 && (
        <div className="mt-12 w-full max-w-4xl">
          <h3 className="text-sm font-semibold text-fg-secondary mb-4 uppercase tracking-wide">Shortcuts</h3>
          <div className="grid grid-cols-4 gap-3">
            {shortcuts.map((site, index) => (
              <button
                key={site.id}
                onClick={() => handleNavigate(site.url)}
                className="flex flex-col items-center gap-2 p-4 bg-bg-secondary border border-border rounded-lg hover:bg-bg-hover hover:border-accent transition-fast group"
              >
                <div className="w-12 h-12 rounded-xl bg-bg-tertiary flex items-center justify-center group-hover:bg-accent-alpha transition-colors">
                  {site.favicon ? (
                    <img src={site.favicon} alt="" className="w-8 h-8" />
                  ) : (
                    <span className="text-xl font-bold text-fg-primary">
                      {site.url.charAt(8).toUpperCase()}
                    </span>
                  )}
                </div>
                <span className="text-sm font-medium text-fg-primary truncate w-24 text-center">
                  {new URL(site.url).hostname.replace('www.', '')}
                </span>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function SettingsPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  return (
    <div className="flex-1 p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold mb-8">Settings</h1>
      <div className="space-y-6">
        <section>
          <h2 className="text-lg font-medium mb-4">Appearance</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-2">Theme</label>
              <select className="w-full max-w-xs px-3 py-2 bg-bg-primary border border-border rounded-md">
                <option value="system">System</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </div>
          </div>
        </section>
        <section>
          <h2 className="text-lg font-medium mb-4">Privacy & Security</h2>
          <div className="space-y-4">
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" className="w-4 h-4 accent-accent" />
              <span>Block ads and trackers</span>
            </label>
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" className="w-4 h-4 accent-accent" />
              <span>Block third-party cookies</span>
            </label>
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" className="w-4 h-4 accent-accent" />
              <span>Send Do Not Track requests</span>
            </label>
          </div>
        </section>
      </div>
    </div>
  );
}

function HistoryPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  const [history, setHistory] = useState<any[]>([]);
  const [search, setSearch] = useState('');

  useEffect(() => {
    invoke<any[]>('history_get', { profile_id: 'default', limit: 100 })
      .then(setHistory)
      .catch(console.error);
  }, []);

  const handleSearch = async (query: string) => {
    setSearch(query);
    if (query.trim()) {
      const results = await invoke<any[]>('history_search', { request: { profile_id: 'default', query, limit: 50 } });
      setHistory(results);
    } else {
      const results = await invoke<any[]>('history_get', { profile_id: 'default', limit: 100 });
      setHistory(results);
    }
  };

  return (
    <div className="flex-1 p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold mb-6">History</h1>
      <div className="relative mb-6">
        <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-fg-muted" />
        <input
          type="text"
          placeholder="Search history..."
          value={search}
          onChange={(e) => handleSearch(e.target.value)}
          className="w-full px-12 py-3 bg-bg-secondary border border-border rounded-lg text-fg-primary placeholder-fg-muted focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent-alpha"
        />
      </div>
      <div className="space-y-2 max-h-[60vh] overflow-auto">
        {history.map((entry) => (
          <button
            key={entry.id}
            onClick={() => invoke('tab_navigate', { request: { window_id: windowId, tab_id: tabId, url: entry.url } })}
            className="flex items-center gap-4 p-3 bg-bg-secondary border border-border rounded-lg hover:bg-bg-hover hover:border-accent transition-fast w-full text-left"
          >
            <div className="w-10 h-10 rounded bg-bg-tertiary flex items-center justify-center flex-shrink-0">
              <Globe className="w-5 h-5 text-fg-muted" />
            </div>
            <div className="flex-1 min-w-0">
              <div className="font-medium truncate">{entry.title}</div>
              <div className="text-sm text-fg-muted truncate">{entry.url}</div>
            </div>
            <Clock className="w-5 h-5 text-fg-muted flex-shrink-0" />
          </button>
        ))}
      </div>
    </div>
  );
}

function BookmarksPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  return (
    <div className="flex-1 p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold mb-6">Bookmarks</h1>
      <div className="text-center text-secondary py-12">
        Bookmarks manager coming soon
      </div>
    </div>
  );
}

function ExtensionsPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  return (
    <div className="flex-1 p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold mb-6">Extensions</h1>
      <div className="text-center text-secondary py-12">
        Extensions manager coming soon
      </div>
    </div>
  );
}

function DownloadsPage({ windowId, tabId }: { windowId: string; tabId: string }) {
  return (
    <div className="flex-1 p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold mb-6">Downloads</h1>
      <div className="text-center text-secondary py-12">
        Downloads manager coming soon
      </div>
    </div>
  );
}

// Import icons
import { Search, Globe, Clock } from 'lucide-react';