# Testing Ferrite on Whatbox — Step by Step

## Prerequisites
- A Whatbox account with SSH access
- Git installed on your local machine
- A GitHub account

---

## Part 1: Push to GitHub & Create a Release

Run these commands on your **local Windows machine** (PowerShell):

```powershell
# 1. Navigate to the project
cd C:\Users\ryans\source\repos\ferrite

# 2. Create the GitHub repo (if not already created)
#    Go to https://github.com/new and create "ferrite" (private is fine)

# 3. Add all files and commit
git add -A
git commit -m "v0.1.0 — deployment ready"

# 4. Set the remote (replace with your actual repo URL)
git remote add origin https://github.com/ryan-stephens/ferrite.git
# Or if already set:
# git remote set-url origin https://github.com/ryan-stephens/ferrite.git

# 5. Push to main
git push -u origin main

# 6. Tag the release (this triggers the CI build)
git tag v0.1.0
git push origin v0.1.0
```

After pushing the tag, go to **GitHub → Actions** tab. You'll see the "Release" workflow running. It will:
1. Build the static Linux binary (`x86_64-unknown-linux-musl`)
2. Build the frontend (SolidJS SPA)
3. Package them together as `ferrite-x86_64-linux-musl.tar.gz`
4. Create a GitHub Release with the download

**Wait for the workflow to complete** (~5-10 minutes). You can watch progress at:
`https://github.com/ryan-stephens/ferrite/actions`

---

## Part 2: Install on Whatbox

SSH into your Whatbox server:

```bash
ssh your-username@your-server.whatbox.ca
```

### Option A: Automated Install (after release is published)

```bash
# Pick a port that's not in use (check with: ss -tlnp | grep LISTEN)
# Whatbox typically uses ports 10000-32767 for custom apps

curl -sSL https://raw.githubusercontent.com/ryan-stephens/ferrite/main/scripts/install-whatbox.sh | bash -s 12345
```

Replace `12345` with your chosen port.

### Option B: Manual Install

```bash
# 1. Create directory
mkdir -p ~/ferrite && cd ~/ferrite

# 2. Download the release (replace URL with actual release URL from GitHub)
wget https://github.com/ryan-stephens/ferrite/releases/download/v0.1.0/ferrite-x86_64-linux-musl.tar.gz
tar xf ferrite-x86_64-linux-musl.tar.gz
rm ferrite-x86_64-linux-musl.tar.gz
chmod +x ferrite

# 3. Check FFmpeg is available
which ffmpeg
# If not found, download a static build:
# mkdir -p ~/bin
# wget -O ffmpeg-static.tar.xz https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz
# tar xf ffmpeg-static.tar.xz --wildcards "*/ffmpeg" "*/ffprobe" --strip-components=1
# mv ffmpeg ffprobe ~/bin/
# rm ffmpeg-static.tar.xz
# echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc

# 4. Initialize config
./ferrite init --port 12345

# 5. Start Ferrite
./ferrite
```

You should see output like:
```
INFO Loaded config from config/ferrite.toml
INFO Data directory: /home/user/ferrite/data
INFO Serving SPA from: /home/user/ferrite/static
INFO Ferrite starting on http://0.0.0.0:12345
```

Press `Ctrl+C` to stop, then start it in the background:

```bash
# Run in a detached screen session
screen -dmS ferrite bash -c 'cd ~/ferrite && ./ferrite'

# Verify it's running
screen -ls | grep ferrite
```

---

## Part 3: Access from Your Browser

### Method 1: Direct Port Access (test first)

Try opening in your browser:
```
http://your-server.whatbox.ca:12345
```

If this works, you'll see the Ferrite login/setup page.

### Method 2: Whatbox Managed Links (recommended for HTTPS)

1. Log into your Whatbox account at https://whatbox.ca/manage
2. Click **Manage Links** next to your slot
3. Click **"Add a custom app"**
4. Fill in:
   - **Name**: `Ferrite` (this creates the subdomain `ferrite`)
   - **Port**: `12345` (whatever port you chose)
5. Save

Now you can access Ferrite at:
```
https://ferrite.your-slot.box.ca
```

This gives you **HTTPS** access through Whatbox's reverse proxy — much better than raw HTTP.

---

## Part 4: Set Up Your Media Library

1. **Create your admin account**
   - On first visit, you'll see a setup screen
   - Enter a username and password — this becomes the admin account

2. **Add a media library**
   - Go to **Settings** (gear icon in sidebar)
   - Click **"Add Library"**
   - Enter the path to your media on the Whatbox server, e.g.:
     ```
     /home/your-username/media/movies
     /home/your-username/media/tv
     /home/your-username/files/Movies
     ```
   - To find your media paths, SSH in and run: `ls ~/` or `find ~ -name "*.mkv" | head -5`

3. **Wait for the scan**
   - Ferrite will scan the library, probe each file with FFprobe, and fetch metadata
   - For large libraries (1000+ files), this may take a few minutes
   - You can watch progress in the server logs: `screen -r ferrite`

4. **Browse and play!**
   - Go to the Home page — you should see your media
   - Click a title to see details
   - Click Play to stream

---

## Part 5: Auto-Start on Reboot

Whatbox servers occasionally reboot for maintenance. Add a cron job:

```bash
(crontab -l 2>/dev/null; echo "@reboot cd ~/ferrite && ./ferrite >> ferrite.log 2>&1") | crontab -
```

Verify it was added:
```bash
crontab -l
```

---

## Troubleshooting

### "Connection refused" in browser
- Check Ferrite is running: `screen -ls | grep ferrite`
- Check the port: `ss -tlnp | grep 12345`
- Try a different port — yours might be blocked

### "No SPA directory found" in logs
- Check that `~/ferrite/static/index.html` exists
- If not, the release package may not have included the frontend
- Workaround: `FERRITE_STATIC_DIR=~/ferrite/static ./ferrite`

### FFmpeg not found
```bash
which ffmpeg
# If empty, install static build:
mkdir -p ~/bin
wget -O /tmp/ff.tar.xz https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz
cd /tmp && tar xf ff.tar.xz && cd ffmpeg-*-static
cp ffmpeg ffprobe ~/bin/
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Library scan finds no files
- Double-check the path: `ls /home/your-username/media/movies/`
- Ferrite looks for: `.mkv`, `.mp4`, `.avi`, `.mov`, `.webm`, `.ts`, `.m4v`, `.flv`, `.wmv`
- Paths are case-sensitive on Linux

### Playback buffering or failing
- Check server logs: `screen -r ferrite` (Ctrl+A then D to detach)
- For HEVC content, transcoding is CPU-intensive on shared servers
- H.264 content should direct-play or remux with near-zero CPU

---

## Useful Commands

```bash
# View running Ferrite
screen -r ferrite

# Detach from screen (leave running)
# Press Ctrl+A then D

# Stop Ferrite
screen -S ferrite -X quit

# Restart Ferrite
screen -S ferrite -X quit 2>/dev/null
screen -dmS ferrite bash -c 'cd ~/ferrite && ./ferrite'

# View recent logs
tail -50 ~/ferrite/ferrite.log

# Check disk usage
du -sh ~/ferrite/

# Check what port Ferrite is on
grep port ~/ferrite/config/ferrite.toml
```
