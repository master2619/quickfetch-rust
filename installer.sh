#!/bin/sh

# Exit immediately if any command fails
set -e

# Temp binary location
TMP_FILE="/tmp/quickfetch.$$"

# Ensure cleanup on exit
cleanup() {
    if [ -f "$TMP_FILE" ]; then
        rm -f "$TMP_FILE"
    fi
}
trap cleanup EXIT INT TERM

# Ensure the script is run as root
check_root() {
    USER_ID=$(id -u)
    if [ "$USER_ID" -ne 0 ]; then
        echo "Error: This script must be run as root. Use sudo." >&2
        exit 1
    fi
}

# Detect system architecture
detect_arch() {
    ARCH_RAW=$(uname -m)
    case "$ARCH_RAW" in
        aarch64 | arm64)
            echo "arm64"
            ;;
        x86_64 | amd64)
            echo "amd64"
            ;;
        i386 | i686)
            echo "386"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Download the correct binary based on architecture
download_binary() {
    ARCH="$1"
    BASE_URL="https://github.com/master2619/quickfetch-rust/releases/download/release"

    if [ "$ARCH" = "arm64" ]; then
        FILE_NAME="quickfetch-rust-arm64"
    else
        FILE_NAME="quickfetch-rust-amd64"
    fi

    DOWNLOAD_URL="${BASE_URL}/${FILE_NAME}"

    echo "Downloading QuickFetch for architecture: $ARCH"
    echo "From URL: $DOWNLOAD_URL"

    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE" || {
        echo "Error: Failed to download binary." >&2
        exit 1
    }

    if [ ! -s "$TMP_FILE" ]; then
        echo "Error: Downloaded file is empty." >&2
        exit 1
    fi
}

# Move binary to destination and set permissions
install_binary() {
    DEST="/usr/bin/quickfetch"

    echo "Installing to $DEST..."
    mv "$TMP_FILE" "$DEST"
    chmod +x "$DEST"

    if [ ! -x "$DEST" ]; then
        echo "Error: Installation failed." >&2
        exit 1
    fi

    echo "✅ QuickFetch installed successfully!"
    echo "👉 Run it by typing: quickfetch"
}

### Main script flow ###
check_root

ARCH_DETECTED=$(detect_arch)

if [ "$ARCH_DETECTED" = "unknown" ]; then
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 1
fi

download_binary "$ARCH_DETECTED"
install_binary

exit 0
