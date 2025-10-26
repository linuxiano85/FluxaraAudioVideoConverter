# Building and Packaging Guide

This document provides detailed instructions for building Fluxara AVC from source and creating distribution packages.

## Prerequisites

### General Requirements

- **Rust**: 1.70 or newer
- **Cargo**: Rust's package manager
- **FFmpeg**: Required for media conversion

### Platform-Specific Requirements

#### Debian/Ubuntu
```bash
sudo apt install build-essential cargo rustc ffmpeg
```

#### Fedora
```bash
sudo dnf install gcc cargo rust ffmpeg
```

#### Arch Linux
```bash
sudo pacman -S base-devel rust cargo ffmpeg
```

## Building from Source

### Debug Build
```bash
cargo build
```

The binary will be in `target/debug/fluxara-avc`

### Release Build (Optimized)
```bash
cargo build --release
```

The optimized binary will be in `target/release/fluxara-avc`

### Testing the Application
```bash
# Show help
./target/release/fluxara-avc --help

# List supported formats
./target/release/fluxara-avc formats

# Convert a file (requires FFmpeg)
./target/release/fluxara-avc convert --input input.mp3 --format wav
```

## Creating Distribution Packages

### Flatpak

Flatpak provides a universal package format for Linux.

#### Prerequisites
```bash
# Debian/Ubuntu
sudo apt install flatpak flatpak-builder

# Fedora
sudo dnf install flatpak flatpak-builder

# Add Flathub repository
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
```

#### Build Steps
1. Install required runtimes:
```bash
flatpak install flathub org.freedesktop.Platform//23.08
flatpak install flathub org.freedesktop.Sdk//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
```

2. Generate cargo sources (required for offline builds):
```bash
# Install Python dependencies
pip3 install aiohttp toml

# Download flatpak-cargo-generator
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py

# Generate cargo sources
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```

3. Build the Flatpak:
```bash
flatpak-builder --force-clean --repo=repo build-dir com.fluxara.AVC.yml
```

4. Install locally:
```bash
flatpak-builder --user --install --force-clean build-dir com.fluxara.AVC.yml
```

5. Run the application:
```bash
flatpak run com.fluxara.AVC
```

### AppImage

AppImage provides a portable application format that works on most Linux distributions.

#### Prerequisites
```bash
# No specific prerequisites - the build script downloads appimagetool automatically
```

#### Build Steps
```bash
./build-appimage.sh
```

This creates `fluxara-avc-x86_64.AppImage`

#### Usage
```bash
chmod +x fluxara-avc-x86_64.AppImage
./fluxara-avc-x86_64.AppImage --help
```

### DEB Package

DEB packages are used by Debian, Ubuntu, Linux Mint, and other Debian-based distributions.

#### Prerequisites
```bash
sudo apt install build-essential debhelper devscripts cargo rustc
```

#### Build Steps
```bash
./build-deb.sh
```

The package will be created in the parent directory as `fluxara-avc_0.1.0-1_amd64.deb`

#### Installation
```bash
sudo dpkg -i ../fluxara-avc_0.1.0-1_amd64.deb
sudo apt-get install -f  # Install dependencies if needed
```

### RPM Package

RPM packages are used by Fedora, RHEL, CentOS, openSUSE, and other RPM-based distributions.

#### Prerequisites
```bash
# Fedora/RHEL/CentOS
sudo dnf install rpm-build rpmdevtools cargo rust

# openSUSE
sudo zypper install rpm-build rpmdevtools cargo rust
```

#### Build Steps
```bash
./build-rpm.sh
```

The package will be created in `~/rpmbuild/RPMS/x86_64/fluxara-avc-0.1.0-1.x86_64.rpm`

#### Installation
```bash
# Fedora/RHEL/CentOS
sudo dnf install ~/rpmbuild/RPMS/x86_64/fluxara-avc-0.1.0-1.x86_64.rpm

# openSUSE
sudo zypper install ~/rpmbuild/RPMS/x86_64/fluxara-avc-0.1.0-1.x86_64.rpm
```

## Advanced Build Options

### Cross-Compilation

To build for different architectures:

```bash
# Install cross-compilation tools
cargo install cross

# Build for ARM64
cross build --target aarch64-unknown-linux-gnu --release

# Build for ARMv7
cross build --target armv7-unknown-linux-gnueabihf --release
```

### Custom Optimization Flags

The release profile in `Cargo.toml` already includes aggressive optimizations:
- LTO (Link Time Optimization)
- Strip symbols
- Maximum optimization level

To customize further, edit `Cargo.toml`:
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true            # Link-time optimization
strip = true          # Strip symbols
codegen-units = 1     # Better optimization
panic = "abort"       # Smaller binary size
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build release
        run: cargo build --release
      - name: Upload binary
        uses: actions/upload-artifact@v2
        with:
          name: fluxara-avc
          path: target/release/fluxara-avc
```

## Troubleshooting

### Common Issues

#### FFmpeg not found
```
Error: FFmpeg is not installed
```
**Solution**: Install FFmpeg using your package manager.

#### Cargo version too old
```
Error: package requires rustc 1.70 or newer
```
**Solution**: Update Rust using `rustup update stable`

#### Missing dependencies during DEB build
```
Error: unmet dependencies
```
**Solution**: Run `sudo apt-get install -f` to install missing dependencies.

#### RPM build fails with permissions error
```
Error: cannot create directory ~/rpmbuild
```
**Solution**: Run `rpmdev-setuptree` to create the RPM build environment.

## Performance Benchmarks

On a typical system with 8 cores:
- Single file conversion: ~3-5 seconds for a 5-minute MP3
- Batch conversion (100 files): ~2-3 minutes with 8 parallel jobs
- Memory usage: ~50-100 MB depending on workload

## Security Considerations

- Fluxara AVC executes FFmpeg as a subprocess
- File permissions are preserved during conversion
- No network access required for core functionality
- All operations are performed locally

## Future Improvements

Planned features:
- GUI interface using GTK4/Libadwaita (see docs/ui/GUI-concept.md)
- Live capture from V4L2/ALSA devices
- VHS Rescue preset with one-command restoration
- Batch workflow presets
- Advanced analog restoration filters

## Support

For issues, feature requests, or contributions, please visit:
https://github.com/linuxiano85/FluxaraAudioVideoConverter
