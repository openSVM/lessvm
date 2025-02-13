#!/bin/bash
set -euo pipefail

# Configuration
NETWORK_ENDPOINT="http://127.0.0.1:8899"
LOG_FILE="hot_reload_results.log"

# Function to log with timestamp
log_entry() {
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] $1" | tee -a "$LOG_FILE"
}

# Function to check if local validator is running
check_validator() {
    if ! solana cluster-version --url "$NETWORK_ENDPOINT" &>/dev/null; then
        log_entry "Local validator not running. Starting validator..."
        ./start_local_validator.sh &
        VALIDATOR_PID=$!
        echo $VALIDATOR_PID > validator.pid
        # Wait for validator to start (max 30 seconds)
        local max_attempts=3
        local attempt=1
        while [ $attempt -le $max_attempts ]; do
            if solana cluster-version --url "$NETWORK_ENDPOINT" &>/dev/null; then
                log_entry "Local validator started successfully"
                return 0
            fi
            log_entry "Waiting for validator to start (attempt $attempt/$max_attempts)..."
            sleep 25
            attempt=$((attempt + 1))
        done
        
        log_entry "ERROR: Validator failed to start within 30 seconds"
        return 1
    fi
    log_entry "Local validator running at $NETWORK_ENDPOINT"
    return 0
}

# Function to cleanup processes
cleanup() {
    log_entry "Cleaning up processes..."
    kill $(jobs -p) 2>/dev/null || true
}

# Main execution
main() {
    # Set up cleanup trap
    trap cleanup EXIT
    
    # Initialize log file
    echo "=== SVM Test Execution $(date -u '+%Y-%m-%d %H:%M:%S UTC') ===" > "$LOG_FILE"
    
    # Start and check local validator
    if ! check_validator; then
        exit 1
    fi
    
    # Initialize SVM
    log_entry "Initializing SVM..."
    ./scripts/init_svm.sh
    
    # Extract program ID and account ID from hot_reload_test output
    log_entry "Running hot_reload_test to get program and account IDs..."
    DEPLOY_OUTPUT=$(./hot_reload_test.sh 2>&1 | tee -a "$LOG_FILE")
    
    # Extract program ID and account ID
    PROGRAM_ID=$(echo "$DEPLOY_OUTPUT" | grep 'Program Id:' | awk '{print $3}')
    DATA_PUBKEY=$(echo "$DEPLOY_OUTPUT" | grep 'New program data account:' | awk '{print $5}')
    
    if [ -z "$PROGRAM_ID" ] || [ -z "$DATA_PUBKEY" ]; then
        log_entry "ERROR: Failed to extract program ID or account ID"
        exit 1
    fi
    
    log_entry "Program ID: $PROGRAM_ID"
    log_entry "Account ID: $DATA_PUBKEY"
    
    # Start deployment monitoring
    log_entry "Starting deployment monitoring..."
    ./scripts/monitor_deployment.sh "$PROGRAM_ID" "$DATA_PUBKEY" &
    
    # Wait for monitoring to initialize
    sleep 20
    
    log_entry "SVM test environment ready"
    log_entry "Monitoring active - Check $LOG_FILE for detailed logs"
    log_entry "Press Ctrl+C to stop monitoring and cleanup"
    
    # Keep script running until interrupted
    while true; do
        sleep 1
    done
}

# Execute main function
main "$@"