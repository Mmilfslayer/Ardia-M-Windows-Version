import { invoke } from '@tauri-apps/api/core';

export interface SyncStatus {
  enabled: boolean;
  lastSync: number | null;
  pendingChanges: number;
  status: 'idle' | 'syncing' | 'error' | 'disabled';
  error?: string;
}

export interface SyncData {
  bookmarks: any[];
  history: any[];
  passwords: any[];
  settings: Record<string, any>;
  extensions: any[];
  openTabs: any[];
}

export async function getSyncStatus(): Promise<SyncStatus> {
  return invoke('sync_get_status');
}

export async function enableSync(credentials: { email: string; token: string }): Promise<void> {
  return invoke('sync_enable', { credentials });
}

export async function disableSync(): Promise<void> {
  return invoke('sync_disable');
}

export async function triggerSync(): Promise<void> {
  return invoke('sync_trigger');
}

export async function getSyncData(): Promise<SyncData> {
  return invoke('sync_get_data');
}

export async function setSyncData(data: Partial<SyncData>): Promise<void> {
  return invoke('sync_set_data', { data });
}

export async function resolveSyncConflict(local: any, remote: any): Promise<any> {
  return invoke('sync_resolve_conflict', { local, remote });
}

export async function getSyncEncryptionKey(): Promise<string | null> {
  return invoke('sync_get_encryption_key');
}

export async function setSyncEncryptionKey(key: string): Promise<void> {
  return invoke('sync_set_encryption_key', { key });
}

// For personal use - sync is optional and can be implemented later
// This is a placeholder implementation
export async function isSyncAvailable(): Promise<boolean> {
  return false;
}