import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { useStore } from '../state/store';
import { X, Pause, Play, Trash2, FileText, Clock, CheckCircle, AlertCircle, Loader2, ExternalLink } from 'lucide-react';

interface DownloadShelfProps {
  downloads: any[];
  onAction: (downloadId: string, action: 'pause' | 'resume' | 'cancel') => void;
  onClose: () => void;
  visible: boolean;
}

export function DownloadShelf({ downloads, onAction, onClose, visible }: DownloadShelfProps) {
  const [animating, setAnimating] = useState<string | null>(null);

  if (!visible || downloads.length === 0) {
    return null;
  }

  const handleRemove = (downloadId: string) => {
    setAnimating(downloadId);
    setTimeout(() => {
      onAction(downloadId, 'cancel');
      setAnimating(null);
    }, 200);
  };

  const handleShowInFolder = async (downloadId: string) => {
    const download = downloads.find(d => d.id === downloadId);
    if (download?.path) {
      await invoke('opener', { path: download.path });
    }
  };

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatTime = (ms: number) => {
    if (ms < 1000) return `${ms}ms`;
    const seconds = Math.floor(ms / 1000);
    if (seconds < 60) return `${seconds}s`;
    const minutes = Math.floor(seconds / 60);
    if (minutes < 60) return `${minutes}m ${seconds % 60}s`;
    const hours = Math.floor(minutes / 60);
    return `${hours}h ${minutes % 60}m`;
  };

  const activeDownloads = downloads.filter(d => d.status === 'downloading' || d.status === 'paused');
  const completedDownloads = downloads.filter(d => d.status === 'completed');

  return (
    <div
      className={`download-shelf fixed bottom-0 left-0 right-0 z-[var(--z-toast)] bg-bg-secondary border-t border-border animate-slide-in ${!visible ? 'hidden' : ''}`}
      role="region"
      aria-label="Downloads"
    >
      <div className="max-w-4xl mx-auto p-4">
        {/* Header */}
        <div className="flex items-center justify-between mb-3">
          <h3 className="font-semibold">Downloads</h3>
          <div className="flex items-center gap-2">
            {completedDownloads.length > 0 && (
              <button
                onClick={() => invoke('download_clear')}
                className="text-sm text-fg-secondary hover:text-fg-primary"
              >
                Clear completed
              </button>
            )}
            <button
              onClick={onClose}
              className="p-1 text-fg-muted hover:text-fg-primary rounded"
              aria-label="Close downloads"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Active downloads */}
        {activeDownloads.length > 0 && (
          <div className="space-y-2 mb-4">
            <h4 className="text-xs font-semibold text-fg-muted uppercase tracking-wide">In Progress</h4>
            {activeDownloads.map((download) => (
              <DownloadItem
                key={download.id}
                download={download}
                onAction={onAction}
                onRemove={handleRemove}
                onShowInFolder={handleShowInFolder}
                animating={animating === download.id}
                formatBytes={formatBytes}
                formatTime={formatTime}
              />
            ))}
          </div>
        )}

        {/* Completed downloads */}
        {completedDownloads.length > 0 && (
          <div className="space-y-2">
            <h4 className="text-xs font-semibold text-fg-muted uppercase tracking-wide">Completed</h4>
            {completedDownloads.map((download) => (
              <DownloadItem
                key={download.id}
                download={download}
                onAction={onAction}
                onRemove={handleRemove}
                onShowInFolder={handleShowInFolder}
                animating={animating === download.id}
                formatBytes={formatBytes}
                formatTime={formatTime}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

interface DownloadItemProps {
  download: any;
  onAction: (downloadId: string, action: 'pause' | 'resume' | 'cancel') => void;
  onRemove: (downloadId: string) => void;
  onShowInFolder: (downloadId: string) => void;
  animating: boolean;
  formatBytes: (bytes: number) => string;
  formatTime: (ms: number) => string;
}

function DownloadItem({
  download,
  onAction,
  onRemove,
  onShowInFolder,
  animating,
  formatBytes,
  formatTime,
}: DownloadItemProps) {
  const isComplete = download.status === 'completed';
  const isFailed = download.status === 'failed';
  const isDownloading = download.status === 'downloading';
  const isPaused = download.status === 'paused';

  return (
    <div
      className={`flex items-center gap-3 p-3 bg-bg-primary border border-border rounded-lg transition-all duration-200 ${
        animating ? 'opacity-0 scale-95' : 'opacity-100 scale-100'
      } ${isComplete ? 'border-success' : ''} ${isFailed ? 'border-error' : ''}`}
      style={{ animation: animating ? 'slideOut 0.2s ease-in forwards' : 'none' }}
    >
      <style jsx>{`
        @keyframes slideOut {
          from { opacity: 1; transform: scale(1); }
          to { opacity: 0; transform: scale(0.95); }
        }
      `}</style>

      {/* File icon */}
      <div className={`w-10 h-10 rounded flex items-center justify-center flex-shrink-0 ${
        isComplete ? 'bg-success-light text-success' :
        isFailed ? 'bg-error-light text-error' :
        'bg-bg-tertiary text-fg-muted'
      }`}>
        {isComplete ? (
          <CheckCircle className="w-5 h-5" />
        ) : isFailed ? (
          <AlertCircle className="w-5 h-5" />
        ) : (
          <FileText className="w-5 h-5" />
        )}
      </div>

      {/* File info */}
      <div className="flex-1 min-w-0">
        <div className="font-medium text-sm truncate">{download.filename}</div>
        <div className="flex items-center gap-4 text-xs text-fg-muted mt-1">
          <span>{formatBytes(download.receivedBytes)} / {formatBytes(download.totalBytes)}</span>
          {download.startTime && (
            <span>{formatTime(Date.now() - download.startTime)}</span>
          )}
          {isDownloading && download.totalBytes > 0 && (
            <span>{Math.round((download.receivedBytes / download.totalBytes) * 100)}%</span>
          )}
        </div>

        {/* Progress bar */}
        <div className="h-1.5 bg-bg-tertiary rounded-full overflow-hidden mt-2">
          <div
            className={`h-full transition-all duration-300 ${
              isComplete ? 'bg-success' :
              isFailed ? 'bg-error' :
              'bg-accent'
            }`}
            style={{ width: `${Math.min(100, Math.max(0, download.progress))}%` }}
          />
        </div>

        {/* Status text */}
        <div className="text-xs text-fg-muted mt-1 flex items-center gap-2">
          {isDownloading && <Loader2 className="w-3 h-3 animate-spin" />}
          {isDownloading && `Downloading... ${download.progress}%`}
          {isPaused && 'Paused'}
          {isComplete && 'Completed'}
          {isFailed && `Failed: ${download.error || 'Unknown error'}`}
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-1">
        {isDownloading && (
          <button
            onClick={() => onAction(download.id, 'pause')}
            className="p-1.5 text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded transition-fast"
            aria-label="Pause download"
          >
            <Pause className="w-4 h-4" />
          </button>
        )}

        {isPaused && (
          <button
            onClick={() => onAction(download.id, 'resume')}
            className="p-1.5 text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded transition-fast"
            aria-label="Resume download"
          >
            <Play className="w-4 h-4" />
          </button>
        )}

        {isComplete && (
          <button
            onClick={() => onShowInFolder(download.id)}
            className="p-1.5 text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded transition-fast"
            aria-label="Show in folder"
          >
            <ExternalLink className="w-4 h-4" />
          </button>
        )}

        <button
          onClick={() => onRemove(download.id)}
          className="p-1.5 text-fg-muted hover:text-fg-primary hover:bg-bg-hover rounded transition-fast"
          aria-label={isComplete || isFailed ? 'Remove from list' : 'Cancel download'}
        >
          <X className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}