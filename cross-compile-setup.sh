#!/bin/bash
set -e

# This script sets up the environment for cross-compiling to aarch64-unknown-linux-gnu
# with proper OpenSSL configuration

# Install required packages if not already installed
if [ "$(uname)" = "Darwin" ]; then
    # macOS
    brew install pkg-config openssl@3
    export OPENSSL_DIR=$(brew --prefix openssl@3)
    
    # Install cross-compilation tools
    brew tap messense/macos-cross-toolchains
    brew install aarch64-unknown-linux-gnu
    
    # Set up environment variables for cross-compilation
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc
    export CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc
    export CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++
    
    # Configure pkg-config for cross-compilation
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_PATH="/usr/local/opt/openssl@3/lib/pkgconfig"
    
    # Use the vendored OpenSSL feature for reqwest
    export CARGO_BUILD_RUSTFLAGS="--cfg reqwest_unstable"
else
    # Linux
    sudo apt-get update
    sudo apt-get install -y pkg-config libssl-dev gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
    
    # Set up environment variables for cross-compilation
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
    
    # Configure pkg-config for cross-compilation
    export PKG_CONFIG_ALLOW_CROSS=1
    export PKG_CONFIG_AARCH64_UNKNOWN_LINUX_GNU_SYSROOT=/usr/aarch64-linux-gnu
    export PKG_CONFIG_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig"
    
    # Use the vendored OpenSSL feature for reqwest
    export CARGO_BUILD_RUSTFLAGS="--cfg reqwest_unstable"
fi

echo "Cross-compilation environment set up for aarch64-unknown-linux-gnu"
echo "Run 'source cross-compile-setup.sh' before building"

# Print the current environment for debugging
echo "Current environment:"
echo "OPENSSL_DIR=$OPENSSL_DIR"
echo "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=$CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER"
echo "PKG_CONFIG_ALLOW_CROSS=$PKG_CONFIG_ALLOW_CROSS"
echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
