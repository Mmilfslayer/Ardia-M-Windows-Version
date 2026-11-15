import { invoke } from '@tauri-apps/api/core';

export interface ImportProfile {
  name: string;
  bookmarks: any[];
  history: any[];
  passwords: any[];
  settings: Record<string, any>;
}

export async function importFromBrowser(browser: 'chrome' | 'edge' | 'firefox' | 'brave' | 'vivaldi', profileId: string): Promise<void> {
  return invoke('import_from_browser', { browser, profile_id: profileId });
}

export async function importFromFile(filePath: string, profileId: string): Promise<void> {
  return invoke('import_from_file', { path: filePath, profile_id: profileId });
}

export async function getAvailableBrowsers(): Promise<string[]> {
  return invoke('import_get_browsers');
}

export async function exportProfile(profileId: string): Promise<ImportProfile> {
  return invoke('export_profile', { profile_id: profileId });
}

export async function importProfile(profile: ImportProfile, profileId: string): Promise<void> {
  return invoke('import_profile', { profile, profile_id: profileId });
}