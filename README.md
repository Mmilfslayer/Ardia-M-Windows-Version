# Ardia Windows

A Windows 11 port of Ardia browser built with **Rust**, **WebView2**, and **Tauri 2.0**.

> **Personal use only** - No code signing, no Store distribution, optimized for older hardware.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust Application                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐             │
│  │  TypeScript │ │   Rust      │ │  WebView2   │             │
│  │  UI/Logic   │ │  Native     │ │  (Edge/     │             │
│  │  (Shared)   │ │  Modules    │ │  Chromium)  │             │
│  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘             │
│         │               │               │                     │
│         ▼               ▼               ▼                     │
│  ┌─────────────────────────────────────────────┐             │
│  │        WebView2 Controller / JSI Bridge     │             │
│  └─────────────────────────────────────────────┘             │
└─────────────────────────────────────────────────────────────┘
```

### Crates

| Crate | Purpose |
|-------|---------|
| `ardia-webview2` | Safe Rust bindings for WebView2 COM interfaces |
| `ardia-core` | Core browser logic (windows, tabs, profiles, passwords, history, bookmarks, extensions, content blocker, downloads, PiP, sidebar, autocomplete, telemetry, IPC) |
| `ardia-ui` | Tauri 2.0 app with TypeScript/React frontend |

## Prerequisites

- **Windows 10/11** (WebView2 is built-in)
- **Rust** (stable, via `rustup`)
- **Node.js 22+** and **pnpm 9+**
- **Microsoft Edge WebView2 Runtime** (usually pre-installed on Windows 11)

## Quick Start

```powershell
# Clone and enter
cd Ardia-Windows

# Build everything (release mode)
.\scripts\build.ps1 -Release

# Run
.\scripts\run.ps1 -Release
```

### Development Mode

```powershell
# Build debug (faster compilation)
.\scripts\build.ps1

# Run with debug logging
.\scripts\run.ps1 -Debug -RemoteDebuggingPort 9222
```

## Project Structure

```
Ardia-Windows/
├── Cargo.toml                          # Workspace root
├── crates/
│   ├── ardia-core/                     # Core browser logic (Rust)
│   │   └── src/
│   │       ├── lib.rs                  # ArdiaCore entry point
│   │       ├── window.rs               # Window management
│   │       ├── tab.rs                  # Tab management
│   │       ├── webview.rs              # WebView wrapper
│   │       ├── profile.rs              # Profile management
│   │       ├── settings.rs             # Settings storage (SQLite)
│   │       ├── extensions.rs           # Extension API (MV3)
│   │       ├── downloads.rs            # Download manager
│   │       ├── history.rs              # History database (FTS5)
│   │       ├── bookmarks.rs            # Bookmarks database
│   │       ├── passwords.rs            # Password manager (DPAPI)
│   │       ├── autocomplete.rs         # URL/omnibox logic
│   │       ├── content_blocker.rs      # Ad/tracker blocking
│   │       ├── sidebar.rs              # Sidebar management
│   │       ├── pip.rs                  # Picture-in-picture
│   │       ├── telemetry.rs            # Local telemetry
│   │       └── ipc.rs                  # IPC with UI process
│   │
│   ├── ardia-ui/                       # UI layer (Tauri + TypeScript)
│   │   ├── src/
│   │   │   ├── main.rs                 # Tauri entry point
│   │   │   ├── commands.rs             # 70+ Tauri commands
│   │   │   ├── events.rs               # Event system
│   │   │   └── state.rs                # App state
│   │   ├── ui/                         # TypeScript/React UI
│   │   │   ├── src/
│   │   │   │   ├── components/         # BrowserWindow, Toolbar, TabStrip, Sidebar, WebView, etc.
│   │   │   │   ├── state/              # Zustand store
│   │   │   │   ├── platform/windows/   # 18 platform adapters
│   │   │   │   ├── styles/             # CSS variables, global styles
│   │   │   │   ├── App.tsx
│   │   │   │   └── main.tsx
│   │   │   ├── tauri.conf.json
│   │   │   ├── package.json
│   │   │   └── vite.config.ts
│   │   └── Cargo.toml
│   │
│   └── ardia-webview2/                 # WebView2 Rust bindings
│       └── src/
│           ├── lib.rs
│           ├── controller.rs
│           ├── environment.rs
│           ├── navigation.rs
│           ├── script.rs
│           ├── cookie.rs
│           ├── permission.rs
│           ├── download.rs
│           ├── devtools.rs
│           └── error.rs
│
├── scripts/
│   ├── build.ps1                       # Build script
│   └── run.ps1                         # Run script
│
├── .github/workflows/
│   └── windows.yml                     # CI/CD (GitHub Actions)
│
├── .milestone/                         # Milestone tracking
│
└── README.md
```

## Features

### Core Browser
- **Multiple windows & tabs** with drag-reorder, pin, mute
- **Profile isolation** (separate data directories)
- **Per-site settings** (cookies, permissions, zoom)

### Privacy & Security
- **Content blocker** (uBlock Origin filter lists, ABP syntax)
- **Password manager** with Windows DPAPI encryption (no master password)
- **Private/Incognito windows**
- **Permission management** (camera, mic, location, notifications)

### Developer Tools
- **DevTools** (F12) via WebView2 CDP
- **Remote debugging** port support
- **Console logging** with `RUST_LOG=debug`

### Extensions
- **Manifest V3** support
- **Content scripts** via `add_script_to_execute_on_document_created`
- **Background service workers**
- **Popup, options, devtools pages**

### UI/UX
- **Vertical tabs** sidebar (Arc/Dia style)
- **Omnibox** with autocomplete (history, bookmarks, search)
- **System theme** (light/dark/auto following Windows)
- **Picture-in-Picture** overlay
- **Download shelf**
- **Toast notifications**

## Configuration

Data stored in `%LOCALAPPDATA%\Ardia\`:

```
Ardia/
├── Profiles/
│   ├── default/
│   │   ├── settings.db
│   │   ├── history.db
│   │   ├── bookmarks.db
│   │   ├── passwords.db
│   │   └── WebView2/          # WebView2 user data
│   └── <profile-name>/
├── Extensions/
└── settings.toml              # Global settings
```

## Building for Distribution

```powershell
# Release build with optimizations
.\scripts\build.ps1 -Release -Clean

# Binary at:
# crates/ardia-ui/target/x86_64-pc-windows-msvc/release/ardia.exe
```

### GitHub Actions CI

Push to `main` triggers:
- Rust build (`cargo build --release --workspace`)
- UI build (`pnpm run build`)
- Unit tests (`cargo test`, `pnpm test`)
- Clippy linting
- Format check
- Artifacts uploaded (30-day retention)

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Tab | `Ctrl+T` |
| New Window | `Ctrl+N` |
| New Private Window | `Ctrl+Shift+N` |
| Close Tab | `Ctrl+W` |
| Close Window | `Ctrl+Shift+W` |
| Reopen Closed Tab | `Ctrl+Shift+T` |
| Focus Address Bar | `Ctrl+L` / `Alt+D` |
| Find in Page | `Ctrl+F` |
| Toggle Sidebar | `Ctrl+Shift+B` |
| Toggle Fullscreen | `F11` |
| DevTools | `F12` |
| Zoom In/Out/Reset | `Ctrl+Plus` / `Ctrl+Minus` / `Ctrl+0` |
| Next/Prev Tab | `Ctrl+Tab` / `Ctrl+Shift+Tab` |

## Why WebView2?

| Factor | WebView2 | Custom Chromium |
|--------|----------|-----------------|
| **Install size** | ~0 MB (built-in) | ~200 MB |
| **Memory (10 tabs)** | ~400 MB | ~800 MB |
| **Build time** | 0 min | 5+ hours |
| **Updates** | Automatic (with Edge) | Manual rebuild |
| **GPU acceleration** | Yes (DirectX) | Yes (Metal/DirectX) |
| **Compatibility** | Windows 10/11 | All platforms |

## Why Tauri?

- **Rust backend** = same language as core
- **WebView2 on Windows** = same engine
- **~10 MB binary** vs 100+ MB Electron
- **Reuse ~60% React components** from macOS Ardia
- **No custom protocol handler needed** (uses `tauri://`)

## Milestone Progress

- [x] M1: Project Setup
- [x] M2: WebView2 Wrapper
- [x] M3: Window/Tab Core
- [x] M4: Profile/Settings
- [x] M5: Passwords/Autofill
- [x] M6: Content Blocker
- [x] M7: History/Bookmarks
- [x] M8: Extensions
- [x] M9: Downloads/PiP
- [x] M10: Sidebar
- [x] M11: Tauri UI Setup
- [x] M12: Port Shared UI
- [x] M13: Platform Adapters
- [x] M14: Theme/Styling
- [x] M15: Build Scripts & CI
- [ ] M16: Testing/Polish
- [ ] M17: First Release

## License

Apache-2.0 (source) / GPL-3.0 (binary, due to WebView2/Helium dependencies)

See `NOTICE` for third-party attributions.

---

**Note**: This is a personal-use port. No warranties, no support, no auto-updater. Build from source or download artifacts from GitHub Actions.