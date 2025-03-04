#!/bin/bash
set -e

# Check if Homebrew is installed
if ! command -v brew &> /dev/null
then
    echo "Homebrew is not installed. Please install it first."
    echo "You can install Homebrew from: https://brew.sh/"
    exit 1
fi

# Create a temporary directory for testing
TEST_DIR=$(mktemp -d)
echo "Created test directory: $TEST_DIR"
cd "$TEST_DIR"

# Download the Homebrew formula file
FORMULA_URL="https://raw.githubusercontent.com/openSVM/lessvm/main/lessvm.rb"
FORMULA_FILE="lessvm.rb"
echo "Downloading Homebrew formula..."
if ! curl -v -sSL -o "$FORMULA_FILE" "$FORMULA_URL"; then
  echo "Error: Failed to download Homebrew formula from $FORMULA_URL"
  exit 1
fi

# Run brew style
echo "Running brew style..."
brew update
brew style --except-cops FormulaAudit/Homepage,FormulaAudit/Desc,FormulaAuditStrict --fix "$FORMULA_FILE" || true

# Clean up
cd -
echo "Test completed. Check the output for any style violations."
echo "You can remove the test directory with: rm -rf $TEST_DIR"
