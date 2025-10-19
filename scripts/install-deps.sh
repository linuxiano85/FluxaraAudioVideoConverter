#!/bin/bash
# Install dependencies for NovaAudioVideoConverter on Linux

set -e

echo "Installing NovaAudioVideoConverter dependencies..."

# Detect distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
else
    echo "Cannot detect Linux distribution"
    exit 1
fi

echo "Detected distribution: $OS"

case $OS in
    ubuntu|debian|pop|linuxmint)
        echo "Installing dependencies for Debian/Ubuntu..."
        sudo apt-get update
        sudo apt-get install -y \
            ffmpeg \
            v4l-utils \
            alsa-utils \
            build-essential \
            pkg-config
        ;;
    
    fedora|rhel|centos)
        echo "Installing dependencies for Fedora/RHEL/CentOS..."
        sudo dnf install -y \
            ffmpeg \
            v4l-utils \
            alsa-utils \
            gcc \
            make \
            pkg-config
        ;;
    
    arch|manjaro)
        echo "Installing dependencies for Arch Linux..."
        sudo pacman -S --needed --noconfirm \
            ffmpeg \
            v4l-utils \
            alsa-utils \
            base-devel
        ;;
    
    opensuse*)
        echo "Installing dependencies for openSUSE..."
        sudo zypper install -y \
            ffmpeg \
            v4l-utils \
            alsa-utils \
            gcc \
            make \
            pkg-config
        ;;
    
    *)
        echo "Unsupported distribution: $OS"
        echo "Please install the following manually:"
        echo "  - FFmpeg"
        echo "  - v4l-utils (for video capture)"
        echo "  - alsa-utils (for audio capture)"
        echo "  - Build tools (gcc, make, pkg-config)"
        exit 1
        ;;
esac

echo ""
echo "Verifying installations..."
echo "FFmpeg version:"
ffmpeg -version | head -n 1

echo ""
echo "FFprobe version:"
ffprobe -version | head -n 1

echo ""
echo "V4L2 tools:"
v4l2-ctl --version || echo "v4l2-ctl not available (optional)"

echo ""
echo "ALSA tools:"
arecord --version | head -n 1 || echo "arecord not available (optional)"

echo ""
echo "✓ Dependencies installed successfully!"
echo ""
echo "To give your user access to video capture devices:"
echo "  sudo usermod -aG video \$USER"
echo ""
echo "To give your user access to audio capture devices:"
echo "  sudo usermod -aG audio \$USER"
echo ""
echo "You may need to log out and back in for group changes to take effect."
