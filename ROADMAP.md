# Ferrite — Product Roadmap

> Last updated: Feb 17, 2026
> Goal: A performant, self-hosted media server — a faster alternative to Plex
> Target platforms: Browser (baseline), iOS app, Smart TV apps

---

## Phase 1 — Deployment Ready ✅ COMPLETE

**Goal:** Deploy Ferrite on a Whatbox seedbox and access it remotely via browser.

All Phase 1 items are complete. Ferrite is deployed and streaming on Whatbox seedbox.

| Item | Status |
|------|--------|
| CORS fix for remote access | ✅ `cors_origins` config field + middleware |
| SPA lookup (exe-relative `static/`) | ✅ `resolve_spa_dir()` + fallback HTML |
| Data directory resolution | ✅ `FERRITE_DATA_DIR` → exe-relative → CWD |
| Environment variable overrides | ✅ 9 env vars (port, host, data dir, ffmpeg, jwt, etc.) |
| `ferrite init` subcommand | ✅ Generates config with random JWT secret |
| CI/CD pipeline | ✅ `ci.yml` (test/fmt/clippy) + `release.yml` (musl build + SPA) |
| Whatbox install script | ✅ `scripts/install-whatbox.sh` with FFmpeg auto-download |
| README + documentation | ✅ README.md, DEPLOYMENT.md, WHATBOX_TESTING.md |
| Docker image | ⏭️ Deferred — single binary simpler for seedbox |

---

## Phase 2 — Browser Experience Polish ← YOU ARE HERE

**Goal:** Feature parity with Plex's browser experience. This becomes the reference implementation for native apps.

Split into **2A** (high-impact, do now) and **2B** (defer until after native app foundation).

### Phase 2A — Core Experience (Do Now)

These items have the highest impact on daily usability.

#### 2A.1 — Up Next / Auto-Play (TV Shows)
- [x] Backend: `GET /api/episodes/{media_item_id}/next` endpoint
- [x] TV library shows grid of shows (not flat episode list) — `ShowsPage` + `ShowDetailPage`
- [x] Show → Season → Episode hierarchy UI (Plex-style)
- [x] "Up Next" overlay in final 30 seconds with 15s countdown + auto-play
- [x] Player remounts correctly on episode change (no stale state)
- **Known bugs (return to fix):**
  - [x] Next episode selection is non-deterministic — **Fixed:** rewrote SQL as two separate CTEs (`same_season_next` + `next_season_first`) joined with `UNION ALL LIMIT 1`, eliminating the ambiguous `OR` condition that could return wrong episodes with gaps in episode/season numbers
  - [x] Player `currentTime` can exceed the displayed max duration — **Fixed:** cap `setCurrentTime` at `knownDuration()` in `onTimeUpdate`; reuse the already-computed `dur` variable to avoid the duplicate declaration
- **Why:** Essential for TV show binge-watching — the #1 use case for media servers

#### 2A.2 — Watched Indicators on Cards ✅
- [x] Progress bar on cards
- [x] Checkmark overlay for completed items
- [x] Unwatched dot for new items (added in last 7 days, unplayed)
- [x] "Mark as watched/unwatched" button on detail page (optimistic UI)
- **Why:** At a glance, users can't tell what they've seen vs. what's new

#### 2A.3 — Admin User Management UI ✅
- [x] Backend: user CRUD, admin flag, password change
- [x] Settings page: list users with role badge, last login, avatar initial
- [x] Create user form (username, password, display name, admin toggle)
- [x] Delete user (with confirmation, cannot delete self)
- [x] Admin reset password for another user
- [x] Section hidden from non-admin users
- **Why:** Can't share Ferrite with friends/family without a way to manage users from the UI

#### 2A.4 — External Subtitle Selection in Player ✅
- [x] Backend: subtitle listing + SRT/ASS → VTT conversion
- [x] Subtitle track picker in player controls
- [x] Load selected VTT subtitle via `<track>` element
- [x] Subtitle preference persists across episodes (sessionStorage per library)
- **Why:** Many MKV files have multiple subtitle tracks; users need to pick them

#### 2A.5 — Quality Selector in Player ✅
- [x] Backend: multi-variant HLS with quality tiers
- [x] Quality badge on button showing current resolution (e.g. `1080p` / `A` for auto)
- [x] Dropdown to manually select quality (Auto / 1080p / 720p / etc.)
- [x] Auto mode uses HLS.js ABR; manual pins via `hls.currentLevel`
- [x] Quality preference persists across episodes (sessionStorage per library)
- **Why:** Users on limited bandwidth need to manually select lower quality

#### 2A.6 — Audio/Subtitle Track Persistence ✅
- [x] Audio track preference persists across episodes (sessionStorage per library)
- [x] Subtitle track preference persists across episodes (sessionStorage per library)
- [x] Quality preference persists across episodes (sessionStorage per library)
- **Note:** sessionStorage scope = browser tab session; clears on tab close (intentional — avoids stale prefs across different shows)
- **Why:** Users who prefer Japanese audio + English subs shouldn't re-select every episode

**Estimated effort:** ~2 weeks
**Exit criteria:** TV binge-watching works smoothly, subtitles are selectable, quality can be controlled, and the admin can manage users from the UI.

### Phase 2B — Polish & Power Features (Defer)

These are valuable but don't block native app development.

#### 2B.1 — Invite System
- [ ] Admin generates invite link/code with optional expiry
- [ ] Invite link opens registration page
- [ ] `invites` table (id, code, created_by, expires_at, used_by)

#### 2B.2 — Per-User Library Access
- [ ] `user_library_access` table (user_id, library_id)
- [ ] Admin assigns which libraries each user can see
- [ ] API filters queries by user access

#### 2B.3 — User Preferences ✅
- [x] `user_preferences` table (migration 010), `preference_repo` with get/set/upsert
- [x] `GET /api/preferences` + `PUT /api/preferences` endpoints
- [x] Default subtitle language — auto-selects matching track on playback
- [x] Default audio language — auto-selects matching track on playback
- [x] Max streaming quality cap (stored, UI in Settings)
- [ ] UI theme preference (deferred — dark-only for now)

#### 2B.4 — Chapter Support ✅
- [x] Extract chapter markers from media files (ffprobe + `-show_chapters`)
- [x] Store in `chapters` table (migration 009), exposed via `GET /api/media/{id}/chapters`
- [x] Display chapter tick marks on player timeline
- [x] Show chapter name in timeline hover tooltip
- [ ] "Skip Intro" / "Skip Credits" buttons (requires intro detection — deferred)

#### 2B.5 — Library Management
- [ ] Bulk operations (mark watched, move to collection)
- [ ] Duplicate detection
- [ ] Missing episode detection for TV shows
- [ ] Manual metadata editing (title, year, genre override)

#### 2B.6 — Enhanced Metadata
- [x] Genre/year/rating filtering UI — filter bar on LibraryPage with genre chips, min rating, year range
- [ ] Cast & crew information from TMDB
- [ ] "More like this" recommendations

#### 2B.7 — Admin Activity Dashboard ✅
- [x] `GET /api/admin/streams` — lists all active HLS sessions (admin-only)
- [x] `AdminPage.tsx` — live activity view, auto-refreshes every 5s
- [x] Sidebar "Activity" nav link
- [ ] Stream history log (deferred — requires persistent storage)

**Estimated effort:** ~3-4 weeks (can be done incrementally)

---

## Phase 3 — API Standardization for Native Apps

**Goal:** A clean, documented, versioned API that native apps can build against.

### 3.1 — API Versioning & Documentation
- [ ] Version all endpoints under `/api/v1/` (keep `/api/` as alias)
- [ ] OpenAPI spec via `utoipa` crate (auto-generated from Axum handlers)
- [ ] Swagger UI embedded at `/api/docs`

### 3.2 — Streaming Protocol Support
- [ ] Ensure HLS works for iOS AVPlayer and Android ExoPlayer (not just hls.js)
- [ ] Server-side quality selection API (client requests specific variant by name)
- [ ] Session management API (list active streams, kill stream by admin)

### 3.3 — Device Registration & Capabilities
- [ ] `POST /api/devices` — register device (name, type, supported codecs)
- [ ] Per-device codec capability reporting → server adjusts transcode strategy
- [ ] Cross-device watch state sync (automatic via existing per-user progress)

### 3.4 — Push Notifications
- [ ] Extend webhook system for device push (APNs for iOS, FCM for Android)
- [ ] "New content added" notifications
- [ ] "Continue watching" reminders

**Estimated effort:** ~2-3 weeks
**Exit criteria:** OpenAPI spec that an iOS developer can build against without reading backend code.

---

## Phase 4 — iOS App

**Goal:** Native iOS app with streaming, offline downloads, and AirPlay.

### 4.1 — Technology
- **Swift + SwiftUI + AVPlayer**
- AVPlayer natively handles HLS (no hls.js)
- AVPlayer handles HDR passthrough on capable devices

### 4.2 — Core Features
- [ ] Server discovery (manual URL entry + Bonjour for local)
- [ ] Authentication (login, token in Keychain)
- [ ] Library browsing (movies, shows, collections)
- [ ] Search
- [ ] Media detail view
- [ ] HLS playback via AVPlayer
- [ ] Resume playback / continue watching
- [ ] Audio/subtitle track selection
- [ ] Background audio

### 4.3 — iOS-Specific
- [ ] AirPlay (built into AVPlayer)
- [ ] Picture-in-Picture
- [ ] Offline downloads (HLS segment caching)
- [ ] Siri Shortcuts ("Play my show on Ferrite")
- [ ] Home screen widget (continue watching)

### 4.4 — Distribution
- [ ] TestFlight beta
- [ ] App Store submission

**Estimated effort:** ~6-8 weeks for MVP
**Exit criteria:** Browse, play, resume, AirPlay to TV, download for offline.

---

## Phase 5 — Smart TV Apps

**Goal:** Playback on living room screens.

### Option A: Web-Based TV Apps (Recommended First)
- [ ] **Samsung Tizen** web app (HTML5 + HLS — reuse SolidJS frontend)
- [ ] **LG webOS** web app (same approach)
- [ ] **Chromecast / Fire TV** via web receiver
- **Why:** Reuses existing web frontend with TV-optimized layout. Much less effort than native.

### Option B: Roku Channel (If Needed)
- [ ] BrightScript + SceneGraph (proprietary — cannot reuse web code)
- [ ] HLS via Roku Video node
- [ ] Remote control (D-pad) navigation
- [ ] Roku Channel Store submission

**Estimated effort:** 2-3 weeks (web TV apps) or 4-6 weeks (Roku native)
**Exit criteria:** Browse, play, resume on living room TV.

---

## Phase 6 — Scale & Reliability

**Goal:** Production-grade for multi-user households and power users.

### 6.1 — Performance
- [ ] Transcode caching (avoid re-encoding popular files)
- [ ] Segment pre-generation (encode ahead of playback position)
- [ ] CDN-friendly headers for reverse proxy caching

### 6.2 — Reliability
- [ ] Health monitoring dashboard
- [ ] Automatic FFmpeg crash recovery
- [ ] Database backup/restore CLI commands
- [ ] Graceful disk-full handling

### 6.3 — Multi-Server (Future)
- [ ] Federated libraries (aggregate from multiple Ferrite instances)
- [ ] Distributed transcoding (offload to dedicated nodes)

**Estimated effort:** Ongoing
**Exit criteria:** 5+ concurrent streams reliably on a mid-range seedbox.

---

---

## Phase 7 — Ferrite Account Service (FAS)

**Goal:** Federated user identity — one Ferrite Account works across multiple self-hosted servers, similar to how a Plex account works across Plex servers.

### Vision

Users create a single **Ferrite Account** at a central hosted service (`accounts.ferrite.app` or self-hosted). They use this account to join any Ferrite server instance — no separate registration per server. Server admins invite users by email/username; the server trusts FAS to authenticate them.

### Architecture

```
┌─────────────────────────────────────────────────────┐
│  Ferrite Account Service (FAS)                      │
│  - User registration & login                        │
│  - Issues OAuth2/OIDC tokens                        │
│  - Tracks which servers a user belongs to           │
│  - Can be self-hosted or use hosted accounts.ferrite│
└───────────────────┬─────────────────────────────────┘
                    │  OAuth2 token validation
          ┌─────────▼──────────┐     ┌──────────────────┐
          │  Ferrite Server A  │     │  Ferrite Server B │
          │  (self-hosted)     │     │  (self-hosted)    │
          │  - Stores FAS ID   │     │  - Stores FAS ID  │
          │  - Local roles     │     │  - Local roles    │
          └────────────────────┘     └──────────────────┘
```

### Auth modes (both supported simultaneously)

- **FAS mode** — user signs in via FAS OAuth flow, server validates token against FAS public key. No password stored on server.
- **Local mode** — traditional username/password, JWT signed by server. Fallback for air-gapped/offline deployments. Current implementation.

### Data model changes

| Table | Current | FAS addition |
|-------|---------|--------------|
| `users` | `username`, `password_hash`, `is_admin` | Add `fas_user_id` (nullable), `fas_email` — local accounts keep password_hash, FAS accounts leave it null |
| `servers` | n/a | New table in FAS: `id`, `name`, `url`, `owner_fas_id`, `created_at` |
| `server_members` | n/a | New table in FAS: `fas_user_id`, `server_id`, `role`, `invited_at` |

### User flows

1. **Join a server (FAS):** User visits server URL → "Sign in with Ferrite Account" → OAuth redirect to FAS → callback with code → server exchanges for session → user is in
2. **Admin invites user:** Admin enters email in Settings → server calls FAS to look up user → stores `fas_user_id` + role locally → user gets notified
3. **Multi-server dashboard:** FAS UI shows all servers the user belongs to with one-click access
4. **Local fallback:** If `fas_url` is absent from config, server falls back to local username/password auth (current behavior)

### Implementation phases

- **FAS-1:** Scaffold FAS as a separate Rust service (Axum + SQLite/Postgres) — user registration, login, OAuth2 token issuance (RFC 6749)
- **FAS-2:** Add FAS token validation path to `ferrite-api` alongside local auth — config: `[auth] fas_url = "https://accounts.ferrite.app"`
- **FAS-3:** Server admin UI for inviting FAS users, managing server roles (replaces current local-only user management)
- **FAS-4:** FAS multi-server dashboard — user can see and switch between all their servers
- **FAS-5:** Mobile/TV app deep linking — apps authenticate once with FAS, discover user's servers automatically

### Key decisions to revisit

- **Hosted vs. self-hosted FAS:** Should `accounts.ferrite.app` be the default, or should every deployment bring its own FAS? (Plex uses hosted; Jellyfin uses local only)
- **Token format:** JWT with FAS-signed public key (simpler) vs. opaque tokens with introspection endpoint (more revocable)
- **Privacy:** FAS knows which servers a user belongs to — acceptable tradeoff for convenience?

**Estimated effort:** FAS-1 through FAS-3 ~6-8 weeks
**Prerequisite:** Phase 3 (stable API) should be complete first so native apps can also use FAS auth

---

## Priority Order Summary

```
Phase 1:  Deployment Ready          ✅ COMPLETE
Phase 2A: Core Browser Experience   ~2 weeks       ← YOU ARE HERE
Phase 2B: Browser Polish            ~3-4 weeks     (incremental, non-blocking)
Phase 3:  API for Native Apps       ~2-3 weeks
Phase 4:  iOS App                   ~6-8 weeks
Phase 5:  Smart TV Apps             ~2-6 weeks
Phase 6:  Scale & Reliability       Ongoing
Phase 7:  Ferrite Account Service   ~6-8 weeks     (after Phase 3)
```

**Path to iOS app:** Phase 2A → Phase 3 → Phase 4 (~10-13 weeks)
**Path to TV apps:** Phase 5 can start in parallel with Phase 4
**Path to FAS:** Phase 3 → Phase 7 (can overlap with Phase 4)

### Next Up (Phase 2A Priority Order)
1. **Up Next auto-play** — biggest UX gap for TV show users ✅
2. **Watched indicators** — can't tell what's new vs. seen at a glance ✅
3. **Admin user management UI** — will be superseded by FAS; defer deep work ✅ (basic impl done)
4. **Subtitle picker in player** — many files have subtitles, no way to select them
5. **Quality selector** — users on limited bandwidth need manual control
6. **Audio/subtitle persistence** — quality-of-life for binge-watching
