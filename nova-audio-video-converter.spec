Name:           nova-audio-video-converter
Version:        0.1.0
Release:        1%{?dist}
Summary:        Modern high-performance audio and video converter

License:        MIT
URL:            https://github.com/linuxiano85/NovaAudioVideoConverter
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.70
BuildRequires:  gcc
Requires:       ffmpeg

%description
Nova Audio/Video Converter is a modern, high-performance command-line
tool for converting audio and video files between various formats.

Features:
* Support for multiple audio formats (MP3, FLAC, WAV, OGG, etc.)
* Support for multiple video formats (MP4, AVI, MKV, WEBM, etc.)
* Parallel processing for batch conversions
* Media file cleaning and optimization
* File information display

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps

install -m 755 target/release/nova-converter %{buildroot}%{_bindir}/nova-converter
install -m 644 com.nova.AudioVideoConverter.desktop %{buildroot}%{_datadir}/applications/
install -m 644 com.nova.AudioVideoConverter.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/

%files
%{_bindir}/nova-converter
%{_datadir}/applications/com.nova.AudioVideoConverter.desktop
%{_datadir}/icons/hicolor/scalable/apps/com.nova.AudioVideoConverter.svg

%changelog
* Thu Oct 09 2025 Nova Team <nova@example.com> - 0.1.0-1
- Initial release
- Modern Rust-based audio/video converter
- Support for multiple formats
- Parallel processing capabilities
- Media file cleaning and optimization
