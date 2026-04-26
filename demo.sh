#!/bin/bash
# OVIS Sandbox - Quick Demo Script
# This script builds and runs the OVIS Sandbox demonstration

set -e  # Exit on any error

echo "🚀 OVIS Sandbox - Active Cryptographic Firewall for Regulated AI"
echo "================================================================"
echo ""

# Check if Docker is available
if command -v docker &> /dev/null; then
    echo "🐳 Docker detected. Building and running with Docker..."
    docker build -t ovis-sandbox .
    docker run --rm ovis-sandbox
elif command -v cargo &> /dev/null; then
    echo "🦀 Docker not found, but Rust/Cargo detected. Building from source..."
    cargo build --release
    ./target/release/ovis-sandbox
else
    echo "❌ Neither Docker nor Rust/Cargo found. Please install:"
    echo "   - Docker: https://docs.docker.com/get-docker/"
    echo "   - or Rust: https://rustup.rs/"
    exit 1
fi

echo ""
echo "✅ Demo completed. For more information, see README.md"
