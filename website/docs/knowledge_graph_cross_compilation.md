# Cross-Compilation Knowledge Graph

## Cross-Compilation for aarch64-unknown-linux-gnu

### Problem
When cross-compiling the LessVM CLI for aarch64-unknown-linux-gnu, OpenSSL cannot be found via pkg-config because pkg-config is not configured for cross-compilation.

### Solution
1. Use the vendored OpenSSL feature in reqwest to avoid the need for pkg-config during cross-compilation.
2. Set up the cross-compilation environment with appropriate environment variables.

### Implementation Details

#### 1. Modified Dependencies
Added the `native-tls-vendored` feature to reqwest in cli/Cargo.toml:
```toml
reqwest = { version = "0.11", features = ["json", "native-tls-vendored"] }
```

This feature makes reqwest use a vendored version of OpenSSL, eliminating the need for pkg-config to find a system-installed version.

#### 2. Environment Setup Script
Created a script (`cross-compile-setup.sh`) that sets up the necessary environment variables for cross-compilation:

- For macOS:
  - Installs required packages: pkg-config, openssl@3
  - Installs cross-compilation toolchain for aarch64-unknown-linux-gnu
  - Sets up environment variables: CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER, CC_aarch64_unknown_linux_gnu, etc.
  - Configures pkg-config for cross-compilation: PKG_CONFIG_ALLOW_CROSS, PKG_CONFIG_PATH

- For Linux:
  - Installs required packages: pkg-config, libssl-dev, gcc-aarch64-linux-gnu, g++-aarch64-linux-gnu
  - Sets up environment variables for cross-compilation
  - Configures pkg-config for cross-compilation

### Usage
1. Source the environment setup script before building:
   ```bash
   source cross-compile-setup.sh
   ```

2. Build for the target platform:
   ```bash
   cargo build --target aarch64-unknown-linux-gnu
   ```

### Technical Explanation
The issue occurs because pkg-config is not configured to look for libraries in the sysroot for the target platform. When cross-compiling, pkg-config needs to be told where to find libraries for the target architecture.

The vendored OpenSSL feature in reqwest is the simplest solution as it bundles OpenSSL with the build, avoiding the need for pkg-config to find it. For other dependencies that might require OpenSSL, the environment variables set in the script ensure that pkg-config can find the appropriate libraries.

### Decision Making Process
1. Identified that the issue was related to OpenSSL not being found during cross-compilation
2. Considered two approaches:
   - Configure pkg-config for cross-compilation
   - Use vendored dependencies where possible
3. Implemented both approaches for maximum compatibility:
   - Modified reqwest to use vendored OpenSSL
   - Created a script to set up the cross-compilation environment

### References
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [pkg-config Cross-Compilation Documentation](https://pkg-config.freedesktop.org/pkg-config-guide.html)
- [reqwest Documentation](https://docs.rs/reqwest/latest/reqwest/)
