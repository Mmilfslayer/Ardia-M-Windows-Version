import { invoke } from '@tauri-apps/api/core';

export interface PasswordEntry {
  id: string;
  originUrl: string;
  actionUrl: string;
  username: string;
  password: string;
  usernameElement?: string;
  passwordElement?: string;
  dateCreated: number;
  dateLastUsed?: number;
  timesUsed: number;
  profileId: string;
  isFederated: boolean;
  federationOrigin?: string;
  displayName?: string;
  iconUrl?: string;
}

export async function getPasswords(originUrl: string, profileId: string): Promise<PasswordEntry[]> {
  return invoke('password_get', { request: { origin_url: originUrl, profile_id: profileId } });
}

export async function getAllPasswords(profileId: string): Promise<PasswordEntry[]> {
  return invoke('password_list', { profile_id: profileId });
}

export async function savePassword(entry: Omit<PasswordEntry, 'id' | 'dateCreated' | 'timesUsed'>): Promise<string> {
  return invoke('password_save', { request: entry });
}

export async function deletePassword(id: string): Promise<void> {
  return invoke('password_delete', { id });
}

export async function fillPassword(windowId: string, tabId: string, passwordId: string): Promise<void> {
  return invoke('password_fill', { window_id: windowId, tab_id: tabId, password_id: passwordId });
}

export async function updatePasswordMetadata(id: string, username?: string, displayName?: string): Promise<void> {
  return invoke('password_update_metadata', { id, username, display_name: displayName });
}