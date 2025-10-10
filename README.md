# NovaAudioVideoConverter

🚀 A modern, high-performance audio and video converter written in **Rust** - **Linux-first**

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux-blue.svg)

## ✨ Features

- 🎵 **Multiple Audio Formats**: MP3, FLAC, WAV, OGG, M4A, AAC, WMA
- 🎬 **Multiple Video Formats**: MP4, AVI, MKV, WEBM, MOV, FLV, WMV
- ⚡ **High Performance**: Multi-threaded parallel processing for batch conversions
- 🎬 **VHS Rescue**: One-click preset for analog capture cleanup
- 🎤 **Linux Capture**: Record from V4L2 video and ALSA audio devices
- 🔧 **Audio Enhancement**: Denoise, normalize, compress, and remove hum
- 🎨 **Video Enhancement**: Deinterlace, stabilize, denoise, sharpen, and color adjust
- 🧹 **Media Cleaning**: Remove metadata and optimize file sizes
- 📊 **File Information**: Display detailed media file information
- 🎨 **Modern CLI**: Beautiful colored output with progress indicators

## 🛠️ Installation

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/linuxiano85/NovaAudioVideoConverter.git
cd NovaAudioVideoConverter

# Install dependencies (FFmpeg, v4l-utils, alsa-utils)
./scripts/install-deps.sh

# Build with Cargo
cargo build --release

# Install (optional)
sudo cp target/release/nova-converter /usr/local/bin/
```

### System Requirements

- **Linux**: Ubuntu 20.04+, Fedora 35+, Arch Linux, or compatible distribution
- **FFmpeg**: Required for media conversion and capture
  - Ubuntu/Debian: `sudo apt install ffmpeg v4l-utils alsa-utils`
  - Fedora: `sudo dnf install ffmpeg v4l-utils alsa-utils`
  - Arch: `sudo pacman -S ffmpeg v4l-utils alsa-utils`

### Device Permissions

For video capture from V4L2 devices:
```bash
sudo usermod -aG video $USER
```

For audio capture from ALSA devices:
```bash
sudo usermod -aG audio $USER
```

**Note**: You need to log out and back in for group changes to take effect.

## 📖 Usage

### Convert Files

Convert a single file:
```bash
nova-converter convert --input video.avi --format mp4 --quality 256k
```

Convert all files in a directory:
```bash
nova-converter convert --input ./videos --format mp4 --recursive --jobs 8
```

Convert with custom codec:
```bash
nova-converter convert --input video.mkv --format mp4 --codec libx265 --quality 192k
```

### Audio Enhancement

Enhance audio with denoise, normalization, and compression:
```bash
nova-converter enhance-audio \
  --input old-recording.wav \
  --output enhanced.wav \
  --denoise \
  --normalize \
  --highpass 80 \
  --notch 60  # Remove 60 Hz hum (use 50 for EU/other regions)
```

Options:
- `--denoise`: Apply FFT-based denoising
- `--normalize`: Loudness normalization (EBU R128)
- `--highpass <freq>`: Remove low-frequency rumble
- `--lowpass <freq>`: Remove high-frequency noise
- `--notch <50|60>`: Remove AC hum at 50 or 60 Hz
- `--compressor`: Dynamic range compression
- `--gate`: Noise gate

### Video Enhancement

Enhance video with deinterlacing, stabilization, and cleanup:
```bash
nova-converter enhance-video \
  --input old-video.avi \
  --output enhanced.mp4 \
  --deinterlace \
  --stabilize \
  --denoise hqdn3d \
  --sharpen \
  --color
```

Options:
- `--deinterlace`: Deinterlace using bwdif (Bob Weaver)
- `--stabilize`: Stabilize shaky footage (deshake)
- `--denoise <type>`: Denoise (none, hqdn3d, nlmeans)
- `--sharpen`: Sharpen using unsharp mask
- `--color`: Adjust brightness and saturation
- `--width` / `--height`: Scale video
- `--aspect <ratio>`: Set display aspect ratio (e.g., 4:3, 16:9)

### VHS Rescue

One-click preset for analog VHS capture cleanup:
```bash
nova-converter vhs-rescue \
  --input vhs-capture.avi \
  --output restored.mp4 \
  --notch 60  # Use 50 for EU/other regions
```

This applies:
- **Video**: Deinterlace, stabilize, denoise (hqdn3d), sharpen, color adjust, 4:3 aspect
- **Audio**: High-pass (80 Hz), low-pass (15 kHz), denoise, hum removal, gate, compressor, loudness normalization

Perfect for:
- VHS tapes
- Hi8/Video8
- Betamax
- Other analog video sources

### Device Capture

List available capture devices:
```bash
nova-converter capture-list
```

Capture from V4L2 video and ALSA audio:
```bash
nova-converter capture \
  --output recording.mp4 \
  --video-device /dev/video0 \
  --audio-device hw:1,0 \
  --deinterlace \
  --width 720 \
  --height 480 \
  --fps 30
```

Archival capture (near-lossless):
```bash
nova-converter capture \
  --output archive.mkv \
  --format mkv \
  --video-device /dev/video0 \
  --audio-device hw:1,0 \
  --archival \
  --deinterlace
```

Capture options:
- `--format <mp4|mkv>`: Output container format
- `--deinterlace`: Apply deinterlacing during capture
- `--stabilize`: Apply stabilization during capture
- `--denoise <type>`: Apply denoising (hqdn3d or nlmeans)
- `--vbitrate <rate>`: Video bitrate (e.g., 5M)
- `--crf <value>`: CRF quality (18-28, lower = better)
- `--abitrate <rate>`: Audio bitrate (e.g., 192k)
- `--archival`: Near-lossless archival mode (MKV + PCM audio)

**Common VHS Capture Settings:**
```bash
# Standard capture (good quality, smaller file)
nova-converter capture \
  --output vhs-tape-01.mp4 \
  --video-device /dev/video0 \
  --audio-device hw:1,0 \
  --deinterlace \
  --width 720 \
  --height 480 \
  --fps 30 \
  --crf 18

# Archival capture (best quality, large file)
nova-converter capture \
  --output vhs-tape-01-archive.mkv \
  --format mkv \
  --video-device /dev/video0 \
  --audio-device hw:1,0 \
  --deinterlace \
  --archival
```

### Clean Media Files

Remove metadata and optimize:
```bash
nova-converter clean --input video.mp4 --metadata --optimize
```

Clean all files in directory:
```bash
nova-converter clean --input ./videos --recursive --metadata
```

### Get File Information

```bash
nova-converter info --input video.mp4
```

### List Supported Formats

```bash
nova-converter formats
```

## 🎯 Common Workflows

### Digitizing VHS Tapes

1. **List devices**:
   ```bash
   nova-converter capture-list
   ```

2. **Test capture** (short recording):
   ```bash
   nova-converter capture \
     --output test.mp4 \
     --video-device /dev/video0 \
     --audio-device hw:1,0 \
     --deinterlace \
     --width 720 --height 480 --fps 30
   ```

3. **Full tape capture**:
   ```bash
   # Press Ctrl+C to stop when tape ends
   nova-converter capture \
     --output tape-01-raw.mkv \
     --format mkv \
     --video-device /dev/video0 \
     --audio-device hw:1,0 \
     --deinterlace \
     --archival
   ```

4. **Apply VHS Rescue**:
   ```bash
   nova-converter vhs-rescue \
     --input tape-01-raw.mkv \
     --output tape-01-restored.mp4 \
     --notch 60
   ```

### Cleaning Up Old Videos

```bash
# Enhance video quality
nova-converter enhance-video \
  --input old-home-movie.avi \
  --output enhanced-movie.mp4 \
  --deinterlace \
  --denoise hqdn3d \
  --sharpen \
  --color

# Enhance audio separately
nova-converter enhance-audio \
  --input old-audio.wav \
  --output clean-audio.wav \
  --denoise \
  --normalize \
  --highpass 80 \
  --notch 60
```

## 🔧 Troubleshooting

### No video/audio devices found

Make sure you're in the `video` and `audio` groups:
```bash
groups  # Check your groups
sudo usermod -aG video,audio $USER
# Log out and back in
```

### FFmpeg not found

Install FFmpeg:
```bash
# Ubuntu/Debian
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch
sudo pacman -S ffmpeg
```

### Capture device permissions

Check device permissions:
```bash
ls -l /dev/video*
ls -l /dev/snd/*
```

### USB capture device not detected

```bash
# List USB devices
lsusb

# Check video devices
v4l2-ctl --list-devices

# Check audio devices
arecord -l
```

## 📦 Building Packages

See [BUILDING.md](BUILDING.md) for detailed instructions on building packages.

## 🏗️ Architecture

Nova Audio/Video Converter is built with modern Rust technologies:

- **clap**: Command-line argument parsing with derive macros
- **tokio**: Asynchronous runtime for high-performance I/O
- **rayon**: Data parallelism for batch processing
- **anyhow**: Flexible error handling
- **which**: Executable detection
- **regex**: Pattern matching for device parsing
- **colored**: Beautiful terminal output
- **serde**: Serialization for configuration and metadata

The application leverages **FFmpeg** for the actual media conversion, providing a user-friendly interface with advanced features like parallel processing, quality presets, batch operations, and Linux V4L2/ALSA capture.

### Modules

- **ffmpeg**: Core FFmpeg interaction layer
- **audio**: Audio enhancement (denoise, normalize, compress)
- **video**: Video enhancement (deinterlace, stabilize, denoise, sharpen)
- **capture**: V4L2 video and ALSA audio capture
- **ai**: AI-based enhancement (placeholder for future features)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [FFmpeg](https://ffmpeg.org/)
- Linux V4L2 and ALSA support
- Inspired by analog video preservation community
