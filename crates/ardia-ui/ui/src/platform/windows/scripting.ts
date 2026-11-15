import { invoke } from '@tauri-apps/api/core';

export interface ScriptInjection {
  windowId: string;
  tabId: string;
  script: string;
  world?: 'main' | 'isolated';
  runAt?: 'document_start' | 'document_end' | 'document_idle';
}

export interface ScriptResult {
  result: any;
  executionId: string;
}

export async function executeScript(injection: ScriptInjection): Promise<ScriptResult> {
  return invoke('scripting_execute', { injection });
}

export async function executeScriptInAllFrames(injection: ScriptInjection): Promise<ScriptResult[]> {
  return invoke('scripting_execute_in_all_frames', { injection });
}

export async function insertCSS(injection: Omit<ScriptInjection, 'script'> & { css: string }): Promise<void> {
  return invoke('scripting_insert_css', { injection });
}

export async function removeCSS(injection: Omit<ScriptInjection, 'script'> & { css: string }): Promise<void> {
  return invoke('scripting_remove_css', { injection });
}

export async function addContentScript(script: {
  id: string;
  matches: string[];
  excludeMatches?: string[];
  script: string;
  runAt: 'document_start' | 'document_end' | 'document_idle';
  world: 'main' | 'isolated';
  profileId: string;
}): Promise<void> {
  return invoke('scripting_add_content_script', { script });
}

export async function removeContentScript(id: string, profileId: string): Promise<void> {
  return invoke('scripting_remove_content_script', { id, profile_id: profileId });
}

export async function getContentScripts(profileId: string): Promise<any[]> {
  return invoke('scripting_get_content_scripts', { profile_id: profileId });
}

export async function updateContentScript(script: {
  id: string;
  matches?: string[];
  excludeMatches?: string[];
  script?: string;
  runAt?: 'document_start' | 'document_end' | 'document_idle';
  world?: 'main' | 'isolated';
  profileId: string;
}): Promise<void> {
  return invoke('scripting_update_content_script', { script });
}

// User scripts (Greasemonkey/Tampermonkey style)
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
}

export async function getUserScripts(profileId: string): Promise<UserScript[]> {
  return invoke('user_scripts_list', { profile_id: profileId });
}

export async function installUserScript(script: Omit<UserScript, 'id'>): Promise<string> {
  return invoke('user_scripts_install', { script });
}

export async function uninstallUserScript(id: string, profileId: string): Promise<void> {
  return invoke('user_scripts_uninstall', { id, profile_id: profileId });
}

export async function enableUserScript(id: string, enabled: boolean, profileId: string): Promise<void> {
  return invoke('user_scripts_set_enabled', { id, enabled, profile_id: profileId });
}

export async function updateUserScript(id: string, updates: Partial<UserScript>, profileId: string): Promise<void> {
  return invoke('user_scripts_update', { id, updates, profile_id: profileId });
}