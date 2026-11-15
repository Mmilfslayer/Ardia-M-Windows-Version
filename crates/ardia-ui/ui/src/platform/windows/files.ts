import { invoke } from '@tauri-apps/api/core';
import { BaseDirectory, readDir, readFile, writeFile, createDir, removeDir, removeFile, exists, copyFile, renameFile, stat } from '@tauri-apps/plugin-fs';

export async function readFileText(path: string, baseDir?: BaseDirectory): Promise<string> {
  return readFile(path, { dir: baseDir });
}

export async function writeFileText(path: string, contents: string, baseDir?: BaseDirectory): Promise<void> {
  return writeFile(path, contents, { dir: baseDir });
}

export async function readFileBinary(path: string, baseDir?: BaseDirectory): Promise<Uint8Array> {
  return readFile(path, { dir: baseDir });
}

export async function writeFileBinary(path: string, contents: Uint8Array, baseDir?: BaseDirectory): Promise<void> {
  return writeFile(path, contents, { dir: baseDir });
}

export async function listDir(path: string, baseDir?: BaseDirectory): Promise<Array<{ name: string; isFile: boolean; isDir: boolean; size: number; modified: number }>> {
  const entries = await readDir(path, { dir: baseDir });
  return entries.map(e => ({
    name: e.name,
    isFile: e.isFile,
    isDir: e.isDirectory,
    size: e.size || 0,
    modified: e.mtime || 0,
  }));
}

export async function createDirectory(path: string, baseDir?: BaseDirectory, recursive = true): Promise<void> {
  return createDir(path, { dir: baseDir, recursive });
}

export async function removeDirectory(path: string, baseDir?: BaseDirectory, recursive = true): Promise<void> {
  return removeDir(path, { dir: baseDir, recursive });
}

export async function deleteFile(path: string, baseDir?: BaseDirectory): Promise<void> {
  return removeFile(path, { dir: baseDir });
}

export async function fileExists(path: string, baseDir?: BaseDirectory): Promise<boolean> {
  return exists(path, { dir: baseDir });
}

export async function copyFileTo(source: string, dest: string, baseDir?: BaseDirectory): Promise<void> {
  return copyFile(source, dest, { dir: baseDir });
}

export async function renameFileTo(oldPath: string, newPath: string, baseDir?: BaseDirectory): Promise<void> {
  return renameFile(oldPath, newPath, { dir: baseDir });
}

export async function getFileInfo(path: string, baseDir?: BaseDirectory): Promise<{ size: number; isFile: boolean; isDir: boolean; modified: number; created: number }> {
  const info = await stat(path, { dir: baseDir });
  return {
    size: info.size,
    isFile: info.isFile,
    isDir: info.isDirectory,
    modified: info.mtime,
    created: info.atime,
  };
}

export async function getAppDataPath(): Promise<string> {
  return invoke('app_get_data_dir');
}

export async function getUserDataPath(): Promise<string> {
  return invoke('app_get_user_data_dir');
}

export async function getCachePath(): Promise<string> {
  return invoke('app_get_cache_dir');
}

export async function getTempPath(): Promise<string> {
  return invoke('app_get_temp_dir');
}