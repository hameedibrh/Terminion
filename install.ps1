# Installs terminion for Windows.
# Per-user (default, no admin required):
#   irm https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.ps1 | iex
# System-wide, for all users (requires an elevated/Administrator PowerShell):
#   &([scriptblock]::Create((irm https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.ps1))) -Global
param(
    [switch]$Global
)
$ErrorActionPreference = "Stop"

$Repo = "hameedibrh/Terminion"
$Target = "x86_64-pc-windows-msvc"

if ($Global) {
    $IsAdmin = ([Security.Principal.WindowsPrincipal] `
        [Security.Principal.WindowsIdentity]::GetCurrent()`
    ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    if (-not $IsAdmin) {
        throw "Global install requires an elevated PowerShell. Right-click PowerShell, choose 'Run as Administrator', then run this installer again with -Global."
    }
    $InstallDir = "$env:ProgramFiles\Terminion"
    $PathScope = "Machine"
} else {
    $InstallDir = "$env:LOCALAPPDATA\terminion"
    $PathScope = "User"
}

# GitHub's "/releases/latest" shortcut only ever resolves to the newest
# *stable* release, so it 404s while every published release is a
# pre-release (e.g. an alpha). Resolve the newest release of any kind
# (including pre-releases) via the API instead.
$Releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases"
if (-not $Releases -or $Releases.Count -eq 0) {
    throw "No releases found for $Repo yet."
}
$Tag = $Releases[0].tag_name

$Url = "https://github.com/$Repo/releases/download/$Tag/terminion-$Target.zip"
# Expand-Archive requires a .zip extension on the path itself, but
# New-TemporaryFile creates a .tmp file, so build the temp path by hand.
$TmpZip = Join-Path ([System.IO.Path]::GetTempPath()) "terminion-$([guid]::NewGuid()).zip"

Write-Host "Installing terminion $Tag$(if ($Global) { ' (system-wide)' })"
Write-Host "Downloading $Url"
Invoke-WebRequest -Uri $Url -OutFile $TmpZip

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Expand-Archive -Force swallows a locked-file error internally (e.g. an
# existing terminion.exe still running, such as an open `terminion shell`)
# and reports success anyway, silently leaving the old binary in place.
# Remove the old exe ourselves first so a lock surfaces as a clear error.
$ExePath = Join-Path $InstallDir "terminion.exe"
if (Test-Path $ExePath) {
    try {
        Remove-Item $ExePath -Force -ErrorAction Stop
    } catch {
        Remove-Item $TmpZip -ErrorAction SilentlyContinue
        throw "Could not replace $ExePath - it looks like terminion is currently running (e.g. an open 'terminion shell'). Close it and run this installer again."
    }
}

Expand-Archive -Path $TmpZip -DestinationPath $InstallDir -Force
Remove-Item $TmpZip

Write-Host "Installed terminion to $InstallDir\terminion.exe"

$ExistingPath = [Environment]::GetEnvironmentVariable("Path", $PathScope)
if ($ExistingPath -notlike "*$InstallDir*") {
    $addToPath = $true
    try {
        $response = Read-Host "Add $InstallDir to your $PathScope PATH so 'terminion' works from any shell? [Y/n]"
        $addToPath = ($response -eq "") -or ($response -match '^[Yy]')
    } catch {
        # Not running interactively (e.g. piped from a non-terminal caller):
        # default to adding it, same as this script always did before.
    }

    if ($addToPath) {
        [Environment]::SetEnvironmentVariable("Path", "$ExistingPath;$InstallDir", $PathScope)
        Write-Host "Added $InstallDir to the $PathScope PATH. Restart your terminal to use 'terminion' directly."
    } else {
        Write-Host "Skipped adding to PATH. Run terminion directly with: $InstallDir\terminion.exe"
    }
}
