import { invoke } from '@tauri-apps/api/core';

export interface TelemetryEvent {
  id: string;
  event: string;
  properties: Record<string, any>;
  timestamp: number;
  profileId: string;
  sessionId: string;
}

export interface TelemetryConfig {
  enabled: boolean;
  sampleRate: number;
  maxEvents: number;
  flushIntervalMs: number;
}

export async function recordEvent(event: string, properties?: Record<string, any>): Promise<void> {
  return invoke('telemetry_record', { event, properties });
}

export async function recordTiming(event: string, durationMs: number, properties?: Record<string, any>): Promise<void> {
  return invoke('telemetry_record_timing', { event, duration_ms: durationMs, properties });
}

export async function recordCount(event: string, count: number, properties?: Record<string, any>): Promise<void> {
  return invoke('telemetry_record_count', { event, count, properties });
}

export async function recordError(event: string, error: Error | string, properties?: Record<string, any>): Promise<void> {
  const errorMessage = error instanceof Error ? error.message : error;
  const errorStack = error instanceof Error ? error.stack : undefined;
  return invoke('telemetry_record_error', { event, error: errorMessage, stack: errorStack, properties });
}

export async function getTelemetryConfig(): Promise<TelemetryConfig> {
  return invoke('telemetry_get_config');
}

export async function setTelemetryConfig(config: Partial<TelemetryConfig>): Promise<void> {
  return invoke('telemetry_set_config', { config });
}

export async function getTelemetryEvents(limit: number = 100, profileId?: string): Promise<TelemetryEvent[]> {
  return invoke('telemetry_get_events', { limit, profile_id: profileId });
}

export async function clearTelemetryEvents(profileId?: string): Promise<void> {
  return invoke('telemetry_clear_events', { profile_id: profileId });
}

export async function exportTelemetryData(profileId?: string): Promise<string> {
  return invoke('telemetry_export', { profile_id: profileId });
}

export async function getTelemetryStats(): Promise<{
  totalEvents: number;
  eventsByType: Record<string, number>;
  errorsCount: number;
  oldestEvent: number | null;
  newestEvent: number | null;
}> {
  return invoke('telemetry_get_stats');
}

// Local-only telemetry - no network transmission
// Events stored in local SQLite database
// User can view/export/delete at any time
export const TELEMETRY_EVENTS = {
  // Browser lifecycle
  APP_START: 'app_start',
  APP_QUIT: 'app_quit',
  WINDOW_OPEN: 'window_open',
  WINDOW_CLOSE: 'window_close',

  // Tab events
  TAB_OPEN: 'tab_open',
  TAB_CLOSE: 'tab_close',
  TAB_NAVIGATE: 'tab_navigate',
  TAB_RELOAD: 'tab_reload',
  TAB_CRASH: 'tab_crash',

  // Navigation
  NAVIGATION_START: 'navigation_start',
  NAVIGATION_COMPLETE: 'navigation_complete',
  NAVIGATION_ERROR: 'navigation_error',

  // Search
  SEARCH_PERFORMED: 'search_performed',
  SEARCH_SUGGESTION_CLICKED: 'search_suggestion_clicked',

  // Bookmarks
  BOOKMARK_ADDED: 'bookmark_added',
  BOOKMARK_REMOVED: 'bookmark_removed',
  BOOKMARK_OPENED: 'bookmark_opened',

  // History
  HISTORY_CLEARED: 'history_cleared',
  HISTORY_ITEM_DELETED: 'history_item_deleted',

  // Downloads
  DOWNLOAD_STARTED: 'download_started',
  DOWNLOAD_COMPLETED: 'download_completed',
  DOWNLOAD_FAILED: 'download_failed',
  DOWNLOAD_CANCELLED: 'download_cancelled',

  // Extensions
  EXTENSION_INSTALLED: 'extension_installed',
  EXTENSION_REMOVED: 'extension_removed',
  EXTENSION_ENABLED: 'extension_enabled',
  EXTENSION_DISABLED: 'extension_disabled',

  // Content blocker
  BLOCKER_BLOCKED_REQUEST: 'blocker_blocked_request',
  BLOCKER_RULES_UPDATED: 'blocker_rules_updated',

  // Settings
  SETTING_CHANGED: 'setting_changed',
  THEME_CHANGED: 'theme_changed',

  // Profile
  PROFILE_CREATED: 'profile_created',
  PROFILE_SWITCHED: 'profile_switched',
  PROFILE_DELETED: 'profile_deleted',

  // Passwords
  PASSWORD_SAVED: 'password_saved',
  PASSWORD_FILLED: 'password_filled',
  PASSWORD_DELETED: 'password_deleted',

  // Sidebar
  SIDEBAR_OPENED: 'sidebar_opened',
  SIDEBAR_CLOSED: 'sidebar_closed',
  SIDEBAR_PANEL_CHANGED: 'sidebar_panel_changed',

  // PiP
  PIP_ENTERED: 'pip_entered',
  PIP_EXITED: 'pip_exited',

  // Permissions
  PERMISSION_GRANTED: 'permission_granted',
  PERMISSION_DENIED: 'permission_denied',

  // DevTools
  DEVTOOLS_OPENED: 'devtools_opened',
  DEVTOOLS_CLOSED: 'devtools_closed',

  // Errors
  UNHANDLED_ERROR: 'unhandled_error',
  PROMISE_REJECTION: 'promise_rejection',
} as const;