#!/bin/bash
# Deterministic Build Script for Tiny Agent Trainer
# Priority 3.2: Build and Packaging
#
# This script produces a statically linked, optimized binary with full auditing.
#
# Usage: ./scripts/build_release.sh [--clean]

set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Catch errors in pipes

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build configuration
PROJECT_NAME="tiny_agent_trainer_rs"
BINARY_NAME="tiny-agent-trainer"
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DATE=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# Target architecture - Windows native build
# For Linux/musl builds, use: x86_64-unknown-linux-musl
TARGET="x86_64-pc-windows-msvc"

# Build directories
BUILD_DIR="target/release"
DIST_DIR="dist"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}🔨 Tiny Agent Trainer - Release Build${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Version:     ${GREEN}${VERSION}${NC}"
echo -e "Target:      ${GREEN}${TARGET}${NC}"
echo -e "Build Date:  ${GREEN}${BUILD_DATE}${NC}"
echo -e "Git Commit:  ${GREEN}${GIT_COMMIT}${NC}"
echo ""

# Parse command line arguments
CLEAN_BUILD=false
if [ "${1:-}" = "--clean" ]; then
    CLEAN_BUILD=true
fi

# Step 1: Clean build (optional)
if [ "$CLEAN_BUILD" = true ]; then
    echo -e "${YELLOW}🧹 Cleaning previous builds...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Clean complete${NC}"
    echo ""
fi

# Step 2: Verify environment
echo -e "${BLUE}🔍 Verifying build environment...${NC}"

# Check Rust version
RUST_VERSION=$(rustc --version)
echo -e "   Rust: ${RUST_VERSION}"

# Check Cargo version
CARGO_VERSION=$(cargo --version)
echo -e "   Cargo: ${CARGO_VERSION}"

# Verify Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found${NC}"
    exit 1
fi

# Verify source files
if [ ! -d "src" ]; then
    echo -e "${RED}❌ Error: src directory not found${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Environment verified${NC}"
echo ""

# Step 3: Run tests
echo -e "${BLUE}🧪 Running test suite...${NC}"
if cargo test --release --quiet; then
    echo -e "${GREEN}✅ All tests passed${NC}"
else
    echo -e "${RED}❌ Tests failed - aborting build${NC}"
    exit 1
fi
echo ""

# Step 4: Build with maximum optimization
echo -e "${BLUE}🔨 Building optimized binary...${NC}"
echo -e "   Target: ${TARGET}"
echo -e "   Profile: release"
echo -e "   LTO: enabled"
echo -e "   Codegen Units: 1"
echo -e "   Strip: enabled"
echo ""

# Build the release binary
RUSTFLAGS="-C target-cpu=native" cargo build \
    --release \
    --target ${TARGET} \
    --bin ${BINARY_NAME}

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Build complete${NC}"
echo ""

# Step 5: Verify binary
echo -e "${BLUE}🔍 Verifying binary...${NC}"

BINARY_PATH="${BUILD_DIR}/${BINARY_NAME}.exe"
if [ ! -f "${BINARY_PATH}" ]; then
    echo -e "${RED}❌ Error: Binary not found at ${BINARY_PATH}${NC}"
    exit 1
fi

# Get binary size
BINARY_SIZE=$(stat -c%s "${BINARY_PATH}" 2>/dev/null || stat -f%z "${BINARY_PATH}" 2>/dev/null || echo "unknown")
BINARY_SIZE_MB=$(echo "scale=2; ${BINARY_SIZE} / 1048576" | bc 2>/dev/null || echo "unknown")

echo -e "   Binary: ${BINARY_PATH}"
echo -e "   Size: ${BINARY_SIZE_MB} MB (${BINARY_SIZE} bytes)"
echo ""

# Test the binary
echo -e "${BLUE}🧪 Testing binary...${NC}"
if "${BINARY_PATH}" --version > /dev/null 2>&1; then
    BINARY_VERSION=$("${BINARY_PATH}" --version)
    echo -e "   ${BINARY_VERSION}"
    echo -e "${GREEN}✅ Binary is functional${NC}"
else
    echo -e "${RED}❌ Binary test failed${NC}"
    exit 1
fi
echo ""

# Step 6: Generate build manifest
echo -e "${BLUE}📝 Generating build manifest...${NC}"

MANIFEST_FILE="${BUILD_DIR}/BUILD_MANIFEST.txt"
cat > "${MANIFEST_FILE}" << EOF
========================================
Tiny Agent Trainer - Build Manifest
========================================

Build Information:
  Version:        ${VERSION}
  Target:         ${TARGET}
  Build Date:     ${BUILD_DATE}
  Git Commit:     ${GIT_COMMIT}
  Rust Version:   ${RUST_VERSION}
  Cargo Version:  ${CARGO_VERSION}

Binary Information:
  Name:           ${BINARY_NAME}.exe
  Size:           ${BINARY_SIZE_MB} MB (${BINARY_SIZE} bytes)
  Path:           ${BINARY_PATH}

Build Configuration:
  Optimization:   opt-level = 3
  LTO:            enabled
  Codegen Units:  1
  Strip:          enabled
  Panic:          abort
  Target CPU:     native

Test Results:
  Status:         All tests passed
  Test Command:   cargo test --release

Verification:
  Binary Test:    Passed
  Version Check:  Passed

Build Script:
  Script:         scripts/build_release.sh
  Clean Build:    ${CLEAN_BUILD}

========================================
Build completed successfully
========================================
EOF

echo -e "${GREEN}✅ Manifest generated: ${MANIFEST_FILE}${NC}"
echo ""

# Step 7: Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Release build complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Binary location: ${BLUE}${BINARY_PATH}${NC}"
echo -e "Binary size:     ${BLUE}${BINARY_SIZE_MB} MB${NC}"
echo -e "Manifest:        ${BLUE}${MANIFEST_FILE}${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Run packaging script: ${BLUE}./scripts/package_release.sh${NC}"
echo -e "  2. Test the binary:      ${BLUE}${BINARY_PATH} check${NC}"
echo -e "  3. Review manifest:      ${BLUE}cat ${MANIFEST_FILE}${NC}"
echo ""
