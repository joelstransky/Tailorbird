#!/bin/sh
set -e

echo "Installing Tailorbird..."

# Ensure executable
chmod +x tailorbird

# Copy binary
install -Dm755 tailorbird /usr/local/bin/tailorbird

# Copy desktop launcher
install -Dm644 tailorbird.desktop /usr/share/applications/tailorbird.desktop

# Copy icon
if [ -f icon.png ]; then
    install -Dm644 icon.png /usr/share/icons/hicolor/256x256/apps/tailorbird.png
fi

# Refresh desktop database
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database /usr/share/applications
fi

echo "✓ Tailorbird installed successfully! You can launch it by running 'tailorbird' or from your application menu."
