#!/bin/bash
set -euo pipefail

# Configuration
NETWORK_ENDPOINT="http://127.0.0.1:8899"
LOG_FILE="hot_reload_results.log"
METRICS_INTERVAL=5  # seconds

# Function to log with timestamp
log_entry() {
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] $1" | tee -a "$LOG_FILE"
}

# Function to monitor program state
monitor_program_state() {
    local program_id=$1
    local account_id=$2
    
    log_entry "Monitoring program state for Program ID: $program_id"
    log_entry "Data Account: $account_id"
    
    # Get initial state
    local initial_data
    initial_data=$(solana account "$account_id" --output json --url "$NETWORK_ENDPOINT")
    log_entry "Initial state captured"
    
    while true; do
        # Get current state
        local current_data
        current_data=$(solana account "$account_id" --output json --url "$NETWORK_ENDPOINT")
        
        # Compare states
        if [ "$initial_data" != "$current_data" ]; then
            log_entry "State change detected!"
            log_entry "Previous state: $initial_data"
            log_entry "Current state: $current_data"
            initial_data=$current_data
        fi
        
        # Collect performance metrics
        {
            echo "=== Performance Metrics ==="
            echo "Slot: $(solana slot --url "$NETWORK_ENDPOINT")"
            echo "Transaction Count: $(solana transaction-count --url "$NETWORK_ENDPOINT")"
            solana block-production --url "$NETWORK_ENDPOINT" | tail -n 5
            echo "=========================="
        } >> "$LOG_FILE"
        
        sleep "$METRICS_INTERVAL"
    done
}

# Function to monitor transaction signatures
monitor_transactions() {
    local program_id=$1
    
    log_entry "Starting transaction monitoring for Program ID: $program_id"
    
    while true; do
        # Get recent signatures
        local signatures
        signatures=$(solana signatures --limit 10 --url "$NETWORK_ENDPOINT" "$program_id")
        
        if [ -n "$signatures" ]; then
            while IFS= read -r signature; do
                # Get transaction details
                local tx_details
                tx_details=$(solana confirm -v --url "$NETWORK_ENDPOINT" "$signature")
                
                log_entry "New transaction detected:"
                log_entry "Signature: $signature"
                log_entry "Details: $tx_details"
                
                # Check for errors
                if echo "$tx_details" | grep -q "Error"; then
                    log_entry "ERROR: Transaction failed!"
                else
                    log_entry "Transaction confirmed successfully"
                fi
            done <<< "$signatures"
        fi
        
        sleep 2
    done
}

# Main execution
main() {
    if [ "$#" -ne 2 ]; then
        echo "Usage: $0 <program_id> <account_id>"
        exit 1
    fi
    
    local program_id=$1
    local account_id=$2
    
    log_entry "Starting deployment monitoring"
    log_entry "Network endpoint: $NETWORK_ENDPOINT"
    
    # Start monitoring processes
    monitor_program_state "$program_id" "$account_id" &
    STATE_MONITOR_PID=$!
    
    monitor_transactions "$program_id" &
    TX_MONITOR_PID=$!
    
    # Trap cleanup
    trap 'kill $STATE_MONITOR_PID $TX_MONITOR_PID 2>/dev/null' EXIT
    
    # Wait for monitoring processes
    wait $STATE_MONITOR_PID $TX_MONITOR_PID
}

# Execute main function with arguments
main "$@"