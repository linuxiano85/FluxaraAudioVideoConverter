#!/bin/bash
set -e

# Build AppImage for Nova Audio/Video Converter

echo "Building Nova Audio/Video Converter AppImage..."

# Build the application
cargo build --release

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/scalable/apps

# Copy files
cp target/release/nova-converter AppDir/usr/bin/
cp com.nova.AudioVideoConverter.desktop AppDir/
cp com.nova.AudioVideoConverter.desktop AppDir/usr/share/applications/
cp com.nova.AudioVideoConverter.svg AppDir/usr/share/icons/hicolor/scalable/apps/
cp com.nova.AudioVideoConverter.svg AppDir/

# Create AppRun script
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"

# Check for ffmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo "Warning: FFmpeg not found. Please install FFmpeg to use this application."
    echo "On Ubuntu/Debian: sudo apt install ffmpeg"
    echo "On Fedora: sudo dnf install ffmpeg"
    echo "On Arch: sudo pacman -S ffmpeg"
fi

exec "${HERE}/usr/bin/nova-converter" "$@"
EOF

chmod +x AppDir/AppRun

# Download appimagetool if not present
if [ ! -f appimagetool-x86_64.AppImage ]; then
    echo "Downloading appimagetool..."
    wget -q https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x appimagetool-x86_64.AppImage
fi

# Build AppImage
ARCH=x86_64 ./appimagetool-x86_64.AppImage AppDir nova-audio-video-converter-x86_64.AppImage

echo "AppImage created: nova-audio-video-converter-x86_64.AppImage"
