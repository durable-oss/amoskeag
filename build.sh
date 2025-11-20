#!/usr/bin/env bash
set -euo pipefail

# Amoskeag Build Script
# This script provides convenient build commands using cargo-zigbuild

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cargo-zigbuild is installed
check_zigbuild() {
    if ! command -v cargo-zigbuild &> /dev/null; then
        error "cargo-zigbuild is not installed"
        info "Install it with: cargo install cargo-zigbuild"
        exit 1
    fi
}

# Parse command line arguments
CMD="${1:-help}"

case "$CMD" in
    build)
        info "Building with cargo..."
        cargo build
        ;;

    test)
        info "Running tests..."
        cargo test --all
        ;;

    release)
        check_zigbuild
        info "Building release with zigbuild..."
        cargo zigbuild --release
        ;;

    zigbuild)
        check_zigbuild
        info "Building with zigbuild..."
        cargo zigbuild
        ;;

    clean)
        info "Cleaning build artifacts..."
        cargo clean
        ;;

    linux-x64)
        check_zigbuild
        info "Cross-compiling for Linux x86_64..."
        cargo zigbuild --target x86_64-unknown-linux-gnu --release
        ;;

    linux-arm64)
        check_zigbuild
        info "Cross-compiling for Linux ARM64..."
        cargo zigbuild --target aarch64-unknown-linux-gnu --release
        ;;

    linux-musl)
        check_zigbuild
        info "Cross-compiling for Linux x86_64 (musl)..."
        cargo zigbuild --target x86_64-unknown-linux-musl --release
        ;;

    install-tools)
        info "Installing cargo-zigbuild..."
        cargo install cargo-zigbuild
        info "Tools installed successfully!"
        ;;

    help|*)
        echo "Amoskeag Build Script"
        echo ""
        echo "Usage: $0 <command>"
        echo ""
        echo "Commands:"
        echo "  build         - Build the project using cargo"
        echo "  test          - Run all tests"
        echo "  clean         - Clean build artifacts"
        echo "  release       - Build optimized release using zigbuild"
        echo "  zigbuild      - Build using cargo-zigbuild"
        echo "  install-tools - Install required build tools"
        echo ""
        echo "Cross-compilation:"
        echo "  linux-x64     - Build for Linux x86_64"
        echo "  linux-arm64   - Build for Linux ARM64"
        echo "  linux-musl    - Build for Linux x86_64 (musl)"
        ;;
esac
