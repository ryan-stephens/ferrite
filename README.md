# Ferrite

A high-performance, self-hosted media server built in Rust. A faster alternative to Plex.

## Features

- **4-tier streaming**: Direct play → remux → audio transcode → full transcode (all via HLS)
- **Video passthrough**: H.264 sources use `-c:v copy` through HLS — near-zero CPU
- **Color-aware tone-mapping**: True HDR (BT.2020/PQ/HLG) gets full tone-mapping; 10-bit SDR gets simple bit-depth conversion
- **Adaptive bitrate**: Multi-variant HLS with 480p/720p/1080p/2160p tiers
- **Audio passthrough**: AAC, MP3, Opus, FLAC pass through; DTS/AC3/EAC3/TrueHD transcode to AAC
- **Multi-audio track selection**: Switch audio tracks from the player
- **Subtitle support**: Embedded extraction (SRT/ASS/SSA) + burn-in for non-extractable formats
- **HW acceleration**: Auto-detect NVENC → QSV → VAAPI → software fallback
- **Multi-user auth**: bcrypt + JWT + API keys + rate limiting
- **SQLite + WAL**: Zero-config database, auto-migrations, portable
- **Library watcher**: Filesystem events trigger auto-rescan
- **DLNA/UPnP**: Local network device discovery
- **Collections & playlists, webhooks, thumbnail sprite sheets**
- **SolidJS SPA**: Modern, responsive browser UI with full-viewport video player

## Quick Start

### Option 1: Pre-built Binary

```bash
# Download latest release
wget https://github.com/ryan-stephens/ferrite/releases/latest/download/ferrite-x86_64-linux-musl.tar.gz
mkdir -p ~/ferrite && cd ~/ferrite
tar xf ~/ferrite-x86_64-linux-musl.tar.gz
rm ~/ferrite-x86_64-linux-musl.tar.gz

# Initialize config (generates random JWT secret)
./ferrite init --port 8080

# Start the server
./ferrite
```

### Option 2: Whatbox Seedbox (One-Command Install)

```bash
curl -sSL https://raw.githubusercontent.com/ryan-stephens/ferrite/main/scripts/install-whatbox.sh | bash -s 8080
```

See [Whatbox Deployment Guide](#whatbox-deployment) below for details.

### Option 3: Build from Source

```bash
# Prerequisites: Rust 1.75+, Node.js 20+, FFmpeg

# Clone
git clone https://github.com/ryan-stephens/ferrite.git
cd ferrite

# Build frontend
cd ferrite-ui && npm ci && npm run build && cd ..

# Build backend
cargo build --release

# Initialize and run
./target/release/ferrite init
./target/release/ferrite
```

## Configuration

Ferrite uses a TOML config file. Generate one with `ferrite init`:

```
~/ferrite/
├── ferrite              # binary
├── config/
│   └── ferrite.toml     # configuration
├── data/
│   ├── ferrite.db       # SQLite database
│   └── cache/           # transcode + image cache
└── static/              # pre-built web UI
```

### Config File Reference

```toml
[server]
host = "0.0.0.0"
port = 8080
cors_origins = []  # empty = allow all origins

[database]
path = "ferrite.db"
max_connections = 16

[scanner]
concurrent_probes = 4
watch_debounce_seconds = 2

[transcode]
ffmpeg_path = "ffmpeg"
ffprobe_path = "ffprobe"
cache_dir = "cache/transcode"
max_concurrent_transcodes = 2
hls_segment_duration = 6
hls_session_timeout_secs = 1800
# hw_accel = "nvenc"  # or "qsv", "vaapi", "software"

[metadata]
image_cache_dir = "cache/images"
rate_limit_per_second = 4
# tmdb_api_key = "your-key"

[auth]
jwt_secret = "your-random-secret"
token_expiry_days = 30

[dlna]
enabled = true
friendly_name = "Ferrite Media Server"
```

### Environment Variables

All config values can be overridden with environment variables:

| Variable | Description |
|---|---|
| `FERRITE_CONFIG` | Config file path (default: `config/ferrite.toml`) |
| `FERRITE_PORT` | Listen port |
| `FERRITE_HOST` | Bind address |
| `FERRITE_DATA_DIR` | Base data directory (DB, cache resolve relative to this) |
| `FERRITE_DB_PATH` | Database file path |
| `FERRITE_FFMPEG_PATH` | FFmpeg binary path |
| `FERRITE_FFPROBE_PATH` | FFprobe binary path |
| `FERRITE_JWT_SECRET` | Auth secret (overrides config file) |
| `FERRITE_STATIC_DIR` | SPA static files directory |

### Path Resolution

Ferrite resolves relative paths in this order:

1. **`$FERRITE_DATA_DIR`** — explicit env var
2. **`<exe_dir>/data/`** — next to the binary (e.g. `~/ferrite/data/`)
3. **CWD** — current working directory (development fallback)

## FFmpeg

Ferrite requires FFmpeg and FFprobe for transcoding. Options:

- **System FFmpeg**: If `ffmpeg` and `ffprobe` are in your `$PATH`, Ferrite will find them automatically
- **Static build**: Download from [johnvansickle.com](https://johnvansickle.com/ffmpeg/) and place in `~/bin/`
- **Custom path**: Set `FERRITE_FFMPEG_PATH` and `FERRITE_FFPROBE_PATH`

## Whatbox Deployment

[Whatbox.ca](https://whatbox.ca) is a shared seedbox provider. Ferrite runs perfectly on Whatbox since it's a single static binary with no root or Docker requirements.

### Automated Install

```bash
# Install with default port 8080
curl -sSL https://raw.githubusercontent.com/ryan-stephens/ferrite/main/scripts/install-whatbox.sh | bash

# Install with custom port
curl -sSL https://raw.githubusercontent.com/ryan-stephens/ferrite/main/scripts/install-whatbox.sh | bash -s 12345
```

### Manual Install

```bash
# 1. Download and extract
mkdir -p ~/ferrite && cd ~/ferrite
wget https://github.com/ryan-stephens/ferrite/releases/latest/download/ferrite-x86_64-linux-musl.tar.gz
tar xf ferrite-x86_64-linux-musl.tar.gz && rm ferrite-x86_64-linux-musl.tar.gz

# 2. Initialize config
./ferrite init --port 8080

# 3. Start in a screen session
screen -dmS ferrite bash -c 'cd ~/ferrite && ./ferrite'

# 4. (Optional) Auto-start on reboot
(crontab -l 2>/dev/null; echo "@reboot cd ~/ferrite && ./ferrite >> ferrite.log 2>&1") | crontab -
```

### Managing Ferrite on Whatbox

```bash
# View the running server
screen -r ferrite

# Detach from screen (leave running)
# Press Ctrl+A then D

# Stop the server
screen -S ferrite -X quit

# Check if running
screen -ls | grep ferrite

# View logs
tail -f ~/ferrite/ferrite.log
```

### Adding Media Libraries

1. Open `http://your-server.whatbox.ca:PORT` in your browser
2. Create your admin account on first visit
3. Go to Settings → Add Library
4. Enter the path to your media (e.g. `/home/user/media/movies`)

## Architecture

Ferrite is built as a Rust workspace with 9 crates:

| Crate | Purpose |
|---|---|
| `ferrite-server` | Binary entry point, CLI, startup |
| `ferrite-core` | Config, shared types |
| `ferrite-db` | SQLite database, repositories, migrations |
| `ferrite-scanner` | Media file discovery, FFprobe, subtitle extraction |
| `ferrite-metadata` | TMDb integration, image caching |
| `ferrite-stream` | HLS streaming, direct play, remux |
| `ferrite-transcode` | FFmpeg orchestration, tone-mapping, HW accel |
| `ferrite-dlna` | DLNA/UPnP server |
| `ferrite-api` | Axum HTTP API, auth, handlers |

Frontend: SolidJS SPA in `ferrite-ui/` with Vite + TailwindCSS + TypeScript + HLS.js.

## License

MIT OR Apache-2.0
