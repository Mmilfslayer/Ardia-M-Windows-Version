import { invoke } from '@tauri-apps/api/core';

export interface Command {
  id: string;
  name: string;
  description: string;
  category: string;
  shortcut?: string;
  action: () => Promise<void> | void;
}

const commandRegistry = new Map<string, Command>();

export function registerCommand(command: Command): void {
  commandRegistry.set(command.id, command);
}

export function unregisterCommand(id: string): void {
  commandRegistry.delete(id);
}

export function getCommand(id: string): Command | undefined {
  return commandRegistry.get(id);
}

export function getAllCommands(): Command[] {
  return Array.from(commandRegistry.values());
}

export function getCommandsByCategory(category: string): Command[] {
  return Array.from(commandRegistry.values()).filter(c => c.category === category);
}

export async function executeCommand(id: string): Promise<void> {
  const command = commandRegistry.get(id);
  if (command) {
    await command.action();
  } else {
    throw new Error(`Command not found: ${id}`);
  }
}

// Built-in commands
registerCommand({
  id: 'new-tab',
  name: 'New Tab',
  description: 'Open a new tab',
  category: 'tabs',
  shortcut: 'Ctrl+T',
  action: async () => {
    await invoke('tab_create', { request: { window_id: '', url: 'about:blank', background: false } });
  },
});

registerCommand({
  id: 'close-tab',
  name: 'Close Tab',
  description: 'Close the current tab',
  category: 'tabs',
  shortcut: 'Ctrl+W',
  action: async () => {
    // Would need active window/tab context
  },
});

registerCommand({
  id: 'reopen-closed-tab',
  name: 'Reopen Closed Tab',
  description: 'Reopen the last closed tab',
  category: 'tabs',
  shortcut: 'Ctrl+Shift+T',
  action: async () => {
    // Implemented in history
  },
});

registerCommand({
  id: 'focus-address-bar',
  name: 'Focus Address Bar',
  description: 'Focus the address bar',
  category: 'navigation',
  shortcut: 'Ctrl+L',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'go-back',
  name: 'Go Back',
  description: 'Navigate back',
  category: 'navigation',
  shortcut: 'Alt+Left',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'go-forward',
  name: 'Go Forward',
  description: 'Navigate forward',
  category: 'navigation',
  shortcut: 'Alt+Right',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'reload',
  name: 'Reload',
  description: 'Reload the current page',
  category: 'navigation',
  shortcut: 'F5',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'toggle-sidebar',
  name: 'Toggle Sidebar',
  description: 'Show/hide the sidebar',
  category: 'view',
  shortcut: 'Ctrl+Shift+B',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'toggle-fullscreen',
  name: 'Toggle Fullscreen',
  description: 'Enter/exit fullscreen mode',
  category: 'view',
  shortcut: 'F11',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'open-devtools',
  name: 'Open DevTools',
  description: 'Open developer tools',
  category: 'developer',
  shortcut: 'F12',
  action: async () => {
    await invoke('window_open_devtools', { window_id: '' });
  },
});

registerCommand({
  id: 'open-settings',
  name: 'Open Settings',
  description: 'Open settings page',
  category: 'settings',
  shortcut: 'Ctrl+,',
  action: async () => {
    await invoke('tab_navigate', { request: { window_id: '', tab_id: '', url: 'ardia://settings' } });
  },
});

registerCommand({
  id: 'open-history',
  name: 'Open History',
  description: 'Open history page',
  category: 'history',
  shortcut: 'Ctrl+H',
  action: async () => {
    await invoke('tab_navigate', { request: { window_id: '', tab_id: '', url: 'ardia://history' } });
  },
});

registerCommand({
  id: 'open-bookmarks',
  name: 'Open Bookmarks',
  description: 'Open bookmarks page',
  category: 'bookmarks',
  shortcut: 'Ctrl+Shift+O',
  action: async () => {
    await invoke('tab_navigate', { request: { window_id: '', tab_id: '', url: 'ardia://bookmarks' } });
  },
});

registerCommand({
  id: 'open-downloads',
  name: 'Open Downloads',
  description: 'Open downloads page',
  category: 'downloads',
  shortcut: 'Ctrl+J',
  action: async () => {
    await invoke('tab_navigate', { request: { window_id: '', tab_id: '', url: 'ardia://downloads' } });
  },
});

registerCommand({
  id: 'open-extensions',
  name: 'Open Extensions',
  description: 'Open extensions page',
  category: 'extensions',
  shortcut: 'Ctrl+Shift+E',
  action: async () => {
    await invoke('tab_navigate', { request: { window_id: '', tab_id: '', url: 'ardia://extensions' } });
  },
});

registerCommand({
  id: 'new-window',
  name: 'New Window',
  description: 'Open a new window',
  category: 'window',
  shortcut: 'Ctrl+N',
  action: async () => {
    await invoke('window_create', { request: { profile_id: 'default' } });
  },
});

registerCommand({
  id: 'new-private-window',
  name: 'New Private Window',
  description: 'Open a new private window',
  category: 'window',
  shortcut: 'Ctrl+Shift+N',
  action: async () => {
    await invoke('window_create', { request: { profile_id: 'private', url: 'about:blank' } });
  },
});

registerCommand({
  id: 'close-window',
  name: 'Close Window',
  description: 'Close the current window',
  category: 'window',
  shortcut: 'Ctrl+Shift+W',
  action: async () => {
    // Handled by frontend
  },
});

registerCommand({
  id: 'minimize-window',
  name: 'Minimize Window',
  description: 'Minimize the current window',
  category: 'window',
  shortcut: 'Ctrl+M',
  action: async () => {
    // Handled by OS
  },
});

registerCommand({
  id: 'zoom-in',
  name: 'Zoom In',
  description: 'Increase page zoom',
  category: 'view',
  shortcut: 'Ctrl+=',
  action: async () => {
    await invoke('webview_set_zoom', { window_id: '', tab_id: '', zoom: 1.1 });
  },
});

registerCommand({
  id: 'zoom-out',
  name: 'Zoom Out',
  description: 'Decrease page zoom',
  category: 'view',
  shortcut: 'Ctrl+-',
  action: async () => {
    await invoke('webview_set_zoom', { window_id: '', tab_id: '', zoom: 0.9 });
  },
});

registerCommand({
  id: 'zoom-reset',
  name: 'Reset Zoom',
  description: 'Reset page zoom to 100%',
  category: 'view',
  shortcut: 'Ctrl+0',
  action: async () => {
    await invoke('webview_set_zoom', { window_id: '', tab_id: '', zoom: 1.0 });
  },
});

registerCommand({
  id: 'find-in-page',
  name: 'Find in Page',
  description: 'Find text in current page',
  category: 'edit',
  shortcut: 'Ctrl+F',
  action: async () => {
    await invoke('webview_find', { window_id: '', tab_id: '', query: '' });
  },
});

registerCommand({
  id: 'print',
  name: 'Print',
  description: 'Print current page',
  category: 'file',
  shortcut: 'Ctrl+P',
  action: async () => {
    await invoke('webview_print', { window_id: '', tab_id: '' });
  },
});

registerCommand({
  id: 'save-page',
  name: 'Save Page As',
  description: 'Save current page to disk',
  category: 'file',
  shortcut: 'Ctrl+S',
  action: async () => {
    await invoke('webview_save', { window_id: '', tab_id: '' });
  },
});

registerCommand({
  id: 'view-source',
  name: 'View Page Source',
  description: 'View the HTML source of the current page',
  category: 'developer',
  shortcut: 'Ctrl+U',
  action: async () => {
    await invoke('webview_view_source', { window_id: '', tab_id: '' });
  },
});