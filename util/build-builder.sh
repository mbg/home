#!/bin/bash
set -euo pipefail

# Ensure a consistent location
cd "$(dirname "$0")/../"

podman build \
    -f ./util/builder/Dockerfile \
    --platform "linux/arm64" \
    -t home-builder:latest .
