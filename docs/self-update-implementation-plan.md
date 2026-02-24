# Self-Update from the UI — Implementation Plan

Ferrite currently ships as a single static musl binary + SPA frontend, deployed to a seedbox (e.g. Whatbox) via `screen`. GitHub Actions builds a release tarball on every tag push. Today, upgrading requires SSH access, downloading the new tarball, extracting, and restarting the process manually.

This plan adds the ability to check for, download, and apply updates directly from the Ferrite Settings UI — zero SSH required.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  Ferrite UI (Settings Page)                                 │
│                                                             │
│  ┌──────────────┐   ┌──────────────┐   ┌────────────────┐  │
│  │ Check for    │──▶│ "v0.1.38     │──▶│ Apply Update   │  │
│  │ Updates      │   │  available"  │   │ (one-click)    │  │
│  └──────────────┘   └──────────────┘   └────────────────┘  │
└────────────────────────────┬────────────────────────────────┘
                             │ REST API
                             ▼
┌─────────────────────────────────────────────────────────────┐
│  ferrite-api  /api/system/update/*                          │
│                                                             │
│  GET  /check   → query GitHub Releases API                  │
│  POST /apply   → download, verify, swap binary, restart     │
└────────────────────────────┬────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│  Update Engine (new module in ferrite-core or ferrite-api)   │
│                                                             │
│  1. Fetch latest release metadata from GitHub               │
│  2. Compare semver against env!("CARGO_PKG_VERSION")        │
│  3. Download tarball to temp dir                            │
│  4. Verify SHA-256 checksum (from release body/asset)       │
│  5. Extract binary + static/ to staging dir                 │
│  6. Atomic swap: rename current → .bak, staging → current   │
│  7. Trigger graceful restart (re-exec or exit with code)    │
└─────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Version Check API + UI Indicator

**Goal**: Let the user know when a new version is available, with zero risk.

### 1.1 — Backend: `GET /api/system/update/check` (admin-only)

**File**: `crates/ferrite-api/src/handlers/system.rs`

- Call the GitHub Releases API: `GET https://api.github.com/repos/ryan-stephens/ferrite/releases/latest`
- Parse the `tag_name` (e.g. `v0.1.38`) and compare against `env!("CARGO_PKG_VERSION")` using semver ordering.
- Return JSON:
  ```json
  {
    "current_version": "0.1.37",
    "latest_version": "0.1.38",
    "update_available": true,
    "release_url": "https://github.com/ryan-stephens/ferrite/releases/tag/v0.1.38",
    "release_notes": "...",
    "published_at": "2026-02-24T00:00:00Z",
    "download_url": "https://github.com/.../ferrite-x86_64-linux-musl.tar.gz",
    "download_size_bytes": 12345678
  }
  ```
- Cache the result in-memory for 15 minutes to avoid hammering the GitHub API.
- Use `reqwest` (already a workspace dependency) for the HTTP call.

**Route registration** in `router.rs`:
```rust
.route("/api/system/update/check", get(system::check_for_update))
```

### 1.2 — Frontend: Update Badge in Settings

**File**: `ferrite-ui/src/pages/Settings.tsx` (or equivalent)

- On the Settings page mount, call `GET /api/system/update/check`.
- If `update_available` is true, show a badge/banner:
  > **Update available**: v0.1.38 — [View release notes] [Apply Update]
- Show current version in the Settings header (already available via `GET /api/system/info`).

### 1.3 — Config: Optional Update Settings

**File**: `crates/ferrite-core/src/config.rs`

Add an `[update]` section to `AppConfig` with `#[serde(default)]` so existing
`ferrite.toml` files remain backward-compatible without requiring users to add
the section:
```rust
// In AppConfig:
#[serde(default)]
pub update: UpdateConfig,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    /// Disable self-update entirely (e.g. if managed by a package manager)
    #[serde(default)]
    pub disabled: bool,
    /// GitHub repo to check for releases (default: "ryan-stephens/ferrite")
    #[serde(default = "default_update_repo")]
    pub repo: String,
    /// Optional GitHub token for higher API rate limits (60/hr unauthenticated → 5000/hr).
    /// Can also be set via `GITHUB_TOKEN` env var.
    #[serde(default)]
    pub github_token: Option<String>,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            repo: default_update_repo(),
            github_token: None,
        }
    }
}

fn default_update_repo() -> String {
    "ryan-stephens/ferrite".to_string()
}
```

This allows users to disable the feature, point to a fork, or supply a GitHub
token for higher rate limits. The token is resolved at runtime with precedence:
`GITHUB_TOKEN` env var > config value > unauthenticated (60 req/hr).

---

## Phase 2: Download & Apply Update

**Goal**: One-click update that downloads, verifies, swaps, and restarts.

### 2.1 — Release Artifact: Add SHA-256 Checksum

**File**: `.github/workflows/release.yml`

After packaging the tarball, compute and upload a checksum file:
```yaml
- name: Compute checksum
  run: sha256sum ferrite-x86_64-linux-musl.tar.gz > ferrite-x86_64-linux-musl.tar.gz.sha256

- name: Create GitHub Release
  uses: softprops/action-gh-release@v2
  with:
    generate_release_notes: true
    files: |
      ferrite-x86_64-linux-musl.tar.gz
      ferrite-x86_64-linux-musl.tar.gz.sha256
```

### 2.2 — Backend: `POST /api/system/update/apply` (admin-only)

**File**: New module `crates/ferrite-api/src/update.rs` (or inline in `system.rs`)

This is the core of the feature. The handler performs these steps:

1. **Pre-flight checks**:
   - Verify the caller is an admin.
   - Verify updates are not disabled in config.
   - Verify no update is already in progress (use an `AtomicBool` guard).
   - Re-check the latest version to confirm the update is still needed.

2. **Download**:
   - Stream the tarball from `download_url` into a temp file under the data directory (e.g. `data/.update/ferrite-update.tar.gz`).
   - Report download progress via a simple polling endpoint or SSE (see 2.4).

3. **Verify integrity**:
   - Download the `.sha256` checksum file from the release.
   - Compute SHA-256 of the downloaded tarball.
   - Compare. Abort if mismatch.

4. **Extract to staging**:
   - Extract tarball to `data/.update/staging/`.
   - Verify the extracted binary exists and is executable.

5. **Atomic swap**:
   - Determine the current binary path via `std::env::current_exe()`.
   - Determine the current static dir (from `resolve_spa_dir()` logic).
   - Rename current binary → `ferrite.bak`.
   - Move staged binary → current binary path.
   - If a `static/` dir exists in the staging area, swap it too.
   - On failure at any step, roll back from `.bak`.

6. **Trigger restart**:
   - Return a success response to the client first.
   - Write a `data/.update/pending-validation` marker file (see 2.3).
   - After a short delay (500ms), initiate graceful shutdown.
   - **Default strategy**: exit with code 42. The wrapper script (Phase 3.3)
     or process manager (`screen`, `systemd`, cron `@reboot`) restarts the
     new binary.
   - **Why not `exec`**: `exec()` replaces the process *including* the tokio
     runtime — any in-flight DB writes, HLS sessions, or webhook dispatches
     will be killed mid-operation. The exit-code approach allows Axum to
     complete graceful shutdown first.

```rust
// Pseudocode for the restart strategy
async fn restart_with_new_binary() {
    // Write validation marker so the wrapper script can detect a failed update
    let marker = data_dir().join(".update/pending-validation");
    let _ = std::fs::write(&marker, "");

    // Give the HTTP response time to flush
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Exit with known code, let process manager / wrapper script restart
    std::process::exit(42);
}
```

### 2.3 — Rollback Safety

- Keep `ferrite.bak` and `static.bak/` until the next successful update.
- Add a `POST /api/system/update/rollback` endpoint (admin-only) that swaps
  back. **Must be POST** — this is a state-mutating operation; GET would risk
  accidental triggers from prefetch, crawlers, or browser link preview.
- On startup, if a `.update/pending-rollback` marker file exists, auto-rollback.

**Startup self-test (concrete mechanism)**:

1. During Phase 2.2 step 6, the update writes a `data/.update/pending-validation`
   marker file *before* exiting.
2. On startup, if the marker exists, the server attempts to bind and serve
   `/api/health`. If it succeeds, it deletes the marker — update validated.
3. If the new binary crashes before deleting the marker, the wrapper script
   (Phase 3.3) detects the marker on the *next* restart attempt and
   automatically swaps `ferrite.bak` → `ferrite`, then restarts.
4. This gives automatic recovery without requiring manual SSH intervention.

### 2.4 — Update Progress Reporting

**Endpoint**: `GET /api/system/update/status`

Returns the current state of an in-progress update:
```json
{
  "state": "downloading",       // idle | downloading | verifying | extracting | swapping | restarting | failed
  "progress_pct": 45,           // 0-100 for download phase
  "downloaded_bytes": 5600000,
  "total_bytes": 12345678,
  "error": null
}
```

The UI polls this every second while an update is in progress, showing a progress bar.

### 2.5 — Frontend: Update Flow

**Settings page update panel** (only visible to admins):

```
┌─────────────────────────────────────────────────┐
│  System Update                                  │
│                                                 │
│  Current version: v0.1.37                       │
│  Latest version:  v0.1.38  ✨ Update available  │
│                                                 │
│  Release notes:                                 │
│  • Fixed watcher test for cross-platform...     │
│  • Added DLNA improvements...                   │
│                                                 │
│  [Apply Update]                                 │
│                                                 │
│  ── Applying update... ──────────────────────── │
│  ████████████░░░░░░░░  45% — Downloading...     │
│                                                 │
└─────────────────────────────────────────────────┘
```

After the update completes and the server restarts:
- The UI detects the connection drop (fetch fails).
- It waits 5 seconds (the new binary needs time to start and bind).
- Then it polls `GET /api/health` every 2 seconds.
- Once the server is back, it reloads the page.
- A toast confirms: "Updated to v0.1.38 successfully!"

---

## Phase 3: Hardening & Polish

### 3.1 — Startup Version Announcement

On startup, log and expose the version prominently:
```
INFO Ferrite v0.1.38 starting on http://0.0.0.0:12335
```

Already partially done via `env!("CARGO_PKG_VERSION")` in `system::info`.

### 3.2 — Automatic Update Check (Background)

- On server startup, spawn a background task that checks for updates every 6 hours.
- If an update is found, log an INFO message.
- Optionally fire a webhook event (`system.update_available`) so external tools can notify.
- Never auto-apply — always require explicit admin action.

### 3.3 — Wrapper Script for Restart Resilience

Ship an optional `ferrite-run.sh` wrapper:
```bash
#!/bin/bash
cd "$(dirname "$0")"
MARKER="data/.update/pending-validation"

while true; do
    # If a pending-validation marker exists and we have a .bak, the previous
    # update failed to start — roll back automatically.
    if [ -f "$MARKER" ] && [ -f "ferrite.bak" ]; then
        echo "[ferrite-run] Update validation failed, rolling back to ferrite.bak..."
        mv ferrite ferrite.failed
        mv ferrite.bak ferrite
        rm -f "$MARKER"
        echo "[ferrite-run] Rollback complete, restarting..."
    fi

    ./ferrite "$@"
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 42 ]; then
        echo "[ferrite-run] Ferrite requested restart (update applied), restarting..."
        sleep 1
        continue
    fi
    echo "[ferrite-run] Ferrite exited with code $EXIT_CODE, not restarting."
    break
done
```

Users who run via `screen -dmS ferrite ./ferrite-run.sh` get automatic restart
after updates **and** automatic rollback if the new binary fails to start.
Document this as the recommended production setup.

### 3.4 — Update History

Store a simple JSON log at `data/.update/history.json`:
```json
[
  {
    "from_version": "0.1.37",
    "to_version": "0.1.38",
    "applied_at": "2026-02-24T02:30:00Z",
    "applied_by": "admin",
    "success": true
  }
]
```

Expose via `GET /api/system/update/history` for the Settings UI.

---

## File Change Summary

| File | Change |
|---|---|
| `crates/ferrite-core/src/config.rs` | Add `UpdateConfig` struct, wire into `AppConfig` |
| `crates/ferrite-api/src/handlers/system.rs` | Add `check_for_update`, `apply_update`, `update_status`, `rollback_update` handlers |
| `crates/ferrite-api/src/router.rs` | Register `/api/system/update/*` routes |
| `crates/ferrite-api/src/state.rs` | Add `update_state: Arc<UpdateState>` to `AppState` |
| `crates/ferrite-server/src/main.rs` | Initialize update state, log version on startup |
| `.github/workflows/release.yml` | Add SHA-256 checksum generation + upload |
| `scripts/ferrite-run.sh` | New restart-resilient wrapper script |
| `ferrite-ui/src/pages/Settings.tsx` | Update check UI, apply button, progress bar, reconnect logic |

## Dependencies

Minimal additions to the workspace `Cargo.toml`:
- **`reqwest`** — already in workspace; **add `"stream"` feature** for `response.bytes_stream()` download progress reporting
- **`sha2`** — new dependency for checksum verification (small, purpose-built; `ring` is available via `jsonwebtoken` but its digest API is less ergonomic for streaming hashes)
- **`semver`** — for proper version comparison (currently can use simple string compare since versions are `0.1.x`)

```toml
# Workspace Cargo.toml changes:
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls", "stream"] }
sha2 = "0.10"
```

---

## Implementation Order

1. **Phase 2.1** — SHA-256 checksum in release workflow *(do first so the next tag already has checksums)*
2. **Phase 1.3** — `UpdateConfig` in `config.rs`
3. **Phase 1.1** — `GET /api/system/update/check` endpoint
4. **Phase 1.2** — UI update badge on Settings page
5. **Phase 2.2** — `POST /api/system/update/apply` endpoint
6. **Phase 2.4** — `GET /api/system/update/status` polling endpoint
7. **Phase 2.5** — UI progress bar + reconnect flow
8. **Phase 2.3** — Rollback safety net
9. **Phase 3.3** — Wrapper script + docs
10. **Phase 3.2** — Background periodic check
11. **Phase 3.4** — Update history log *(lowest priority, nice-to-have)*

Estimated effort: ~3-4 sessions of focused work. Phase 1 alone (check + badge) is a single session and delivers immediate value.

---

## Security Considerations

- **Admin-only**: All update endpoints require admin authentication.
- **HTTPS only**: GitHub API and download URLs are HTTPS. `reqwest` with `rustls-tls` handles this.
- **Checksum verification**: SHA-256 of the downloaded tarball must match the published checksum. This prevents corruption and MITM attacks on the download.
- **Trust boundary caveat**: Both the tarball and `.sha256` file come from the same GitHub release. A compromised GitHub account would compromise both. This is acceptable for the threat model (we trust GitHub as the release host). A future enhancement could add GPG signature verification for defense-in-depth.
- **No arbitrary code execution**: The update only replaces the Ferrite binary and static assets from a known GitHub release. No shell commands, no eval.
- **Rollback**: If the new binary fails to start, the `.bak` copy is available for automatic recovery (via wrapper script) or manual recovery via SSH.
- **Config preserved**: Updates never touch `config/`, `data/`, or `ferrite.db`. Only the binary and `static/` are replaced.
- **Rate limiting**: Unauthenticated GitHub API requests are limited to 60/hr per IP. The 15-minute cache (Phase 1.1) and 6-hour background check (Phase 3.2) stay well within this. Users needing higher limits can set `github_token` in config or the `GITHUB_TOKEN` env var.
