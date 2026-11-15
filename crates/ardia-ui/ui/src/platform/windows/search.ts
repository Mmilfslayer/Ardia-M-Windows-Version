import { invoke } from '@tauri-apps/api/core';

export interface SearchEngine {
  id: string;
  name: string;
  keyword: string;
  searchUrl: string;
  suggestUrl?: string;
  iconUrl?: string;
  isDefault: boolean;
  isBuiltIn: boolean;
}

export interface SearchSuggestion {
  text: string;
  type: 'search' | 'navigation' | 'history' | 'bookmark';
  url?: string;
}

export async function getSearchEngines(profileId: string): Promise<SearchEngine[]> {
  return invoke('search_get_engines', { profile_id: profileId });
}

export async function getDefaultSearchEngine(profileId: string): Promise<SearchEngine> {
  return invoke('search_get_default', { profile_id: profileId });
}

export async function setDefaultSearchEngine(engineId: string, profileId: string): Promise<void> {
  return invoke('search_set_default', { engine_id: engineId, profile_id: profileId });
}

export async function addSearchEngine(engine: Omit<SearchEngine, 'id' | 'isBuiltIn'>, profileId: string): Promise<string> {
  return invoke('search_add_engine', { engine, profile_id: profileId });
}

export async function removeSearchEngine(engineId: string, profileId: string): Promise<void> {
  return invoke('search_remove_engine', { engine_id: engineId, profile_id: profileId });
}

export async function editSearchEngine(engineId: string, updates: Partial<SearchEngine>, profileId: string): Promise<void> {
  return invoke('search_edit_engine', { engine_id: engineId, updates, profile_id: profileId });
}

export async function getSearchSuggestions(query: string, profileId: string): Promise<SearchSuggestion[]> {
  return invoke('search_get_suggestions', { query, profile_id: profileId });
}

export async function searchWeb(query: string, engineId?: string, profileId?: string): Promise<string> {
  return invoke('search_web', { query, engine_id: engineId, profile_id: profileId });
}

export async function getSearchEngineKeywords(profileId: string): Promise<Record<string, string>> {
  return invoke('search_get_keywords', { profile_id: profileId });
}

// Built-in search engines
export const BUILTIN_SEARCH_ENGINES: Omit<SearchEngine, 'id' | 'isBuiltIn'>[] = [
  {
    name: 'Google',
    keyword: 'g',
    searchUrl: 'https://www.google.com/search?q={searchTerms}',
    suggestUrl: 'https://suggestqueries.google.com/complete/search?client=chrome&q={searchTerms}',
    iconUrl: 'https://www.google.com/favicon.ico',
    isDefault: true,
  },
  {
    name: 'Bing',
    keyword: 'b',
    searchUrl: 'https://www.bing.com/search?q={searchTerms}',
    suggestUrl: 'https://api.bing.com/osjson.aspx?query={searchTerms}',
    iconUrl: 'https://www.bing.com/favicon.ico',
    isDefault: false,
  },
  {
    name: 'DuckDuckGo',
    keyword: 'd',
    searchUrl: 'https://duckduckgo.com/?q={searchTerms}',
    suggestUrl: 'https://duckduckgo.com/ac/?q={searchTerms}',
    iconUrl: 'https://duckduckgo.com/favicon.ico',
    isDefault: false,
  },
  {
    name: 'GitHub',
    keyword: 'gh',
    searchUrl: 'https://github.com/search?q={searchTerms}',
    iconUrl: 'https://github.githubassets.com/favicon.ico',
    isDefault: false,
  },
  {
    name: 'Wikipedia',
    keyword: 'w',
    searchUrl: 'https://en.wikipedia.org/w/index.php?search={searchTerms}',
    iconUrl: 'https://en.wikipedia.org/favicon.ico',
    isDefault: false,
  },
];