.PHONY: help build test clean install-tools zigbuild release

help:
	@echo "Amoskeag Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build         - Build the project using cargo"
	@echo "  test          - Run all tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  install-tools - Install required build tools (cargo-zigbuild)"
	@echo "  zigbuild      - Build using cargo-zigbuild"
	@echo "  release       - Build optimized release using zigbuild"
	@echo ""
	@echo "Cross-compilation targets:"
	@echo "  linux-x64     - Build for Linux x86_64"
	@echo "  linux-arm64   - Build for Linux ARM64"
	@echo "  linux-musl    - Build for Linux x86_64 (musl)"

# Standard cargo build
build:
	cargo build

# Run tests
test:
	cargo test --all

# Clean build artifacts
clean:
	cargo clean

# Install required tools
install-tools:
	@echo "Installing cargo-zigbuild..."
	cargo install cargo-zigbuild
	@echo "Done! Tools installed successfully."

# Build with zigbuild (default target)
zigbuild:
	cargo zigbuild

# Release build with zigbuild
release:
	cargo zigbuild --release

# Cross-compilation targets
linux-x64:
	cargo zigbuild --target x86_64-unknown-linux-gnu --release

linux-arm64:
	cargo zigbuild --target aarch64-unknown-linux-gnu --release

linux-musl:
	cargo zigbuild --target x86_64-unknown-linux-musl --release

# Common development workflow
dev: test
	cargo build

# CI/CD workflow
ci: test release

# Format code
fmt:
	cargo fmt --all

# Check code without building
check:
	cargo check --all

# Run clippy
clippy:
	cargo clippy --all -- -D warnings
