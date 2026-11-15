import { invoke } from '@tauri-apps/api/core';

export interface Profile {
  id: string;
  name: string;
  path: string;
  isDefault: boolean;
  createdAt: number;
  lastUsedAt: number;
  settings: Record<string, any>;
  theme: 'light' | 'dark' | 'system';
}

export async function getProfiles(): Promise<Profile[]> {
  return invoke('profile_list');
}

export async function getCurrentProfile(): Promise<Profile> {
  return invoke('profile_current');
}

export async function createProfile(name: string): Promise<Profile> {
  return invoke('profile_create', { name });
}

export async function deleteProfile(profileId: string): Promise<void> {
  return invoke('profile_delete', { profile_id: profileId });
}

export async function switchProfile(profileId: string): Promise<Profile> {
  return invoke('profile_switch', { profile_id: profileId });
}

export async function renameProfile(profileId: string, name: string): Promise<Profile> {
  return invoke('profile_rename', { profile_id: profileId, name });
}

export async function getProfileSettings(profileId: string): Promise<Record<string, any>> {
  return invoke('profile_get_settings', { profile_id: profileId });
}

export async function setProfileSetting(profileId: string, key: string, value: any): Promise<void> {
  return invoke('profile_set_setting', { profile_id: profileId, key, value });
}

export async function getProfilePath(profileId: string): Promise<string> {
  return invoke('profile_get_path', { profile_id: profileId });
}

export async function setProfileTheme(profileId: string, theme: 'light' | 'dark' | 'system'): Promise<void> {
  return invoke('profile_set_theme', { profile_id: profileId, theme });
}

export async function cloneProfile(profileId: string, newName: string): Promise<Profile> {
  return invoke('profile_clone', { profile_id: profileId, name: newName });
}

export async function exportProfileData(profileId: string): Promise<any> {
  return invoke('profile_export', { profile_id: profileId });
}

export async function importProfileData(data: any, name: string): Promise<Profile> {
  return invoke('profile_import', { data, name });
}