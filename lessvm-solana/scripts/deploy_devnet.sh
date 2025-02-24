#!/bin/bash

# Exit on error
set -e

# Configuration
DEVNET_RPC="https://api.devnet.solana.com"
PROGRAM_KEYPAIR="lessvm-keypair.json"
DEPLOY_KEYPAIR="deploy-keypair.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Starting LessVM deployment to Devnet...${NC}"

# Check if keypairs exist
if [ ! -f "$PROGRAM_KEYPAIR" ]; then
    echo -e "${YELLOW}Generating program keypair...${NC}"
    solana-keygen new --no-bip39-passphrase -o "$PROGRAM_KEYPAIR"
fi

if [ ! -f "$DEPLOY_KEYPAIR" ]; then
    echo -e "${YELLOW}Generating deploy keypair...${NC}"
    solana-keygen new --no-bip39-passphrase -o "$DEPLOY_KEYPAIR"
fi

# Switch to devnet
solana config set --url $DEVNET_RPC

# Airdrop some SOL for deployment
echo -e "${YELLOW}Requesting airdrop...${NC}"
solana airdrop 2 --keypair "$DEPLOY_KEYPAIR"

# Build with optimizations
echo -e "${YELLOW}Building program with optimizations...${NC}"
RUSTFLAGS="-C target-cpu=native" cargo build-bpf --release

# Get program ID
PROGRAM_ID=$(solana-keygen pubkey "$PROGRAM_KEYPAIR")
echo -e "${GREEN}Program ID: ${PROGRAM_ID}${NC}"

# Deploy to devnet
echo -e "${YELLOW}Deploying to Devnet...${NC}"
solana program deploy \
    --keypair "$DEPLOY_KEYPAIR" \
    --program-id "$PROGRAM_KEYPAIR" \
    target/deploy/lessvm_solana.so

# Verify deployment
echo -e "${YELLOW}Verifying deployment...${NC}"
solana program show "$PROGRAM_ID"

echo -e "${GREEN}Deployment complete!${NC}"
echo -e "${YELLOW}Program ID: ${PROGRAM_ID}${NC}"
echo -e "${YELLOW}Explorer URL: https://explorer.solana.com/address/${PROGRAM_ID}?cluster=devnet${NC}"

# Create test script
echo -e "${YELLOW}Creating version test script...${NC}"
cat > test_version.js << EOF
const { Connection, PublicKey, Transaction, TransactionInstruction } = require('@solana/web3.js');
const { serialize } = require('borsh');

async function main() {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const programId = new PublicKey('${PROGRAM_ID}');
    
    // Create GetVersion instruction
    const instruction = new TransactionInstruction({
        keys: [],
        programId,
        data: Buffer.from([3]) // GetVersion instruction index
    });

    // Send transaction
    const transaction = new Transaction().add(instruction);
    const signature = await connection.sendTransaction(transaction, []);
    
    // Wait for confirmation and get transaction logs
    await connection.confirmTransaction(signature);
    const tx = await connection.getTransaction(signature, { commitment: 'confirmed' });
    console.log('Transaction logs:', tx.meta.logMessages);
}

main().catch(console.error);
EOF

# Install dependencies and run test
echo -e "${YELLOW}Installing dependencies...${NC}"
npm install @solana/web3.js borsh

echo -e "${YELLOW}Running version test...${NC}"
node test_version.js