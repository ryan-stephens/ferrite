# Ferrite — Deployment Readiness Audit

> Audit date: Feb 17, 2026
> Purpose: Assess readiness for seedbox / cloud / cross-platform deployment

---

## Current State — What's Solid

| Area | Status | Notes |
|---|---|---|
| **Single binary** | ✅ | Rust compiles to one `ferrite` executable — no runtime, no JVM, no interpreter |
| **SQLite database** | ✅ | Zero-config embedded DB with WAL mode, auto-migrations on startup, portable single file |
| **CLI interface** | ✅ | `clap` with `--config`, `--port` override, `hash-password` subcommand |
| **TOML config** | ✅ | Clean `config/ferrite.toml` with sensible defaults; runs without config file |
| **HW accel auto-detect** | ✅ | Probes FFmpeg for NVENC/QSV/VAAPI at startup, falls back to software libx264 |
| **Graceful shutdown** | ✅ | Ctrl+C kills all FFmpeg child processes, cleans HLS cache |
| **Cross-platform sockets** | ✅ | SSDP already has `#[cfg(unix)]` for `SO_REUSEPORT` |
| **No hardcoded OS paths** | ✅ | All paths come from config (FFmpeg, DB, cache dirs) |
| **Streaming pipeline** | ✅ | 4-tier strategy (direct play → remux → audio transcode → full transcode), all via HLS with video/audio passthrough |
| **Frontend SPA** | ✅ | Pre-built `ferrite-ui/dist/` served by tower-http; embedded HTML fallback if dist missing |
| **Auth system** | ✅ | Multi-user with bcrypt passwords, JWT tokens, API keys, login rate limiting |
| **105 tests passing** | ✅ | Full workspace test suite green |

---

## Critical Gaps — Must Fix Before Deployment

### 1. CORS Blocks Remote Access

**File:** `crates/ferrite-api/src/router.rs` lines 107–114

The CORS policy only allows `localhost`, `127.0.0.1`, `0.0.0.0`, and `[::1]`. A seedbox accessed by IP address or domain name will have all browser requests blocked.

**Fix:** Add a `cors_origins` field to `ServerConfig`. If empty/absent, allow all origins (or mirror the `Origin` header). If populated, restrict to the listed origins. This is the single biggest blocker for remote deployment.

```rust
// Current (broken for remote):
.allow_origin(AllowOrigin::predicate(|origin, _| {
    origin.starts_with(b"http://localhost") || ...
}))

// Needed:
// If cors_origins is empty → AllowOrigin::any()
// If cors_origins has entries → AllowOrigin::list(...)
```

---

### 2. SPA Path is Hardcoded Relative

**File:** `crates/ferrite-api/src/router.rs` line 89

```rust
let spa_dir = std::path::PathBuf::from("ferrite-ui/dist");
```

This only works when running from the repo root. If the binary is installed to `/usr/local/bin/ferrite`, it won't find the UI assets.

**Fix:** Add a `static_dir` field to `ServerConfig` with a smart default:
- Check `$FERRITE_STATIC_DIR` env var first
- Then check relative to the binary location (`exe_dir/static/`)
- Then check `ferrite-ui/dist/` (dev mode)
- Fall back to embedded HTML

---

### 3. All Default Paths Are Relative to CWD

**File:** `crates/ferrite-core/src/config.rs` — `AppConfig::default()`

Current defaults:
- `database.path` = `"ferrite.db"` (relative)
- `transcode.cache_dir` = `"cache/transcode"` (relative)
- `metadata.image_cache_dir` = `"cache/images"` (relative)

These resolve relative to whatever directory the user happens to `cd` into before running the binary. On a seedbox with systemd, the CWD is typically `/` or `/root`.

**Fix:** Use platform-appropriate data directories:
- **Linux:** `$XDG_DATA_HOME/ferrite/` or `~/.local/share/ferrite/`
- **macOS:** `~/Library/Application Support/ferrite/`
- **Windows:** `%APPDATA%/ferrite/`

Or add a top-level `data_dir` config field that all relative paths resolve against. The `dirs` crate handles this cross-platform.

---

### 4. No Environment Variable Overrides

Seedbox environments and containers often configure services via env vars rather than config files. Currently the only override is `--port` via CLI.

**Fix:** Support env var overrides for critical settings:
- `FERRITE_PORT` — listen port
- `FERRITE_HOST` — bind address
- `FERRITE_DB_PATH` — database file location
- `FERRITE_CONFIG` — config file path
- `FERRITE_DATA_DIR` — base data directory
- `FERRITE_FFMPEG_PATH` / `FERRITE_FFPROBE_PATH` — FFmpeg locations
- `FERRITE_JWT_SECRET` — auth secret (avoid putting secrets in config files)
- `RUST_LOG` — already works for log level via `tracing_subscriber`

---

### 5. No `.gitignore`

The repo has no `.gitignore`. The following are generated/local and should not be committed:
- `target/` — Rust build artifacts
- `ferrite.db` / `ferrite.db-wal` / `ferrite.db-shm` — SQLite database
- `cache/` — transcode cache, image cache
- `ferrite-ui/dist/` — built frontend
- `ferrite-ui/node_modules/` — npm dependencies
- `*.log` — log files (`ferrite-stdout.log`, `ferrite-stderr.log`, `ferrite-perf.log`)
- `nul` — Windows artifact already in repo root
- `config/ferrite.toml` — user-specific config (ship a `.example` instead)

---

### 6. No CI/CD Pipeline

No GitHub Actions workflow exists. Need cross-platform release builds.

**Fix:** Create `.github/workflows/ci.yml` with:
- **Test:** `cargo test --workspace` on Linux, macOS, Windows
- **Build:** Release binaries for:
  - `x86_64-unknown-linux-gnu` (seedboxes, most servers)
  - `x86_64-unknown-linux-musl` (static binary, works everywhere)
  - `aarch64-unknown-linux-gnu` (ARM servers, Raspberry Pi)
  - `x86_64-apple-darwin` / `aarch64-apple-darwin` (macOS Intel/Apple Silicon)
  - `x86_64-pc-windows-msvc` (Windows)
- **Frontend:** `cd ferrite-ui && npm ci && npm run build` before packaging
- **Release:** Upload binaries + bundled UI as GitHub Release assets (`.tar.gz` for Linux/Mac, `.zip` for Windows)

---

### 7. No Install Script / Service Files

Seedbox users need a simple way to install and run as a background service.

**Fix — Linux systemd unit** (`ferrite.service`):
```ini
[Unit]
Description=Ferrite Media Server
After=network.target

[Service]
Type=simple
User=ferrite
Group=ferrite
ExecStart=/usr/local/bin/ferrite --config /etc/ferrite/ferrite.toml
Restart=on-failure
RestartSec=5
WorkingDirectory=/var/lib/ferrite
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

**Fix — Install script** (`install.sh`):
- Download latest release binary for the platform
- Create `ferrite` user/group
- Create `/etc/ferrite/`, `/var/lib/ferrite/`, `/var/cache/ferrite/`
- Install binary to `/usr/local/bin/ferrite`
- Install systemd unit
- Generate default config with random JWT secret
- Enable and start service

---

### 8. No README

No documentation exists for end users. Need a `README.md` covering:
- What Ferrite is (one-liner + screenshot)
- Quick start (download binary, run, open browser)
- Configuration reference (all TOML fields)
- FFmpeg dependency (how to install on each OS)
- Library setup (adding media folders)
- Seedbox deployment guide
- Building from source
- API reference (or link to it)

---

## Nice-to-Have (Post-Launch)

| Item | Priority | Notes |
|---|---|---|
| **Docker image** | Medium | `Dockerfile` with multi-stage build (Rust + Node → slim runtime with FFmpeg) |
| **HTTPS/TLS** | Medium | Built-in TLS via `axum-server` with `rustls`, or document reverse proxy (nginx/caddy) |
| **Config file auto-generation** | Low | `ferrite init` subcommand that creates config with random JWT secret |
| **Health check endpoint** | ✅ Done | `/api/health` already exists |
| **Log rotation** | Low | Currently logs to stdout; systemd journal handles rotation. Could add file rotation. |
| **Backup/restore** | Low | SQLite is a single file — document `cp ferrite.db ferrite.db.bak` |
| **Update mechanism** | Low | Self-update from GitHub releases, or document manual update steps |

---

## External Dependency: FFmpeg

FFmpeg is the **only external runtime dependency**. It must be installed separately.

| Platform | Install Command |
|---|---|
| **Ubuntu/Debian** | `sudo apt install ffmpeg` |
| **Fedora/RHEL** | `sudo dnf install ffmpeg` |
| **Arch** | `sudo pacman -S ffmpeg` |
| **macOS** | `brew install ffmpeg` |
| **Windows** | Download from https://ffmpeg.org/download.html, add to PATH |
| **Seedbox (no root)** | Download static build from https://johnvansickle.com/ffmpeg/, set `ffmpeg_path`/`ffprobe_path` in config |

The config already supports custom paths (`transcode.ffmpeg_path`, `transcode.ffprobe_path`), so users without root access can point to a local FFmpeg binary.

---

## Recommended Fix Order

1. **`.gitignore`** — 5 minutes, prevents committing junk
2. **CORS fix** — 30 minutes, unblocks remote access entirely
3. **Env var overrides** — 1 hour, critical for seedbox/container deployment
4. **SPA path resolution** — 30 minutes, makes installed binary find the UI
5. **Data directory defaults** — 1 hour, makes paths work without explicit config
6. **systemd unit + install script** — 1 hour, enables seedbox deployment
7. **README** — 2 hours, enables other people to use it
8. **GitHub Actions CI** — 2 hours, enables automated cross-platform releases
