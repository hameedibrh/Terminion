# Installs the latest terminion release for Windows.
# Usage: irm https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.ps1 | iex
$ErrorActionPreference = "Stop"

$Repo = "hameedibrh/Terminion"
$InstallDir = "$env:LOCALAPPDATA\terminion"
$Target = "x86_64-pc-windows-msvc"

$Url = "https://github.com/$Repo/releases/latest/download/terminion-$Target.zip"
$TmpZip = New-TemporaryFile

Write-Host "Downloading $Url"
Invoke-WebRequest -Uri $Url -OutFile $TmpZip

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $TmpZip -DestinationPath $InstallDir -Force
Remove-Item $TmpZip

Write-Host "Installed terminion to $InstallDir\terminion.exe"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to your user PATH. Restart your terminal to use 'terminion' directly."
}
