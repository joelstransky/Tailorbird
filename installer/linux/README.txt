================================================================================
                    TAILORBIRD — LINUX INSTALLATION & HELP
================================================================================

Thank you for using Tailorbird!

Tailorbird is a specialized dual-pane desktop browser for job hunting,
application tracking, and form filling. It is 100% open-source, completely
offline & local-first, and contains NO telemetry or tracking.

Source code and issues:
https://github.com/joelstransky/Tailorbird


--------------------------------------------------------------------------------
1. SYSTEM DEPENDENCIES (WebKitGTK & GTK3)
--------------------------------------------------------------------------------
Tailorbird utilizes WebKitGTK and GTK 3 for its native browser and window engine.
Before running Tailorbird, ensure the runtime libraries are installed:

• Ubuntu / Debian / Pop!_OS / Linux Mint:
    sudo apt update
    sudo apt install -y libwebkit2gtk-4.1-0 libgtk-3-0 libssl3

• Fedora / RHEL / CentOS Stream:
    sudo dnf install -y webkit2gtk4.1 gtk3 openssl

• Arch Linux / Manjaro:
    sudo pacman -S webkit2gtk-4.1 openssl

• openSUSE Tumbleweed / Leap:
    sudo zypper install libwebkit2gtk-4_1-0 libgtk-3-0 libopenssl3


--------------------------------------------------------------------------------
2. QUICK START (Running from Terminal)
--------------------------------------------------------------------------------
You can run the executable directly from this folder:

    chmod +x tailorbird
    ./tailorbird


--------------------------------------------------------------------------------
3. SYSTEM-WIDE INSTALLATION (Optional)
--------------------------------------------------------------------------------
To install Tailorbird into your system path and application launcher:

Option A — Run the included install helper script:
    sudo ./install.sh

Option B — Manual installation commands:
    # 1. Install binary
    sudo install -Dm755 tailorbird /usr/local/bin/tailorbird

    # 2. Install desktop launcher
    sudo install -Dm644 tailorbird.desktop /usr/share/applications/tailorbird.desktop

    # 3. Install icon
    sudo install -Dm644 icon.png /usr/share/icons/hicolor/256x256/apps/tailorbird.png

    # 4. Refresh desktop database
    sudo update-desktop-database 2>/dev/null || true


--------------------------------------------------------------------------------
4. LOCAL STORAGE & DATA LOCATION
--------------------------------------------------------------------------------
All candidate data, profiles, and outreach notes are stored locally in standard
XDG directory format:
    ~/.local/share/tailorbird/tailorbird_data.json
    (or inside ~/.config/tailorbird/)


--------------------------------------------------------------------------------
5. UNINSTALLATION
--------------------------------------------------------------------------------
To remove Tailorbird:
    sudo rm -f /usr/local/bin/tailorbird
    sudo rm -f /usr/share/applications/tailorbird.desktop
    sudo rm -f /usr/share/icons/hicolor/256x256/apps/tailorbird.png
    sudo update-desktop-database 2>/dev/null || true

To remove stored data and settings:
    rm -rf ~/.local/share/tailorbird
    rm -rf ~/.config/tailorbird

================================================================================
