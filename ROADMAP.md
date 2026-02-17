# Ferrite — Product Roadmap

> Last updated: Feb 17, 2026
> Goal: A performant, self-hosted media server — a faster alternative to Plex
> Target platforms: Browser (baseline), iOS app, Roku TV app

---

## What's Built (Completed)

### Backend (Rust)
- **9 crates**: server, core, db, scanner, metadata, stream, transcode, dlna, api
- **4-tier streaming**: direct play → remux → audio transcode → full transcode (all via HLS)
- **Video passthrough**: H.264 sources use `-c:v copy` through HLS (near-zero CPU)
- **Color-aware tone-mapping**: true HDR (BT.2020/PQ/HLG) gets full zscale+tonemap; 10-bit SDR gets simple bit-depth conversion
- **Adaptive bitrate**: multi-variant HLS with 480p/720p/1080p/2160p tiers
- **Audio passthrough**: AAC, MP3, Opus, FLAC pass through; DTS/AC3/EAC3/TrueHD transcode to AAC
- **Multi-audio track selection**: API + frontend picker
- **Embedded subtitle extraction**: SRT/ASS/SSA from MKV containers
- **Subtitle burn-in**: for non-extractable formats
- **HW accel auto-detect**: NVENC → QSV → VAAPI → software fallback
- **VP9-in-MKV**: remux to WebM
- **Thumbnail sprite sheets**: for scrubber previews
- **DLNA/UPnP**: for local network discovery
- **Multi-user auth**: bcrypt + JWT + API keys + rate limiting
- **SQLite + WAL**: zero-config, auto-migrations, portable
- **Library watcher**: filesystem events trigger auto-rescan
- **Collections/playlists, webhooks**
- **111 tests passing**

### Frontend (SolidJS SPA)
- **Routed SPA**: Home, Search, Library, Media Detail, Player, Settings, Login
- **Video player**: full-viewport, auto-hiding controls, drag-to-seek, keyboard shortcuts, PiP, buffering indicator, volume persistence
- **HLS.js**: adaptive streaming with quality switching
- **Design system**: Tailwind + glass-morphism + animations
- **Responsive layout**: collapsible sidebar, grid/list toggle

---

## Phase 1 — Deployment Ready (Browser Baseline)

**Goal:** Deploy Ferrite on a Whatbox seedbox and access it remotely via browser.

> Primary target: [Whatbox.ca](https://whatbox.ca) shared seedbox (no root, no Docker, no systemd).
> Ferrite must run as a single static binary from `$HOME` with zero system dependencies beyond FFmpeg.

### Whatbox Constraints
| Constraint | Impact |
|---|---|
| **No root access** | No Docker, no systemd, no `apt install` |
| **Shared server** | Must be lightweight — no GPU, software transcode only |
| **No service manager** | Use `screen`/`tmux` or `cron @reboot` to keep running |
| **Install to `$HOME`** | All paths under `~/ferrite/` — binary, config, DB, cache |
| **Pre-built binary** | Must ship a static `x86_64-unknown-linux-musl` binary |
| **FFmpeg** | Whatbox may have it; otherwise use static build from johnvansickle.com |

### Target Directory Layout on Whatbox
```
~/ferrite/
├── ferrite                  # static Linux binary
├── config/
│   └── ferrite.toml         # configuration
├── data/
│   ├── ferrite.db           # SQLite database
│   └── images/              # poster/backdrop cache
├── cache/
│   └── transcode/           # HLS segment cache
└── static/                  # pre-built SPA (ferrite-ui/dist)
```

### 1.1 — CORS Fix for Remote Access
- [ ] Add `cors_origins` to `ServerConfig` (empty = allow all origins)
- [ ] Update CORS middleware in `router.rs`
- **Why:** Currently blocks all non-localhost browser requests. Whatbox is accessed by IP or domain.

### 1.2 — Embed SPA into Binary (or exe-relative lookup)
- [ ] Option A (**preferred**): Use `rust-embed` or `include_dir!` to compile `ferrite-ui/dist/` into the binary at build time — true single-binary deployment, no separate `static/` folder needed
- [ ] Option B (fallback): Check exe-relative `static/` → `$FERRITE_STATIC_DIR` → `ferrite-ui/dist/` → embedded HTML fallback
- **Why:** On a seedbox, fewer files = simpler. A single binary that serves its own UI is ideal.

### 1.3 — Data Directory Resolution
- [ ] Add `data_dir` config field — all relative paths resolve against this
- [ ] Resolution order: `$FERRITE_DATA_DIR` → config file value → exe-relative `data/` → CWD
- [ ] Default sub-paths: `data_dir/ferrite.db`, `data_dir/images/`, `cache/transcode/`
- **Why:** On Whatbox, user runs from `~/ferrite/` — paths must resolve correctly without `cd`

### 1.4 — Environment Variable Overrides
- [ ] `FERRITE_PORT` — listen port (Whatbox assigns available ports)
- [ ] `FERRITE_HOST` — bind address (default `0.0.0.0`)
- [ ] `FERRITE_DATA_DIR` — base data directory
- [ ] `FERRITE_FFMPEG_PATH` / `FERRITE_FFPROBE_PATH` — custom FFmpeg location
- [ ] `FERRITE_JWT_SECRET` — auth secret (avoid putting in config file)
- [ ] `FERRITE_CONFIG` — config file path
- **Why:** Seedbox users often configure via env vars in `.bashrc` or wrapper scripts

### 1.5 — `ferrite init` Subcommand
- [ ] `ferrite init` creates `config/ferrite.toml` with random JWT secret + sensible defaults
- [ ] `ferrite init --port 12345` pre-fills the port
- [ ] Prints the generated config and next steps
- **Why:** First-run experience on a seedbox should be: download, `./ferrite init`, `./ferrite`

### 1.6 — CI/CD Pipeline (GitHub Actions)
- [ ] Test: `cargo test --workspace` on Linux, macOS, Windows
- [ ] Build: **static musl binary** (`x86_64-unknown-linux-musl`) — runs on any Linux without glibc
- [ ] Build: macOS (x86_64 + aarch64), Windows (x86_64-msvc)
- [ ] Frontend: `cd ferrite-ui && npm ci && npm run build` before packaging
- [ ] Release: upload as GitHub Release assets (`.tar.gz` for Linux/Mac, `.zip` for Windows)
- [ ] Release artifact includes: binary + `static/` (pre-built SPA) + example config
- **Why:** Whatbox users need a single `wget` + `tar xf` to get everything

### 1.7 — Whatbox Install Script
- [ ] `install-whatbox.sh`: one-command setup for Whatbox specifically
  ```bash
  #!/bin/bash
  # Usage: curl -sSL https://raw.githubusercontent.com/.../install-whatbox.sh | bash
  FERRITE_DIR="$HOME/ferrite"
  mkdir -p "$FERRITE_DIR"/{config,data,cache}
  cd "$FERRITE_DIR"
  # Download latest release
  wget -q "https://github.com/.../releases/latest/download/ferrite-x86_64-linux-musl.tar.gz"
  tar xf ferrite-x86_64-linux-musl.tar.gz
  rm ferrite-x86_64-linux-musl.tar.gz
  # Check for FFmpeg
  if ! command -v ffmpeg &>/dev/null; then
    echo "FFmpeg not found. Downloading static build..."
    wget -q "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"
    tar xf ffmpeg-release-amd64-static.tar.xz --strip-components=1 -C "$HOME/bin" \
      "*/ffmpeg" "*/ffprobe"
    rm ffmpeg-release-amd64-static.tar.xz
  fi
  # Initialize config
  ./ferrite init --port ${1:-8080}
  echo ""
  echo "Ferrite installed to $FERRITE_DIR"
  echo "Start with: cd $FERRITE_DIR && screen -S ferrite ./ferrite"
  echo "Or add to crontab: @reboot cd $FERRITE_DIR && ./ferrite >> ferrite.log 2>&1"
  ```
- [ ] Add `@reboot` cron example for auto-start after server reboot
- **Why:** Whatbox users should go from zero to running in one command

### 1.8 — README + Documentation
- [ ] README.md: what Ferrite is, quick start, config reference
- [ ] **Whatbox deployment guide** (primary): step-by-step with screenshots
- [ ] FFmpeg setup guide (system vs static build)
- [ ] Building from source guide
- **Why:** Nobody can use it without docs

### 1.9 — Docker Image (Future)
- [ ] Multi-stage Dockerfile: Rust + Node build → slim runtime with FFmpeg
- [ ] `docker-compose.yml` with volume mounts
- [ ] GPU passthrough docs for NVENC
- **Priority:** Lower — for VPS/home server users, not Whatbox

**Estimated effort:** ~2 weeks
**Exit criteria:** A Whatbox user can run `install-whatbox.sh`, add a media library path, and stream to their browser remotely with correct colors, seeking, and subtitle support.

---

## Phase 2 — Browser Experience Polish

**Goal:** Feature parity with Plex's browser experience. This becomes the reference implementation for native apps.

### 2.1 — Watch State & Continue Watching
- [ ] Track playback position per-user per-media (persist on pause/close/periodic)
- [ ] Resume from last position on play
- [ ] "Continue Watching" row on home page (already has UI placeholder)
- [ ] Mark as watched/unwatched
- [ ] Watched indicator on cards (progress bar or checkmark)

### 2.2 — TV Show Organization
- [ ] Season/episode grouping in UI (show → seasons → episodes)
- [ ] Episode detail view with next/previous navigation
- [ ] "Up Next" auto-play (play next episode when current finishes)
- [ ] Season poster art, episode thumbnails

### 2.3 — Metadata Enhancement
- [ ] TMDb/OMDb integration for movie/show metadata (synopsis, cast, ratings, genres)
- [ ] Automatic poster/backdrop/fanart fetching
- [ ] Manual metadata editing (title, year, genre override)
- [ ] Genre/year/rating filtering on library pages

### 2.4 — Multi-User Management (Plex-like)

> The backend already has: `users` table with `is_admin`, JWT auth, admin-only user creation,
> password change, `playback_progress.user_id` FK. This phase builds the full experience.

**Admin — User Management UI (Settings page)**
- [ ] List all users with role, last login, created date
- [ ] Create user form (username, password, display name, admin toggle)
- [ ] Delete user (with confirmation — cascades watch history)
- [ ] Disable/enable user (soft-disable: keep data, block login)
- [ ] Promote/demote admin role
- [ ] Reset password for another user (admin action)

**Invite System**
- [ ] Admin generates invite link/code with optional expiry
- [ ] Invite link opens registration page (no admin needed to complete)
- [ ] Optional: limit max users (config: `max_users = 10`)
- [ ] DB: `invites` table (id, code, created_by, expires_at, used_by, used_at)

**Per-User Library Access**
- [ ] `user_library_access` table (user_id, library_id, granted_by, granted_at)
- [ ] Admin assigns which libraries each user can see
- [ ] Default: new users see all libraries (configurable)
- [ ] API filters library/media queries by user access
- [ ] UI: library access checkboxes on user edit screen

**Per-User Watch State**
- [ ] Ensure all watch state queries filter by authenticated `user_id`
- [ ] Each user has independent watch progress, watched status, continue watching
- [ ] Admin "activity" view: who's watching what, current transcode load

**User Preferences**
- [ ] `user_preferences` table (user_id, key, value) or JSON column
- [ ] Default audio language (e.g., "English" — auto-select matching track)
- [ ] Default subtitle language (or "off")
- [ ] Max streaming quality (cap variant selection for bandwidth-limited users)
- [ ] UI theme preference (future: light/dark)
- [ ] Preferences UI on user profile page

### 2.5 — Playback Improvements
- [ ] Chapter support (skip intro/credits markers)
- [ ] Audio/subtitle track persistence per show (remember language preference)
- [ ] Transcode quality selector in player UI
- [ ] Bandwidth estimation and auto-quality (HLS ABR is there, but UI feedback needed)

### 2.6 — Library Management
- [ ] Bulk operations (mark watched, delete, move to collection)
- [ ] Duplicate detection
- [ ] Missing episode detection for TV shows
- [ ] Manual scan trigger per-library from UI

**Estimated effort:** ~4-6 weeks
**Exit criteria:** A non-technical user can browse, search, play, resume, and manage their media library entirely from the browser with a polished experience comparable to Plex.

---

## Phase 3 — API Standardization for Native Apps

**Goal:** A clean, documented, versioned API that native apps can build against.

### 3.1 — API Versioning & Documentation
- [ ] Version all endpoints under `/api/v1/`
- [ ] OpenAPI/Swagger spec auto-generated from route definitions
- [ ] API documentation site (or embed in settings page)

### 3.2 — Streaming Protocol Support
- [ ] Ensure HLS works cleanly for all clients (browser, iOS AVPlayer, Roku)
- [ ] Server-side quality selection API (client requests specific variant)
- [ ] Bandwidth negotiation endpoint (client reports connection speed)
- [ ] Session management API (list active streams, kill stream)

### 3.3 — Device Registration & Sync
- [ ] Device registration endpoint (name, type, capabilities)
- [ ] Per-device codec capability reporting (what can this device direct-play?)
- [ ] Cross-device watch state sync (pause on TV, resume on phone)

### 3.4 — Push Notifications
- [ ] Webhook system (already exists) extended for device push
- [ ] "New content added" notifications
- [ ] "Continue watching" reminders

**Estimated effort:** ~2-3 weeks
**Exit criteria:** A complete OpenAPI spec that an iOS or Roku developer can build against without reading backend code.

---

## Phase 4 — iOS App

**Goal:** Native iOS app with streaming, offline downloads, and AirPlay.

### 4.1 — Technology Choice
- **Recommended:** Swift + SwiftUI + AVPlayer
- AVPlayer natively handles HLS (no hls.js needed)
- AVPlayer handles HDR passthrough on capable devices
- SwiftUI for modern, declarative UI

### 4.2 — Core Features
- [ ] Server discovery (manual URL entry + mDNS/Bonjour for local)
- [ ] Authentication (login, token storage in Keychain)
- [ ] Library browsing (movies, shows, collections)
- [ ] Search
- [ ] Media detail view
- [ ] HLS playback via AVPlayer (direct play + transcoded)
- [ ] Resume playback / continue watching
- [ ] Audio/subtitle track selection
- [ ] Background audio (continue playing when app is backgrounded)

### 4.3 — iOS-Specific Features
- [ ] AirPlay support (built into AVPlayer)
- [ ] Picture-in-Picture
- [ ] Offline downloads (download HLS segments for offline viewing)
- [ ] CarPlay audio (for music libraries)
- [ ] Siri Shortcuts ("Play my show on Ferrite")
- [ ] Widget (continue watching widget on home screen)

### 4.4 — Distribution
- [ ] TestFlight for beta testing
- [ ] App Store submission
- [ ] Or: self-signed IPA for sideloading (AltStore compatible)

**Estimated effort:** ~6-8 weeks for MVP
**Exit criteria:** Browse library, play/resume content, AirPlay to TV, download for offline.

---

## Phase 5 — Roku TV App

**Goal:** Native Roku channel for direct TV playback.

### 5.1 — Technology
- **Required:** BrightScript + SceneGraph (Roku's proprietary framework)
- Roku natively handles HLS via its Video node
- Limited codec support — transcoding is critical here

### 5.2 — Core Features
- [ ] Channel setup (server URL entry, login)
- [ ] Library browsing (grid layout, poster art)
- [ ] Media detail screen
- [ ] HLS playback (Roku Video node handles HLS natively)
- [ ] Resume playback
- [ ] Audio/subtitle track selection (Roku supports HLS alternate audio)
- [ ] Remote control navigation (D-pad, play/pause, back)

### 5.3 — Roku-Specific Considerations
- [ ] Roku supports: H.264, HEVC (some models), AAC, AC3, EAC3
- [ ] Report device capabilities to server → server decides transcode strategy
- [ ] Deep linking (launch directly to a specific title)
- [ ] Roku Search integration (appear in Roku's global search)

### 5.4 — Distribution
- [ ] Roku Developer account
- [ ] Private channel (sideloaded via developer mode) for testing
- [ ] Public channel submission to Roku Channel Store

**Estimated effort:** ~4-6 weeks for MVP
**Exit criteria:** Browse, play, resume on Roku TV with proper transcoding for device capabilities.

---

## Phase 6 — Scale & Reliability

**Goal:** Production-grade for multi-user households and power users.

### 6.1 — Performance
- [ ] Transcode caching (cache popular transcodes to avoid re-encoding)
- [ ] Segment pre-generation (transcode ahead of playback position)
- [ ] Connection pooling for SQLite (or migrate to PostgreSQL for multi-server)
- [ ] CDN-friendly headers for reverse proxy caching

### 6.2 — Reliability
- [ ] Health monitoring dashboard
- [ ] Automatic FFmpeg crash recovery (restart failed transcodes)
- [ ] Database backup/restore commands
- [ ] Graceful handling of disk-full conditions

### 6.3 — Multi-Server (Future)
- [ ] Federated libraries (aggregate media from multiple Ferrite instances)
- [ ] Distributed transcoding (offload to dedicated transcode nodes)

**Estimated effort:** Ongoing
**Exit criteria:** Handles 5+ concurrent streams reliably on a mid-range seedbox.

---

## Priority Order Summary

```
Phase 1: Deployment Ready          ~2 weeks     ← YOU ARE HERE
Phase 2: Browser Polish            ~4-6 weeks
Phase 3: API for Native Apps       ~2-3 weeks
Phase 4: iOS App                   ~6-8 weeks
Phase 5: Roku App                  ~4-6 weeks
Phase 6: Scale & Reliability       Ongoing
```

**Total to MVP (browser + iOS + Roku):** ~20-26 weeks

### Recommended Starting Point

Start Phase 1 immediately — specifically the **Docker image** and **CORS fix**, since those unblock real-world testing on your seedbox. Once you can deploy and stream remotely, you'll discover the real UX gaps that Phase 2 needs to address.

Phase 3 (API standardization) should happen *before* starting native apps, so you design the API once and build both apps against it.
