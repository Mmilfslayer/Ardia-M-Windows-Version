import { invoke } from '@tauri-apps/api/core';

export type SiteSettingType =
  | 'cookies'
  | 'images'
  | 'javascript'
  | 'popups'
  | 'notifications'
  | 'geolocation'
  | 'camera'
  | 'microphone'
  | 'midi'
  | 'ssl-certificates'
  | 'auto-downloads'
  | 'unsandboxed-plugins'
  | 'protected-media-identifier'
  | 'clipboard'
  | 'payment-handler'
  | 'idle-detection'
  | 'window-management';

export type SiteSettingValue = 'allow' | 'block' | 'ask' | 'session';

export interface SiteSetting {
  origin: string;
  setting: SiteSettingType;
  value: SiteSettingValue;
  profileId: string;
}

export async function getSiteSettings(origin: string, profileId: string): Promise<Record<SiteSettingType, SiteSettingValue>> {
  return invoke('site_settings_get', { origin, profile_id: profileId });
}

export async function getSiteSetting(origin: string, setting: SiteSettingType, profileId: string): Promise<SiteSettingValue> {
  return invoke('site_settings_get_one', { origin, setting, profile_id: profileId });
}

export async function setSiteSetting(origin: string, setting: SiteSettingType, value: SiteSettingValue, profileId: string): Promise<void> {
  return invoke('site_settings_set', { origin, setting, value, profile_id: profileId });
}

export async function clearSiteSettings(origin: string, profileId: string): Promise<void> {
  return invoke('site_settings_clear', { origin, profile_id: profileId });
}

export async function clearAllSiteSettings(profileId: string): Promise<void> {
  return invoke('site_settings_clear_all', { profile_id: profileId });
}

export async function getAllSiteSettings(profileId: string): Promise<SiteSetting[]> {
  return invoke('site_settings_list', { profile_id: profileId });
}

export async function getSitePermissions(origin: string, profileId: string): Promise<Record<SiteSettingType, SiteSettingValue>> {
  return getSiteSettings(origin, profileId);
}

export async function setSitePermission(origin: string, setting: SiteSettingType, value: SiteSettingValue, profileId: string): Promise<void> {
  return setSiteSetting(origin, setting, value, profileId);
}

// Cookie-specific settings
export interface CookieSetting {
  origin: string;
  setting: 'allow' | 'block' | 'session_only' | 'block_third_party';
  profileId: string;
}

export async function getCookieSetting(origin: string, profileId: string): Promise<CookieSetting['setting']> {
  return invoke('site_settings_get_cookie_setting', { origin, profile_id: profileId });
}

export async function setCookieSetting(origin: string, setting: CookieSetting['setting'], profileId: string): Promise<void> {
  return invoke('site_settings_set_cookie_setting', { origin, setting, profile_id: profileId });
}

// Zoom settings
export async function getZoomLevel(origin: string, profileId: string): Promise<number> {
  return invoke('site_settings_get_zoom', { origin, profile_id: profileId });
}

export async function setZoomLevel(origin: string, level: number, profileId: string): Promise<void> {
  return invoke('site_settings_set_zoom', { origin, level, profile_id: profileId });
}

export async function resetZoomLevel(origin: string, profileId: string): Promise<void> {
  return invoke('site_settings_reset_zoom', { origin, profile_id: profileId });
}