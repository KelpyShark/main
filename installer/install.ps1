# KelpyShark Installer — Windows (PowerShell)
#
# Usage:
#   irm https://github.com/KelpyShark/main/raw/main/installer/install.ps1 | iex
#
# What this does:
#   1. Checks for Rust/Cargo (installs if missing)
#   2. Clones the KelpyShark repository
#   3. Builds the toolchain from source
#   4. Installs the `kelpy` binary to ~/.kelpyshark/bin
#   5. Adds ~/.kelpyshark/bin to PATH

$ErrorActionPreference = "Stop"

$KELPYSHARK_HOME = Join-Path $env:USERPROFILE ".kelpyshark"
$KELPYSHARK_BIN = Join-Path $KELPYSHARK_HOME "bin"
$KELPYSHARK_REPO = "https://github.com/kelpyshark/main.git"

Write-Host ""
Write-Host "  KelpyShark Installer" -ForegroundColor Cyan
Write-Host "  ====================" -ForegroundColor Cyan
Write-Host ""

#  Check prerequisites 

function Test-Command($cmd) {
    try { Get-Command $cmd -ErrorAction Stop | Out-Null; return $true }
    catch { return $false }
}

# Check for git
if (-not (Test-Command "git")) {
    Write-Host "[" -NoNewLine
    Write-Host "ERROR" -ForegroundColor Red -NoNewLine
    Write-Host "]" -NoNewLine
    Write-Host " Git is required but not installed." -ForegroundColor Red
    Write-Host "  Install from: https://git-scm.com/download/win"
    exit 1
}

# Check for Rust/Cargo
if (-not (Test-Command "cargo")) {
    Write-Host "[" -NoNewLine
    Write-Host "INFO" -ForegroundColor Blue -NoNewLine
    Write-Host "]" -NoNewLine
    Write-Host " Rust/Cargo not found. Installing via rustup..." -ForegroundColor Yellow
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupExe = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe
    Start-Process -FilePath $rustupExe -ArgumentList "-y" -Wait -NoNewWindow
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    Write-Host "[" -NoNewLine
    Write-Host "INFO" -ForegroundColor Blue -NoNewLine
    Write-Host "]" -NoNewLine
    Write-Host " Rust installed." -ForegroundColor Green
}

Write-Host "[" -NoNewLine
Write-Host "INFO" -ForegroundColor Blue -NoNewLine
Write-Host "]" -NoNewLine
Write-Host " Rust: $(rustc --version)"
Write-Host "[" -NoNewLine
Write-Host "INFO" -ForegroundColor Blue -NoNewLine
Write-Host "]" -NoNewLine
Write-Host " Cargo: $(cargo --version)"
Write-Host ""

#  Clone / update repository 

$TEMP_DIR = Join-Path $env:TEMP "kelpyshark_install"
if (Test-Path $TEMP_DIR) { Remove-Item -Recurse -Force $TEMP_DIR }

Write-Host "[" -NoNewLine
Write-Host "INFO" -ForegroundColor Blue -NoNewLine
Write-Host "]" -NoNewLine
Write-Host " Cloning KelpyShark..." -ForegroundColor Yellow
try {
    git clone --depth 1 $KELPYSHARK_REPO $TEMP_DIR 2>$null
}
catch {
    Write-Host "[" -NoNewLine
    Write-Host "WARN" -ForegroundColor Yellow -NoNewLine
    Write-Host "]" -NoNewLine
    Write-Host " Could not clone from remote. Building from local source..." -ForegroundColor Yellow
    $TEMP_DIR = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
}

$BUILD_DIR = $TEMP_DIR
if (-not (Test-Path (Join-Path $BUILD_DIR "Cargo.toml"))) {
    $BUILD_DIR = Join-Path $TEMP_DIR "kelpyshark"
}

#  Build 

Write-Host "[" -NoNewLine
Write-Host "INFO" -ForegroundColor Blue -NoNewLine
Write-Host "]" -NoNewLine
Write-Host " Building KelpyShark..."
Push-Location $BUILD_DIR
cargo build --release --bin kelpyshark --bin ks --bin kshark
Pop-Location

#  Install 

Write-Host "[" -NoNewLine
Write-Host "INFO" -ForegroundColor Blue -NoNewLine
Write-Host "]" -NoNewLine
Write-Host " Installing to $KELPYSHARK_BIN..."
New-Item -ItemType Directory -Force -Path $KELPYSHARK_BIN | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $KELPYSHARK_HOME "registry") | Out-Null

Copy-Item (Join-Path $BUILD_DIR "target\release\kelpyshark.exe") (Join-Path $KELPYSHARK_BIN "kelpyshark.exe") -Force
Copy-Item (Join-Path $BUILD_DIR "target\release\ks.exe")          (Join-Path $KELPYSHARK_BIN "ks.exe")          -Force
Copy-Item (Join-Path $BUILD_DIR "target\release\kshark.exe")      (Join-Path $KELPYSHARK_BIN "kshark.exe")      -Force

#  Update PATH 

$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*kelpyshark*") {
    [Environment]::SetEnvironmentVariable("PATH", "$KELPYSHARK_BIN;$currentPath", "User")
    $env:PATH = "$KELPYSHARK_BIN;$env:PATH"
    Write-Host "[" -NoNewLine
    Write-Host "INFO" -ForegroundColor Blue -NoNewLine
    Write-Host "]" -NoNewLine
    Write-Host " Added $KELPYSHARK_BIN to user PATH"
}

Write-Host ""
Write-Host "  KelpyShark installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "  You can run programs with any of:"
Write-Host "    kelpyshark run hello.ks"
Write-Host "    kshark run hello.ks"
Write-Host "    ks run hello.ks"
Write-Host ""
Write-Host "  Restart your terminal to use these commands from anywhere." -ForegroundColor Yellow
