import { invoke } from '@tauri-apps/api/core';
import { Menu, MenuItem, PredefinedMenuItem, Submenu } from '@tauri-apps/api/menu';

export interface MenuItemOptions {
  id?: string;
  label: string;
  enabled?: boolean;
  checked?: boolean;
  accelerator?: string;
  icon?: string;
  action?: () => void;
  submenu?: MenuItemOptions[];
}

export async function createMenu(items: MenuItemOptions[]): Promise<Menu> {
  const menuItems = await Promise.all(items.map(createMenuItem));
  return Menu.new({ items: menuItems });
}

async function createMenuItem(options: MenuItemOptions): Promise<MenuItem | PredefinedMenuItem | Submenu> {
  if (options.submenu) {
    const subItems = await Promise.all(options.submenu.map(createMenuItem));
    return Submenu.new({ label: options.label, enabled: options.enabled ?? true, items: subItems });
  }

  if (options.id === 'separator') {
    return PredefinedMenuItem.separator();
  }

  return MenuItem.new({
    id: options.id,
    label: options.label,
    enabled: options.enabled ?? true,
    checked: options.checked,
    accelerator: options.accelerator,
    icon: options.icon,
    action: options.action ? () => options.action!() : undefined,
  });
}

export async function createContextMenu(items: MenuItemOptions[]): Promise<Menu> {
  return createMenu(items);
}

export async function showContextMenu(menu: Menu, x: number, y: number): Promise<void> {
  return menu.popup({ x, y });
}

export async function setApplicationMenu(menu: Menu): Promise<void> {
  return Menu.setAsAppMenu(menu);
}

export async function removeApplicationMenu(): Promise<void> {
  return Menu.removeAppMenu();
}

// Standard menus
export async function createDefaultAppMenu(): Promise<Menu> {
  return createMenu([
    {
      label: 'File',
      submenu: [
        { label: 'New Tab', id: 'new-tab', accelerator: 'Ctrl+T' },
        { label: 'New Window', id: 'new-window', accelerator: 'Ctrl+N' },
        { label: 'New Private Window', id: 'new-private-window', accelerator: 'Ctrl+Shift+N' },
        { id: 'separator' },
        { label: 'Open File...', id: 'open-file', accelerator: 'Ctrl+O' },
        { label: 'Save Page As...', id: 'save-page', accelerator: 'Ctrl+S' },
        { id: 'separator' },
        { label: 'Print...', id: 'print', accelerator: 'Ctrl+P' },
        { id: 'separator' },
        { label: 'Close Tab', id: 'close-tab', accelerator: 'Ctrl+W' },
        { label: 'Close Window', id: 'close-window', accelerator: 'Ctrl+Shift+W' },
        { id: 'separator' },
        { label: 'Quit', id: 'quit', accelerator: 'Ctrl+Q' },
      ],
    },
    {
      label: 'Edit',
      submenu: [
        { label: 'Undo', id: 'undo', accelerator: 'Ctrl+Z' },
        { label: 'Redo', id: 'redo', accelerator: 'Ctrl+Y' },
        { id: 'separator' },
        { label: 'Cut', id: 'cut', accelerator: 'Ctrl+X' },
        { label: 'Copy', id: 'copy', accelerator: 'Ctrl+C' },
        { label: 'Paste', id: 'paste', accelerator: 'Ctrl+V' },
        { label: 'Select All', id: 'select-all', accelerator: 'Ctrl+A' },
        { id: 'separator' },
        { label: 'Find', id: 'find', accelerator: 'Ctrl+F' },
        { label: 'Find Next', id: 'find-next', accelerator: 'Ctrl+G' },
        { label: 'Find Previous', id: 'find-previous', accelerator: 'Ctrl+Shift+G' },
      ],
    },
    {
      label: 'View',
      submenu: [
        { label: 'Reload', id: 'reload', accelerator: 'F5' },
        { label: 'Force Reload', id: 'force-reload', accelerator: 'Ctrl+F5' },
        { id: 'separator' },
        { label: 'Zoom In', id: 'zoom-in', accelerator: 'Ctrl+=' },
        { label: 'Zoom Out', id: 'zoom-out', accelerator: 'Ctrl+-' },
        { label: 'Reset Zoom', id: 'zoom-reset', accelerator: 'Ctrl+0' },
        { id: 'separator' },
        { label: 'Toggle Fullscreen', id: 'toggle-fullscreen', accelerator: 'F11' },
        { label: 'Toggle Sidebar', id: 'toggle-sidebar', accelerator: 'Ctrl+Shift+B' },
        { id: 'separator' },
        { label: 'Developer Tools', id: 'devtools', accelerator: 'F12' },
      ],
    },
    {
      label: 'History',
      submenu: [
        { label: 'Show History', id: 'show-history', accelerator: 'Ctrl+H' },
        { label: 'Recently Closed', id: 'recently-closed' },
        { id: 'separator' },
        { label: 'Clear History...', id: 'clear-history' },
      ],
    },
    {
      label: 'Bookmarks',
      submenu: [
        { label: 'Bookmark This Tab', id: 'bookmark-tab', accelerator: 'Ctrl+D' },
        { label: 'Bookmark All Tabs', id: 'bookmark-all-tabs', accelerator: 'Ctrl+Shift+D' },
        { label: 'Show Bookmarks', id: 'show-bookmarks', accelerator: 'Ctrl+Shift+O' },
        { id: 'separator' },
        { label: 'Bookmark Manager', id: 'bookmark-manager' },
      ],
    },
    {
      label: 'Window',
      submenu: [
        { label: 'Minimize', id: 'minimize', accelerator: 'Ctrl+M' },
        { label: 'Zoom', id: 'zoom' },
        { id: 'separator' },
        { label: 'Next Tab', id: 'next-tab', accelerator: 'Ctrl+Tab' },
        { label: 'Previous Tab', id: 'prev-tab', accelerator: 'Ctrl+Shift+Tab' },
        { label: 'Move Tab to New Window', id: 'move-tab-new-window' },
        { id: 'separator' },
        { label: 'Bring All to Front', id: 'bring-all-front' },
      ],
    },
    {
      label: 'Help',
      submenu: [
        { label: 'About Ardia', id: 'about' },
        { label: 'Check for Updates', id: 'check-updates' },
        { label: 'Report Issue', id: 'report-issue' },
        { label: 'Keyboard Shortcuts', id: 'shortcuts' },
      ],
    },
  ]);
}