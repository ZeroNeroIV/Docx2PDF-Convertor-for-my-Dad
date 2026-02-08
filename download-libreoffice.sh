#!/bin/bash

# Script to download and prepare LibreOffice Portable for bundling
# Run this from the project root directory

set -e

echo "LibreOffice Downloader for Docx2PDF Converter"
echo "============================================="
echo ""

# Create directories
mkdir -p libreoffice/win libreoffice/linux

echo "NOTE: This script will guide you on downloading LibreOffice."
echo "Due to size and licensing, you'll need to download manually."
echo ""

# Windows
if [ ! -f "libreoffice/libreoffice-win.zip" ]; then
    echo "Windows LibreOffice:"
    echo "1. Download LibreOffice Portable from: https://portableapps.com/apps/office/libreoffice_portable"
    echo "2. Extract the downloaded file"
    echo "3. Zip the extracted folder contents (not the folder itself)"
    echo "4. Save as: libreoffice/libreoffice-win.zip"
    echo ""
    echo "Or use this direct link (may change):"
    echo "   https://downloadarchive.documentfoundation.org/libreoffice/old/latest/deb/x86_64/"
    echo ""
fi

# Linux
if [ ! -f "libreoffice/libreoffice-linux.zip" ]; then
    echo "Linux LibreOffice:"
    echo "Option 1 - Download AppImage:"
    echo "   https://www.libreoffice.org/download/appimage/"
    echo ""
    echo "Option 2 - Use system package manager:"
    echo "   sudo apt install libreoffice-writer"  # Debian/Ubuntu
    echo "   sudo dnf install libreoffice-writer"   # Fedora
    echo ""
    echo "If using system LibreOffice, the app will use it directly."
    echo ""
fi

echo "After downloading, place the files in:"
echo "   libreoffice/libreoffice-win.zip"
echo "   libreoffice/libreoffice-linux.zip"
echo ""

# Check current status
echo "Current status:"
if [ -f "libreoffice/libreoffice-win.zip" ]; then
    echo "  ✓ Windows bundle found ($(du -h libreoffice/libreoffice-win.zip | cut -f1))"
else
    echo "  ✗ Windows bundle missing"
fi

if [ -f "libreoffice/libreoffice-linux.zip" ]; then
    echo "  ✓ Linux bundle found ($(du -h libreoffice/libreoffice-linux.zip | cut -f1))"
else
    echo "  ✗ Linux bundle missing (optional if using system LibreOffice)"
fi

echo ""
echo "Ready to build when bundles are in place!"
