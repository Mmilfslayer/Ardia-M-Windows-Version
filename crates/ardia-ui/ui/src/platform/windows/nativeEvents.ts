import { listen, emit, Event, UnlistenFn } from '@tauri-apps/api/event';

export type NativeEventName =
  | 'window_created'
  | 'window_closed'
  | 'window_focused'
  | 'window_bounds_changed'
  | 'window_title_changed'
  | 'tab_created'
  | 'tab_closed'
  | 'tab_navigated'
  | 'tab_loading_started'
  | 'tab_loading_finished'
  | 'tab_title_changed'
  | 'tab_favicon_changed'
  | 'tab_can_go_back_changed'
  | 'tab_can_go_forward_changed'
  | 'tab_muted_changed'
  | 'tab_pinned_changed'
  | 'tab_crashed'
  | 'active_tab_changed'
  | 'profile_changed'
  | 'settings_changed'
  | 'bookmark_created'
  | 'bookmark_updated'
  | 'bookmark_deleted'
  | 'history_item_added'
  | 'history_cleared'
  | 'download_started'
  | 'download_progress'
  | 'download_completed'
  | 'download_failed'
  | 'download_cancelled'
  | 'extension_installed'
  | 'extension_uninstalled'
  | 'extension_enabled_changed'
  | 'blocker_stats_updated'
  | 'sidebar_toggled'
  | 'sidebar_panel_changed'
  | 'pip_entered'
  | 'pip_exited'
  | 'permission_requested'
  | 'permission_granted'
  | 'permission_denied';

export interface NativeEventMap {
  window_created: { windowId: string; profileId: string };
  window_closed: { windowId: string };
  window_focused: { windowId: string };
  window_bounds_changed: { windowId: string; bounds: { x: number; y: number; width: number; height: number } };
  window_title_changed: { windowId: string; title: string };
  tab_created: { windowId: string; tabId: string; url: string };
  tab_closed: { windowId: string; tabId: string };
  tab_navigated: { windowId: string; tabId: string; url: string; isNewNavigation: boolean };
  tab_loading_started: { windowId: string; tabId: string; url: string };
  tab_loading_finished: { windowId: string; tabId: string; url: string; success: boolean };
  tab_title_changed: { windowId: string; tabId: string; title: string };
  tab_favicon_changed: { windowId: string; tabId: string; favicon: string | null };
  tab_can_go_back_changed: { windowId: string; tabId: string; canGoBack: boolean };
  tab_can_go_forward_changed: { windowId: string; tabId: string; canGoForward: boolean };
  tab_muted_changed: { windowId: string; tabId: string; muted: boolean };
  tab_pinned_changed: { windowId: string; tabId: string; pinned: boolean };
  tab_crashed: { windowId: string; tabId: string; error: string };
  active_tab_changed: { windowId: string; tabId: string | null };
  profile_changed: { profileId: string };
  settings_changed: { key: string; value: any; profileId?: string };
  bookmark_created: { bookmark: any; profileId: string };
  bookmark_updated: { bookmark: any; profileId: string };
  bookmark_deleted: { id: string; profileId: string };
  history_item_added: { item: any; profileId: string };
  history_cleared: { profileId: string };
  download_started: { download: any; profileId: string };
  download_progress: { downloadId: string; progress: number; totalBytes: number; receivedBytes: number };
  download_completed: { downloadId: string; path: string; profileId: string };
  download_failed: { downloadId: string; error: string; profileId: string };
  download_cancelled: { downloadId: string; profileId: string };
  extension_installed: { extensionId: string; profileId: string };
  extension_uninstalled: { extensionId: string; profileId: string };
  extension_enabled_changed: { extensionId: string; enabled: boolean; profileId: string };
  blocker_stats_updated: { stats: any; profileId: string };
  sidebar_toggled: { windowId: string; open: boolean };
  sidebar_panel_changed: { windowId: string; panel: string };
  pip_entered: { windowId: string; tabId: string };
  pip_exited: { windowId: string };
  permission_requested: { windowId: string; tabId: string; permission: string; origin: string };
  permission_granted: { windowId: string; tabId: string; permission: string; origin: string };
  permission_denied: { windowId: string; tabId: string; permission: string; origin: string };
}

export async function listenToNativeEvent<K extends NativeEventName>(
  event: K,
  handler: (payload: NativeEventMap[K]) => void
): Promise<UnlistenFn> {
  return listen<K>(event, (e: Event<NativeEventMap[K]>) => handler(e.payload));
}

export async function emitNativeEvent<K extends NativeEventName>(
  event: K,
  payload: NativeEventMap[K]
): Promise<void> {
  return emit(event, payload);
}

// Helper to listen to multiple events
export async function listenToNativeEvents(
  handlers: Partial<Record<NativeEventName, (payload: any) => void>>
): Promise<UnlistenFn[]> {
  const unlistenFns: UnlistenFn[] = [];
  for (const [event, handler] of Object.entries(handlers)) {
    if (handler) {
      const unlisten = await listenToNativeEvent(event as NativeEventName, handler);
      unlistenFns.push(unlisten);
    }
  }
  return unlistenFns;
}

// Cleanup helper
export function cleanupListeners(unlistenFns: UnlistenFn[]): void {
  for (const unlisten of unlistenFns) {
    unlisten();
  }
}