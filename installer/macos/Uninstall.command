#!/bin/bash
# ==============================================================================
# Tailorbird — macOS Uninstaller Script
# Completely removes Tailorbird application, data, cache, and preferences.
# ==============================================================================

set -e

echo ""
echo "============================================================"
echo "           Tailorbird — Application & Data Removal"
echo "============================================================"
echo ""
echo "This script will completely uninstall Tailorbird and remove:"
echo "  1. /Applications/Tailorbird.app (or ~/Applications/Tailorbird.app)"
echo "  2. ~/Library/Application Support/Tailorbird (Saved profiles, settings, history)"
echo "  3. ~/Library/Caches/com.joelstransky.tailorbird"
echo "  4. ~/Library/Preferences/com.joelstransky.tailorbird.plist"
echo "  5. ~/Library/WebKit/com.joelstransky.tailorbird"
echo ""

read -p "Are you sure you want to proceed with full uninstallation? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstallation canceled."
    exit 0
fi

echo ""
echo "Stopping any running instances of Tailorbird..."
killall tailorbird 2>/dev/null || true
killall Tailorbird 2>/dev/null || true

echo "Removing Application bundle..."
if [ -d "/Applications/Tailorbird.app" ]; then
    rm -rf "/Applications/Tailorbird.app"
    echo "  ✓ Removed /Applications/Tailorbird.app"
fi

if [ -d "$HOME/Applications/Tailorbird.app" ]; then
    rm -rf "$HOME/Applications/Tailorbird.app"
    echo "  ✓ Removed ~/Applications/Tailorbird.app"
fi

echo "Removing saved settings, candidate profiles, and work history..."
if [ -d "$HOME/Library/Application Support/Tailorbird" ]; then
    rm -rf "$HOME/Library/Application Support/Tailorbird"
    echo "  ✓ Removed ~/Library/Application Support/Tailorbird"
fi

echo "Removing cache and web data..."
rm -rf "$HOME/Library/Caches/com.joelstransky.tailorbird" 2>/dev/null || true
rm -rf "$HOME/Library/WebKit/com.joelstransky.tailorbird" 2>/dev/null || true
rm -f "$HOME/Library/Preferences/com.joelstransky.tailorbird.plist" 2>/dev/null || true

echo ""
echo "============================================================"
echo "  ✓ Tailorbird and all associated data have been removed."
echo "============================================================"
echo ""
