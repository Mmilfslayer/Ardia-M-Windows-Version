import { invoke } from '@tauri-apps/api/core';

export interface Extension {
  id: string;
  name: string;
  version: string;
  description: string;
  enabled: boolean;
  icon?: string;
  permissions: string[];
  path: string;
  manifestVersion: number;
  homepageUrl?: string;
  optionsUrl?: string;
}

export async function getExtensions(): Promise<Extension[]> {
  return invoke('extension_get_all');
}

export async function getExtension(id: string): Promise<Extension | null> {
  return invoke('extension_get', { extension_id: id });
}

export async function installExtension(path: string): Promise<string> {
  return invoke('extension_install', { path });
}

export async function uninstallExtension(id: string): Promise<void> {
  return invoke('extension_uninstall', { extension_id: id });
}

export async function enableExtension(id: string): Promise<void> {
  return invoke('extension_enable', { extension_id: id });
}

export async function disableExtension(id: string): Promise<void> {
  return invoke('extension_disable', { extension_id: id });
}

export async function updateExtension(id: string): Promise<void> {
  return invoke('extension_update', { extension_id: id });
}

export async function getExtensionOptionsUrl(id: string): Promise<string | null> {
  return invoke('extension_get_options_url', { extension_id: id });
}

export async function openExtensionOptions(id: string): Promise<void> {
  return invoke('extension_open_options', { extension_id: id });
}

export async function reloadExtension(id: string): Promise<void> {
  return invoke('extension_reload', { extension_id: id });
}

export async function getExtensionPermissions(id: string): Promise<string[]> {
  return invoke('extension_get_permissions', { extension_id: id });
}

export async function setExtensionPermission(id: string, permission: string, granted: boolean): Promise<void> {
  return invoke('extension_set_permission', { extension_id: id, permission, granted });
}