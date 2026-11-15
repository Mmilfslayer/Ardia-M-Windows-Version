#!/usr/bin/env pwsh
# Ardia Windows - Run Script
# Runs the application in development or release mode

param(
    [switch]$Release,
    [switch]$Dev,
    [string]$Profile = "default",
    [string]$Url = "",
    [switch]$Private,
    [string]$DataDir = "",
    [switch]$Debug,
    [int]$RemoteDebuggingPort = 0,
    [switch]$Background
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition | Split-Path -Parent
$uiPath = Join-Path $repoRoot "crates\ardia-ui\ui"

Write-Host "=== Ardia Windows Run ===" -ForegroundColor Cyan
Write-Host "Repo: $repoRoot" -ForegroundColor Gray

# Determine binary path
$target = "x86_64-pc-windows-msvc"
$binaryDir = Join-Path $repoRoot "crates\ardia-ui\target\$target"
if ($Release) {
    $binaryDir = Join-Path $binaryDir "release"
    Write-Host "Mode: Release" -ForegroundColor Gray
} else {
    $binaryDir = Join-Path $binaryDir "debug"
    Write-Host "Mode: Debug" -ForegroundColor Gray
}

$binaryPath = Join-Path $binaryDir "ardia.exe"

# Check if binary exists
if (-not (Test-Path $binaryPath)) {
    Write-Host "Binary not found at $binaryPath" -ForegroundColor Red
    Write-Host "Run .\scripts\build.ps1 first" -ForegroundColor Yellow
    exit 1
}

# Prepare environment variables
$env:ARDIA_PROFILE = $Profile
if ($Url) { $env:ARDIA_STARTUP_URL = $Url }
if ($Private) { $env:ARDIA_PRIVATE = "1" }
if ($DataDir) {
    $env:ARDIA_DATA_DIR = $DataDir
    Write-Host "Data directory: $DataDir" -ForegroundColor Gray
} else {
    $env:ARDIA_DATA_DIR = Join-Path $env:LOCALAPPDATA "Ardia"
}

if ($RemoteDebuggingPort -gt 0) {
    $env:ARDIA_REMOTE_DEBUGGING_PORT = $RemoteDebuggingPort.ToString()
    Write-Host "Remote debugging port: $RemoteDebuggingPort" -ForegroundColor Gray
}

if ($Debug) {
    $env:RUST_LOG = "debug,tauri=info,wry=info"
    $env:ARDIA_DEBUG = "1"
    Write-Host "Debug logging enabled" -ForegroundColor Gray
}

if ($Background) {
    $env:ARDIA_BACKGROUND = "1"
    Write-Host "Background mode enabled" -ForegroundColor Gray
}

# Run the application
Write-Host "`n--- Starting Ardia ---" -ForegroundColor Yellow
Write-Host "Binary: $binaryPath" -ForegroundColor Gray
Write-Host "Profile: $Profile" -ForegroundColor Gray
if ($Url) { Write-Host "Startup URL: $Url" -ForegroundColor Gray }

# Use Start-Process for background mode, otherwise run directly
if ($Background) {
    $process = Start-Process -FilePath $binaryPath -WindowStyle Hidden -PassThru
    Write-Host "Started in background (PID: $($process.Id))" -ForegroundColor Green
} else {
    & $binaryPath
    $exitCode = $LASTEXITCODE
    Write-Host "`n--- Ardia exited with code $exitCode ---" -ForegroundColor Yellow
    exit $exitCode
}