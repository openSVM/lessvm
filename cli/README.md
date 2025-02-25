# LessVM CLI

A command-line interface for managing the full lifecycle of LessVM applications on Solana.

## Installation

### Using Shell Script (macOS and Linux)

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

### Manual Download (macOS, Linux, Windows)

Download the appropriate binary for your platform from the [releases page](https://github.com/openSVM/lessvm/releases).

#### macOS

```bash
# For Intel Macs
curl -L https://github.com/openSVM/lessvm/releases/latest/download/lessvm-x86_64-apple-darwin.tar.xz | tar xJ
sudo mv lessvm /usr/local/bin/

# For Apple Silicon (M1/M2) Macs
curl -L https://github.com/openSVM/lessvm/releases/latest/download/lessvm-aarch64-apple-darwin.tar.xz | tar xJ
sudo mv lessvm /usr/local/bin/
```

#### Linux

```bash
# For x86_64 architecture
curl -L https://github.com/openSVM/lessvm/releases/latest/download/lessvm-x86_64-unknown-linux-gnu.tar.xz | tar xJ
sudo mv lessvm /usr/local/bin/

# For ARM64 architecture
curl -L https://github.com/openSVM/lessvm/releases/latest/download/lessvm-aarch64-unknown-linux-gnu.tar.xz | tar xJ
sudo mv lessvm /usr/local/bin/
```

### From Source

```bash
cargo install lessvm
```

## Usage

```bash
lessvm [OPTIONS] <COMMAND>
```

### Global Options

- `-v, --verbose`: Enable verbose logging
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Commands

#### Create a New Project

```bash
lessvm new <NAME> [--template <TEMPLATE>]
```

Creates a new LessVM project with the specified name. Optionally specify a template (default: basic).

#### Build Project

```bash
lessvm build [--path <PATH>]
```

Builds the LessVM application, compiling and optimizing the code for deployment.

#### Deploy to Solana

```bash
lessvm deploy [--path <PATH>] [--cluster <CLUSTER>]
```

Deploys the built application to the specified Solana cluster (default: devnet).

#### Check Status

```bash
lessvm status [--path <PATH>]
```

Checks the deployment status of the application.

#### Update Deployment

```bash
lessvm update [--path <PATH>] [--hot-reload]
```

Updates the deployed application. Use `--hot-reload` for hot reloading when possible.

#### View Logs

```bash
lessvm logs [--path <PATH>] [--follow]
```

Views application logs. Use `--follow` to stream logs in real-time.

## Project Structure

A typical LessVM project has the following structure:

```
my-lessvm-app/
├── src/
│   └── main.less      # Main application code
├── tests/
│   └── main_test.less # Test files
├── build/             # Build artifacts
└── lessvm.toml       # Project configuration
```

## Configuration

The `lessvm.toml` file contains project configuration:

```toml
[project]
name = "my-app"
version = "0.1.0"
template = "basic"

[solana]
cluster = "devnet"
program_id = "optional-deployed-program-id"
keypair_path = "~/.config/solana/id.json"

[build]
target = "solana"
optimization_level = "release"
```

## Development

### Building from Source

To build the CLI tool from source:

```bash
git clone https://github.com/openSVM/lessvm
cd lessvm/cli
cargo build --release
```

### Creating Releases

This project uses [cargo-dist](https://github.com/axodotdev/cargo-dist) to automate the release process. To create a new release:

1. Update version in `Cargo.toml`

2. Tag a new version:
   ```bash
   git tag -a v0.1.1 -m "Release v0.1.1"
   git push origin v0.1.1
   ```

3. GitHub Actions will automatically:
   - Build binaries for all supported platforms
   - Create installers (shell script, PowerShell script, Homebrew)
   - Generate release artifacts
   - Create a GitHub release
   - Publish to package managers

To test the release process locally:

```bash
cd cli
cargo dist build --artifacts=all --target=<your-platform>
```

## License

MIT License
