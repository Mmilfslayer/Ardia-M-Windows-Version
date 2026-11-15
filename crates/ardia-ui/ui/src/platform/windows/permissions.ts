import { invoke } from '@tauri-apps/api/core';

export type PermissionType =
  | 'camera'
  | 'microphone'
  | 'location'
  | 'notifications'
  | 'clipboard-read'
  | 'clipboard-write'
  | 'geolocation'
  | 'midi'
  | 'midi-sysex'
  | 'payment-handler'
  | 'persistent-storage'
  | 'screen-wake-lock'
  | 'storage-access'
  | 'window-management';

export type PermissionState = 'granted' | 'denied' | 'ask';

export interface PermissionRequest {
  windowId: string;
  tabId: string;
  permission: PermissionType;
  origin: string;
  embeddingOrigin?: string;
}

export async function requestPermission(request: PermissionRequest): Promise<PermissionState> {
  return invoke('permission_request', { request });
}

export async function getPermission(origin: string, permission: PermissionType): Promise<PermissionState> {
  return invoke('permission_get', { origin, permission });
}

export async function setPermission(origin: string, permission: PermissionType, state: PermissionState): Promise<void> {
  return invoke('permission_set', { origin, permission, state });
}

export async function getPermissionsForOrigin(origin: string): Promise<Record<PermissionType, PermissionState>> {
  return invoke('permission_get_all_for_origin', { origin });
}

export async function resetPermissionsForOrigin(origin: string): Promise<void> {
  return invoke('permission_reset_origin', { origin });
}

export async function getAllPermissions(profileId: string): Promise<Array<{ origin: string; permission: PermissionType; state: PermissionState }>> {
  return invoke('permission_list', { profile_id: profileId });
}

export async function clearAllPermissions(profileId: string): Promise<void> {
  return invoke('permission_clear_all', { profile_id: profileId });
}

// Permission descriptors for WebView2
export interface PermissionDescriptor {
  name: PermissionType;
}

export async function queryPermission(descriptor: PermissionDescriptor, origin: string): Promise<PermissionState> {
  return getPermission(origin, descriptor.name);
}