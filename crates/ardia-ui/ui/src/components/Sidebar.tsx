import React, { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { useStore } from '../state/store';
import {
  LayoutDashboard, Bookmark, History, Extensions, Download, Settings,
  ChevronRight, ChevronLeft, ChevronDown, ChevronUp, Plus, Search,
  X, Edit, Trash2, Folder, FileText, Clock, Globe, Link2, ExternalLink
} from 'lucide-react';

interface SidebarProps {
  windowId: string;
  isOpen: boolean;
  activePanel: string;
  width: number;
  isCollapsed: boolean;
  tabs: any[];
  activeTabId: string | null;
  onSetActiveTab: (tabId: string) => void;
  onCloseTab: (tabId: string) => void;
  onPinTab: (tabId: string, pinned: boolean) => void;
  onSetPanel: (panel: string) => void;
}

export function Sidebar({
  windowId,
  isOpen,
  activePanel,
  width,
  isCollapsed,
  tabs,
  activeTabId,
  onSetActiveTab,
  onCloseTab,
  onPinTab,
  onSetPanel,
}: SidebarProps) {
  const [bookmarks, setBookmarks] = useState<any[]>([]);
  const [history, setHistory] = useState<any[]>([]);
  const [extensions, setExtensions] = useState<any[]>([]);
  const [downloads, setDownloads] = useState<any[]>([]);
  const [bookmarkSearch, setBookmarkSearch] = useState('');
  const [historySearch, setHistorySearch] = useState('');
  const [expandedFolders, setExpandedFolders] = useState<string[]>([]);

  // Load data for panels
  useEffect(() => {
    loadBookmarks();
    loadHistory();
    loadExtensions();
  }, []);

  const loadBookmarks = async () => {
    try {
      const data = await invoke<any[]>('bookmark_get_tree', { profile_id: 'default' });
      setBookmarks(data);
    } catch (error) {
      console.error('Failed to load bookmarks:', error);
    }
  };

  const loadHistory = async () => {
    try {
      const data = await invoke<any[]>('history_get', { profile_id: 'default', limit: 100 });
      setHistory(data);
    } catch (error) {
      console.error('Failed to load history:', error);
    }
  };

  const loadExtensions = async () => {
    try {
      const data = await invoke<any[]>('extension_get_all');
      setExtensions(data);
    } catch (error) {
      console.error('Failed to load extensions:', error);
    }
  };

  const handleTabClick = useCallback((tabId: string) => {
    onSetActiveTab(tabId);
    // Close sidebar on mobile after selecting tab
    if (window.innerWidth < 1024) {
      // onToggleSidebar(false); // Would need to pass this
    }
  }, [onSetActiveTab]);

  const handleTabClose = useCallback((e: React.MouseEvent, tabId: string) => {
    e.stopPropagation();
    onCloseTab(tabId);
  }, [onCloseTab]);

  const handleTabPin = useCallback((e: React.MouseEvent, tabId: string, pinned: boolean) => {
    e.stopPropagation();
    onPinTab(tabId, !pinned);
  }, [onPinTab]);

  const toggleFolder = useCallback((folderId: string) => {
    setExpandedFolders(prev =>
      prev.includes(folderId)
        ? prev.filter(id => id !== folderId)
        : [...prev, folderId]
    );
  }, []);

  const handleBookmarkClick = useCallback(async (bookmark: any) => {
    if (bookmark.url) {
      await invoke('tab_navigate', {
        request: { window_id: windowId, tab_id: activeTabId, url: bookmark.url },
      });
    }
  }, [windowId, activeTabId]);

  const handleHistoryClick = useCallback(async (entry: any) => {
    await invoke('tab_navigate', {
      request: { window_id: windowId, tab_id: activeTabId, url: entry.url },
    });
  }, [windowId, activeTabId]);

  const handleDownloadAction = useCallback(async (downloadId: string, action: 'pause' | 'resume' | 'cancel') => {
    await invoke(`download_${action}`, { download_id: downloadId });
  }, []);

  const handleClearDownloads = useCallback(async () => {
    await invoke('download_clear');
    setDownloads([]);
  }, []);

  const handleSearchBookmarks = useCallback(async (query: string) => {
    setBookmarkSearch(query);
    if (query.trim()) {
      try {
        const results = await invoke<any[]>('bookmark_search', {
          request: { profile_id: 'default', query },
        });
        setBookmarks(results);
      } catch (error) {
        console.error('Bookmark search failed:', error);
      }
    } else {
      loadBookmarks();
    }
  }, []);

  const handleSearchHistory = useCallback(async (query: string) => {
    setHistorySearch(query);
    if (query.trim()) {
      try {
        const results = await invoke<any[]>('history_search', {
          request: { profile_id: 'default', query, limit: 50 },
        });
        setHistory(results);
      } catch (error) {
        console.error('History search failed:', error);
      }
    } else {
      loadHistory();
    }
  }, []);

  const renderBookmarks = useCallback((items: any[], depth = 0) => (
    items.map((item, index) => {
      if (item.isFolder) {
        const isExpanded = expandedFolders.includes(item.id);
        return (
          <div key={item.id} className="pl-2">
            <button
              onClick={() => toggleFolder(item.id)}
              className="flex items-center gap-2 w-full px-2 py-1 text-left text-sm text-fg-secondary hover:text-fg-primary hover:bg-bg-hover rounded transition-fast"
              aria-expanded={isExpanded}
            >
              <ChevronRight className={`w-4 h-4 transition-fast ${isExpanded ? 'rotate-90' : ''}`} />
              <Folder className="w-4 h-4" />
              <span className="truncate font-medium">{item.title}</span>
            </button>
            {isExpanded && item.children && (
              <div className="mt-1 border-l border-border pl-2">
                {renderBookmarks(item.children, depth + 1)}
              </div>
            )}
          </div>
        );
      } else {
        return (
          <button
            key={item.id}
            onClick={() => handleBookmarkClick(item)}
            className="flex items-center gap-2 w-full px-2 py-1 text-left text-sm text-fg-secondary hover:text-fg-primary hover:bg-bg-hover rounded transition-fast group"
          >
            <Globe className="w-4 h-4 flex-shrink-0" />
            <span className="truncate">{item.title}</span>
          </button>
        );
      }
    })
  ), [expandedFolders, toggleFolder, handleBookmarkClick]);

  const renderHistory = useCallback(() => (
    history.map((entry) => (
      <button
        key={entry.id}
        onClick={() => handleHistoryClick(entry)}
        className="flex items-center gap-3 w-full px-2 py-2 text-left hover:bg-bg-hover rounded transition-fast group"
      >
        <div className="w-8 h-8 rounded bg-bg-tertiary flex items-center justify-center flex-shrink-0">
          <Globe className="w-4 h-4 text-fg-muted" />
        </div>
        <div className="flex-1 min-w-0">
          <div className="font-medium text-sm truncate">{entry.title}</div>
          <div className="text-xs text-fg-muted truncate">{entry.url}</div>
        </div>
        <Clock className="w-4 h-4 text-fg-muted flex-shrink-0" />
      </button>
    ))
  ), [history, handleHistoryClick]);

  const renderExtensions = useCallback(() => (
    extensions.map((ext) => (
      <div key={ext.id} className="flex items-center gap-3 p-3 bg-bg-tertiary rounded-lg">
        <div className="w-10 h-10 rounded bg-bg-secondary flex items-center justify-center flex-shrink-0">
          {ext.icon ? (
            <img src={ext.icon} alt="" className="w-6 h-6 rounded" />
          ) : (
            <Extensions className="w-5 h-5 text-fg-muted" />
          )}
        </div>
        <div className="flex-1 min-w-0">
          <div className="font-medium text-sm truncate">{ext.name}</div>
          <div className="text-xs text-fg-muted">{ext.version} • {ext.permissions.length} permissions</div>
        </div>
        <label className="relative inline-flex items-center cursor-pointer">
          <input
            type="checkbox"
            checked={ext.enabled}
            onChange={() => invoke('extension_enable', { extension_id: ext.id })}
            className="sr-only peer"
          />
          <div className="w-10 h-6 bg-border peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-accent rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent"></div>
        </label>
      </div>
    ))
  ), [extensions]);

  const renderDownloads = useCallback(() => (
    downloads.map((download) => (
      <div key={download.id} className="p-3 bg-bg-tertiary rounded-lg">
        <div className="flex items-center gap-3 mb-2">
          <div className="w-8 h-8 rounded bg-bg-secondary flex items-center justify-center flex-shrink-0">
            <FileText className="w-4 h-4 text-fg-muted" />
          </div>
          <div className="flex-1 min-w-0">
            <div className="font-medium text-sm truncate">{download.filename}</div>
            <div className="text-xs text-fg-muted">
              {download.status === 'downloading' ? `Downloading... ${download.progress}%` :
               download.status === 'completed' ? 'Completed' :
               download.status === 'failed' ? `Failed: ${download.error}` : 'Paused'}
            </div>
          </div>
        </div>
        <div className="h-1.5 bg-bg-secondary rounded-full overflow-hidden">
          <div
            className="h-full bg-accent transition-all duration-300"
            style={{ width: `${download.progress}%` }}
          />
        </div>
        <div className="flex items-center gap-2 mt-2">
          {download.status === 'downloading' && (
            <button
              onClick={() => handleDownloadAction(download.id, 'pause')}
              className="text-xs text-fg-secondary hover:text-fg-primary"
            >
              Pause
            </button>
          )}
          {download.status === 'paused' && (
            <button
              onClick={() => handleDownloadAction(download.id, 'resume')}
              className="text-xs text-fg-secondary hover:text-fg-primary"
            >
              Resume
            </button>
          )}
          <button
            onClick={() => handleDownloadAction(download.id, 'cancel')}
            className="text-xs text-fg-secondary hover:text-fg-primary ml-auto"
          >
            Cancel
          </button>
        </div>
      </div>
    ))
  ), [downloads, handleDownloadAction]);

  if (!isOpen) {
    return (
      <div
        className="sidebar-collapsed fixed top-0 left-0 bottom-0 z-50 bg-bg-secondary border-r border-border transition-all duration-200 flex flex-col"
        style={{ width: isCollapsed ? '48px' : '0' }}
      >
        <button
          onClick={() => onSetPanel('vertical_tabs')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'vertical_tabs' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Vertical Tabs"
          data-tooltip="Vertical Tabs"
        >
          <LayoutDashboard className="w-5 h-5" />
        </button>
        <button
          onClick={() => onSetPanel('bookmarks')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'bookmarks' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Bookmarks"
          data-tooltip="Bookmarks"
        >
          <Bookmark className="w-5 h-5" />
        </button>
        <button
          onClick={() => onSetPanel('history')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'history' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="History"
          data-tooltip="History"
        >
          <History className="w-5 h-5" />
        </button>
        <button
          onClick={() => onSetPanel('extensions')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'extensions' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Extensions"
          data-tooltip="Extensions"
        >
          <Extensions className="w-5 h-5" />
        </button>
        <button
          onClick={() => onSetPanel('downloads')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'downloads' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Downloads"
          data-tooltip="Downloads"
        >
          <Download className="w-5 h-5" />
        </button>
        <div className="flex-1" />
        <button
          onClick={() => onSetPanel('settings')}
          className={`p-3 flex items-center justify-center transition-fast ${activePanel === 'settings' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Settings"
          data-tooltip="Settings"
        >
          <Settings className="w-5 h-5" />
        </button>
      </div>
    );
  }

  const panelIcons: Record<string, any> = {
    vertical_tabs: LayoutDashboard,
    bookmarks: Bookmark,
    history: History,
    extensions: Extensions,
    downloads: Download,
    settings: Settings,
  };

  const ActiveIcon = panelIcons[activePanel] || LayoutDashboard;

  return (
    <div
      className="sidebar fixed top-0 left-0 bottom-0 z-50 bg-bg-secondary border-r border-border flex flex-col transition-all duration-200"
      style={{ width: isCollapsed ? 48 : width }}
    >
      {/* Panel tabs */}
      <div className="flex items-center gap-1 p-2 border-b border-border">
        <button
          onClick={() => onSetPanel('vertical_tabs')}
          className={`flex-1 p-2 flex items-center justify-center gap-2 rounded-md transition-fast ${activePanel === 'vertical_tabs' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Vertical Tabs"
        >
          <LayoutDashboard className="w-5 h-5" />
          {!isCollapsed && <span className="text-sm font-medium">Tabs</span>}
        </button>
        <button
          onClick={() => onSetPanel('bookmarks')}
          className={`flex-1 p-2 flex items-center justify-center gap-2 rounded-md transition-fast ${activePanel === 'bookmarks' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Bookmarks"
        >
          <Bookmark className="w-5 h-5" />
          {!isCollapsed && <span className="text-sm font-medium">Bookmarks</span>}
        </button>
        <button
          onClick={() => onSetPanel('history')}
          className={`flex-1 p-2 flex items-center justify-center gap-2 rounded-md transition-fast ${activePanel === 'history' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="History"
        >
          <History className="w-5 h-5" />
          {!isCollapsed && <span className="text-sm font-medium">History</span>}
        </button>
        <button
          onClick={() => onSetPanel('extensions')}
          className={`flex-1 p-2 flex items-center justify-center gap-2 rounded-md transition-fast ${activePanel === 'extensions' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Extensions"
        >
          <Extensions className="w-5 h-5" />
          {!isCollapsed && <span className="text-sm font-medium">Extensions</span>}
        </button>
        <button
          onClick={() => onSetPanel('downloads')}
          className={`flex-1 p-2 flex items-center justify-center gap-2 rounded-md transition-fast ${activePanel === 'downloads' ? 'bg-bg-hover text-accent' : 'text-fg-muted hover:text-fg-primary'}`}
          aria-label="Downloads"
        >
          <Download className="w-5 h-5" />
          {!isCollapsed && <span className="text-sm font-medium">Downloads</span>}
        </button>
      </div>

      {/* Panel content */}
      <div className="flex-1 overflow-auto p-2">
        {activePanel === 'vertical_tabs' && (
          <div className="flex flex-col gap-1 h-full">
            {tabs.length === 0 ? (
              <div className="flex-1 flex items-center justify-center text-fg-muted text-sm">
                No tabs open
              </div>
            ) : (
              tabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => handleTabClick(tab.id)}
                  onContextMenu={(e) => { e.preventDefault(); /* TODO: context menu */ }}
                  className={`flex items-center gap-2 p-2 rounded-md transition-fast ${
                    tab.id === activeTabId ? 'bg-bg-hover text-fg-primary' : 'text-fg-secondary hover:bg-bg-hover hover:text-fg-primary'
                  } ${tab.pinned ? 'border-l-2 border-accent' : ''}`}
                >
                  {tab.favicon && <img src={tab.favicon} alt="" className="w-16 h-16 rounded flex-shrink-0" />}
                  {!tab.favicon && <div className="w-16 h-16 rounded flex-shrink-0 flex items-center justify-center text-xs text-fg-muted">🔒</div>}
                  <div className="flex-1 min-w-0 truncate text-sm">{tab.title || 'New Tab'}</div>
                  {tab.muted && <VolumeX className="w-4 h-4 text-warning flex-shrink-0" />}
                  {!tab.pinned && (
                    <button
                      onClick={(e) => handleTabClose(e, tab.id)}
                      className="p-1 text-fg-muted hover:text-fg-primary hover:bg-bg-active rounded flex-shrink-0"
                    >
                      <X className="w-3 h-3" />
                    </button>
                  )}
                </button>
              ))
            )}
          </div>
        )}

        {activePanel === 'bookmarks' && (
          <div className="flex flex-col gap-2">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-fg-muted" />
              <input
                type="text"
                placeholder="Search bookmarks..."
                value={bookmarkSearch}
                onChange={(e) => handleSearchBookmarks(e.target.value)}
                className="w-full pl-10 pr-4 py-2 bg-bg-primary border border-border rounded-md text-sm"
              />
            </div>
            <div className="flex-1 overflow-auto">
              {bookmarks.length === 0 ? (
                <div className="flex-1 flex items-center justify-center text-fg-muted text-sm">
                  No bookmarks found
                </div>
              ) : (
                renderBookmarks(bookmarks)
              )}
            </div>
          </div>
        )}

        {activePanel === 'history' && (
          <div className="flex flex-col gap-2">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-fg-muted" />
              <input
                type="text"
                placeholder="Search history..."
                value={historySearch}
                onChange={(e) => handleSearchHistory(e.target.value)}
                className="w-full pl-10 pr-4 py-2 bg-bg-primary border border-border rounded-md text-sm"
              />
            </div>
            <div className="flex-1 overflow-auto">
              {history.length === 0 ? (
                <div className="flex-1 flex items-center justify-center text-fg-muted text-sm">
                  No history found
                </div>
              ) : (
                renderHistory()
              )}
            </div>
          </div>
        )}

        {activePanel === 'extensions' && (
          <div className="flex flex-col gap-2">
            <div className="flex items-center justify-between mb-2">
              <span className="font-medium text-sm">Extensions</span>
              <button className="toolbar-button" onClick={() => invoke('extension_install', { path: '' })}>
                <Plus className="w-5 h-5" />
              </button>
            </div>
            <div className="flex-1 overflow-auto">
              {extensions.length === 0 ? (
                <div className="flex-1 flex items-center justify-center text-fg-muted text-sm">
                  No extensions installed
                </div>
              ) : (
                renderExtensions()
              )}
            </div>
          </div>
        )}

        {activePanel === 'downloads' && (
          <div className="flex flex-col gap-2">
            <div className="flex items-center justify-between mb-2">
              <span className="font-medium text-sm">Downloads</span>
              {downloads.some(d => d.status === 'completed') && (
                <button
                  onClick={handleClearDownloads}
                  className="text-xs text-fg-secondary hover:text-fg-primary"
                >
                  Clear completed
                </button>
              )}
            </div>
            <div className="flex-1 overflow-auto">
              {downloads.length === 0 ? (
                <div className="flex-1 flex items-center justify-center text-fg-muted text-sm">
                  No downloads
                </div>
              ) : (
                renderDownloads()
              )}
            </div>
          </div>
        )}

        {activePanel === 'settings' && (
          <div className="flex flex-col gap-4">
            <h3 className="font-medium">Settings</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">Theme</label>
                <select
                  value={useStore.getState().theme}
                  onChange={(e) => useStore.getState().setTheme(e.target.value as any)}
                  className="w-full px-3 py-2 bg-bg-primary border border-border rounded-md text-sm"
                >
                  <option value="system">System</option>
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-2">Content Blocker</label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={useStore.getState().blockerEnabled}
                    onChange={(e) => useStore.getState().setBlockerEnabled(e.target.checked)}
                    className="w-4 h-4 accent-accent"
                  />
                  <span className="text-sm">Block ads and trackers</span>
                </label>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Resize handle */}
      {!isCollapsed && (
        <div
          className="w-1 cursor-col-resize hover:bg-accent-alpha transition-colors"
          onMouseDown={(e) => {
            e.preventDefault();
            const startX = e.clientX;
            const startWidth = width;
            const handleMouseMove = (moveEvent: MouseEvent) => {
              const newWidth = Math.max(200, Math.min(600, startWidth + (moveEvent.clientX - startX)));
              useStore.getState().setSidebarWidth(newWidth);
              invoke('sidebar_set_width', { width: newWidth });
            };
            const handleMouseUp = () => {
              document.removeEventListener('mousemove', handleMouseMove);
              document.removeEventListener('mouseup', handleMouseUp);
            };
            document.addEventListener('mousemove', handleMouseMove);
            document.addEventListener('mouseup', handleMouseUp);
          }}
        />
      )}
    </div>
  );
}