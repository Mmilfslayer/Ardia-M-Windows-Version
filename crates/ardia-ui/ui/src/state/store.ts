import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface Tab {
  id: string;
  windowId: string;
  url: string;
  title: string;
  favicon?: string;
  loading: boolean;
  loadProgress: number;
  canGoBack: boolean;
  canGoForward: boolean;
  pinned: boolean;
  muted: boolean;
  active: boolean;
  index: number;
}

export interface WindowState {
  id: string;
  profileId: string;
  tabs: Tab[];
  activeTabId: string | null;
  bounds: { x: number; y: number; width: number; height: number };
  maximized: boolean;
  fullscreen: boolean;
  sidebarOpen: boolean;
  sidebarPanel: 'vertical_tabs' | 'bookmarks' | 'history' | 'extensions' | 'downloads' | 'settings';
  sidebarWidth: number;
  sidebarCollapsed: boolean;
}

export interface Download {
  id: string;
  url: string;
  filename: string;
  path?: string;
  progress: number;
  receivedBytes: number;
  totalBytes: number;
  status: 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled';
  error?: string;
  startTime: number;
  endTime?: number;
}

export interface Bookmark {
  id: string;
  parentId?: string;
  index: number;
  title: string;
  url?: string;
  dateAdded: number;
  dateModified?: number;
  isFolder: boolean;
  children?: Bookmark[];
}

export interface HistoryEntry {
  id: string;
  url: string;
  title: string;
  visitCount: number;
  lastVisitTime: number;
  typedCount: number;
  transitionType: string;
}

export interface Extension {
  id: string;
  name: string;
  version: string;
  description: string;
  enabled: boolean;
  icon?: string;
  permissions: string[];
}

export interface PasswordEntry {
  id: string;
  originUrl: string;
  actionUrl: string;
  username: string;
  password: string;
  dateCreated: number;
  dateLastUsed?: number;
  timesUsed: number;
}

export interface AutocompleteResult {
  url: string;
  title: string;
  description?: string;
  relevance: number;
  resultType: 'url' | 'search' | 'bookmark' | 'history' | 'calculator' | 'bang' | 'extension';
  iconUrl?: string;
  isBookmark: boolean;
  isHistory: boolean;
}

export interface BlockerStats {
  blockedCount: number;
  allowedCount: number;
  redirectedCount: number;
  cosmeticHiddenCount: number;
}

export interface FilterRule {
  id: number;
  rule: string;
  action: 'block' | 'allow' | 'redirect' | 'upgrade' | 'noop';
  domains: string[];
  excludeDomains: string[];
  resourceTypes: string[];
  isEnabled: boolean;
}

export interface SidebarState {
  isOpen: boolean;
  activePanel: 'vertical_tabs' | 'bookmarks' | 'history' | 'extensions' | 'downloads' | 'settings';
  width: number;
  isCollapsed: boolean;
}

export interface PipWindow {
  id: string;
  tabId: string;
  windowId: string;
  bounds: { x: number; y: number; width: number; height: number };
  isMuted: boolean;
  playbackRate: number;
  state: 'inactive' | 'entering' | 'active' | 'exiting';
}

export interface TelemetryEvent {
  id: string;
  eventType: string;
  timestamp: number;
  sessionId: string;
  profileId?: string;
  properties: Record<string, unknown>;
  durationMs?: number;
}

interface AppState {
  // Windows
  windows: Record<string, WindowState>;
  activeWindowId: string | null;

  // Theme
  theme: 'light' | 'dark' | 'system';
  systemTheme: 'light' | 'dark';

  // Downloads
  downloads: Download[];
  showDownloadShelf: boolean;

  // Sidebar
  sidebar: SidebarState;

  // PiP
  pipWindows: PipWindow[];

  // Content blocker
  blockerEnabled: boolean;
  blockerStats: BlockerStats;
  filterRules: FilterRule[];

  // Autocomplete
  autocompleteResults: AutocompleteResult[];
  autocompleteLoading: boolean;

  // Extensions
  extensions: Extension[];

  // Bookmarks
  bookmarks: Bookmark[];

  // History
  history: HistoryEntry[];

  // Passwords
  passwords: PasswordEntry[];

  // Settings
  settings: Record<string, unknown>;

  // Actions
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  setSystemTheme: (theme: 'light' | 'dark') => void;
  toggleTheme: () => void;

  // Window actions
  addWindow: (id: string, profileId: string) => void;
  removeWindow: (id: string) => void;
  setActiveWindow: (id: string) => void;
  updateWindowBounds: (id: string, bounds: WindowState['bounds']) => void;
  setWindowMaximized: (id: string, maximized: boolean) => void;
  setWindowFullscreen: (id: string, fullscreen: boolean) => void;

  // Tab actions
  addTab: (windowId: string, tab: Omit<Tab, 'windowId'>) => void;
  removeTab: (windowId: string, tabId: string) => void;
  updateTab: (windowId: string, tabId: string, updates: Partial<Tab>) => void;
  setActiveTab: (windowId: string, tabId: string) => void;
  reorderTabs: (windowId: string, tabIds: string[]) => void;
  pinTab: (windowId: string, tabId: string, pinned: boolean) => void;
  muteTab: (windowId: string, tabId: string, muted: boolean) => void;
  closeOtherTabs: (windowId: string, tabId: string) => void;
  closeTabsToRight: (windowId: string, tabId: string) => void;
  duplicateTab: (windowId: string, tabId: string) => void;

  // Download actions
  addDownload: (download: Download) => void;
  updateDownload: (id: string, updates: Partial<Download>) => void;
  removeDownload: (id: string) => void;
  clearCompletedDownloads: () => void;
  toggleDownloadShelf: () => void;

  // Sidebar actions
  setSidebarOpen: (open: boolean) => void;
  setSidebarPanel: (panel: SidebarState['activePanel']) => void;
  setSidebarWidth: (width: number) => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
  toggleSidebar: () => void;

  // PiP actions
  addPipWindow: (pip: PipWindow) => void;
  removePipWindow: (id: string) => void;
  updatePipWindow: (id: string, updates: Partial<PipWindow>) => void;

  // Content blocker actions
  setBlockerEnabled: (enabled: boolean) => void;
  setBlockerStats: (stats: BlockerStats) => void;
  setFilterRules: (rules: FilterRule[]) => void;
  addFilterRule: (rule: FilterRule) => void;
  removeFilterRule: (id: number) => void;

  // Autocomplete actions
  setAutocompleteResults: (results: AutocompleteResult[]) => void;
  setAutocompleteLoading: (loading: boolean) => void;

  // Extensions actions
  setExtensions: (extensions: Extension[]) => void;
  addExtension: (extension: Extension) => void;
  updateExtension: (id: string, updates: Partial<Extension>) => void;
  removeExtension: (id: string) => void;

  // Bookmarks actions
  setBookmarks: (bookmarks: Bookmark[]) => void;
  addBookmark: (bookmark: Bookmark) => void;
  updateBookmark: (id: string, updates: Partial<Bookmark>) => void;
  removeBookmark: (id: string) => void;

  // History actions
  setHistory: (history: HistoryEntry[]) => void;
  addHistoryEntry: (entry: HistoryEntry) => void;
  removeHistoryEntry: (id: string) => void;
  clearHistory: () => void;

  // Passwords actions
  setPasswords: (passwords: PasswordEntry[]) => void;
  addPassword: (password: PasswordEntry) => void;
  updatePassword: (id: string, updates: Partial<PasswordEntry>) => void;
  removePassword: (id: string) => void;

  // Settings actions
  setSetting: (key: string, value: unknown) => void;
  getSetting: (key: string) => unknown;
}

export const useStore = create<AppState>()(
  persist(
    (set, get) => ({
      // Initial state
      windows: {},
      activeWindowId: null,
      theme: 'system',
      systemTheme: 'light',
      downloads: [],
      showDownloadShelf: false,
      sidebar: {
        isOpen: true,
        activePanel: 'vertical_tabs',
        width: 280,
        isCollapsed: false,
      },
      pipWindows: [],
      blockerEnabled: true,
      blockerStats: { blockedCount: 0, allowedCount: 0, redirectedCount: 0, cosmeticHiddenCount: 0 },
      filterRules: [],
      autocompleteResults: [],
      autocompleteLoading: false,
      extensions: [],
      bookmarks: [],
      history: [],
      passwords: [],
      settings: {},

      // Theme actions
      setTheme: (theme) => set({ theme }),
      setSystemTheme: (systemTheme) => set({ systemTheme }),
      toggleTheme: () => set((state) => ({
        theme: state.theme === 'light' ? 'dark' : 'light'
      })),

      // Window actions
      addWindow: (id, profileId) => set((state) => ({
        windows: {
          ...state.windows,
          [id]: {
            id,
            profileId,
            tabs: [],
            activeTabId: null,
            bounds: { x: 100, y: 100, width: 1200, height: 800 },
            maximized: false,
            fullscreen: false,
            sidebarOpen: true,
            sidebarPanel: 'vertical_tabs',
            sidebarWidth: 280,
            sidebarCollapsed: false,
          },
        },
        activeWindowId: state.activeWindowId || id,
      })),
      removeWindow: (id) => set((state) => {
        const { [id]: removed, ...windows } = state.windows;
        return {
          windows,
          activeWindowId: state.activeWindowId === id ? Object.keys(windows)[0] || null : state.activeWindowId,
        };
      }),
      setActiveWindow: (id) => set({ activeWindowId: id }),
      updateWindowBounds: (id, bounds) => set((state) => ({
        windows: {
          ...state.windows,
          [id]: { ...state.windows[id], bounds },
        },
      })),
      setWindowMaximized: (id, maximized) => set((state) => ({
        windows: {
          ...state.windows,
          [id]: { ...state.windows[id], maximized },
        },
      })),
      setWindowFullscreen: (id, fullscreen) => set((state) => ({
        windows: {
          ...state.windows,
          [id]: { ...state.windows[id], fullscreen },
        },
      })),

      // Tab actions
      addTab: (windowId, tab) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        const newTabs = [...window.tabs, { ...tab, windowId }];
        return {
          windows: {
            ...state.windows,
            [windowId]: { ...window, tabs: newTabs, activeTabId: tab.id },
          },
        };
      }),
      removeTab: (windowId, tabId) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        const newTabs = window.tabs.filter((t) => t.id !== tabId);
        let newActiveTabId = window.activeTabId;
        if (window.activeTabId === tabId) {
          newActiveTabId = newTabs[newTabs.length - 1]?.id || null;
        }
        return {
          windows: {
            ...state.windows,
            [windowId]: { ...window, tabs: newTabs, activeTabId: newActiveTabId },
          },
        };
      }),
      updateTab: (windowId, tabId, updates) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: window.tabs.map((t) => (t.id === tabId ? { ...t, ...updates } : t)),
            },
          },
        };
      }),
      setActiveTab: (windowId, tabId) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: { ...window, activeTabId: tabId },
          },
        };
      }),
      reorderTabs: (windowId, tabIds) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        const tabMap = new Map(window.tabs.map(t => [t.id, t]));
        const newTabs = tabIds.map((id, index) => ({ ...tabMap.get(id)!, index }));
        return {
          windows: {
            ...state.windows,
            [windowId]: { ...window, tabs: newTabs },
          },
        };
      }),
      pinTab: (windowId, tabId, pinned) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: window.tabs.map((t) => (t.id === tabId ? { ...t, pinned } : t)),
            },
          },
        };
      }),
      muteTab: (windowId, tabId, muted) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: window.tabs.map((t) => (t.id === tabId ? { ...t, muted } : t)),
            },
          },
        };
      }),
      closeOtherTabs: (windowId, tabId) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: window.tabs.filter((t) => t.id === tabId || t.pinned),
              activeTabId: tabId,
            },
          },
        };
      }),
      closeTabsToRight: (windowId, tabId) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        const tabIndex = window.tabs.findIndex(t => t.id === tabId);
        if (tabIndex === -1) return state;
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: window.tabs.filter((t, i) => i <= tabIndex || t.pinned),
            },
          },
        };
      }),
      duplicateTab: (windowId, tabId) => set((state) => {
        const window = state.windows[windowId];
        if (!window) return state;
        const tab = window.tabs.find(t => t.id === tabId);
        if (!tab) return state;
        const newTab = { ...tab, id: crypto.randomUUID(), index: tab.index + 1 };
        return {
          windows: {
            ...state.windows,
            [windowId]: {
              ...window,
              tabs: [
                ...window.tabs.slice(0, tab.index + 1),
                newTab,
                ...window.tabs.slice(tab.index + 1).map(t => ({ ...t, index: t.index + 1 })),
              ],
              activeTabId: newTab.id,
            },
          },
        };
      }),

      // Download actions
      addDownload: (download) => set((state) => ({
        downloads: [download, ...state.downloads],
        showDownloadShelf: true,
      })),
      updateDownload: (id, updates) => set((state) => ({
        downloads: state.downloads.map((d) => (d.id === id ? { ...d, ...updates } : d)),
      })),
      removeDownload: (id) => set((state) => ({
        downloads: state.downloads.filter((d) => d.id !== id),
      })),
      clearCompletedDownloads: () => set((state) => ({
        downloads: state.downloads.filter((d) => d.status !== 'completed'),
      })),
      toggleDownloadShelf: () => set((state) => ({
        showDownloadShelf: !state.showDownloadShelf,
      })),

      // Sidebar actions
      setSidebarOpen: (isOpen) => set((state) => ({
        sidebar: { ...state.sidebar, isOpen },
      })),
      setSidebarPanel: (activePanel) => set((state) => ({
        sidebar: { ...state.sidebar, activePanel, isOpen: true },
      })),
      setSidebarWidth: (width) => set((state) => ({
        sidebar: { ...state.sidebar, width },
      })),
      setSidebarCollapsed: (isCollapsed) => set((state) => ({
        sidebar: { ...state.sidebar, isCollapsed, width: isCollapsed ? 48 : state.sidebar.width },
      })),
      toggleSidebar: () => set((state) => ({
        sidebar: { ...state.sidebar, isOpen: !state.sidebar.isOpen },
      })),

      // PiP actions
      addPipWindow: (pip) => set((state) => ({
        pipWindows: [...state.pipWindows, pip],
      })),
      removePipWindow: (id) => set((state) => ({
        pipWindows: state.pipWindows.filter((p) => p.id !== id),
      })),
      updatePipWindow: (id, updates) => set((state) => ({
        pipWindows: state.pipWindows.map((p) => (p.id === id ? { ...p, ...updates } : p)),
      })),

      // Content blocker actions
      setBlockerEnabled: (blockerEnabled) => set({ blockerEnabled }),
      setBlockerStats: (blockerStats) => set({ blockerStats }),
      setFilterRules: (filterRules) => set({ filterRules }),
      addFilterRule: (rule) => set((state) => ({
        filterRules: [...state.filterRules, rule],
      })),
      removeFilterRule: (id) => set((state) => ({
        filterRules: state.filterRules.filter((r) => r.id !== id),
      })),

      // Autocomplete actions
      setAutocompleteResults: (autocompleteResults) => set({ autocompleteResults }),
      setAutocompleteLoading: (autocompleteLoading) => set({ autocompleteLoading }),

      // Extensions actions
      setExtensions: (extensions) => set({ extensions }),
      addExtension: (extension) => set((state) => ({
        extensions: [...state.extensions, extension],
      })),
      updateExtension: (id, updates) => set((state) => ({
        extensions: state.extensions.map((e) => (e.id === id ? { ...e, ...updates } : e)),
      })),
      removeExtension: (id) => set((state) => ({
        extensions: state.extensions.filter((e) => e.id !== id),
      })),

      // Bookmarks actions
      setBookmarks: (bookmarks) => set({ bookmarks }),
      addBookmark: (bookmark) => set((state) => ({
        bookmarks: [...state.bookmarks, bookmark],
      })),
      updateBookmark: (id, updates) => set((state) => ({
        bookmarks: state.bookmarks.map((b) => (b.id === id ? { ...b, ...updates } : b)),
      })),
      removeBookmark: (id) => set((state) => ({
        bookmarks: state.bookmarks.filter((b) => b.id !== id),
      })),

      // History actions
      setHistory: (history) => set({ history }),
      addHistoryEntry: (entry) => set((state) => ({
        history: [entry, ...state.history],
      })),
      removeHistoryEntry: (id) => set((state) => ({
        history: state.history.filter((h) => h.id !== id),
      })),
      clearHistory: () => set({ history: [] }),

      // Passwords actions
      setPasswords: (passwords) => set({ passwords }),
      addPassword: (password) => set((state) => ({
        passwords: [...state.passwords, password],
      })),
      updatePassword: (id, updates) => set((state) => ({
        passwords: state.passwords.map((p) => (p.id === id ? { ...p, ...updates } : p)),
      })),
      removePassword: (id) => set((state) => ({
        passwords: state.passwords.filter((p) => p.id !== id),
      })),

      // Settings actions
      setSetting: (key, value) => set((state) => ({
        settings: { ...state.settings, [key]: value },
      })),
      getSetting: (key) => get().settings[key],
    }),
    {
      name: 'ardia-store',
      partialize: (state) => ({
        theme: state.theme,
        sidebar: state.sidebar,
        blockerEnabled: state.blockerEnabled,
        settings: state.settings,
      }),
    }
  )
);