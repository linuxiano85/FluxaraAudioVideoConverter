# Fluxara AVC

🎬 **Linux-first analog restoration & conversion with FFmpeg**

> **Note**: This project was previously named **NovaAudioVideoConverter**. The repository will be renamed to `fluxara-avc` by the owner after this rebrand PR is merged. GitHub will create automatic redirects from the old URL.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Linux](https://img.shields.io/badge/platform-Linux-blue.svg)

## What is Fluxara AVC?

Fluxara AVC is a powerful command-line tool for converting and restoring analog media on Linux. Built on FFmpeg, it specializes in:

- **VHS tape digitization** with noise reduction and deinterlacing
- **Audio cassette restoration** with cleanup filters
- **Batch conversion** with parallel processing
- **Flexible media conversion** between dozens of formats

Whether you're digitizing old family videos or managing a media library, Fluxara AVC makes it simple and fast.

## 🚀 Quick Start

### Installation from Source

```bash
# Clone the repository
git clone https://github.com/linuxiano85/NovaAudioVideoConverter.git
cd NovaAudioVideoConverter

# Build with Cargo
cargo build --release

# Install to system (optional)
sudo cp target/release/fluxara-avc /usr/local/bin/
```

### System Requirements

**Required:**
- **FFmpeg**: `sudo apt install ffmpeg` (Ubuntu/Debian) or `sudo dnf install ffmpeg` (Fedora)

**Recommended for capture:**
- **v4l-utils**: Video4Linux utilities for video capture
- **alsa-utils**: ALSA utilities for audio capture

### Quick Examples

```bash
# Convert a video file
fluxara-avc convert --input video.avi --format mp4

# VHS tape rescue with deinterlacing and noise reduction
fluxara-avc convert --input vhs-tape.mkv --format mp4 --codec libx264 --quality 192k

# Batch convert directory (8 parallel jobs)
fluxara-avc convert --input ./videos --format mp4 --recursive --jobs 8

# Get file information
fluxara-avc info --input video.mp4

# List all supported formats
fluxara-avc formats
```

## 📖 Commands & Usage

### Convert

Convert audio/video files between formats with full control over quality and codecs.

```bash
fluxara-avc convert [OPTIONS]

Options:
  -i, --input <PATH>       Input file or directory
  -f, --format <FORMAT>    Output format (mp3, mp4, mkv, etc.)
  -o, --output <DIR>       Output directory (default: current directory)
  -r, --recursive          Process directories recursively
  -q, --quality <QUALITY>  Audio quality: 64k, 128k, 192k, 256k, 320k (default: 192k)
  -c, --codec <CODEC>      Video codec: libx264, libx265, libvpx, etc.
  -j, --jobs <N>           Parallel jobs (default: 4)
```

**Examples:**

```bash
# Basic conversion
fluxara-avc convert -i input.avi -f mp4

# High-quality audio
fluxara-avc convert -i song.wav -f mp3 -q 320k

# Batch with custom codec
fluxara-avc convert -i ./videos -f mp4 -c libx265 -r -j 8
```

### Clean

Remove metadata and optimize media files without re-encoding.

```bash
fluxara-avc clean [OPTIONS]

Options:
  -i, --input <PATH>    Input file or directory
  -m, --metadata        Remove all metadata
  -o, --optimize        Optimize file size
  -r, --recursive       Process recursively
```

**Examples:**

```bash
# Remove metadata from a video
fluxara-avc clean -i video.mp4 --metadata

# Clean all files in directory
fluxara-avc clean -i ./videos -m -o -r
```

### Info

Display detailed information about media files.

```bash
fluxara-avc info --input <FILE>
```

**Example output:**

```
ℹ File: video.mp4

• Duration: 125.50 seconds
• Size: 45 MB
• Bit rate: 2850 kbps

• Streams:
  → Stream #0: video (h264)
    Resolution: 1920x1080
  → Stream #1: audio (aac)
```

### Formats

List all supported audio and video formats.

```bash
fluxara-avc formats
```

## 🎞️ VHS Rescue Preset

Fluxara AVC is designed with analog media restoration in mind. For VHS tapes, use these recommended settings:

```bash
# Standard VHS tape (NTSC/PAL)
fluxara-avc convert \
  --input vhs-capture.mkv \
  --format mp4 \
  --codec libx264 \
  --quality 192k
```

**Manual FFmpeg command for advanced VHS rescue:**

```bash
ffmpeg -i vhs-capture.mkv \
  -vf "yadif=1,hqdn3d=4:3:6:4.5,deshake" \
  -c:v libx264 -preset slow -crf 20 \
  -c:a aac -b:a 192k \
  output.mp4
```

Filters explained:
- `yadif=1`: Deinterlace (VHS is interlaced)
- `hqdn3d`: High-quality denoise (removes VHS static)
- `deshake`: Stabilize shaky footage

> **Future Enhancement**: A dedicated `fluxara-avc rescue` command with automatic VHS preset is planned!

## 📹 Capture from Analog Devices (V4L2/ALSA)

### List Available Devices

```bash
# Video devices
v4l2-ctl --list-devices

# Audio devices
arecord -l
```

### Capture Example

Use FFmpeg directly to capture from Video4Linux2 and ALSA:

```bash
# Capture from /dev/video0 with audio from hw:1,0
ffmpeg -f v4l2 -input_format uyvy422 -video_size 720x480 -i /dev/video0 \
       -f alsa -i hw:1,0 \
       -c:v ffv1 -c:a pcm_s16le \
       capture-$(date +%Y%m%d-%H%M%S).mkv
```

**Recommended capture settings:**
- **Video codec**: FFV1 (lossless) or HuffYUV
- **Audio codec**: PCM (uncompressed) or FLAC
- **Container**: Matroska (MKV)

After capture, use Fluxara AVC to convert to compressed formats:

```bash
fluxara-avc convert -i capture.mkv -f mp4 -c libx264 -q 192k
```

> **Future Enhancement**: A dedicated `fluxara-avc capture` command with GUI preview is in development!

## ✨ Features

- 🎵 **Audio Formats**: MP3, FLAC, WAV, OGG, M4A, AAC, WMA
- 🎬 **Video Formats**: MP4, AVI, MKV, WEBM, MOV, FLV, WMV
- ⚡ **Parallel Processing**: Multi-threaded batch conversions
- 🧹 **Media Cleaning**: Remove metadata and optimize files
- 📊 **File Information**: Detailed media inspection
- 🎨 **Modern CLI**: Colored output with clear status indicators
- 🔧 **Flexible**: Custom quality and codec settings
- 🐧 **Linux-First**: Native performance on Linux systems

## 📦 Building Packages

See [BUILDING.md](BUILDING.md) for detailed instructions on creating:

- **Flatpak**: Universal Linux package
- **AppImage**: Portable, no installation required
- **DEB**: For Debian, Ubuntu, and derivatives
- **RPM**: For Fedora, RHEL, CentOS, and derivatives

Quick package builds:

```bash
# DEB package
./build-deb.sh

# RPM package
./build-rpm.sh

# AppImage
./build-appimage.sh
```

## 🏗️ Architecture

Fluxara AVC is built with modern Rust:

- **clap**: CLI argument parsing with derive macros
- **tokio**: Async runtime for I/O
- **rayon**: Parallel batch processing
- **anyhow**: Error handling
- **indicatif**: Progress indicators
- **colored**: Terminal colors
- **serde**: JSON metadata parsing

**Backend**: FFmpeg handles all encoding/decoding work

## 🎨 Branding & Identity

Fluxara uses a modern indigo-to-violet gradient representing the transformation from analog to digital. See [docs/brand/IDENTITY.md](docs/brand/IDENTITY.md) for:

- Color palette and typography
- Icon design guidelines
- Linux desktop integration standards

**Icon assets**: Available in [icons/svg/](icons/svg/)

## 🔮 Future Roadmap

- **GUI Application**: GTK4/Libadwaita interface (see [docs/ui/GUI-concept.md](docs/ui/GUI-concept.md))
- **Live Capture**: Real-time capture from V4L2/ALSA devices
- **VHS Rescue Command**: One-command VHS restoration with presets
- **Enhanced Filters**: Specialized analog media restoration
- **Batch Presets**: Save and load workflow configurations

## 🤝 Contributing

Contributions are welcome! Areas where help is needed:

- Testing on different Linux distributions
- Additional format presets and filters
- Documentation improvements
- GUI development (GTK4/Rust)

Please feel free to open issues or submit pull requests.

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [FFmpeg](https://ffmpeg.org/)
- Inspired by the Linux multimedia community
- Icons follow [GNOME HIG](https://developer.gnome.org/hig/)

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Brand Identity**: [docs/brand/IDENTITY.md](docs/brand/IDENTITY.md)
- **Name Decision**: [docs/brand/NAME-DECISION.md](docs/brand/NAME-DECISION.md)
- **GUI Concept**: [docs/ui/GUI-concept.md](docs/ui/GUI-concept.md)
- **Building Guide**: [BUILDING.md](BUILDING.md)

---

**Made with ❤️ for the Linux community**

