import { invoke } from '@tauri-apps/api/core';

export interface PowerState {
  onBattery: boolean;
  batteryLevel: number | null;
  batteryCharging: boolean;
  powerSaveMode: boolean;
}

export interface SystemIdleState {
  isIdle: boolean;
  idleTimeMs: number;
}

export interface NetworkState {
  online: boolean;
  connectionType: 'wifi' | 'ethernet' | 'cellular' | 'unknown';
  effectiveType: '2g' | '3g' | '4g' | '5g' | 'wifi' | 'unknown';
}

export async function getPowerState(): Promise<PowerState> {
  return invoke('system_get_power_state');
}

export async function getIdleState(): Promise<SystemIdleState> {
  return invoke('system_get_idle_state');
}

export async function getNetworkState(): Promise<NetworkState> {
  return invoke('system_get_network_state');
}

export async function preventSleep(reason: string): Promise<string> {
  return invoke('system_prevent_sleep', { reason });
}

export async function allowSleep(token: string): Promise<void> {
  return invoke('system_allow_sleep', { token });
}

export async function keepScreenOn(enabled: boolean): Promise<void> {
  return invoke('system_keep_screen_on', { enabled });
}

// Event listeners for system changes
export type SystemSignalHandler = (state: any) => void;

const listeners: Map<string, SystemSignalHandler[]> = new Map();

export function addSystemSignalListener(signal: 'power' | 'idle' | 'network', handler: SystemSignalHandler): () => void {
  const handlers = listeners.get(signal) || [];
  handlers.push(handler);
  listeners.set(signal, handlers);

  return () => {
    const currentHandlers = listeners.get(signal) || [];
    const index = currentHandlers.indexOf(handler);
    if (index >= 0) {
      currentHandlers.splice(index, 1);
    }
  };
}

export function removeSystemSignalListener(signal: 'power' | 'idle' | 'network', handler: SystemSignalHandler): void {
  const handlers = listeners.get(signal) || [];
  const index = handlers.indexOf(handler);
  if (index >= 0) {
    handlers.splice(index, 1);
  }
}

// Simulated events for Tauri (would be implemented via Tauri events in production)
let powerStateInterval: number | null = null;
let idleStateInterval: number | null = null;
let networkStateInterval: number | null = null;

export function startSystemMonitoring(): void {
  if (powerStateInterval === null) {
    powerStateInterval = window.setInterval(async () => {
      const state = await getPowerState();
      const handlers = listeners.get('power') || [];
      handlers.forEach(h => h(state));
    }, 30000);
  }

  if (idleStateInterval === null) {
    idleStateInterval = window.setInterval(async () => {
      const state = await getIdleState();
      const handlers = listeners.get('idle') || [];
      handlers.forEach(h => h(state));
    }, 5000);
  }

  if (networkStateInterval === null) {
    networkStateInterval = window.setInterval(async () => {
      const state = await getNetworkState();
      const handlers = listeners.get('network') || [];
      handlers.forEach(h => h(state));
    }, 10000);
  }
}

export function stopSystemMonitoring(): void {
  if (powerStateInterval !== null) {
    clearInterval(powerStateInterval);
    powerStateInterval = null;
  }
  if (idleStateInterval !== null) {
    clearInterval(idleStateInterval);
    idleStateInterval = null;
  }
  if (networkStateInterval !== null) {
    clearInterval(networkStateInterval);
    networkStateInterval = null;
  }
}