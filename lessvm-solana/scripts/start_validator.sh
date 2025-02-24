#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Starting local validator...${NC}"

# Check if validator is already running
if solana cluster-version &>/dev/null; then
    echo -e "${GREEN}Local validator is already running${NC}"
    exit 0
fi

# Start local validator
solana-test-validator \
    --reset \
    --quiet \
    --bind-address 0.0.0.0 \
    --rpc-port 8899 \
    --faucet-port 9900 \
    --ledger .validator &

# Wait for validator to start
echo -e "${YELLOW}Waiting for validator to start...${NC}"
until solana cluster-version &>/dev/null; do
    sleep 1
done

echo -e "${GREEN}Local validator is running${NC}"