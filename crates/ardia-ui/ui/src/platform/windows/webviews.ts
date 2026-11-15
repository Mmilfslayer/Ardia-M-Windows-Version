import { invoke } from '@tauri-apps/api/core';

export interface WebViewConfig {
  windowId: string;
  tabId: string;
  profileId: string;
  incognito: boolean;
  userDataFolder: string;
  enableGpu: boolean;
  enableWebgl: boolean;
  backgroundColor?: string;
  userAgent?: string;
  extraHeaders?: Record<string, string>;
}

export interface WebViewState {
  tabId: string;
  windowId: string;
  url: string;
  title: string;
  loading: boolean;
  canGoBack: boolean;
  canGoForward: boolean;
  muted: boolean;
  processId: number | null;
}

export async function createWebView(config: WebViewConfig): Promise<{ webviewId: string }> {
  return invoke('webview_create', { config });
}

export async function destroyWebView(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_destroy', { window_id: windowId, tab_id: tabId });
}

export async function navigateWebView(windowId: string, tabId: string, url: string): Promise<void> {
  return invoke('webview_navigate', { window_id: windowId, tab_id: tabId, url });
}

export async function reloadWebView(windowId: string, tabId: string, bypassCache: boolean = false): Promise<void> {
  return invoke('webview_reload', { window_id: windowId, tab_id: tabId, bypass_cache: bypassCache });
}

export async function stopWebView(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_stop', { window_id: windowId, tab_id: tabId });
}

export async function goBackWebView(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_go_back', { window_id: windowId, tab_id: tabId });
}

export async function goForwardWebView(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_go_forward', { window_id: windowId, tab_id: tabId });
}

export async function executeScriptInWebView(windowId: string, tabId: string, script: string, world: 'main' | 'isolated' = 'isolated'): Promise<any> {
  return invoke('webview_execute_script', { window_id: windowId, tab_id: tabId, script, world });
}

export async function executeScriptInAllFrames(windowId: string, tabId: string, script: string, world: 'main' | 'isolated' = 'isolated'): Promise<any[]> {
  return invoke('webview_execute_script_all_frames', { window_id: windowId, tab_id: tabId, script, world });
}

export async function addScriptToExecuteOnDocumentCreated(windowId: string, tabId: string, script: string): Promise<string> {
  return invoke('webview_add_script_on_document_created', { window_id: windowId, tab_id: tabId, script });
}

export async function removeScriptToExecuteOnDocumentCreated(windowId: string, tabId: string, scriptId: string): Promise<void> {
  return invoke('webview_remove_script_on_document_created', { window_id: windowId, tab_id: tabId, script_id: scriptId });
}

export async function insertCSSInWebView(windowId: string, tabId: string, css: string): Promise<void> {
  return invoke('webview_insert_css', { window_id: windowId, tab_id: tabId, css });
}

export async function removeCSSInWebView(windowId: string, tabId: string, css: string): Promise<void> {
  return invoke('webview_remove_css', { window_id: windowId, tab_id: tabId, css });
}

export async function setWebViewBounds(windowId: string, tabId: string, bounds: { x: number; y: number; width: number; height: number }): Promise<void> {
  return invoke('webview_set_bounds', { window_id: windowId, tab_id: tabId, bounds });
}

export async function getWebViewState(windowId: string, tabId: string): Promise<WebViewState> {
  return invoke('webview_get_state', { window_id: windowId, tab_id: tabId });
}

export async function setWebViewUserAgent(windowId: string, tabId: string, userAgent: string): Promise<void> {
  return invoke('webview_set_user_agent', { window_id: windowId, tab_id: tabId, user_agent: userAgent });
}

export async function setWebViewZoom(windowId: string, tabId: string, zoomFactor: number): Promise<void> {
  return invoke('webview_set_zoom', { window_id: windowId, tab_id: tabId, zoom_factor: zoomFactor });
}

export async function getWebViewZoom(windowId: string, tabId: string): Promise<number> {
  return invoke('webview_get_zoom', { window_id: windowId, tab_id: tabId });
}

export async function printWebView(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_print', { window_id: windowId, tab_id: tabId });
}

export async function printToPdfWebView(windowId: string, tabId: string, options?: {
  landscape?: boolean;
  margins?: { top: number; bottom: number; left: number; right: number };
  pageSize?: 'A4' | 'A3' | 'Legal' | 'Letter' | 'Tabloid';
}): Promise<Uint8Array> {
  return invoke('webview_print_to_pdf', { window_id: windowId, tab_id: tabId, options });
}

export async function captureScreenshotWebView(windowId: string, tabId: string): Promise<string> {
  return invoke('webview_capture_screenshot', { window_id: windowId, tab_id: tabId });
}

export async function showDevTools(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_show_devtools', { window_id: windowId, tab_id: tabId });
}

export async function closeDevTools(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_close_devtools', { window_id: windowId, tab_id: tabId });
}

export async function isDevToolsOpen(windowId: string, tabId: string): Promise<boolean> {
  return invoke('webview_is_devtools_open', { window_id: windowId, tab_id: tabId });
}

export async function setWebViewMuted(windowId: string, tabId: string, muted: boolean): Promise<void> {
  return invoke('webview_set_muted', { window_id: windowId, tab_id: tabId, muted });
}

export async function getWebViewMuted(windowId: string, tabId: string): Promise<boolean> {
  return invoke('webview_get_muted', { window_id: windowId, tab_id: tabId });
}

export async function requestPictureInPicture(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_request_pip', { window_id: windowId, tab_id: tabId });
}

export async function exitPictureInPicture(windowId: string, tabId: string): Promise<void> {
  return invoke('webview_exit_pip', { window_id: windowId, tab_id: tabId });
}

export async function isPictureInPicture(windowId: string, tabId: string): Promise<boolean> {
  return invoke('webview_is_pip', { window_id: windowId, tab_id: tabId });
}

// Cookie management
export async function getCookies(url: string, profileId: string): Promise<Array<{ name: string; value: string; domain: string; path: string; expires: number; secure: boolean; httpOnly: boolean; sameSite: string }>> {
  return invoke('webview_get_cookies', { url, profile_id: profileId });
}

export async function setCookie(url: string, cookie: { name: string; value: string; domain?: string; path?: string; expires?: number; secure?: boolean; httpOnly?: boolean; sameSite?: string }, profileId: string): Promise<void> {
  return invoke('webview_set_cookie', { url, cookie, profile_id: profileId });
}

export async function deleteCookie(url: string, name: string, profileId: string): Promise<void> {
  return invoke('webview_delete_cookie', { url, name, profile_id: profileId });
}

export async function deleteAllCookies(profileId: string): Promise<void> {
  return invoke('webview_delete_all_cookies', { profile_id: profileId });
}

export async function flushCookies(profileId: string): Promise<void> {
  return invoke('webview_flush_cookies', { profile_id: profileId });
}

// Permission management
export async function setPermission(windowId: string, tabId: string, permission: string, state: 'allow' | 'deny' | 'ask'): Promise<void> {
  return invoke('webview_set_permission', { window_id: windowId, tab_id: tabId, permission, state });
}

export async function getPermission(windowId: string, tabId: string, permission: string): Promise<'allow' | 'deny' | 'ask'> {
  return invoke('webview_get_permission', { window_id: windowId, tab_id: tabId, permission });
}

// Download handling
export async function setDownloadPath(windowId: string, tabId: string, path: string): Promise<void> {
  return invoke('webview_set_download_path', { window_id: windowId, tab_id: tabId, path });
}

export async function getDownloadPath(windowId: string, tabId: string): Promise<string> {
  return invoke('webview_get_download_path', { window_id: windowId, tab_id: tabId });
}

// Context menu
export async function setContextMenu(windowId: string, tabId: string, items: Array<{ id: string; label: string; enabled?: boolean; checked?: boolean; submenu?: any[] }>): Promise<void> {
  return invoke('webview_set_context_menu', { window_id: windowId, tab_id: tabId, items });
}

// Find in page
export async function findInPage(windowId: string, tabId: string, query: string, options?: { forward?: boolean; matchCase?: boolean; findNext?: boolean }): Promise<{ activeMatchOrdinal: number; numberOfMatches: number; selectionRect: { x: number; y: number; width: number; height: number } }> {
  return invoke('webview_find_in_page', { window_id: windowId, tab_id: tabId, query, options });
}

export async function stopFinding(windowId: string, tabId: string, action: 'clear' | 'keep' | 'activate'): Promise<void> {
  return invoke('webview_stop_finding', { window_id: windowId, tab_id: tabId, action });
}