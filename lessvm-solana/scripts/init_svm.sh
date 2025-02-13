#!/bin/bash
set -euo pipefail

# Configuration
NETWORK_ENDPOINT="http://127.0.0.1:8899"
MAX_RETRIES=5
CONFIRMATION_LEVEL="confirmed"
LOG_FILE="init_reload_results.log"

# Initialize logging
echo "=== SVM Initialization $(date -u '+%Y-%m-%d %H:%M:%S UTC') ===" > "INIT.LOG"

# Function to log with timestamp
log_entry() {
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] $1" | tee -a "INIT.LOG"
}

# Function to verify program ID
verify_program_id() {
    local program_id=$1
    log_entry "Verifying program ID: $program_id"
    if ! solana program show "$program_id" --url "$NETWORK_ENDPOINT" &>/dev/null; then
        log_entry "ERROR: Invalid program ID: $program_id"
        return 1
    fi
    log_entry "Program ID verification successful"
    return 0
}

# Function to monitor transaction
monitor_transaction() {
    local signature=$1
    local retry_count=0
    
    while [ $retry_count -lt $MAX_RETRIES ]; do
        if solana confirm -v --url "$NETWORK_ENDPOINT" "$signature"; then
            log_entry "Transaction confirmed: $signature"
            return 0
        fi
        retry_count=$((retry_count + 1))
        log_entry "Retry $retry_count/$MAX_RETRIES for transaction $signature"
        sleep 1
    done
    
    log_entry "ERROR: Transaction failed after $MAX_RETRIES retries: $signature"
    return 1
}

# Initialize SVM connection
log_entry "Initializing SVM connection to $NETWORK_ENDPOINT"
if ! solana config set --url "$NETWORK_ENDPOINT"; then
    log_entry "ERROR: Failed to set Solana config"
    exit 1
fi

# Set confirmation level
log_entry "Setting confirmation level to $CONFIRMATION_LEVEL"
solana config set --commitment "$CONFIRMATION_LEVEL"

# Start performance monitoring
log_entry "Starting performance monitoring"
{
    while true; do
        echo "--- Performance Metrics $(date -u '+%Y-%m-%d %H:%M:%S UTC') ---"
        solana transaction-count --url "$NETWORK_ENDPOINT"
        solana block-production --url "$NETWORK_ENDPOINT"
        solana validators --url "$NETWORK_ENDPOINT"
        sleep 5
    done
} >> "perf.log" &
MONITOR_PID=$!

# Trap for cleanup
trap 'kill $MONITOR_PID 2>/dev/null' EXIT

# Execute hot_reload_test.sh with monitoring
log_entry "Starting hot reload test execution"
if ./hot_reload_test.sh 2>&1 | tee -a "INIT.LOG"; then
    log_entry "Hot reload test completed successfully"
else
    log_entry "ERROR: Hot reload test failed"
    exit 1
fi

# Final status report
log_entry "SVM initialization and test execution completed"
echo "Complete execution log available in: INIT.LOG"