# NovaAudioVideoConverter

🚀 A modern, high-performance audio and video converter written in **Rust**

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- 🎵 **Multiple Audio Formats**: MP3, FLAC, WAV, OGG, M4A, AAC, WMA
- 🎬 **Multiple Video Formats**: MP4, AVI, MKV, WEBM, MOV, FLV, WMV
- ⚡ **High Performance**: Multi-threaded parallel processing for batch conversions
- 🧹 **Media Cleaning**: Remove metadata and optimize file sizes
- 📊 **File Information**: Display detailed media file information
- 🎨 **Modern CLI**: Beautiful colored output with progress indicators
- 🔧 **Flexible**: Customizable quality settings and codec options

## 🛠️ Installation

### Binary Releases

Download pre-built packages from the releases page:

- **Flatpak**: Universal Linux package
- **AppImage**: Portable, no installation required
- **DEB**: For Debian, Ubuntu, and derivatives
- **RPM**: For Fedora, RHEL, CentOS, and derivatives

### From Source

```bash
# Clone the repository
git clone https://github.com/linuxiano85/NovaAudioVideoConverter.git
cd NovaAudioVideoConverter

# Build with Cargo
cargo build --release

# Install (optional)
sudo cp target/release/nova-converter /usr/local/bin/
```

### System Requirements

- **FFmpeg**: Required for media conversion
  - Ubuntu/Debian: `sudo apt install ffmpeg`
  - Fedora: `sudo dnf install ffmpeg`
  - Arch: `sudo pacman -S ffmpeg`

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

## 📦 Building Packages

### Flatpak

```bash
# Install flatpak-builder
sudo apt install flatpak-builder  # Debian/Ubuntu
sudo dnf install flatpak-builder  # Fedora

# Generate cargo sources
python3 -m pip install aiohttp toml
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json

# Build flatpak
flatpak-builder --force-clean build-dir com.nova.AudioVideoConverter.yml
```

### AppImage

```bash
./build-appimage.sh
```

This will create `nova-audio-video-converter-x86_64.AppImage`

### DEB Package

```bash
./build-deb.sh
```

The package will be created in the parent directory.

### RPM Package

```bash
./build-rpm.sh
```

The package will be created in `~/rpmbuild/RPMS/`.

## 🏗️ Architecture

Nova Audio/Video Converter is built with modern Rust technologies:

- **clap**: Command-line argument parsing with derive macros
- **tokio**: Asynchronous runtime for high-performance I/O
- **rayon**: Data parallelism for batch processing
- **anyhow**: Flexible error handling
- **indicatif**: Progress bars and status indicators
- **colored**: Beautiful terminal output
- **serde**: Serialization for configuration and metadata

The application leverages **FFmpeg** for the actual media conversion, providing a user-friendly interface with advanced features like parallel processing, quality presets, and batch operations.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [FFmpeg](https://ffmpeg.org/)
- Icons and design inspired by modern Linux applications
