#!/bin/bash
set -euo pipefail

echo "Starting local Solana validator setup..."

# Configuration
LEDGER_PATH="test-ledger"
RPC_PORT=8899
FAUCET_PORT=9900
FAUCET_KEYPAIR="faucet-keypair.json"

# Clean up any existing ledger
if [ -d "$LEDGER_PATH" ]; then
    echo "Cleaning up existing ledger..."
    rm -rf "$LEDGER_PATH"
fi

# Create a keypair for the faucet if it doesn't exist
if [ ! -f "$FAUCET_KEYPAIR" ]; then
    echo "Generating faucet keypair..."
    solana-keygen new --no-bip39-passphrase -o "$FAUCET_KEYPAIR"
fi

# Get the faucet address
FAUCET_ADDRESS=$(solana-keygen pubkey "$FAUCET_KEYPAIR")
echo "Faucet address: $FAUCET_ADDRESS"

# Start the validator in the background
echo "Starting local validator..."
solana-test-validator \
    --ledger "$LEDGER_PATH" \
    --rpc-port $RPC_PORT \
    --faucet-port $FAUCET_PORT \
    --faucet-sol 1000 \
    --reset \
    &

# Store the validator PID
VALIDATOR_PID=$!

# Wait for validator to start
echo "Waiting for validator to start..."
sleep 5

# Configure CLI to use local cluster
echo "Configuring Solana CLI for local cluster..."
solana config set --url "http://127.0.0.1:$RPC_PORT"

# Start the faucet in the background
echo "Starting faucet..."
solana-faucet \
    --keypair "$FAUCET_KEYPAIR" \
    --port $FAUCET_PORT \
    --url "http://127.0.0.1:$RPC_PORT" \
    &

# Store the faucet PID
FAUCET_PID=$!

echo "Local Solana devnet is running!"
echo "RPC URL: http://127.0.0.1:$RPC_PORT"
echo "Faucet URL: http://127.0.0.1:$FAUCET_PORT"
echo "Faucet address: $FAUCET_ADDRESS"
echo
echo "Use Ctrl+C to stop the local devnet"

# Function to cleanup on exit
cleanup() {
    echo "Shutting down local devnet..."
    kill $VALIDATOR_PID 2>/dev/null || true
    kill $FAUCET_PID 2>/dev/null || true
    wait $VALIDATOR_PID 2>/dev/null || true
    wait $FAUCET_PID 2>/dev/null || true
    echo "Local devnet stopped"
}

# Register the cleanup function to run on script exit
trap cleanup EXIT

# Wait for Ctrl+C
wait $VALIDATOR_PID 