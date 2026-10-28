#!/usr/bin/env bash
set -euo pipefail

# ruPING Launchpad PPA Source Package Builder
# Usage: ./scripts/build-ppa-source.sh [DISTRO] [GPG_KEY_ID]
# Example: ./scripts/build-ppa-source.sh noble [KEY_ID]

DISTRO="${1:-noble}"
GPG_KEY="${2:-}"
REV="${3:-1}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="$(grep -m1 '^version = ' "${ROOT_DIR}/Cargo.toml" | cut -d '"' -f2)"
DEB_VERSION="${VERSION}-1~${DISTRO}${REV}"
BUILD_DIR="${ROOT_DIR}/build-ppa"

echo "==> Preparing ruPING for Launchpad PPA (Target: Ubuntu ${DISTRO})..."

rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}/ruping-${VERSION}"

# Copy source tree excluding target, git, build-ppa
rsync -av --exclude 'target' \
          --exclude '.git' \
          --exclude 'dist' \
          --exclude 'build-ppa' \
          --exclude 'vendor' \
          "${ROOT_DIR}/" "${BUILD_DIR}/ruping-${VERSION}/"

cd "${BUILD_DIR}/ruping-${VERSION}"

echo "==> Vendoring crate dependencies for offline Launchpad builder..."
CARGO_BIN="$(command -v cargo || echo "${HOME}/.cargo/bin/cargo")"
"${CARGO_BIN}" vendor vendor

# Ensure Cargo.lock is v3 for compatibility with Launchpad's Cargo 1.75
if [ -f Cargo.lock ]; then
    sed -i 's/version = 4/version = 3/' Cargo.lock
fi

mkdir -p .cargo
cat << 'EOF' > .cargo/config.toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

# Update debian/changelog for target distribution
cat << EOF > debian/changelog
ruping (${DEB_VERSION}) ${DISTRO}; urgency=medium

  * PPA release for Ubuntu ${DISTRO}.

 -- Giuliano (forjules) <forjules@proton.me>  $(date -R)
EOF

echo "==> Creating orig.tar.gz tarball..."
cd "${BUILD_DIR}"
tar --exclude='debian' -czf "ruping_${VERSION}.orig.tar.gz" "ruping-${VERSION}"

echo "==> Building Debian source package..."
cd "${BUILD_DIR}"
dpkg-source -b "ruping-${VERSION}"

cd "${BUILD_DIR}/ruping-${VERSION}"
dpkg-genchanges -S > "../ruping_${DEB_VERSION}_source.changes"

cd "${BUILD_DIR}"
if [ -n "${GPG_KEY}" ]; then
    echo "==> Signing source package with GPG key ${GPG_KEY}..."
    debsign -k "${GPG_KEY}" "ruping_${DEB_VERSION}_source.changes"
fi

echo ""
echo "================================================================="
echo "✅ Source package built successfully in: ${BUILD_DIR}"
ls -lh "${BUILD_DIR}"/*.dsc "${BUILD_DIR}"/*_source.changes
echo "================================================================="
echo ""
if [ -z "${GPG_KEY}" ]; then
    echo "To sign this source package before uploading:"
    echo "  debsign -k <YOUR_GPG_KEY_ID> ${BUILD_DIR}/ruping_${DEB_VERSION}_source.changes"
    echo ""
fi
echo "To upload to Launchpad PPA:"
echo "  dput ppa:<your-launchpad-username>/<ppa-name> ${BUILD_DIR}/ruping_${DEB_VERSION}_source.changes"
echo ""
