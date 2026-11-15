import { invoke } from '@tauri-apps/api/core';

export interface UserScript {
  id: string;
  name: string;
  namespace?: string;
  version: string;
  description?: string;
  author?: string;
  match: string[];
  excludeMatch?: string[];
  include?: string[];
  exclude?: string[];
  runAt: 'document_start' | 'document_end' | 'document_idle';
  grant?: string[];
  require?: string[];
  script: string;
  enabled: boolean;
  profileId: string;
  createdAt: number;
  updatedAt: number;
}

export interface UserScriptMetadata {
  name: string;
  namespace?: string;
  version: string;
  description?: string;
  author?: string;
  match?: string[];
  excludeMatch?: string[];
  include?: string[];
  exclude?: string[];
  runAt?: 'document_start' | 'document_end' | 'document_idle';
  grant?: string[];
  require?: string[];
  icon?: string;
  icon64?: string;
  downloadURL?: string;
  updateURL?: string;
  supportURL?: string;
  homepage?: string;
  license?: string;
}

export async function getUserScripts(profileId: string): Promise<UserScript[]> {
  return invoke('user_scripts_list', { profile_id: profileId });
}

export async function getUserScript(id: string, profileId: string): Promise<UserScript> {
  return invoke('user_scripts_get', { id, profile_id: profileId });
}

export async function installUserScript(script: Omit<UserScript, 'id' | 'createdAt' | 'updatedAt'>): Promise<string> {
  return invoke('user_scripts_install', { script });
}

export async function installUserScriptFromUrl(url: string, profileId: string): Promise<string> {
  return invoke('user_scripts_install_from_url', { url, profile_id: profileId });
}

export async function installUserScriptFromFile(filePath: string, profileId: string): Promise<string> {
  return invoke('user_scripts_install_from_file', { path: filePath, profile_id: profileId });
}

export async function uninstallUserScript(id: string, profileId: string): Promise<void> {
  return invoke('user_scripts_uninstall', { id, profile_id: profileId });
}

export async function enableUserScript(id: string, enabled: boolean, profileId: string): Promise<void> {
  return invoke('user_scripts_set_enabled', { id, enabled, profile_id: profileId });
}

export async function updateUserScript(id: string, updates: Partial<Omit<UserScript, 'id' | 'profileId' | 'createdAt'>>, profileId: string): Promise<void> {
  return invoke('user_scripts_update', { id, updates, profile_id: profileId });
}

export async function getUserScriptMetadata(scriptContent: string): Promise<UserScriptMetadata> {
  return invoke('user_scripts_parse_metadata', { script: scriptContent });
}

export async function exportUserScripts(profileId: string): Promise<UserScript[]> {
  return invoke('user_scripts_export', { profile_id: profileId });
}

export async function importUserScripts(scripts: UserScript[], profileId: string): Promise<void> {
  return invoke('user_scripts_import', { scripts, profile_id: profileId });
}

export async function checkUserScriptUpdates(profileId: string): Promise<Array<{ id: string; currentVersion: string; latestVersion: string; updateUrl: string }>> {
  return invoke('user_scripts_check_updates', { profile_id: profileId });
}

export async function updateUserScriptFromSource(id: string, profileId: string): Promise<void> {
  return invoke('user_scripts_update_from_source', { id, profile_id: profileId });
}

// Greasemonkey API compatibility
export const GM_API = {
  // These would be injected into the page context
  getValue: (key: string, defaultValue?: any) => invoke('gm_get_value', { key, default_value: defaultValue }),
  setValue: (key: string, value: any) => invoke('gm_set_value', { key, value }),
  deleteValue: (key: string) => invoke('gm_delete_value', { key }),
  listValues: () => invoke('gm_list_values'),
  addStyle: (css: string) => invoke('gm_add_style', { css }),
  xmlHttpRequest: (details: any) => invoke('gm_xmlhttp_request', { details }),
  download: (details: any) => invoke('gm_download', { details }),
  notification: (details: any) => invoke('gm_notification', { details }),
  openInTab: (url: string, options?: any) => invoke('gm_open_in_tab', { url, options }),
  registerMenuCommand: (name: string, fn: () => void) => invoke('gm_register_menu_command', { name }),
  unregisterMenuCommand: (id: number) => invoke('gm_unregister_menu_command', { id }),
  info: { script: { name: '', version: '', author: '', description: '' } },
  log: (...args: any[]) => invoke('gm_log', { args }),
};