# 4 Browser Build Script (PowerShell)
# This script automates the build process for 4 Browser on Windows

param(
    [string]$BuildType = "release",
    [string]$Features = ""
)

# Set error action to stop on errors
$ErrorActionPreference = "Stop"

Write-Host "🔨 4 Browser Build Script" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan
Write-Host ""

# Check Rust installation
try {
    $cargo = & cargo --version 2>&1
    Write-Host "✓ Rust found" -ForegroundColor Green
    & rustc --version
    & cargo --version
} catch {
    Write-Host "❌ Rust not installed!" -ForegroundColor Red
    Write-Host "Install from: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# Determine build configuration
if ($BuildType -eq "debug") {
    $buildFlag = ""
    $buildMode = "debug"
    Write-Host "Building debug version..." -ForegroundColor Yellow
} else {
    $buildFlag = "--release"
    $buildMode = "release"
    Write-Host "Building release version..." -ForegroundColor Yellow
}

# Build
Write-Host "📦 Compiling..." -ForegroundColor Cyan

try {
    if ([string]::IsNullOrEmpty($Features)) {
        & cargo build $buildFlag.Split()
    } else {
        Write-Host "🎨 With features: $Features" -ForegroundColor Cyan
        & cargo build $buildFlag.Split() --features $Features
    }
} catch {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Determine binary path
$binaryName = "4browser.exe"
$binaryPath = Join-Path "target" $buildMode $binaryName

# Check if build succeeded
if (Test-Path $binaryPath) {
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📍 Binary location:" -ForegroundColor Cyan
    Write-Host "   $(Resolve-Path $binaryPath)" -ForegroundColor White
    Write-Host ""
    Write-Host "🚀 To run the browser:" -ForegroundColor Green
    Write-Host "   .\$binaryPath" -ForegroundColor White
    Write-Host ""
} else {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
