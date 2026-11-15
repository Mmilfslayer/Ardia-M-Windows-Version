import { invoke } from '@tauri-apps/api/core';

export interface LaunchConfig {
  url?: string;
  profile?: string;
  private?: boolean;
  windowId?: string;
  tabId?: string;
}

export function parseLaunchArgs(): LaunchConfig {
  // In Tauri, args are passed via the invoke system
  // This would be populated by the Rust side
  return {
    url: undefined,
    profile: 'default',
    private: false,
  };
}

export async function getLaunchConfig(): Promise<LaunchConfig> {
  return invoke('app_get_launch_config');
}

export async function setLaunchConfig(config: LaunchConfig): Promise<void> {
  return invoke('app_set_launch_config', { config });
}

export function isFirstLaunch(): boolean {
  // Would check a flag in settings
  return false;
}

export function setFirstLaunchComplete(): void {
  // Would set a flag in settings
}