#!/bin/bash
# Installation script for NovaAudioVideoConverter

echo "======================================"
echo "NovaAudioVideoConverter - Installation"
echo "======================================"
echo ""

# Detect Linux distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo "Cannot detect Linux distribution"
    exit 1
fi

echo "Detected distribution: $DISTRO"
echo ""

# Install system dependencies based on distribution
echo "Installing system dependencies..."

case $DISTRO in
    ubuntu|debian)
        sudo apt update
        sudo apt install -y python3 python3-pip python3-pyqt5 ffmpeg handbrake-cli libdvdread-dev libdvdcss2
        ;;
    fedora)
        sudo dnf install -y python3 python3-pip python3-qt5 ffmpeg handbrake-cli libdvdread-devel libdvdcss
        ;;
    arch|manjaro)
        sudo pacman -S --noconfirm python python-pip python-pyqt5 ffmpeg handbrake-cli libdvdread libdvdcss
        ;;
    opensuse*)
        sudo zypper install -y python3 python3-pip python3-qt5 ffmpeg handbrake-cli libdvdread-devel libdvdcss2
        ;;
    *)
        echo "Unsupported distribution. Please install dependencies manually:"
        echo "  - Python 3.7+"
        echo "  - PyQt5"
        echo "  - FFmpeg"
        echo "  - HandBrake CLI"
        echo "  - libdvdread and libdvdcss"
        exit 1
        ;;
esac

if [ $? -ne 0 ]; then
    echo "Error installing system dependencies"
    exit 1
fi

echo ""
echo "Installing Python dependencies..."
pip3 install --user -r requirements.txt

if [ $? -ne 0 ]; then
    echo "Error installing Python dependencies"
    exit 1
fi

echo ""
echo "Creating desktop entry..."

# Create desktop entry
DESKTOP_FILE="$HOME/.local/share/applications/novaaudiovideoconverter.desktop"
mkdir -p "$HOME/.local/share/applications"

cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Type=Application
Name=NovaAudioVideoConverter
Comment=Professional Audio and Video Converter with AI Enhancement
Exec=python3 $(pwd)/main.py
Icon=multimedia-video-player
Terminal=false
Categories=AudioVideo;Video;AudioVideoEditing;
Keywords=video;audio;converter;dvd;subtitle;
EOF

chmod +x "$DESKTOP_FILE"

echo ""
echo "======================================"
echo "Installation completed successfully!"
echo "======================================"
echo ""
echo "You can now run NovaAudioVideoConverter:"
echo "  - From terminal: python3 main.py"
echo "  - From application menu: Search for 'NovaAudioVideoConverter'"
echo ""
echo "For more information, see README.md"
echo ""
