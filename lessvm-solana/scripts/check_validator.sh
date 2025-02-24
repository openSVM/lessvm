#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Checking local validator...${NC}"

# Try to connect to local validator
if ! solana cluster-version &>/dev/null; then
    echo -e "${RED}Local validator not running${NC}"
    echo -e "${YELLOW}Starting local validator...${NC}"
    
    # Start validator in background
    solana-test-validator \
        --reset \
        --quiet \
        --bind-address 0.0.0.0 \
        --rpc-port 8899 \
        --faucet-port 9900 \
        --ledger .validator &

    # Wait for validator to start
    echo -e "${YELLOW}Waiting for validator to start...${NC}"
    while ! solana cluster-version &>/dev/null; do
        sleep 1
    done
fi

echo -e "${GREEN}Local validator is running${NC}"