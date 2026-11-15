import { invoke } from '@tauri-apps/api/core';

export interface MediaPermissionRequest {
  windowId: string;
  tabId: string;
  audio: boolean;
  video: boolean;
}

export async function requestMediaPermission(request: MediaPermissionRequest): Promise<'granted' | 'denied' | 'dismissed'> {
  return invoke('media_request_permission', { request });
}

export async function getMediaPermissions(origin: string): Promise<{ audio: string; video: string }> {
  return invoke('media_get_permissions', { origin });
}

export async function setMediaPermission(origin: string, type: 'audio' | 'video', state: 'allow' | 'block' | 'ask'): Promise<void> {
  return invoke('media_set_permission', { origin, type, state });
}

export async function enumerateDevices(): Promise<Array<{ deviceId: string; label: string; kind: 'audioinput' | 'audiooutput' | 'videoinput' }>> {
  return invoke('media_enumerate_devices');
}

export async function getUserMedia(constraints: MediaStreamConstraints): Promise<MediaStream> {
  // This would use the browser's getUserMedia
  return navigator.mediaDevices.getUserMedia(constraints);
}

export async function checkMediaPermission(origin: string, type: 'camera' | 'microphone'): Promise<'granted' | 'denied' | 'prompt'> {
  return invoke('media_check_permission', { origin, type });
}