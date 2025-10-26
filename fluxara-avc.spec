Name:           fluxara-avc
Version:        0.1.0
Release:        1%{?dist}
Summary:        Linux-first analog restoration & conversion with FFmpeg

License:        MIT
URL:            https://github.com/linuxiano85/FluxaraAudioVideoConverter
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.70
BuildRequires:  gcc
Requires:       ffmpeg

%description
Fluxara AVC is a Linux-first analog restoration and conversion toolkit
built on FFmpeg, specializing in VHS rescue and vintage media digitization.

Features:
* VHS tape digitization with noise reduction and deinterlacing
* Audio cassette restoration with cleanup filters
* Batch conversion with parallel processing
* Flexible media conversion between dozens of formats

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps

install -m 755 target/release/fluxara-avc %{buildroot}%{_bindir}/fluxara-avc
install -m 644 com.fluxara.AVC.desktop %{buildroot}%{_datadir}/applications/
install -m 644 com.fluxara.AVC.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/

%files
%{_bindir}/fluxara-avc
%{_datadir}/applications/com.fluxara.AVC.desktop
%{_datadir}/icons/hicolor/scalable/apps/com.fluxara.AVC.svg

%changelog
* Thu Oct 10 2025 Fluxara Team <fluxara@example.com> - 0.1.0-1
- Initial release of Fluxara AVC
- Rebrand from NovaAudioVideoConverter
- Linux-first analog restoration toolkit
- VHS rescue and vintage media digitization
- Built on FFmpeg with Rust
