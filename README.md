# lessVM

A lightweight virtual machine implementation for Solana blockchain.

## Project Structure

- `lessvm-solana/` - Core VM implementation in Rust
  - `src/` - Source code
    - `vm/` - Virtual machine core implementation
    - `solana/` - Solana-specific integration code
  - `deployless/` - Deployment utilities
  - `scripts/` - Build and deployment scripts

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