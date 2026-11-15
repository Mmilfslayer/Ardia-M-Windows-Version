import { invoke } from '@tauri-apps/api/core';

export interface FilterRule {
  id: number;
  rule: string;
  action: 'block' | 'allow' | 'redirect' | 'upgrade' | 'noop';
  domains: string[];
  excludeDomains: string[];
  resourceTypes: string[];
  isEnabled: boolean;
}

export interface BlockerStats {
  blockedCount: number;
  allowedCount: number;
  redirectedCount: number;
  cosmeticHiddenCount: number;
}

export async function isEnabled(): Promise<boolean> {
  return invoke('blocker_is_enabled');
}

export async function setEnabled(enabled: boolean): Promise<void> {
  return invoke('blocker_toggle', { enabled });
}

export async function getRules(): Promise<FilterRule[]> {
  return invoke('blocker_get_rules');
}

export async function addRule(rule: string, action: string, domains?: string[], excludeDomains?: string[], resourceTypes?: string[]): Promise<number> {
  return invoke('blocker_add_rule', { request: { rule, action, domains, exclude_domains: excludeDomains, resource_types: resourceTypes } });
}

export async function removeRule(ruleId: number): Promise<void> {
  return invoke('blocker_remove_rule', { rule_id: ruleId });
}

export async function getStats(): Promise<BlockerStats> {
  return invoke('blocker_get_stats');
}

export async function updateFilterLists(): Promise<void> {
  return invoke('blocker_update_lists');
}

export async function getFilterLists(): Promise<Array<{ name: string; url: string; enabled: boolean; lastUpdated?: number; rulesCount: number }>> {
  return invoke('blocker_get_filter_lists');
}

export async function enableFilterList(name: string, enabled: boolean): Promise<void> {
  return invoke('blocker_enable_filter_list', { name, enabled });
}

export async function addCosmeticFilter(selector: string, domains: string[]): Promise<void> {
  return invoke('blocker_add_cosmetic_filter', { selector, domains });
}

export async function removeCosmeticFilter(selector: string): Promise<void> {
  return invoke('blocker_remove_cosmetic_filter', { selector });
}

export async function getCosmeticFilters(url: string): Promise<string[]> {
  return invoke('blocker_get_cosmetic_filters', { url });
}