#!/usr/bin/env bash
set -euo pipefail

# Release script for Amoskeag
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.1.0

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

# Remove 'v' prefix if provided
VERSION="${VERSION#v}"
TAG="v${VERSION}"

echo "Releasing Amoskeag ${TAG}"

# Ensure we're on master and up to date
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "master" ] && [ "$BRANCH" != "main" ]; then
    echo "Error: Must be on master or main branch (currently on $BRANCH)"
    exit 1
fi

echo "Pulling latest changes..."
git pull --rebase

cargo clean

echo "Running tests..."
cargo test --workspace

echo "Checking formatting..."
cargo fmt --all -- --check

# Check if tag exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Error: Tag $TAG already exists"
    exit 1
fi

echo ""
echo "All checks passed. Ready to release ${TAG}"
echo "This will:"
echo "  1. Create git tag ${TAG}"
echo "  2. Push tag to origin"
echo "  3. Trigger GitHub Actions release workflow"
echo ""
read -p "Continue? [y/N] " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted"
    exit 1
fi

echo "Creating tag ${TAG}..."
git tag -a "$TAG" -m "Release ${TAG}"

echo "Pushing tag to origin..."
git push origin "$TAG"

echo ""
echo "Release ${TAG} initiated!"
echo "Monitor the release at: https://github.com/durable-oss/amoskeag/actions"
