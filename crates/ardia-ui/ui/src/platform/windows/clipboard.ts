import { invoke } from '@tauri-apps/api/core';
import { readText, writeText, readImage, writeImage } from '@tauri-apps/plugin-clipboard-manager';

export async function readClipboardText(): Promise<string> {
  return readText();
}

export async function writeClipboardText(text: string): Promise<void> {
  return writeText(text);
}

export async function readClipboardImage(): Promise<string | null> {
  return readImage();
}

export async function writeClipboardImage(image: string): Promise<void> {
  return writeImage(image);
}

export async function clearClipboard(): Promise<void> {
  return writeText('');
}

export async function hasClipboardText(): Promise<boolean> {
  try {
    const text = await readText();
    return text.length > 0;
  } catch {
    return false;
  }
}

export async function hasClipboardImage(): Promise<boolean> {
  try {
    const image = await readImage();
    return image !== null;
  } catch {
    return false;
  }
}