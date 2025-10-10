#!/bin/bash
set -e

# Build RPM package for Fluxara AVC

echo "Building Fluxara AVC RPM package..."

# Install dependencies if needed
if ! command -v rpmbuild &> /dev/null; then
    echo "Installing build dependencies..."
    sudo dnf install -y rpm-build rpmdevtools || sudo yum install -y rpm-build rpmdevtools
fi

# Setup RPM build environment
rpmdev-setuptree || mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create source tarball
VERSION="0.1.0"
TARBALL="fluxara-avc-${VERSION}.tar.gz"

cd ..
tar --exclude='.git' --exclude='target' --exclude='AppDir' \
    -czf ~/rpmbuild/SOURCES/${TARBALL} NovaAudioVideoConverter
cd -

# Copy spec file
cp fluxara-avc.spec ~/rpmbuild/SPECS/

# Build RPM
rpmbuild -ba ~/rpmbuild/SPECS/fluxara-avc.spec

echo "RPM package built successfully!"
echo "Package location: ~/rpmbuild/RPMS/"
ls -lh ~/rpmbuild/RPMS/*/fluxara-avc-*.rpm
