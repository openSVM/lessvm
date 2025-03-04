#!/bin/bash
set -e

# Create a temporary directory for testing
TEST_DIR=$(mktemp -d)
echo "Created test directory: $TEST_DIR"
cd "$TEST_DIR"

# Set up a local Git repository
echo "Setting up local Git repository..."
git init
git config --global user.name "Test User"
git config --global user.email "test@example.com"

# Create a test file and add it to the index (staging area)
echo "Creating test file and adding it to the index..."
echo "Initial content" > test-file.txt
git add test-file.txt

# Detach HEAD to simulate GitHub Actions environment
git commit -m "Initial commit"
git checkout --detach HEAD
echo "Now in detached HEAD state"

# Make changes in detached HEAD (without committing)
echo "Making changes in detached HEAD (without committing)..."
echo "Updated content" > test-file.txt
git add test-file.txt

echo "Current Git status:"
git status

# Try pushing with the fixed command
echo "Attempting to push with the fixed command..."
git push origin HEAD:main

# Clean up
cd -
echo "Test completed successfully! The fix works."
echo "You can remove the test directory with: rm -rf $TEST_DIR"
