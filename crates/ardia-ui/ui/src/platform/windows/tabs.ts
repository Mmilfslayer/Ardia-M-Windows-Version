import { invoke } from '@tauri-apps/api/core';

export interface Tab {
  id: string;
  windowId: string;
  url: string;
  title: string;
  favicon: string | null;
  loading: boolean;
  canGoBack: boolean;
  canGoForward: boolean;
  pinned: boolean;
  muted: boolean;
  incognito: boolean;
  index: number;
  active: boolean;
  pageState: 'loading' | 'loaded' | 'error' | 'crashed';
  errorMessage?: string;
}

export interface CreateTabOptions {
  url?: string;
  active?: boolean;
  pinned?: boolean;
  incognito?: boolean;
  openerTabId?: string;
}

export async function getTabs(windowId: string): Promise<Tab[]> {
  return invoke('tabs_list', { window_id: windowId });
}

export async function getTab(windowId: string, tabId: string): Promise<Tab> {
  return invoke('tabs_get', { window_id: windowId, tab_id: tabId });
}

export async function createTab(windowId: string, options: CreateTabOptions = {}): Promise<Tab> {
  return invoke('tabs_create', { window_id: windowId, options });
}

export async function closeTab(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_close', { window_id: windowId, tab_id: tabId });
}

export async function closeTabs(windowId: string, tabIds: string[]): Promise<void> {
  return invoke('tabs_close_multiple', { window_id: windowId, tab_ids: tabIds });
}

export async function closeOtherTabs(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_close_others', { window_id: windowId, tab_id: tabId });
}

export async function closeTabsToRight(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_close_to_right', { window_id: windowId, tab_id: tabId });
}

export async function navigateTab(windowId: string, tabId: string, url: string): Promise<void> {
  return invoke('tabs_navigate', { window_id: windowId, tab_id: tabId, url });
}

export async function reloadTab(windowId: string, tabId: string, bypassCache: boolean = false): Promise<void> {
  return invoke('tabs_reload', { window_id: windowId, tab_id: tabId, bypass_cache: bypassCache });
}

export async function goBack(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_go_back', { window_id: windowId, tab_id: tabId });
}

export async function goForward(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_go_forward', { window_id: windowId, tab_id: tabId });
}

export async function setTabActive(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_set_active', { window_id: windowId, tab_id: tabId });
}

export async function setTabPinned(windowId: string, tabId: string, pinned: boolean): Promise<void> {
  return invoke('tabs_set_pinned', { window_id: windowId, tab_id: tabId, pinned });
}

export async function setTabMuted(windowId: string, tabId: string, muted: boolean): Promise<void> {
  return invoke('tabs_set_muted', { window_id: windowId, tab_id: tabId, muted });
}

export async function reorderTabs(windowId: string, tabIds: string[]): Promise<void> {
  return invoke('tabs_reorder', { window_id: windowId, tab_ids: tabIds });
}

export async function moveTabToWindow(tabId: string, sourceWindowId: string, targetWindowId: string, index: number): Promise<Tab> {
  return invoke('tabs_move_to_window', { tab_id: tabId, source_window_id: sourceWindowId, target_window_id: targetWindowId, index });
}

export async function duplicateTab(windowId: string, tabId: string): Promise<Tab> {
  return invoke('tabs_duplicate', { window_id: windowId, tab_id: tabId });
}

export async function captureTabScreenshot(windowId: string, tabId: string): Promise<string> {
  return invoke('tabs_capture_screenshot', { window_id: windowId, tab_id: tabId });
}

export async function printTab(windowId: string, tabId: string): Promise<void> {
  return invoke('tabs_print', { window_id: windowId, tab_id: tabId });
}

export async function saveTabAsPdf(windowId: string, tabId: string, path: string): Promise<void> {
  return invoke('tabs_save_as_pdf', { window_id: windowId, tab_id: tabId, path });
}

// Tab groups (future feature)
export interface TabGroup {
  id: string;
  windowId: string;
  name: string;
  color: string;
  tabIds: string[];
  collapsed: boolean;
}

export async function getTabGroups(windowId: string): Promise<TabGroup[]> {
  return invoke('tab_groups_list', { window_id: windowId });
}

export async function createTabGroup(windowId: string, name: string, color: string, tabIds: string[]): Promise<TabGroup> {
  return invoke('tab_groups_create', { window_id: windowId, name, color, tab_ids: tabIds });
}

export async function updateTabGroup(groupId: string, updates: Partial<TabGroup>): Promise<void> {
  return invoke('tab_groups_update', { group_id: groupId, updates });
}

export async function deleteTabGroup(groupId: string): Promise<void> {
  return invoke('tab_groups_delete', { group_id: groupId });
}

export async function addTabToGroup(tabId: string, groupId: string): Promise<void> {
  return invoke('tab_groups_add_tab', { tab_id: tabId, group_id: groupId });
}

export async function removeTabFromGroup(tabId: string): Promise<void> {
  return invoke('tab_groups_remove_tab', { tab_id: tabId });
}