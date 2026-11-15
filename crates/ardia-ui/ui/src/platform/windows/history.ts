import { invoke } from '@tauri-apps/api/core';

export interface HistoryEntry {
  id: string;
  url: string;
  title: string;
  visitCount: number;
  lastVisitTime: number;
  typedCount: number;
  transitionType: string;
}

export async function getHistory(profileId: string, limit = 100, offset = 0): Promise<HistoryEntry[]> {
  return invoke('history_get', { profile_id: profileId, limit, offset });
}

export async function searchHistory(profileId: string, query: string, limit = 50): Promise<HistoryEntry[]> {
  return invoke('history_search', { request: { profile_id: profileId, query, limit } });
}

export async function addHistoryEntry(url: string, title: string, profileId: string, transitionType: string = 'link', isTyped = false): Promise<string> {
  return invoke('history_add', { url, title, profile_id: profileId, transition_type: transitionType, is_typed: isTyped });
}

export async function deleteHistoryEntry(id: string): Promise<void> {
  return invoke('history_delete', { id });
}

export async function clearHistory(profileId: string): Promise<void> {
  return invoke('history_clear', { profile_id: profileId });
}

export async function getMostVisited(profileId: string, limit = 10): Promise<HistoryEntry[]> {
  return invoke('history_most_visited', { profile_id: profileId, limit });
}

export async function getRecentlyClosed(profileId: string, limit = 10): Promise<HistoryEntry[]> {
  return invoke('history_recently_closed', { profile_id: profileId, limit });
}