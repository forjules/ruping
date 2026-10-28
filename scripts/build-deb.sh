#!/usr/bin/env bash
set -euo pipefail

# Build .deb package for ruPING
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="2.0.0-1"
PKG_NAME="ruping_${VERSION}_amd64"
DIST_DIR="${ROOT_DIR}/dist"
PKG_DIR="${DIST_DIR}/${PKG_NAME}"

CARGO_BIN="$(command -v cargo || echo "${HOME}/.cargo/bin/cargo")"

echo "==> Building release binary..."
"${CARGO_BIN}" build --release --manifest-path "${ROOT_DIR}/Cargo.toml"

echo "==> Creating Debian package layout..."
rm -rf "${PKG_DIR}"
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/bin"
mkdir -p "${PKG_DIR}/usr/share/doc/ruping"
mkdir -p "${PKG_DIR}/usr/share/man/man1"

cat << EOF > "${PKG_DIR}/DEBIAN/control"
Package: ruping
Version: ${VERSION}
Section: net
Priority: optional
Architecture: amd64
Maintainer: jSSH.app <forjules@proton.me>
Description: High-performance TCP port ping utility written in Rust
 ruPING is a network utility designed to test TCP connectivity and measure
 round-trip connection latency to arbitrary hosts and ports, similar to
 ICMP ping but over TCP. It is a full Linux reimplementation of paping.
EOF

cp "${ROOT_DIR}/target/release/ruping" "${PKG_DIR}/usr/bin/ruping"
chmod 755 "${PKG_DIR}/usr/bin/ruping"
cp "${ROOT_DIR}/README.md" "${PKG_DIR}/usr/share/doc/ruping/"

echo "==> Building .deb package..."
dpkg-deb --build --root-owner-group "${PKG_DIR}" "${DIST_DIR}/${PKG_NAME}.deb"

echo "==> Package created at: ${DIST_DIR}/${PKG_NAME}.deb"
