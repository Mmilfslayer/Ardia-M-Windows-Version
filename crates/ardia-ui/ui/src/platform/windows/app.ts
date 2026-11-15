import { invoke } from '@tauri-apps/api/core';

export interface AppInfo {
  name: string;
  version: string;
  buildNumber: string;
  channel: 'stable' | 'beta' | 'nightly';
  platform: 'windows';
  arch: 'x64' | 'arm64';
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke('app_get_info');
}

export async function getVersion(): Promise<string> {
  return invoke('app_get_version');
}

export async function checkForUpdates(): Promise<{ available: boolean; version?: string; notes?: string }> {
  return invoke('app_check_updates');
}

export async function restartApp(): Promise<void> {
  return invoke('app_restart');
}

export async function quitApp(): Promise<void> {
  return invoke('app_quit');
}

export async function openExternal(url: string): Promise<void> {
  return invoke('app_open_external', { url });
}

export async function showItemInFolder(path: string): Promise<void> {
  return invoke('app_show_in_folder', { path });
}

export async function getPath(name: 'home' | 'appData' | 'userData' | 'temp' | 'exe' | 'desktop' | 'documents' | 'downloads' | 'music' | 'pictures' | 'videos'): Promise<string> {
  return invoke('app_get_path', { name });
}

export async function setLoginItemSettings(settings: { openAtLogin: boolean; openAsHidden?: boolean }): Promise<void> {
  return invoke('app_set_login_item', { settings });
}

export async function getLoginItemSettings(): Promise<{ openAtLogin: boolean; openAsHidden: boolean }> {
  return invoke('app_get_login_item');
}

export function onWindowAllClosed(callback: () => void): () => void {
  // Tauri handles this via event system
  const unlisten = invoke('app_on_window_all_closed', { callback: () => callback() });
  return () => unlisten;
}

export function onBeforeQuit(callback: (preventDefault: () => void) => void): () => void {
  const unlisten = invoke('app_on_before_quit', { callback });
  return () => unlisten;
}