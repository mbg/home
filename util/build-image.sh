#!/bin/bash
set -euo pipefail

# Ensure a consistent location
cd "$(dirname "$0")/../"

# Build the main image
podman build -f Dockerfile --platform "linux/arm64" -t home:latest .
