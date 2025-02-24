#!/bin/bash
# Test script for GoReleaser configuration
# This script runs GoReleaser in snapshot mode to test the configuration without publishing

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Testing GoReleaser configuration...${NC}"
echo ""

# Check if GoReleaser is installed
if ! command -v goreleaser &> /dev/null; then
    echo "GoReleaser is not installed. Please install it first:"
    echo "  brew install goreleaser (macOS)"
    echo "  curl -sfL https://goreleaser.com/static/run | bash (Linux)"
    exit 1
fi

# Run GoReleaser in snapshot mode
echo -e "${BLUE}Running GoReleaser in snapshot mode...${NC}"
echo ""

# Navigate to the cli directory if not already there
if [[ $(basename "$PWD") != "cli" ]]; then
    if [[ -d "cli" ]]; then
        cd cli
    else
        echo "Please run this script from the project root or cli directory"
        exit 1
    fi
fi

# Run GoReleaser in snapshot mode
goreleaser release --snapshot --clean --skip=publish

echo ""
echo -e "${GREEN}Test completed successfully!${NC}"
echo ""
echo "The build artifacts can be found in the dist/ directory."
echo "You can inspect them to verify that the build process works as expected."
echo ""
echo "To create an actual release:"
echo "1. Tag a new version: git tag -a v0.1.1 -m \"Release v0.1.1\""
echo "2. Push the tag: git push origin v0.1.1"
echo "3. GitHub Actions will automatically build and publish the release"
