#!/bin/bash
set -e

# Build RPM package for Nova Audio/Video Converter

echo "Building Nova Audio/Video Converter RPM package..."

# Install dependencies if needed
if ! command -v rpmbuild &> /dev/null; then
    echo "Installing build dependencies..."
    sudo dnf install -y rpm-build rpmdevtools || sudo yum install -y rpm-build rpmdevtools
fi

# Setup RPM build environment
rpmdev-setuptree || mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create source tarball
VERSION="0.1.0"
TARBALL="nova-audio-video-converter-${VERSION}.tar.gz"

cd ..
tar --exclude='.git' --exclude='target' --exclude='AppDir' \
    -czf ~/rpmbuild/SOURCES/${TARBALL} NovaAudioVideoConverter
cd -

# Copy spec file
cp nova-audio-video-converter.spec ~/rpmbuild/SPECS/

# Build RPM
rpmbuild -ba ~/rpmbuild/SPECS/nova-audio-video-converter.spec

echo "RPM package built successfully!"
echo "Package location: ~/rpmbuild/RPMS/"
ls -lh ~/rpmbuild/RPMS/*/nova-audio-video-converter-*.rpm
