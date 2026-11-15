#!/usr/bin/env pwsh
# Ardia Windows - Build Script
# Builds the entire workspace: Rust + TypeScript/UI

param(
    [switch]$Release,
    [switch]$Clean,
    [switch]$SkipUi,
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition | Split-Path -Parent
$uiPath = Join-Path $repoRoot "crates\ardia-ui\ui"
$cargoTarget = if ($Release) { "--release" } else { "" }

Write-Host "=== Ardia Windows Build ===" -ForegroundColor Cyan
Write-Host "Repo: $repoRoot" -ForegroundColor Gray
Write-Host "Release: $Release" -ForegroundColor Gray
Write-Host "Clean: $Clean" -ForegroundColor Gray

# Clean if requested
if ($Clean) {
    Write-Host "`n--- Cleaning ---" -ForegroundColor Yellow
    cargo clean --workspace
    if (Test-Path (Join-Path $uiPath "node_modules")) {
        Remove-Item (Join-Path $uiPath "node_modules") -Recurse -Force
    }
    if (Test-Path (Join-Path $uiPath "dist")) {
        Remove-Item (Join-Path $uiPath "dist") -Recurse -Force
    }
    if (Test-Path (Join-Path $repoRoot "crates\ardia-ui\target")) {
        Remove-Item (Join-Path $repoRoot "crates\ardia-ui\target") -Recurse -Force
    }
}

# Check Rust toolchain
Write-Host "`n--- Checking Rust Toolchain ---" -ForegroundColor Yellow
$rustVersion = cargo --version
Write-Host "Rust: $rustVersion" -ForegroundColor Gray

# Check if webview2 is available
Write-Host "`n--- Checking WebView2 Runtime ---" -ForegroundColor Yellow
try {
    $webview2Path = "${env:ProgramFiles(x86)}\Microsoft\EdgeWebView\Application\"
    if (Test-Path $webview2Path) {
        $versions = Get-ChildItem $webview2Path | Where-Object { $_.PSIsContainer } | Sort-Object Name -Descending
        if ($versions.Count -gt 0) {
            Write-Host "WebView2 found: $($versions[0].Name)" -ForegroundColor Green
        } else {
            Write-Host "WebView2 runtime not found - please install Microsoft Edge WebView2 Runtime" -ForegroundColor Red
        }
    } else {
        Write-Host "WebView2 runtime not found - please install Microsoft Edge WebView2 Runtime" -ForegroundColor Red
    }
} catch {
    Write-Host "Could not check WebView2: $_" -ForegroundColor Red
}

# Build Rust workspace
Write-Host "`n--- Building Rust Workspace ---" -ForegroundColor Yellow
$cargoArgs = @("build", "--workspace", $cargoTarget, "--target", $Target)
if ($Release) { $cargoArgs += "--locked" }
& cargo @cargoArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Build UI if not skipped
if (-not $SkipUi) {
    Write-Host "`n--- Building UI (TypeScript/React) ---" -ForegroundColor Yellow

    if (-not (Test-Path (Join-Path $uiPath "package.json"))) {
        Write-Host "package.json not found at $uiPath" -ForegroundColor Red
        exit 1
    }

    # Install dependencies if needed
    if (-not (Test-Path (Join-Path $uiPath "node_modules"))) {
        Write-Host "Installing npm dependencies..." -ForegroundColor Gray
        Push-Location $uiPath
        & pnpm install --frozen-lockfile
        Pop-Location
        if ($LASTEXITCODE -ne 0) {
            Write-Host "pnpm install failed!" -ForegroundColor Red
            exit $LASTEXITCODE
        }
    }

    # Build TypeScript/UI
    Push-Location $uiPath
    & pnpm run build
    Pop-Location

    if ($LASTEXITCODE -ne 0) {
        Write-Host "UI build failed!" -ForegroundColor Red
        exit $LASTEXITCODE
    }
}

# Summary
Write-Host "`n=== Build Complete ===" -ForegroundColor Green
$binaryPath = Join-Path $repoRoot "crates\ardia-ui\target\$Target"
if ($Release) { $binaryPath = Join-Path $binaryPath "release" } else { $binaryPath = Join-Path $binaryPath "debug" }
$binaryPath = Join-Path $binaryPath "ardia.exe"

if (Test-Path $binaryPath) {
    Write-Host "Binary: $binaryPath" -ForegroundColor Cyan
    $size = (Get-Item $binaryPath).Length / 1MB
    Write-Host "Size: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
} else {
    Write-Host "Binary not found at expected path" -ForegroundColor Yellow
}

Write-Host "`nTo run: .\scripts\run.ps1" -ForegroundColor Gray