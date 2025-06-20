#!/usr/bin/env bash
#
# validate.sh - Validate that both ESP32-S3 and ESP32-C6 builds work correctly
#

set -e

echo "🔧 ESP-NOW Blinky Project Validation"
echo "====================================="

# Check if just is installed
if ! command -v just &> /dev/null; then
    echo "❌ 'just' command not found. Install with: cargo install just"
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ 'cargo' command not found. Install Rust toolchain first."
    exit 1
fi

echo "✅ Prerequisites check passed"
echo ""

# Test ESP32-S3 build
echo "🔨 Testing ESP32-S3 build..."
if just check-s3; then
    echo "✅ ESP32-S3 check passed"
else
    echo "❌ ESP32-S3 check failed"
    exit 1
fi

# Test ESP32-C6 build  
echo "🔨 Testing ESP32-C6 build..."
if just check-c6; then
    echo "✅ ESP32-C6 check passed"
else
    echo "❌ ESP32-C6 check failed"
    exit 1
fi

echo ""
echo "🎉 All validation tests passed!"
echo ""
echo "Next steps:"
echo "  • Connect an ESP32-S3 and run: just flash-s3"
echo "  • Connect an ESP32-C6 and run: just flash-c6" 
echo "  • Monitor serial output with: just monitor-s3 or just monitor-c6"
echo ""
echo "Available commands:"
echo "  • just --list       # Show all available commands"
echo "  • just build-all    # Build for both targets"
echo "  • just clean        # Clean build artifacts"
