import { invoke } from '@tauri-apps/api/core';

export interface DownloadItem {
  id: string;
  url: string;
  filename: string;
  path?: string;
  progress: number;
  receivedBytes: number;
  totalBytes: number;
  status: 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled';
  error?: string;
  startTime: number;
  endTime?: number;
  referrer?: string;
  mimeType?: string;
}

export async function startDownload(
  url: string,
  filename?: string,
  referrer?: string
): Promise<string> {
  return invoke('download_start', { request: { url, filename, referrer } });
}

export async function cancelDownload(downloadId: string): Promise<void> {
  return invoke('download_cancel', { download_id: downloadId });
}

export async function pauseDownload(downloadId: string): Promise<void> {
  return invoke('download_pause', { download_id: downloadId });
}

export async function resumeDownload(downloadId: string): Promise<void> {
  return invoke('download_resume', { download_id: downloadId });
}

export async function getDownloads(): Promise<DownloadItem[]> {
  return invoke('download_get_list');
}

export async function clearCompletedDownloads(): Promise<void> {
  return invoke('download_clear');
}

export async function getDownloadPath(): Promise<string> {
  return invoke('download_get_path');
}

export async function setDownloadPath(path: string): Promise<void> {
  return invoke('download_set_path', { path });
}

export async function showDownloadInFolder(downloadId: string): Promise<void> {
  return invoke('download_show_in_folder', { download_id: downloadId });
}

export async function openDownload(downloadId: string): Promise<void> {
  return invoke('download_open', { download_id: downloadId });
}

export async function removeDownload(downloadId: string): Promise<void> {
  return invoke('download_remove', { download_id: downloadId });
}

export async function retryDownload(downloadId: string): Promise<void> {
  return invoke('download_retry', { download_id: downloadId });
}