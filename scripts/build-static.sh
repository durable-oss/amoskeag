#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Default target
TARGET="${1:-x86_64-unknown-linux-musl}"

echo "Building static binary for $TARGET..."

# Install cargo-zigbuild if not present
if ! command -v cargo-zigbuild &> /dev/null; then
    echo "Installing cargo-zigbuild..."
    cargo install cargo-zigbuild
fi

# Add the target if not already installed
rustup target add "$TARGET" 2>/dev/null || true

# Build with zigbuild
cargo zigbuild --release --target "$TARGET" -p amoskeag-cli

# Verify static linking
BINARY="target/$TARGET/release/amoskeag"
if [ -f "$BINARY" ]; then
    echo "Binary built: $BINARY"
    file "$BINARY"
    if command -v ldd &> /dev/null; then
        echo "Checking linking:"
        ldd "$BINARY" 2>&1 || echo "Statically linked (no dynamic dependencies)"
    fi
fi
