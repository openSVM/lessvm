# LessVM IDELess Installation Guide

This guide provides detailed instructions for installing and setting up the LessVM IDELess development environment on different operating systems.

## Table of Contents

- [System Requirements](#system-requirements)
- [Windows Installation](#windows-installation)
- [macOS Installation](#macos-installation)
- [Linux Installation](#linux-installation)
- [Verifying Installation](#verifying-installation)
- [Troubleshooting](#troubleshooting)
- [Updating IDELess](#updating-ideless)
- [Advanced Configuration](#advanced-configuration)

## System Requirements

### Minimum Requirements
- **OS**: Windows 10/11, macOS 10.15+, or Ubuntu 20.04+
- **Processor**: Dual-core 2GHz or higher
- **Memory**: 8GB RAM
- **Storage**: 1GB available space
- **Display**: 1280x720 resolution

### Recommended Requirements
- **OS**: Windows 11, macOS 12+, or Ubuntu 22.04+
- **Processor**: Quad-core 3GHz or higher
- **Memory**: 16GB RAM
- **Storage**: 2GB available space
- **Display**: 1920x1080 resolution or higher

### Prerequisites
- **Node.js**: v16.x or higher
- **npm**: v8.x or higher
- **Git**: Latest version
- **Solana CLI**: Latest version

## Windows Installation

### Step 1: Install Prerequisites

1. **Install Node.js and npm**:
   - Download the LTS version from [Node.js official website](https://nodejs.org/)
   - Run the installer and follow the installation wizard
   - Verify installation by opening Command Prompt and running:
     ```
     node --version
     npm --version
     ```

2. **Install Git**:
   - Download from [Git for Windows](https://gitforwindows.org/)
   - Run the installer and follow the installation wizard
   - Verify installation by running:
     ```
     git --version
     ```

3. **Install Solana CLI**:
   - Open PowerShell as Administrator and run:
     ```
     curl https://release.solana.com/v1.16.0/solana-install-init-x86_64-pc-windows-msvc.exe --output C:\solana-install-tmp\solana-install-init.exe --create-dirs
     C:\solana-install-tmp\solana-install-init.exe v1.16.0
     ```
   - Verify installation by running:
     ```
     solana --version
     ```

### Step 2: Install IDELess

1. **Download the Installer**:
   - Go to [IDELess Releases](https://github.com/lessvm/ideless/releases)
   - Download the latest `IDELess-Setup-x.x.x.exe` file

2. **Run the Installer**:
   - Double-click the downloaded file
   - Follow the installation wizard
   - Choose installation location (default is recommended)
   - Select additional components if prompted
   - Complete the installation

3. **Launch IDELess**:
   - Launch from the Start Menu or desktop shortcut
   - The application will perform first-time setup

### Step 3: Configure IDELess

1. **Set Up Solana Connection**:
   - In IDELess, go to Settings > Deployment
   - Ensure OpenSVM RPC servers are selected as default
   - Configure your Solana wallet (optional)

2. **Configure Project Defaults**:
   - Go to Settings > Projects
   - Set default project location
   - Configure default project settings

## macOS Installation

### Step 1: Install Prerequisites

1. **Install Homebrew** (if not already installed):
   - Open Terminal and run:
     ```
     /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
     ```

2. **Install Node.js and npm**:
   - In Terminal, run:
     ```
     brew install node
     ```
   - Verify installation:
     ```
     node --version
     npm --version
     ```

3. **Install Git** (if not already installed):
   - In Terminal, run:
     ```
     brew install git
     ```
   - Verify installation:
     ```
     git --version
     ```

4. **Install Solana CLI**:
   - In Terminal, run:
     ```
     sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
     ```
   - Add Solana to your PATH:
     ```
     echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc
     source ~/.zshrc
     ```
   - Verify installation:
     ```
     solana --version
     ```

### Step 2: Install IDELess

1. **Download the Installer**:
   - Go to [IDELess Releases](https://github.com/lessvm/ideless/releases)
   - Download the latest `IDELess-x.x.x.dmg` file

2. **Run the Installer**:
   - Open the downloaded DMG file
   - Drag IDELess to the Applications folder
   - Eject the disk image

3. **Launch IDELess**:
   - Open Applications folder and double-click IDELess
   - If prompted about an unidentified developer:
     - Right-click (or Control-click) the app and select "Open"
     - Click "Open" in the dialog that appears
   - The application will perform first-time setup

### Step 3: Configure IDELess

1. **Set Up Solana Connection**:
   - In IDELess, go to Settings > Deployment
   - Ensure OpenSVM RPC servers are selected as default
   - Configure your Solana wallet (optional)

2. **Configure Project Defaults**:
   - Go to Settings > Projects
   - Set default project location
   - Configure default project settings

## Linux Installation

### Step 1: Install Prerequisites

1. **Install Node.js and npm**:
   - For Ubuntu/Debian:
     ```
     curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -
     sudo apt-get install -y nodejs
     ```
   - For Fedora:
     ```
     sudo dnf install nodejs
     ```
   - Verify installation:
     ```
     node --version
     npm --version
     ```

2. **Install Git** (if not already installed):
   - For Ubuntu/Debian:
     ```
     sudo apt-get install git
     ```
   - For Fedora:
     ```
     sudo dnf install git
     ```
   - Verify installation:
     ```
     git --version
     ```

3. **Install Solana CLI**:
   - In Terminal, run:
     ```
     sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
     ```
   - Add Solana to your PATH:
     ```
     echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
     source ~/.bashrc
     ```
   - Verify installation:
     ```
     solana --version
     ```

### Step 2: Install IDELess

1. **Download the Installer**:
   - Go to [IDELess Releases](https://github.com/lessvm/ideless/releases)
   - Download the appropriate package for your distribution:
     - For Debian/Ubuntu: `ideless_x.x.x_amd64.deb`
     - For Fedora/RHEL: `ideless-x.x.x.x86_64.rpm`
     - For other distributions: `IDELess-x.x.x.AppImage`

2. **Install the Package**:
   - For Debian/Ubuntu:
     ```
     sudo dpkg -i ideless_x.x.x_amd64.deb
     sudo apt-get install -f
     ```
   - For Fedora/RHEL:
     ```
     sudo rpm -i ideless-x.x.x.x86_64.rpm
     ```
   - For AppImage:
     ```
     chmod +x IDELess-x.x.x.AppImage
     ```

3. **Launch IDELess**:
   - For package installations, search for IDELess in your application launcher
   - For AppImage, double-click the AppImage file or run it from terminal
   - The application will perform first-time setup

### Step 3: Configure IDELess

1. **Set Up Solana Connection**:
   - In IDELess, go to Settings > Deployment
   - Ensure OpenSVM RPC servers are selected as default
   - Configure your Solana wallet (optional)

2. **Configure Project Defaults**:
   - Go to Settings > Projects
   - Set default project location
   - Configure default project settings

## Verifying Installation

After installing IDELess, verify that all components are working correctly:

1. **Create a New Project**:
   - Click "New Project" on the welcome screen
   - Select "LessVM Project"
   - Enter a project name and location
   - Click "Create"

2. **Verify Editor Functionality**:
   - Create a new file
   - Verify syntax highlighting works
   - Test code completion

3. **Verify Debugger**:
   - Open a sample program
   - Set a breakpoint
   - Start debugging
   - Verify execution stops at the breakpoint

4. **Verify Simulator**:
   - Open a sample program
   - Run the simulator
   - Verify program execution and state visualization

5. **Verify Deployment**:
   - Go to Deployment settings
   - Verify connection to Solana network (using OpenSVM RPC)
   - Test connection

## Troubleshooting

### Common Installation Issues

#### Node.js Version Issues
- **Symptom**: Error message about incompatible Node.js version
- **Solution**: Install the required Node.js version (v16.x or higher)
  ```
  # For nvm users
  nvm install 16
  nvm use 16
  ```

#### Permission Issues on Linux/macOS
- **Symptom**: "Permission denied" errors during installation
- **Solution**: Use sudo for installation commands or fix permissions
  ```
  sudo chmod +x ./installer.sh
  ```

#### Missing Dependencies on Linux
- **Symptom**: Application fails to start with missing library errors
- **Solution**: Install required dependencies
  ```
  # For Ubuntu/Debian
  sudo apt-get install libgtk-3-0 libnotify4 libnss3 libxss1 libxtst6 xdg-utils libatspi2.0-0 libuuid1 libsecret-1-0
  ```

#### Solana CLI Configuration Issues
- **Symptom**: "solana: command not found" or connection errors
- **Solution**: Verify Solana CLI installation and configuration
  ```
  solana --version
  solana config get
  ```

### Getting Help

If you encounter issues not covered in this guide:

1. **Check Documentation**:
   - Visit [docs.lessvm.org/ideless](https://docs.lessvm.org/ideless) for comprehensive documentation

2. **Community Support**:
   - Join the [Discord community](https://discord.gg/lessvm) for help from other users and developers

3. **GitHub Issues**:
   - Check [existing issues](https://github.com/lessvm/ideless/issues) for known problems
   - Create a new issue with detailed information about your problem

4. **Email Support**:
   - Contact support@lessvm.org for direct assistance

## Updating IDELess

IDELess includes an auto-update feature that will notify you when updates are available. To manually check for updates:

1. **Using the Application**:
   - Go to Help > Check for Updates
   - Follow the prompts to download and install the update

2. **Manual Update**:
   - Download the latest version from [IDELess Releases](https://github.com/lessvm/ideless/releases)
   - Install using the same method as your initial installation
   - Your settings and projects will be preserved

## Advanced Configuration

### Custom Installation Locations

#### Windows
- During installation, you can specify a custom installation directory
- Default location: `C:\Program Files\IDELess`

#### macOS
- Applications are typically installed in the `/Applications` folder
- User data is stored in `~/Library/Application Support/IDELess`

#### Linux
- Package installations typically use standard locations
- AppImage can be run from any location with execute permissions

### Command Line Installation

For automated or scripted installations:

#### Windows (using Chocolatey)
```
choco install ideless
```

#### macOS (using Homebrew)
```
brew install --cask ideless
```

#### Linux (using apt for Ubuntu/Debian)
```
curl -sL https://lessvm.org/ideless/linux/debian/gpg | sudo apt-key add -
echo "deb [arch=amd64] https://lessvm.org/ideless/linux/debian stable main" | sudo tee /etc/apt/sources.list.d/ideless.list
sudo apt update
sudo apt install ideless
```

### Environment Variables

IDELess respects the following environment variables:

- `IDELESS_HOME`: Base directory for IDELess configuration
- `IDELESS_USER_DATA_DIR`: Directory for user data
- `IDELESS_EXTENSIONS_DIR`: Directory for extensions
- `IDELESS_LOG_LEVEL`: Log level (debug, info, warn, error)

To set environment variables:

#### Windows
```
setx IDELESS_LOG_LEVEL "debug"
```

#### macOS/Linux
```
echo 'export IDELESS_LOG_LEVEL="debug"' >> ~/.bashrc
source ~/.bashrc
```

### Configuration File Locations

IDELess stores configuration in the following locations:

#### Windows
- User settings: `%APPDATA%\IDELess\User\settings.json`
- Logs: `%APPDATA%\IDELess\logs`

#### macOS
- User settings: `~/Library/Application Support/IDELess/User/settings.json`
- Logs: `~/Library/Logs/IDELess`

#### Linux
- User settings: `~/.config/IDELess/User/settings.json`
- Logs: `~/.config/IDELess/logs`

### Portable Installation

IDELess can be run in portable mode, which stores all data within the application directory:

1. **Create a `data` directory** in the IDELess installation folder
2. **Launch with the `--portable` flag**:
   ```
   IDELess.exe --portable
   ```
   or create a shortcut with this flag

This is useful for running IDELess from a USB drive or network location.

## Next Steps

After installation, we recommend:

1. **Completing the Quick Start Guide**: See [Quick Start Guide](./quick_start.md)
2. **Exploring Sample Projects**: Available in Help > Sample Projects
3. **Customizing Your Environment**: Adjust settings to match your preferences
4. **Learning Keyboard Shortcuts**: See [Keyboard Shortcuts](./keyboard_shortcuts.md)
5. **Joining the Community**: Connect with other LessVM developers on Discord