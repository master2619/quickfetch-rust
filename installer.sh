#!/bin/bash

# Ensure the script is run as root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root. Please use sudo."
    exit 1
fi

# Set variables
REPO_URL="https://github.com/master2619/quickfetch/releases/download/release-3/quickfetch"
DEST_PATH="/usr/bin/quickfetch"

# Download the latest release
echo "Downloading the latest release from $REPO_URL..."
wget -O quickfetch "$REPO_URL"

# Check if the download was successful
if [ $? -ne 0 ]; then
    echo "Error downloading the file. Please check the URL and try again."
    exit 1
fi

# Move the binary to /usr/bin/
echo "Moving the binary to $DEST_PATH..."
mv quickfetch "$DEST_PATH"

# Make the binary executable
chmod +x "$DEST_PATH"

# Verify the installation
if [ -f "$DEST_PATH" ]; then
    echo "QuickFetch has been successfully installed to $DEST_PATH."
else
    echo "Installation failed. Please check the permissions and try again."
    exit 1
fi

echo "Installation complete. You can run QuickFetch by typing 'quickfetch' in the terminal."

exit 0
