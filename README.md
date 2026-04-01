# Seris

A calm and precise Discord bot written in **Rust**, built with **serenity**.

Seris speaks softly, but executes with certainty. Designed to be minimal, reliable, and efficient — nothing more than what is needed.

---

## 🌙 Philosophy

> *Silence over noise. Clarity over excess.*

Seris is not flashy. She focuses on correctness, stability, and clean execution. Every feature exists for a reason.

---

## ✨ Features

* Slash commands (Discord Interactions)
* Clean permission boundaries
* Predictable behavior
* Minimal Docker footprint
* Fast startup, low memory usage

---

## 🧭 Commands

All commands are **slash commands** (`/`).

1. **Ping**
   `/ping` — Confirms responsiveness.

2. **Clear Messages**
   `/clear` — Removes messages (restricted permissions).

3. **NASA – Astronomy Picture of the Day**
   `/nasa apod` — Displays NASA’s daily image.

4. **Random Anime**
   `/anime random` — Suggests an anime title.

---

## ⚙️ Requirements

* Rust **1.83+**
* Discord Bot Token
* Optional: Docker

---

## 🔐 Configuration

Seris loads its settings from a TOML config file.

Default config file locations:

* Linux/macOS: `~/.config/seris/config.toml`
* Custom path: set `SERIS_CONFIG_FILE=/path/to/config.toml`

Example `config.toml`:

```toml
discord_token = "..."
nasa_api_key = "..."
```
* `discord_token`: Discord bot token
* `nasa_api_key`: Required for NASA commands
* `SERIS_CONFIG_FILE`: Optional custom path to the TOML config file

---

## ▶️ Running Locally

```bash
cargo run --release
```

## 📦 GitHub Release Assets

When a GitHub Release is published, the workflow in `.github/workflows/release-assets.yml` builds a Linux release bundle and attaches it to the release.

Included assets:

* `seris-<tag>-x86_64-unknown-linux-gnu.tar.gz`
* `seris-<tag>-x86_64-unknown-linux-gnu.tar.gz.sha256`

The archive contains:

* `seris`
* `install.sh`
* `config.example.toml`
* `README.md`

## 🛠️ System Install With Boot Service

The release archive includes an `install.sh` for Linux hosts using `systemd`.

Example:

```bash
tar -xzf seris-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
cd seris-v0.1.0-x86_64-unknown-linux-gnu
sudo ./install.sh
```

The installer:

* copies the binary to `/opt/seris/seris`
* creates `/opt/seris/.config/seris/config.toml` when missing
* creates a `seris` system user/group
* installs `/etc/systemd/system/seris.service`
* runs `systemctl enable --now seris.service`

After installation, fill in `/opt/seris/.config/seris/config.toml` and restart if needed:

```bash
sudo systemctl restart seris.service
```

---

## 🐳 Docker (Minimal Image)

Seris is built to run in **ultra-minimal containers**.

### Build

```bash
docker build -t seris .
```

### Run

```bash
docker run --env-file .env seris
```

* Image size: **~4–6 MB**
* Static binary
* No shell, no package manager
* Runs as non-root

---

## 🧼 Production Notes

* Uses `rustls` (no OpenSSL)
* Compatible with `scratch` or `distroless`
* Reduced attack surface
* Deterministic behavior

---

## 📜 License

MIT

---

> *Seris does not rush. She executes.*
