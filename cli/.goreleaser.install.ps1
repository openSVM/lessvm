# lessvm-cli installer script for Windows
# This script detects the architecture and downloads the appropriate binary

# Print banner
Write-Host "  _                 __     ____  __    ____ _     ___ " -ForegroundColor Blue
Write-Host " | |    ___  ___ __\ \   / /  \/  |  / ___| |   |_ _|" -ForegroundColor Blue
Write-Host " | |   / _ \/ __/ __\ \ / /| |\/| | | |   | |    | | " -ForegroundColor Blue
Write-Host " | |__|  __/\__ \__ \\ V / | |  | | | |___| |___ | | " -ForegroundColor Blue
Write-Host " |_____\___||___/___/ \_/  |_|  |_|  \____|_____|___|" -ForegroundColor Blue
Write-Host ""
Write-Host "LessVM CLI Installer for Windows"
Write-Host "==============================="
Write-Host ""

# Detect architecture
$arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
if ($arch -eq "AMD64") {
    $arch = "x86_64"
} elseif ($arch -eq "ARM64") {
    $arch = "aarch64"
} else {
    Write-Host "Unsupported architecture: $arch" -ForegroundColor Red
    exit 1
}

Write-Host "Detected architecture: $arch" -ForegroundColor Green
Write-Host ""

# Get latest version
Write-Host "Fetching latest version..." -ForegroundColor Blue
try {
    $latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/openSVM/lessvm/releases/latest"
    $version = $latestRelease.tag_name -replace "v", ""
} catch {
    Write-Host "Failed to fetch latest version. Please check your internet connection." -ForegroundColor Red
    exit 1
}

Write-Host "Latest version: $version" -ForegroundColor Green
Write-Host ""

# Set download URL
$downloadUrl = "https://github.com/openSVM/lessvm/releases/download/v$version/lessvm-cli_windows_$arch.zip"

# Set install directory
$installDir = "$env:USERPROFILE\bin"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
    Write-Host "Created directory: $installDir" -ForegroundColor Green
}

# Download and install
Write-Host "Downloading lessvm-cli..." -ForegroundColor Blue
$tempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
New-Item -ItemType Directory -Path $tempDir | Out-Null

$zipPath = "$tempDir\lessvm-cli.zip"
Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath

Write-Host "Extracting files..." -ForegroundColor Blue
Expand-Archive -Path $zipPath -DestinationPath $tempDir

# Copy to install directory
Copy-Item -Path "$tempDir\lessvm-cli.exe" -Destination "$installDir\lessvm-cli.exe" -Force

# Clean up
Remove-Item -Path $tempDir -Recurse -Force

Write-Host "Installation complete!" -ForegroundColor Green
Write-Host ""

# Check if install directory is in PATH
$path = [System.Environment]::GetEnvironmentVariable("PATH", "User")
if ($path -notlike "*$installDir*") {
    Write-Host "Adding $installDir to your PATH..." -ForegroundColor Yellow
    $newPath = "$path;$installDir"
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    Write-Host "PATH updated. You may need to restart your terminal for changes to take effect." -ForegroundColor Yellow
    Write-Host ""
}

Write-Host "lessvm-cli is now installed at: $installDir\lessvm-cli.exe" -ForegroundColor Green
Write-Host ""
Write-Host "To get started, run:" -ForegroundColor Blue
Write-Host "  lessvm-cli --help"
Write-Host ""
Write-Host "For more information, visit:" -ForegroundColor Blue
Write-Host "  https://github.com/openSVM/lessvm"
