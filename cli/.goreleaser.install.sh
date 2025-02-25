#!/bin/bash

# lessvm-cli installer script
# This script detects the OS and architecture and downloads the appropriate binary

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print banner
echo -e "${BLUE}"
echo "  _                 __     ____  __    ____ _     ___ "
echo " | |    ___  ___ __\ \   / /  \/  |  / ___| |   |_ _|"
echo " | |   / _ \/ __/ __\ \ / /| |\/| | | |   | |    | | "
echo " | |__|  __/\__ \__ \\ V / | |  | | | |___| |___ | | "
echo " |_____\___||___/___/ \_/  |_|  |_|  \____|_____|___|"
echo -e "${NC}"
echo "LessVM CLI Installer"
echo "===================="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     OS=linux;;
    Darwin*)    OS=macos;;
    CYGWIN*|MINGW*|MSYS*) OS=windows;;
    *)          echo -e "${RED}Unsupported OS: ${OS}${NC}" && exit 1;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64|amd64) ARCH=x86_64;;
    arm64|aarch64) ARCH=aarch64;;
    *)          echo -e "${RED}Unsupported architecture: ${ARCH}${NC}" && exit 1;;
esac

echo -e "${GREEN}Detected OS: ${OS}${NC}"
echo -e "${GREEN}Detected architecture: ${ARCH}${NC}"
echo ""

# Get latest version
echo -e "${BLUE}Fetching latest version...${NC}"
VERSION=$(curl -s https://api.github.com/repos/openSVM/lessvm/releases/latest | grep -o '"tag_name": ".*"' | sed 's/"tag_name": "//;s/"//;s/^v//')

if [ -z "$VERSION" ]; then
    echo -e "${RED}Failed to fetch latest version. Please check your internet connection.${NC}"
    exit 1
fi

echo -e "${GREEN}Latest version: ${VERSION}${NC}"
echo ""

# Set download URL
DOWNLOAD_URL="https://github.com/openSVM/lessvm/releases/download/v${VERSION}/lessvm_${OS}_${ARCH}.tar.gz"
if [ "$OS" = "windows" ]; then
    DOWNLOAD_URL="https://github.com/openSVM/lessvm/releases/download/v${VERSION}/lessvm_${OS}_${ARCH}.zip"
fi

# Set install directory
INSTALL_DIR="/usr/local/bin"
if [ "$OS" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Download and install
echo -e "${BLUE}Downloading lessvm...${NC}"
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

if [ "$OS" = "windows" ]; then
    curl -L -o lessvm.zip "$DOWNLOAD_URL"
    unzip lessvm.zip
    mv lessvm.exe "$INSTALL_DIR/"
else
    curl -L "$DOWNLOAD_URL" | tar xz
    sudo mv lessvm "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/lessvm"
fi

# Clean up
cd - > /dev/null
rm -rf "$TMP_DIR"

echo -e "${GREEN}Installation complete!${NC}"
echo ""

# Verify installation
if command -v lessvm >/dev/null 2>&1; then
    echo -e "${GREEN}lessvm is now installed and available in your PATH.${NC}"
    echo -e "Version: $(lessvm --version)"
else
    echo -e "${YELLOW}lessvm is installed but not in your PATH.${NC}"
    echo -e "You can run it using: ${BLUE}$INSTALL_DIR/lessvm${NC}"
    
    if [ "$OS" = "windows" ]; then
        echo -e "${YELLOW}Add $INSTALL_DIR to your PATH to use lessvm from any location.${NC}"
    fi
fi

echo ""
echo -e "${BLUE}To get started, run:${NC}"
echo -e "  lessvm --help"
echo ""
echo -e "${BLUE}For more information, visit:${NC}"
echo -e "  https://github.com/openSVM/lessvm"
