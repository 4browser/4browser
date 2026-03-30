#!/bin/bash

# 4 Browser Build Script
# This script automates the build process for 4 Browser

set -e  # Exit on error

echo "🔨 4 Browser Build Script"
echo "========================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust not installed!${NC}"
    echo "Install from: https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓ Rust found${NC}"
rustc --version
cargo --version

# Parse arguments
BUILD_TYPE=${1:-release}
FEATURES=${2:-""}

if [ "$BUILD_TYPE" = "debug" ]; then
    BUILD_FLAG=""
    echo -e "${YELLOW}Building debug version...${NC}"
else
    BUILD_FLAG="--release"
    echo -e "${YELLOW}Building release version...${NC}"
fi

# Build
echo "📦 Compiling..."
if [ -z "$FEATURES" ]; then
    cargo build $BUILD_FLAG
else
    echo "🎨 With features: $FEATURES"
    cargo build $BUILD_FLAG --features "$FEATURES"
fi

# Determine binary path
if [ "$BUILD_TYPE" = "debug" ]; then
    BINARY="./target/debug/4browser"
else
    BINARY="./target/release/4browser"
fi

# Check if build succeeded
if [ -f "$BINARY" ] || [ -f "$BINARY.exe" ]; then
    echo -e "${GREEN}✅ Build successful!${NC}"
    echo ""
    echo "📍 Binary location:"
    echo "   $BINARY"
    echo ""
    echo "🚀 To run the browser:"
    echo "   $BINARY"
    echo ""
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi
