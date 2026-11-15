import { invoke } from '@tauri-apps/api/core';

export interface NotificationOptions {
  title: string;
  body: string;
  icon?: string;
  tag?: string;
  requireInteraction?: boolean;
  silent?: boolean;
  data?: Record<string, any>;
  actions?: Array<{ action: string; title: string }>;
}

export interface NotificationPermissionState {
  permission: 'granted' | 'denied' | 'default';
}

export async function requestNotificationPermission(): Promise<'granted' | 'denied'> {
  return invoke('notification_request_permission');
}

export async function getNotificationPermission(): Promise<NotificationPermissionState> {
  return invoke('notification_get_permission');
}

export async function showNotification(options: NotificationOptions): Promise<string> {
  return invoke('notification_show', { options });
}

export async function closeNotification(tag: string): Promise<void> {
  return invoke('notification_close', { tag });
}

export async function getActiveNotifications(): Promise<Array<{ tag: string; title: string; body: string }>> {
  return invoke('notification_get_active');
}

export async function clearAllNotifications(): Promise<void> {
  return invoke('notification_clear_all');
}

// Windows Toast notification (alternative to system notifications)
export async function showWindowsToast(title: string, body: string, options?: {
  icon?: string;
  duration?: 'short' | 'long';
  actions?: Array<{ content: string; arguments: string }>;
}): Promise<void> {
  return invoke('notification_show_toast', { title, body, options });
}