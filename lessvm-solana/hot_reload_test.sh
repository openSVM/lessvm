#!/bin/bash
set -euo pipefail

# Configuration
NETWORK_ENDPOINT="http://127.0.0.1:8899"
PROGRAM_DATA_SPACE=1024  # Space for program data in bytes

# Function to log with timestamp
log_entry() {
    echo "$1"
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] $1"
}

# Generate test keypair
log_entry "Generating test keypair..."
TEST_KEYPAIR="$(mktemp -t test-keypair-XXXXXXXX.json)"
solana-keygen new --force --silent -o "$TEST_KEYPAIR" --no-bip39-passphrase
TEST_PUBKEY=$(solana-keygen pubkey "$TEST_KEYPAIR")
log_entry "Test public key: $TEST_PUBKEY"

# Request airdrop
log_entry "Requesting airdrop..."
solana airdrop 2 "$TEST_PUBKEY" --url "$NETWORK_ENDPOINT"

# Build the program

# Extract program ID
PROGRAM_ID=LESSgfHQvSK6W3RcS4sVjYaTjNYGay1e9ojR8uoCVyv
log_entry "Program ID: $PROGRAM_ID"

# Create test program - Simple Addition (from knowledge graph)
log_entry "Creating test program..."
cat > test_program_initial.hex << EOF
# Simple Addition Program
01 05     # PUSH1 5
01 03     # PUSH1 3
10        # ADD
43        # LOG
FF        # HALT
EOF

# Convert hex to binary
xxd -r -p test_program_initial.hex test_program_initial.bin

# Create program data account
log_entry "Creating program data account..."
DATA_KEYPAIR="data-account.json"
solana-keygen new --force --silent -o "$DATA_KEYPAIR" --no-bip39-passphrase
DATA_PUBKEY=$(solana-keygen pubkey "$DATA_KEYPAIR")

# Get minimum balance for rent exemption
MIN_BALANCE=$(solana rent --url "$NETWORK_ENDPOINT" "$PROGRAM_DATA_SPACE" | grep 'minimum:' | awk '{print $3}')
log_entry "Creating account with space: $PROGRAM_DATA_SPACE bytes, balance: $MIN_BALANCE lamports"

# Create the account
solana create account \
    --url "$NETWORK_ENDPOINT" \
    --keypair "$TEST_KEYPAIR" \
    "$DATA_PUBKEY" \
    "$MIN_BALANCE" \
    "$PROGRAM_DATA_SPACE"

# Write initial program
log_entry "Writing initial program..."
solana program write \
    --url "$NETWORK_ENDPOINT" \
    --keypair "$TEST_KEYPAIR" \
    "$DATA_PUBKEY" \
    test_program_initial.bin

# Execute initial program
log_entry "Executing initial program..."
solana program invoke \
    --url "$NETWORK_ENDPOINT" \
    --keypair "$TEST_KEYPAIR" \
    "$PROGRAM_ID" \
    --account "$DATA_PUBKEY:rwx"

# Create updated program - MulDiv operation (from knowledge graph)
log_entry "Creating updated program..."
cat > test_program_updated.hex << EOF
# MulDiv Program
01 0A     # PUSH1 10
01 05     # PUSH1 5
01 02     # PUSH1 2
14        # MULDIV
43        # LOG
FF        # HALT
EOF

# Convert updated hex to binary
xxd -r -p test_program_updated.hex test_program_updated.bin

# Write updated program (hot reload)
log_entry "Hot reloading updated program..."
solana program write \
    --url "$NETWORK_ENDPOINT" \
    --keypair "$TEST_KEYPAIR" \
    "$DATA_PUBKEY" \
    test_program_updated.bin

# Execute updated program
log_entry "Executing updated program..."
solana program invoke \
    --url "$NETWORK_ENDPOINT" \
    --keypair "$TEST_KEYPAIR" \
    "$PROGRAM_ID" \
    --account "$DATA_PUBKEY:rwx"

# Cleanup

log_entry "Test completed successfully"