use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;
use crate::tab::TabId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipState {
    Inactive,
    Entering,
    Active,
    Exiting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipWindow {
    pub id: PipId,
    pub tab_id: TabId,
    pub window_id: String,
    pub bounds: PipBounds,
    pub is_muted: bool,
    pub playback_rate: f64,
    pub state: PipState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PipId(pub Uuid);

impl PipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for PipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PipBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for PipBounds {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 320.0,
            height: 180.0,
        }
    }
}

pub struct PipManager {
    pip_windows: Arc<RwLock<HashMap<PipId, PipWindow>>>,
    active_pip: Arc<RwLock<Option<PipId>>>,
}

impl PipManager {
    pub fn new() -> Self {
        Self {
            pip_windows: Arc::new(RwLock::new(HashMap::new())),
            active_pip: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn enter_pip(&self, tab_id: TabId, window_id: String) -> CoreResult<PipId> {
        let pip_id = PipId::new();
        let bounds = PipBounds::default();

        let pip_window = PipWindow {
            id: pip_id,
            tab_id,
            window_id,
            bounds,
            is_muted: false,
            playback_rate: 1.0,
            state: PipState::Entering,
        };

        {
            let mut pips = self.pip_windows.write().await;
            pips.insert(pip_id, pip_window);
        }

        {
            let mut active = self.active_pip.write().await;
            *active = Some(pip_id);
        }

        // Transition to active after a short delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        self.set_state(pip_id, PipState::Active).await?;

        Ok(pip_id)
    }

    pub async fn exit_pip(&self, pip_id: PipId) -> CoreResult<()> {
        self.set_state(pip_id, PipState::Exiting).await?;

        // Remove after animation
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let mut pips = self.pip_windows.write().await;
        pips.remove(&pip_id);

        let mut active = self.active_pip.write().await;
        if *active == Some(pip_id) {
            *active = None;
        }

        Ok(())
    }

    pub async fn set_state(&self, pip_id: PipId, state: PipState) -> CoreResult<()> {
        let mut pips = self.pip_windows.write().await;
        if let Some(pip) = pips.get_mut(&pip_id) {
            pip.state = state;
        }
        Ok(())
    }

    pub async fn update_bounds(&self, pip_id: PipId, bounds: PipBounds) -> CoreResult<()> {
        let mut pips = self.pip_windows.write().await;
        if let Some(pip) = pips.get_mut(&pip_id) {
            pip.bounds = bounds;
        }
        Ok(())
    }

    pub async fn set_muted(&self, pip_id: PipId, muted: bool) -> CoreResult<()> {
        let mut pips = self.pip_windows.write().await;
        if let Some(pip) = pips.get_mut(&pip_id) {
            pip.is_muted = muted;
        }
        Ok(())
    }

    pub async fn set_playback_rate(&self, pip_id: PipId, rate: f64) -> CoreResult<()> {
        let mut pips = self.pip_windows.write().await;
        if let Some(pip) = pips.get_mut(&pip_id) {
            pip.playback_rate = rate.clamp(0.25, 4.0);
        }
        Ok(())
    }

    pub async fn get_pip(&self, pip_id: PipId) -> Option<PipWindow> {
        let pips = self.pip_windows.read().await;
        pips.get(&pip_id).cloned()
    }

    pub async fn get_active_pip(&self) -> Option<PipWindow> {
        let active = self.active_pip.read().await;
        if let Some(id) = *active {
            let pips = self.pip_windows.read().await;
            pips.get(&id).cloned()
        } else {
            None
        }
    }

    pub async fn get_pip_for_tab(&self, tab_id: TabId) -> Option<PipWindow> {
        let pips = self.pip_windows.read().await;
        pips.values().find(|p| p.tab_id == tab_id).cloned()
    }

    pub async fn get_all_pips(&self) -> Vec<PipWindow> {
        let pips = self.pip_windows.read().await;
        pips.values().cloned().collect()
    }

    pub async fn is_tab_in_pip(&self, tab_id: TabId) -> bool {
        let pips = self.pip_windows.read().await;
        pips.values().any(|p| p.tab_id == tab_id)
    }

    pub async fn toggle_mute(&self, pip_id: PipId) -> CoreResult<Option<bool>> {
        let mut pips = self.pip_windows.write().await;
        if let Some(pip) = pips.get_mut(&pip_id) {
            pip.is_muted = !pip.is_muted;
            Ok(Some(pip.is_muted))
        } else {
            Ok(None)
        }
    }

    pub async fn close_all(&self) -> CoreResult<()> {
        let pips: Vec<PipId> = {
            let pips = self.pip_windows.read().await;
            pips.keys().cloned().collect()
        };

        for pip_id in pips {
            self.exit_pip(pip_id).await?;
        }

        Ok(())
    }
}

impl Default for PipManager {
    fn default() -> Self {
        Self::new()
    }
}

// JavaScript to inject for PiP support
pub const PIP_SCRIPT: &str = r#"
(function() {
    'use strict';

    window.ardiaPip = {
        currentVideo: null,
        pipWindow: null,

        async enter() {
            const videos = document.querySelectorAll('video');
            if (videos.length === 0) return false;

            // Find the most likely video (largest, playing, etc.)
            let bestVideo = null;
            let bestScore = -1;

            for (const video of videos) {
                if (video.readyState < 2) continue;
                const rect = video.getBoundingClientRect();
                const area = rect.width * rect.height;
                let score = area;
                if (!video.paused) score += 10000;
                if (video.hasAttribute('controls')) score += 5000;
                if (score > bestScore) {
                    bestScore = score;
                    bestVideo = video;
                }
            }

            if (!bestVideo) return false;

            try {
                this.currentVideo = bestVideo;
                await bestVideo.requestPictureInPicture();
                return true;
            } catch (e) {
                console.error('PiP request failed:', e);
                return false;
            }
        },

        async exit() {
            if (document.pictureInPictureElement) {
                try {
                    await document.exitPictureInPicture();
                    return true;
                } catch (e) {
                    console.error('PiP exit failed:', e);
                    return false;
                }
            }
            return false;
        },

        toggleMute() {
            if (this.currentVideo) {
                this.currentVideo.muted = !this.currentVideo.muted;
                return this.currentVideo.muted;
            }
            return null;
        },

        setPlaybackRate(rate) {
            if (this.currentVideo) {
                this.currentVideo.playbackRate = rate;
                return true;
            }
            return false;
        },

        getState() {
            return {
                isInPip: !!document.pictureInPictureElement,
                video: this.currentVideo ? {
                    src: this.currentVideo.src,
                    muted: this.currentVideo.muted,
                    playbackRate: this.currentVideo.playbackRate,
                    paused: this.currentVideo.paused,
                    duration: this.currentVideo.duration,
                    currentTime: this.currentVideo.currentTime,
                } : null,
            };
        },
    };

    // Listen for PiP events
    document.addEventListener('enterpictureinpicture', (e) => {
        window.ardiaPip.currentVideo = e.pictureInPictureWindow;
        if (window.ardiaOnPipEnter) {
            window.ardiaOnPipEnter(e.pictureInPictureWindow);
        }
    });

    document.addEventListener('leavepictureinpicture', () => {
        if (window.ardiaOnPipLeave) {
            window.ardiaOnPipLeave();
        }
        window.ardiaPip.currentVideo = null;
    });

    document.addEventListener('resize', (e) => {
        if (e.target === document.pictureInPictureElement && window.ardiaOnPipResize) {
            const win = e.target;
            window.ardiaOnPipResize({
                width: win.width,
                height: win.height,
            });
        }
    });

    console.log('Ardia PiP script injected');
})();
"#;

pub const PIP_STYLES: &str = r#"
/* Picture-in-Picture window styles */
:root {
    --pip-border-radius: 8px;
    --pip-box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    --pip-z-index: 2147483647;
}

picture-in-picture-window {
    border-radius: var(--pip-border-radius) !important;
    box-shadow: var(--pip-box-shadow) !important;
    z-index: var(--pip-z-index) !important;
}

/* Custom PiP overlay controls (injected into page) */
.ardia-pip-overlay {
    position: fixed;
    bottom: 8px;
    right: 8px;
    display: flex;
    gap: 4px;
    padding: 8px;
    background: rgba(0, 0, 0, 0.8);
    border-radius: 8px;
    backdrop-filter: blur(10px);
    z-index: 2147483646;
    opacity: 0;
    transition: opacity 0.2s ease;
    pointer-events: none;
}

.ardia-pip-overlay:hover,
.ardia-pip-overlay:focus-within {
    opacity: 1;
    pointer-events: auto;
}

.ardia-pip-btn {
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.2);
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s ease;
}

.ardia-pip-btn:hover {
    background: rgba(255, 255, 255, 0.3);
}

.ardia-pip-btn:active {
    background: rgba(255, 255, 255, 0.4);
}

.ardia-pip-btn svg {
    width: 18px;
    height: 18px;
}
"#;