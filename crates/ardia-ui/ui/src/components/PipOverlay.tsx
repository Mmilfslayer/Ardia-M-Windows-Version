import React, { useState, useEffect, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';
import { Volume2, VolumeX, Play, Pause, SkipBack, SkipForward, RotateCw, Maximize2, X } from 'lucide-react';

interface PipOverlayProps {
  pipWindows: any[];
  onExit: (pipId: string) => void;
}

export function PipOverlay({ pipWindows, onExit }: PipOverlayProps) {
  const [activePip, setActivePip] = useState(pipWindows[0]);
  const [showControls, setShowControls] = useState(false);
  const [volume, setVolume] = useState(1);
  const [playbackRate, setPlaybackRate] = useState(1);
  const pipRef = useRef<HTMLDivElement>(null);

  // Handle keyboard shortcuts for PiP
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!activePip) return;

      switch (e.key) {
        case ' ':
        case 'k':
          e.preventDefault();
          togglePlayPause();
          break;
        case 'ArrowLeft':
          e.preventDefault();
          seek(-10);
          break;
        case 'ArrowRight':
          e.preventDefault();
          seek(10);
          break;
        case 'ArrowUp':
          e.preventDefault();
          adjustVolume(0.1);
          break;
        case 'ArrowDown':
          e.preventDefault();
          adjustVolume(-0.1);
          break;
        case 'm':
          toggleMute();
          break;
        case '>':
        case '.':
          changePlaybackRate(0.25);
          break;
        case '<':
        case ',':
          changePlaybackRate(-0.25);
          break;
        case 'f':
          toggleFullscreen();
          break;
        case 'Escape':
          onExit(activePip.id);
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [activePip, onExit]);

  const togglePlayPause = useCallback(async () => {
    if (!activePip) return;
    // Send play/pause command to the video element in PiP window
    await invoke('pip_send_command', {
      pip_id: activePip.id,
      command: activePip.paused ? 'play' : 'pause',
    });
  }, [activePip]);

  const seek = useCallback(async (seconds: number) => {
    if (!activePip) return;
    await invoke('pip_send_command', {
      pip_id: activePip.id,
      command: 'seek',
      value: seconds,
    });
  }, [activePip]);

  const adjustVolume = useCallback(async (delta: number) => {
    if (!activePip) return;
    const newVolume = Math.max(0, Math.min(1, volume + delta));
    setVolume(newVolume);
    await invoke('pip_send_command', {
      pip_id: activePip.id,
      command: 'volume',
      value: newVolume,
    });
  }, [activePip, volume]);

  const toggleMute = useCallback(async () => {
    if (!activePip) return;
    await invoke('pip_send_command', {
      pip_id: activePip.id,
      command: 'mute',
    });
    // Volume will be updated via event
  }, [activePip]);

  const changePlaybackRate = useCallback(async (delta: number) => {
    if (!activePip) return;
    const rates = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2, 2.5, 3, 4];
    const currentIndex = rates.indexOf(playbackRate);
    const newIndex = Math.max(0, Math.min(rates.length - 1, currentIndex + (delta > 0 ? 1 : -1)));
    const newRate = rates[newIndex];
    setPlaybackRate(newRate);
    await invoke('pip_set_rate', { pip_id: activePip.id, rate: newRate });
  }, [activePip, playbackRate]);

  const toggleFullscreen = useCallback(async () => {
    if (!activePip) return;
    await invoke('pip_send_command', {
      pip_id: activePip.id,
      command: 'fullscreen',
    });
  }, [activePip]);

  const handlePipClick = useCallback((pip: any) => {
    setActivePip(pip);
  }, []);

  const handleMouseEnter = useCallback(() => setShowControls(true), []);
  const handleMouseLeave = useCallback(() => setShowControls(false), []);

  if (pipWindows.length === 0) return null;

  return (
    <div
      ref={pipRef}
      className="pip-overlay fixed bottom-4 right-4 z-[var(--z-tooltip)] flex flex-col gap-2 pointer-events-none"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {pipWindows.map((pip) => (
        <div
          key={pip.id}
          className={`pip-window relative bg-bg-modal border border-border rounded-xl shadow-xl overflow-hidden transition-all duration-200 ${
            pip.id === activePip.id ? 'ring-2 ring-accent' : 'opacity-70'
          } pointer-events-auto`}
          onClick={() => handlePipClick(pip)}
          style={{
            width: pip.bounds.width,
            height: pip.bounds.height,
            transform: `translate(${pip.bounds.x}px, ${pip.bounds.y}px)`,
          }}
        >
          {/* The actual PiP video element would be here, managed by the WebView */}
          <div
            className="w-full h-full bg-black relative"
            id={`pip-video-${pip.id}`}
          >
            {/* Video element is injected by WebView */}
          </div>

          {/* Controls overlay */}
          {showControls && pip.id === activePip.id && (
            <div className="absolute bottom-0 left-0 right-0 p-3 bg-gradient-to-t from-black/80 to-transparent flex items-center gap-2">
              <button
                onClick={(e) => { e.stopPropagation(); togglePlayPause(); }}
                className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                aria-label={activePip.paused ? 'Play' : 'Pause'}
              >
                {activePip.paused ? <Play className="w-5 h-5" /> : <Pause className="w-5 h-5" />}
              </button>

              <button
                onClick={(e) => { e.stopPropagation(); seek(-10); }}
                className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                aria-label="Rewind 10 seconds"
              >
                <SkipBack className="w-5 h-5" />
              </button>

              <button
                onClick={(e) => { e.stopPropagation(); seek(10); }}
                className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                aria-label="Forward 10 seconds"
              >
                <SkipForward className="w-5 h-5" />
              </button>

              <div className="flex-1" />

              <div className="flex items-center gap-1">
                <button
                  onClick={(e) => { e.stopPropagation(); toggleMute(); }}
                  className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                  aria-label={activePip.isMuted ? 'Unmute' : 'Mute'}
                >
                  {activePip.isMuted || volume === 0 ? <VolumeX className="w-5 h-5" /> : <Volume2 className="w-5 h-5" />}
                </button>

                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={activePip.isMuted ? 0 : volume}
                  onChange={(e) => {
                    e.stopPropagation();
                    const newVolume = parseFloat(e.target.value);
                    setVolume(newVolume);
                    invoke('pip_send_command', {
                      pip_id: pip.id,
                      command: 'volume',
                      value: newVolume,
                    });
                  }}
                  className="w-20 h-1 bg-white/20 rounded-full appearance-none accent-white"
                />
              </div>

              <div className="flex items-center gap-2">
                <span className="text-white text-xs px-2 py-1 bg-white/10 rounded">{playbackRate.toFixed(2)}x</span>
                <button
                  onClick={(e) => { e.stopPropagation(); changePlaybackRate(-0.25); }}
                  className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                  aria-label="Decrease playback speed"
                >
                  <RotateCw className="w-4 h-4 -rotate-90" />
                </button>
                <button
                  onClick={(e) => { e.stopPropagation(); changePlaybackRate(0.25); }}
                  className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                  aria-label="Increase playback speed"
                >
                  <RotateCw className="w-4 h-4" />
                </button>
              </div>

              <button
                onClick={(e) => { e.stopPropagation(); toggleFullscreen(); }}
                className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                aria-label="Toggle fullscreen"
              >
                <Maximize2 className="w-5 h-5" />
              </button>

              <button
                onClick={(e) => { e.stopPropagation(); onExit(pip.id); }}
                className="p-2 rounded-full bg-white/10 hover:bg-white/20 text-white transition-colors"
                aria-label="Exit Picture in Picture"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
          )}
        </div>
      ))}

      {/* Mini controller for inactive PiP windows */}
      {pipWindows.length > 1 && (
        <div className="pip-mini-controller fixed bottom-4 left-4 z-[var(--z-tooltip)] flex gap-1 bg-bg-modal border border-border rounded-lg p-2 pointer-events-auto">
          {pipWindows.map((pip) => (
            <button
              key={pip.id}
              onClick={() => handlePipClick(pip)}
              className={`p-2 rounded transition-colors ${pip.id === activePip.id ? 'bg-accent text-white' : 'bg-bg-tertiary text-fg-secondary hover:bg-bg-hover'}`}
              aria-label={`Switch to ${pip.title || 'PiP window'}`}
            >
              <Popcorn className="w-4 h-4" />
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

// Import Popcorn icon
import { Popcorn } from 'lucide-react';