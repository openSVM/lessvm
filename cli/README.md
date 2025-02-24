# LessVM CLI

A command-line interface for managing the full lifecycle of LessVM applications on Solana.

## Installation

```bash
cargo install lessvm-cli
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

To build the CLI tool from source:

```bash
git clone https://github.com/your-org/lessvm-cli
cd lessvm-cli
cargo build --release
```

## License

MIT License