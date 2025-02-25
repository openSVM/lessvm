# Release Process

This document describes the release process for the lessvm-cli tool.

## Prerequisites

- Rust toolchain installed
- GPG key for signing releases (if using signed releases)
- GitHub access with permissions to create releases

## Release Steps

1. **Update Version**
   - Update the version in `Cargo.toml`
   - Update CHANGELOG.md with the new version's changes
   - Commit these changes

2. **Create Git Tag**
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

3. **Automated Release Process**
   The GitHub Actions workflow will automatically:
   - Build the CLI for all supported platforms
   - Create installers (shell script, PowerShell script, Homebrew)
   - Generate release artifacts
   - Create a GitHub release
   - Publish to package managers

## Supported Platforms

The CLI is automatically built and packaged for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

## Installation Methods

Users can install the CLI through various methods:

### Using Shell Script (Unix-like systems)
```bash
curl -L https://github.com/openSVM/lessvm/releases/latest/download/lessvm-installer.sh | sh
```

### Using PowerShell (Windows)
```powershell
irm https://github.com/openSVM/lessvm/releases/latest/download/lessvm-installer.ps1 | iex
```

### Using Homebrew (macOS and Linux)
```bash
brew tap openSVM/lessvm
brew install lessvm
```

## Troubleshooting

If you encounter issues during the release process:

1. Check the GitHub Actions logs for any build failures
2. Verify that all required secrets are properly set in the repository
3. Ensure the version numbers are consistent across all files
4. Check that the GPG signing key is properly configured (if using signed releases)

## Manual Release (if needed)

If you need to create a release manually:

1. Install cargo-dist:
   ```bash
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/cargo-dist/releases/latest/download/cargo-dist-installer.sh | sh
   ```

2. Build and package locally:
   ```bash
   cd cli
   cargo dist build --artifacts=all --target=x86_64-unknown-linux-gnu
   ```

3. The packaged artifacts will be available in `target/dist/`
