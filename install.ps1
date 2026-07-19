# Installs the latest terminion release for Windows.
# Usage: irm https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.ps1 | iex
$ErrorActionPreference = "Stop"

$Repo = "hameedibrh/Terminion"
$InstallDir = "$env:LOCALAPPDATA\terminion"
$Target = "x86_64-pc-windows-msvc"

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

Write-Host "Installing terminion $Tag"
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

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $addToPath = $true
    try {
        $response = Read-Host "Add $InstallDir to your PATH so 'terminion' works from any shell? [Y/n]"
        $addToPath = ($response -eq "") -or ($response -match '^[Yy]')
    } catch {
        # Not running interactively (e.g. piped from a non-terminal caller):
        # default to adding it, same as this script always did before.
    }

    if ($addToPath) {
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        Write-Host "Added $InstallDir to your user PATH. Restart your terminal to use 'terminion' directly."
    } else {
        Write-Host "Skipped adding to PATH. Run terminion directly with: $InstallDir\terminion.exe"
    }
}
