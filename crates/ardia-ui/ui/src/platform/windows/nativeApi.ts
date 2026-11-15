import { invoke } from '@tauri-apps/api/core';

export interface InvokeOptions {
  command: string;
  args?: Record<string, any>;
}

export async function invokeCommand<T>(command: string, args?: Record<string, any>): Promise<T> {
  return invoke<T>(command, args);
}

export async function invokeAsync<T>(command: string, args?: Record<string, any>): Promise<T> {
  return invoke<T>(command, args);
}

// Common command helpers
export const commands = {
  // App
  getVersion: () => invoke<string>('app_get_version'),
  getPlatform: () => invoke<string>('app_get_platform'),
  quit: () => invoke('app_quit'),
  restart: () => invoke('app_restart'),

  // Window
  createWindow: (options?: any) => invoke('window_create', { options }),
  closeWindow: (windowId: string) => invoke('window_close', { window_id: windowId }),
  focusWindow: (windowId: string) => invoke('window_focus', { window_id: windowId }),
  getWindowBounds: (windowId: string) => invoke('window_get_bounds', { window_id: windowId }),
  setWindowBounds: (windowId: string, bounds: any) => invoke('window_set_bounds', { window_id: windowId, bounds }),
  setWindowTitle: (windowId: string, title: string) => invoke('window_set_title', { window_id: windowId, title }),
  toggleFullscreen: (windowId: string) => invoke('window_toggle_fullscreen', { window_id: windowId }),
  minimizeWindow: (windowId: string) => invoke('window_minimize', { window_id: windowId }),
  maximizeWindow: (windowId: string) => invoke('window_maximize', { window_id: windowId }),

  // Tab
  createTab: (windowId: string, options?: any) => invoke('tab_create', { window_id: windowId, options }),
  closeTab: (windowId: string, tabId: string) => invoke('tab_close', { window_id: windowId, tab_id: tabId }),
  navigateTab: (windowId: string, tabId: string, url: string) => invoke('tab_navigate', { window_id: windowId, tab_id: tabId, url }),
  reloadTab: (windowId: string, tabId: string) => invoke('tab_reload', { window_id: windowId, tab_id: tabId }),
  goBack: (windowId: string, tabId: string) => invoke('tab_go_back', { window_id: windowId, tab_id: tabId }),
  goForward: (windowId: string, tabId: string) => invoke('tab_go_forward', { window_id: windowId, tab_id: tabId }),
  getTabInfo: (windowId: string, tabId: string) => invoke('tab_get_info', { window_id: windowId, tab_id: tabId }),
  setActiveTab: (windowId: string, tabId: string) => invoke('tab_set_active', { window_id: windowId, tab_id: tabId }),
  reorderTabs: (windowId: string, tabIds: string[]) => invoke('tab_reorder', { window_id: windowId, tab_ids: tabIds }),
  pinTab: (windowId: string, tabId: string, pinned: boolean) => invoke('tab_pin', { window_id: windowId, tab_id: tabId, pinned }),
  muteTab: (windowId: string, tabId: string, muted: boolean) => invoke('tab_mute', { window_id: windowId, tab_id: tabId, muted }),

  // Profile
  getProfiles: () => invoke('profile_list'),
  createProfile: (name: string) => invoke('profile_create', { name }),
  deleteProfile: (profileId: string) => invoke('profile_delete', { profile_id: profileId }),
  switchProfile: (profileId: string) => invoke('profile_switch', { profile_id: profileId }),
  getCurrentProfile: () => invoke('profile_current'),

  // Settings
  getSetting: (key: string, profileId?: string) => invoke('settings_get', { key, profile_id: profileId }),
  setSetting: (key: string, value: any, profileId?: string) => invoke('settings_set', { key, value, profile_id: profileId }),
  getAllSettings: (profileId?: string) => invoke('settings_get_all', { profile_id: profileId }),
  resetSettings: (profileId?: string) => invoke('settings_reset', { profile_id: profileId }),

  // Bookmarks
  getBookmarks: (profileId: string) => invoke('bookmarks_list', { profile_id: profileId }),
  createBookmark: (bookmark: any, profileId: string) => invoke('bookmarks_create', { bookmark, profile_id: profileId }),
  updateBookmark: (id: string, bookmark: any, profileId: string) => invoke('bookmarks_update', { id, bookmark, profile_id: profileId }),
  deleteBookmark: (id: string, profileId: string) => invoke('bookmarks_delete', { id, profile_id: profileId }),
  createBookmarkFolder: (name: string, parentId: string | null, profileId: string) => invoke('bookmarks_create_folder', { name, parent_id: parentId, profile_id: profileId }),

  // History
  getHistory: (query: string, limit: number, profileId: string) => invoke('history_search', { query, limit, profile_id: profileId }),
  getRecentHistory: (limit: number, profileId: string) => invoke('history_recent', { limit, profile_id: profileId }),
  deleteHistoryItem: (id: string, profileId: string) => invoke('history_delete', { id, profile_id: profileId }),
  clearHistory: (profileId: string) => invoke('history_clear', { profile_id: profileId }),

  // Passwords
  getPasswords: (originUrl: string, profileId: string) => invoke('password_get', { request: { origin_url: originUrl, profile_id: profileId } }),
  getAllPasswords: (profileId: string) => invoke('password_list', { profile_id: profileId }),
  savePassword: (entry: any) => invoke('password_save', { request: entry }),
  deletePassword: (id: string) => invoke('password_delete', { id }),
  fillPassword: (windowId: string, tabId: string, passwordId: string) => invoke('password_fill', { window_id: windowId, tab_id: tabId, password_id: passwordId }),

  // Downloads
  getDownloads: (profileId: string) => invoke('downloads_list', { profile_id: profileId }),
  cancelDownload: (downloadId: string) => invoke('downloads_cancel', { download_id: downloadId }),
  removeDownload: (downloadId: string) => invoke('downloads_remove', { download_id: downloadId }),
  clearDownloads: (profileId: string) => invoke('downloads_clear', { profile_id: profileId }),
  openDownload: (downloadId: string) => invoke('downloads_open', { download_id: downloadId }),
  showDownloadInFolder: (downloadId: string) => invoke('downloads_show_in_folder', { download_id: downloadId }),

  // Extensions
  getExtensions: (profileId: string) => invoke('extensions_list', { profile_id: profileId }),
  installExtension: (path: string, profileId: string) => invoke('extensions_install', { path, profile_id: profileId }),
  uninstallExtension: (extensionId: string, profileId: string) => invoke('extensions_uninstall', { extension_id: extensionId, profile_id: profileId }),
  enableExtension: (extensionId: string, enabled: boolean, profileId: string) => invoke('extensions_set_enabled', { extension_id: extensionId, enabled, profile_id: profileId }),
  getExtensionDetails: (extensionId: string, profileId: string) => invoke('extensions_get_details', { extension_id: extensionId, profile_id: profileId }),

  // Content Blocker
  getBlockerRules: (profileId: string) => invoke('blocker_get_rules', { profile_id: profileId }),
  setBlockerRules: (rules: string[], profileId: string) => invoke('blocker_set_rules', { rules, profile_id: profileId }),
  toggleBlocker: (enabled: boolean, profileId: string) => invoke('blocker_toggle', { enabled, profile_id: profileId }),
  getBlockerStats: (profileId: string) => invoke('blocker_get_stats', { profile_id: profileId }),

  // Sidebar
  getSidebarState: (windowId: string) => invoke('sidebar_get_state', { window_id: windowId }),
  setSidebarState: (windowId: string, state: any) => invoke('sidebar_set_state', { window_id: windowId, state }),
  toggleSidebar: (windowId: string) => invoke('sidebar_toggle', { window_id: windowId }),

  // PiP
  enterPiP: (windowId: string, tabId: string) => invoke('pip_enter', { window_id: windowId, tab_id: tabId }),
  exitPiP: (windowId: string) => invoke('pip_exit', { window_id: windowId }),
  getPiPState: (windowId: string) => invoke('pip_get_state', { window_id: windowId }),

  // Autocomplete
  getAutocompleteSuggestions: (query: string, profileId: string) => invoke('autocomplete_suggest', { query, profile_id: profileId }),

  // Telemetry
  recordEvent: (event: string, properties?: Record<string, any>) => invoke('telemetry_record', { event, properties }),
  getTelemetryEvents: (limit: number) => invoke('telemetry_get_events', { limit }),

  // Import/Export
  importFromBrowser: (browser: string, profileId: string) => invoke('import_from_browser', { browser, profile_id: profileId }),
  importFromFile: (path: string, profileId: string) => invoke('import_from_file', { path, profile_id: profileId }),
  exportProfile: (profileId: string) => invoke('export_profile', { profile_id: profileId }),
};