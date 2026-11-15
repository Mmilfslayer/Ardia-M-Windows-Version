import { invoke } from '@tauri-apps/api/core';

export async function getConfigDir(): Promise<string> {
  return invoke('app_get_config_dir');
}

export async function getDataDir(): Promise<string> {
  return invoke('app_get_data_dir');
}

export async function getCacheDir(): Promise<string> {
  return invoke('app_get_cache_dir');
}

export async function getLogDir(): Promise<string> {
  return invoke('app_get_log_dir');
}

export async function getProfileDir(profileId: string): Promise<string> {
  return invoke('app_get_profile_dir', { profile_id: profileId });
}

export async function getTempDir(): Promise<string> {
  return invoke('app_get_temp_dir');
}