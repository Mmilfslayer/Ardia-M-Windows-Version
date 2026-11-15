import { invoke } from '@tauri-apps/api/core';

export async function focusWindow(windowId: string): Promise<void> {
  return invoke('window_focus', { window_id: windowId });
}

export async function isWindowFocused(windowId: string): Promise<boolean> {
  return invoke('window_is_focused', { window_id: windowId });
}

export async function getFocusedWindow(): Promise<string | null> {
  return invoke('window_get_focused');
}

export async function setAlwaysOnTop(windowId: string, alwaysOnTop: boolean): Promise<void> {
  return invoke('window_set_always_on_top', { window_id: windowId, always_on_top: alwaysOnTop });
}

export async function flashWindow(windowId: string, flash: boolean): Promise<void> {
  return invoke('window_flash', { window_id: windowId, flash });
}

export function onWindowFocusChanged(callback: (windowId: string | null) => void): () => void {
  // Would need event listener
  return () => {};
}