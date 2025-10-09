#!/bin/bash
set -e

# Build DEB package for Nova Audio/Video Converter

echo "Building Nova Audio/Video Converter DEB package..."

# Install dependencies if needed
if ! command -v dpkg-buildpackage &> /dev/null; then
    echo "Installing build dependencies..."
    sudo apt-get update
    sudo apt-get install -y build-essential debhelper devscripts
fi

# Build the package
dpkg-buildpackage -us -uc -b

echo "DEB package built successfully!"
echo "Package location: ../nova-audio-video-converter_*.deb"
