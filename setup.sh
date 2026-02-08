#!/bin/bash

# Setup script for Docx2PDF Converter
# This script installs all dependencies and prepares the project for building

set -e

echo "==================================="
echo "Docx2PDF Converter - Setup Script"
echo "==================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if Node.js is installed
echo "Checking prerequisites..."
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed"
    echo "Please install Node.js v18 or later from https://nodejs.org/"
    exit 1
fi

NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    print_warning "Node.js version is $NODE_VERSION. Recommended: v18 or later"
else
    print_status "Node.js $(node --version)"
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    print_error "npm is not installed"
    exit 1
fi
print_status "npm $(npm --version)"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
print_status "Rust $(cargo --version | cut -d' ' -f2)"

# Install Node dependencies
echo ""
echo "Installing Node dependencies..."
npm install
print_status "Node dependencies installed"

# Check for Tauri CLI
if ! command -v tauri &> /dev/null; then
    echo "Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
    print_status "Tauri CLI installed"
else
    print_status "Tauri CLI already installed"
fi

# Install Rust dependencies (build once to download crates)
echo ""
echo "Installing Rust dependencies..."
cd src-tauri
cargo fetch
print_status "Rust dependencies downloaded"
cd ..

# Check LibreOffice status
echo ""
echo "Checking LibreOffice bundles..."
if [ -f "libreoffice/libreoffice-win.zip" ]; then
    print_status "Windows LibreOffice bundle found ($(du -h libreoffice/libreoffice-win.zip | cut -f1))"
else
    print_warning "Windows LibreOffice bundle not found"
    echo "  → Run ./download-libreoffice.sh for instructions"
fi

if [ -f "libreoffice/libreoffice-linux.zip" ]; then
    print_status "Linux LibreOffice bundle found ($(du -h libreoffice/libreoffice-linux.zip | cut -f1))"
else
    print_warning "Linux LibreOffice bundle not found"
    echo "  → System LibreOffice will be used if available"
    echo "  → Run ./download-libreoffice.sh to bundle it"
fi

echo ""
echo "==================================="
echo "Setup Complete!"
echo "==================================="
echo ""
echo "Next steps:"
echo "  1. Ensure LibreOffice bundles are in place (optional on Linux)"
echo "  2. Run: npm run tauri:dev     (for development)"
echo "  3. Run: npm run tauri:build   (to create release build)"
echo ""
