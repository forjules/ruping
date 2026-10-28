#!/usr/bin/env bash
set -e

# ruPING Installer
# https://github.com/forjules/ruping

echo "==> Installing ruPING..."

ARCH=$(uname -m)
OS=$(uname -s)

if [ "$OS" != "Linux" ]; then
    echo "Error: ruPING is designed for Linux systems."
    exit 1
fi

INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    SUDO="sudo"
else
    SUDO=""
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# If built binary exists in current directory or target/release
if [ -f "$SCRIPT_DIR/target/release/ruping" ]; then
    $SUDO cp "$SCRIPT_DIR/target/release/ruping" "$INSTALL_DIR/ruping"
elif [ -f "$SCRIPT_DIR/ruping" ]; then
    $SUDO cp "$SCRIPT_DIR/ruping" "$INSTALL_DIR/ruping"
elif command -v cargo >/dev/null 2>&1 || [ -x "$HOME/.cargo/bin/cargo" ]; then
    CARGO_BIN="$(command -v cargo || echo "$HOME/.cargo/bin/cargo")"
    echo "==> Building from source via Cargo..."
    "$CARGO_BIN" build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"
    $SUDO cp "$SCRIPT_DIR/target/release/ruping" "$INSTALL_DIR/ruping"
else
    echo "Error: Prebuilt binary not found and Cargo is not installed."
    exit 1
fi

$SUDO chmod 755 "$INSTALL_DIR/ruping"
echo "==> Successfully installed ruPING to $INSTALL_DIR/ruping"
echo "==> Run 'ruping' to get started."
