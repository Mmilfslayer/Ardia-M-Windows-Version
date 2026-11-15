import { invoke } from '@tauri-apps/api/core';

export async function openDevTools(windowId: string, tabId: string): Promise<void> {
  return invoke('window_open_devtools', { window_id: windowId });
}

export async function closeDevTools(windowId: string): Promise<void> {
  return invoke('window_close_devtools', { window_id: windowId });
}

export async function isDevToolsOpen(windowId: string): Promise<boolean> {
  return invoke('window_is_devtools_open', { window_id: windowId });
}

export async function executeScript(windowId: string, tabId: string, script: string): Promise<string> {
  return invoke('webview_execute_script', { window_id: windowId, tab_id: tabId, script });
}

export async function addScriptToExecuteOnDocumentCreated(windowId: string, tabId: string, script: string): Promise<string> {
  return invoke('webview_add_script', { window_id: windowId, tab_id: tabId, script });
}

export async function removeScriptToExecuteOnDocumentCreated(windowId: string, tabId: string, scriptId: string): Promise<void> {
  return invoke('webview_remove_script', { window_id: windowId, tab_id: tabId, script_id: scriptId });
}

export async function inspectElement(windowId: string, tabId: string, x: number, y: number): Promise<void> {
  return invoke('webview_inspect_element', { window_id: windowId, tab_id: tabId, x, y });
}

export async function getDevToolsProtocolVersion(): Promise<string> {
  return invoke('devtools_get_protocol_version');
}

export async function sendDevToolsCommand(windowId: string, tabId: string, method: string, params: any): Promise<any> {
  return invoke('devtools_send_command', { window_id: windowId, tab_id: tabId, method, params });
}

export function onDevToolsMessage(windowId: string, callback: (message: any) => void): () => void {
  // Would need event listener setup
  return () => {};
}