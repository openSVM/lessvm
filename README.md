# lessVM [![Release](https://github.com/openSVM/lessvm/actions/workflows/release.yml/badge.svg)](https://github.com/openSVM/lessvm/actions/workflows/release.yml)

A lightweight virtual machine implementation for Solana blockchain.

## Installation

### CLI Tool

The LessVM CLI tool provides a convenient way to create, build, and deploy LessVM applications.

#### One-Line Installation

**macOS and Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/openSVM/lessvm/main/cli/.goreleaser.install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/openSVM/lessvm/main/cli/.goreleaser.install.ps1 | iex
```

#### Using Homebrew (macOS and Linux)

```bash
brew tap openSVM/tap
brew install lessvm
```

#### For more installation options, see [CLI README](cli/README.md)

## Project Structure

- `lessvm-solana/` - Core VM implementation in Rust
  - `src/` - Source code
    - `vm/` - Virtual machine core implementation
    - `solana/` - Solana-specific integration code
  - `deployless/` - Deployment utilities
  - `scripts/` - Build and deployment scripts

- `cli/` - Command-line interface for managing LessVM applications
  - Provides tools for creating, building, and deploying LessVM applications
  - Cross-platform support (macOS, Linux, Windows)

- `website/` - Documentation website
  - `docs/` - Documentation pages
  - `index.html` - Main landing page
  - `styles.css` - Styling

- `lessvm-lore/` - Specification and design documents
  - Contains detailed specs for VM implementation
  - Architecture and design decisions

## Development

The project consists of two main parts:

1. **VM Implementation** - Written in Rust, located in `lessvm-solana/`
2. **Documentation Website** - Static HTML/CSS website in `website/`

## Building

### VM Implementation
```bash
cd lessvm-solana
cargo build
```

### Website
The documentation website is built using static HTML and CSS. No build step required.

## Testing

```bash
cd lessvm-solana
cargo test
```

For local Solana validator testing:
```bash
./start_local_validator.sh
```

## Documentation

Visit the documentation website for:
- [Architecture Overview](website/docs/architecture.html)
- [Examples](website/docs/examples.html)
- [Instructions Set](website/docs/instructions.html)
- [Solana Integration](website/docs/solana.html)

## License

[Add license information]
