import { invoke } from '@tauri-apps/api/core';
import { open, save, message, confirm, ask } from '@tauri-apps/plugin-dialog';

export interface OpenDialogOptions {
  title?: string;
  defaultPath?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
  multiple?: boolean;
  directory?: boolean;
}

export interface SaveDialogOptions {
  title?: string;
  defaultPath?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
}

export async function showOpenDialog(options: OpenDialogOptions = {}): Promise<string | string[] | null> {
  return open({
    title: options.title,
    defaultPath: options.defaultPath,
    filters: options.filters,
    multiple: options.multiple,
    directory: options.directory,
  });
}

export async function showSaveDialog(options: SaveDialogOptions = {}): Promise<string | null> {
  return save({
    title: options.title,
    defaultPath: options.defaultPath,
    filters: options.filters,
  });
}

export async function showMessageDialog(
  message: string,
  type: 'info' | 'warning' | 'error' | 'question' = 'info',
  title: string = 'Ardia',
  buttons: string[] = ['OK']
): Promise<number> {
  return message(message, {
    kind: type,
    title,
    buttons,
  });
}

export async function showConfirmDialog(
  message: string,
  title: string = 'Confirm',
  okLabel: string = 'OK',
  cancelLabel: string = 'Cancel'
): Promise<boolean> {
  return confirm(message, { title, okLabel, cancelLabel });
}

export async function showInputDialog(
  message: string,
  title: string = 'Input',
  defaultValue: string = '',
  okLabel: string = 'OK',
  cancelLabel: string = 'Cancel'
): Promise<string | null> {
  return ask(message, { title, defaultValue, okLabel, cancelLabel });
}

export async function showFilePicker(options: OpenDialogOptions = {}): Promise<string | string[] | null> {
  return open({
    ...options,
    directory: false,
    multiple: options.multiple ?? false,
  });
}

export async function showFolderPicker(options: OpenDialogOptions = {}): Promise<string | null> {
  const result = open({
    ...options,
    directory: true,
    multiple: false,
  });
  return Array.isArray(result) ? result[0] : result;
}