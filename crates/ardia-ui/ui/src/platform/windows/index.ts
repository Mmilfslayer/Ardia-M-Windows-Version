// Ardia Windows Platform Adapters
// All platform-specific implementations for Windows using Tauri commands

// Core platform modules
export * from './app';
export * from './autofill';
export * from './bookmarks';
export * from './clipboard';
export * from './commands';
export * from './configFolder';
export * from './contentBlocker';
export * from './dev';
export * from './dialogs';
export * from './downloads';
export * from './extensions';
export * from './files';
export * from './focus';
export * from './history';
export * from './importModule';
export * from './keychain';
export * from './launchConfig';
export * from './media';
export * from './menus';
export * from './nativeApi';
export * from './nativeEvents';
export * from './notifications';
export * from './permissions';
export * from './profiles';
export * from './scripting';
export * from './search';
export * from './secureRandom';
export * from './siteSettings';
export * from './sync';
export * from './systemSignals';
export * from './tabs';
export * from './telemetryLocal';
export * from './userScripts';
export * from './webviews';

// Re-export commonly used types
export type { MenuItemOptions } from './menus';
export type { Profile } from './profiles';
export type { Tab } from './tabs';
export type { WebViewConfig, WebViewState } from './webviews';
export type { SearchEngine, SearchSuggestion } from './search';
export type { PermissionType, PermissionState } from './permissions';
export type { SiteSettingType, SiteSettingValue } from './siteSettings';
export type { UserScript } from './userScripts';
export type { TelemetryEvent, TelemetryConfig } from './telemetryLocal';
export type { NotificationOptions } from './notifications';
export type { PowerState, SystemIdleState, NetworkState } from './systemSignals';