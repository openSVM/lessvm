# Release Process for lessvm-cli

This document describes the release process for the lessvm-cli tool using GoReleaser.

## Prerequisites

- [GoReleaser](https://goreleaser.com/) installed on your machine
- GitHub access token with `repo` scope
- Rust toolchain installed

## Installation

### Install GoReleaser

On macOS:
```bash
brew install goreleaser
```

On Linux:
```bash
curl -sfL https://goreleaser.com/static/run | bash
```

### Set up GitHub Token

GoReleaser needs a GitHub token to create releases and upload artifacts. You can create a token at https://github.com/settings/tokens.

Once you have a token, set it as an environment variable:

```bash
export GITHUB_TOKEN=your-github-token
```

## Release Process

### 1. Update Version

Update the version in `cli/Cargo.toml`:

```toml
[package]
name = "lessvm-cli"
version = "0.1.1"  # Update this line
```

### 2. Commit Changes

Commit all changes to the repository:

```bash
git add .
git commit -m "Prepare release v0.1.1"
git push origin main
```

### 3. Create a Tag

Create a new tag for the release:

```bash
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

### 4. Automatic Release

Once the tag is pushed, GitHub Actions will automatically:
1. Build the binaries for all platforms
2. Create a GitHub release
3. Upload the binaries to the release
4. Update the Homebrew formula

You can monitor the progress in the "Actions" tab of your GitHub repository.

## Testing Locally

To test the release process locally without publishing:

```bash
cd cli
goreleaser release --snapshot --clean --skip=publish
```

This will create a snapshot release in the `dist` directory, which you can inspect to verify that the build process works as expected.

## Release Artifacts

GoReleaser will create the following artifacts:

- Binary files for each supported platform (macOS, Linux, Windows)
- Archives (tar.gz for macOS/Linux, zip for Windows)
- Checksums file
- Homebrew formula

## Homebrew Tap

The Homebrew formula will be published to the `openSVM/homebrew-tap` repository. Users can install the CLI tool using:

```bash
brew tap openSVM/tap
brew install lessvm-cli
```

## Troubleshooting

### Common Issues

1. **GitHub Token Issues**

   If you see authentication errors, make sure your GitHub token is set correctly and has the required permissions.

   ```bash
   export GITHUB_TOKEN=your-github-token
   ```

2. **Build Failures**

   If the build fails, check the error messages in the GitHub Actions logs. Common issues include:
   
   - Missing dependencies
   - Compilation errors
   - Test failures

3. **Homebrew Formula Issues**

   If the Homebrew formula fails to publish, check:
   
   - The `tap` section in `.goreleaser.yml`
   - GitHub token permissions
   - Repository access

### Manual Release

If the automatic release fails, you can perform a manual release:

```bash
cd cli
goreleaser release --clean
```

This will create a release and upload the artifacts to GitHub.

## Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update CHANGELOG.md (if applicable)
- [ ] Commit all changes
- [ ] Create and push a new tag
- [ ] Verify GitHub Actions workflow completes successfully
- [ ] Test the released binaries
- [ ] Verify Homebrew formula works correctly
