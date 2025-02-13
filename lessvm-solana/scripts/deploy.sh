#!/bin/bash

# Exit on error
set -e

# Configuration
ECLIPSE_RPC="http://127.0.0.1:8899"
PROGRAM_KEYPAIR="lessvm-keypair.json"
DEPLOY_KEYPAIR="deploy-keypair.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Starting LessVM deployment to Eclipse Mainnet...${NC}"

# Check if keypairs exist
if [ ! -f "$PROGRAM_KEYPAIR" ]; then
    echo -e "${YELLOW}Generating program keypair...${NC}"
    solana-keygen new --no-bip39-passphrase -o "$PROGRAM_KEYPAIR"
fi

if [ ! -f "$DEPLOY_KEYPAIR" ]; then
    echo -e "${RED}Error: Deploy keypair not found. Please create one with:${NC}"
    echo "solana-keygen new --no-bip39-passphrase -o $DEPLOY_KEYPAIR"
    exit 1
fi

# Build with optimizations
echo -e "${YELLOW}Building program with optimizations...${NC}"
RUSTFLAGS="-C target-cpu=native -C target-feature=+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+avx" \
    cargo build-bpf --release

# Get program ID
PROGRAM_ID=$(solana-keygen pubkey "$PROGRAM_KEYPAIR")
echo -e "${GREEN}Program ID: ${PROGRAM_ID}${NC}"

# Deploy to Eclipse
echo -e "${YELLOW}Deploying to Eclipse Mainnet...${NC}"
solana program deploy \
    --url $ECLIPSE_RPC \
    --keypair "$DEPLOY_KEYPAIR" \
    --program-id "$PROGRAM_KEYPAIR" \
    target/deploy/lessvm_solana.so

# Verify deployment
echo -e "${YELLOW}Verifying deployment...${NC}"
solana program show \
    --url $ECLIPSE_RPC \
    "$PROGRAM_ID"

echo -e "${GREEN}Deployment complete!${NC}"
echo -e "${YELLOW}Program ID: ${PROGRAM_ID}${NC}"
echo -e "${YELLOW}Explorer URL: https://explorer.eclipse.com/address/${PROGRAM_ID}${NC}"

# Print next steps
echo -e "\n${GREEN}Next steps:${NC}"
echo "1. Save your Program ID: $PROGRAM_ID"
echo "2. Update your client applications with the new Program ID"
echo "3. Monitor program usage at https://explorer.eclipse.com/address/${PROGRAM_ID}"
echo "4. Run tests against the deployed program:"
echo "   ECLIPSE_RPC=$ECLIPSE_RPC PROGRAM_ID=$PROGRAM_ID cargo test --release"