#!/bin/bash
set -euo pipefail

# Ferrite Media Server — Whatbox Seedbox Installer
# Usage: curl -sSL https://raw.githubusercontent.com/ryan-stephens/ferrite/main/scripts/install-whatbox.sh | bash
# Or:    bash install-whatbox.sh [port]

FERRITE_DIR="$HOME/ferrite"
FERRITE_PORT="${1:-8080}"
REPO="ryan-stephens/ferrite"

echo "╔══════════════════════════════════════╗"
echo "║   Ferrite Media Server Installer     ║"
echo "║   Target: $FERRITE_DIR"
echo "╚══════════════════════════════════════╝"
echo ""

# Create directory structure
echo "→ Creating directory structure..."
mkdir -p "$FERRITE_DIR"/{config,data,cache/transcode,cache/images,static}

cd "$FERRITE_DIR"

# Download latest release
echo "→ Downloading latest release..."
LATEST_URL=$(curl -sSL "https://api.github.com/repos/$REPO/releases/latest" \
    | grep -o '"browser_download_url": *"[^"]*linux-musl\.tar\.gz"' \
    | grep -o 'https://[^"]*' \
    | head -1)

if [ -z "$LATEST_URL" ]; then
    echo "ERROR: Could not find latest release. Check https://github.com/$REPO/releases"
    echo "You can manually download and extract the binary to $FERRITE_DIR/"
    exit 1
fi

echo "  From: $LATEST_URL"
wget -q --show-progress -O ferrite-release.tar.gz "$LATEST_URL"
tar xf ferrite-release.tar.gz
rm ferrite-release.tar.gz
chmod +x ferrite

# Check for FFmpeg
echo ""
echo "→ Checking for FFmpeg..."
if command -v ffmpeg &>/dev/null; then
    FFMPEG_PATH=$(command -v ffmpeg)
    echo "  Found: $FFMPEG_PATH"
    FFMPEG_VERSION=$(ffmpeg -version 2>&1 | head -1)
    echo "  Version: $FFMPEG_VERSION"
else
    echo "  FFmpeg not found in PATH."
    echo "  Downloading static FFmpeg build..."
    mkdir -p "$HOME/bin"

    wget -q --show-progress -O ffmpeg-static.tar.xz \
        "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"

    # Extract just ffmpeg and ffprobe binaries
    tar xf ffmpeg-static.tar.xz --wildcards "*/ffmpeg" "*/ffprobe" --strip-components=1
    mv ffmpeg ffprobe "$HOME/bin/" 2>/dev/null || true
    rm -f ffmpeg-static.tar.xz

    # Ensure ~/bin is in PATH
    if ! echo "$PATH" | grep -q "$HOME/bin"; then
        echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME/.bashrc"
        export PATH="$HOME/bin:$PATH"
        echo "  Added ~/bin to PATH in .bashrc"
    fi

    echo "  Installed ffmpeg and ffprobe to ~/bin/"
fi

# Initialize config
echo ""
echo "→ Initializing configuration..."
if [ ! -f config/ferrite.toml ]; then
    ./ferrite init --port "$FERRITE_PORT" --output-dir config
else
    echo "  Config already exists at config/ferrite.toml (skipping)"
fi

# Print summary
echo ""
echo "╔══════════════════════════════════════╗"
echo "║   Installation Complete!             ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "Directory: $FERRITE_DIR"
echo "Config:    $FERRITE_DIR/config/ferrite.toml"
echo "Port:      $FERRITE_PORT"
echo ""
echo "Start Ferrite:"
echo "  cd $FERRITE_DIR && ./ferrite"
echo ""
echo "Run in background with screen:"
echo "  screen -dmS ferrite bash -c 'cd $FERRITE_DIR && ./ferrite'"
echo ""
echo "Auto-start on reboot (add to crontab):"
echo "  (crontab -l 2>/dev/null; echo \"@reboot cd $FERRITE_DIR && ./ferrite >> ferrite.log 2>&1\") | crontab -"
echo ""
echo "View logs:"
echo "  screen -r ferrite"
echo ""
