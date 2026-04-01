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

* Linux/macOS: `$XDG_CONFIG_HOME/seris/config.toml` when `XDG_CONFIG_HOME` is set
* Otherwise: `~/.config/seris/config.toml`
* Custom path: set `SERIS_CONFIG_FILE=/path/to/config.toml`

Example `config.toml`:

```toml
discord_token = "..."
nasa_api_key = "..."
```
* `discord_token`: Discord bot token
* `nasa_api_key`: Required for NASA commands

Seris reads application settings from TOML only. `SERIS_CONFIG_FILE` is only used to override the config file path.

---

## ▶️ Running Locally

```bash
cargo run --release
```

## 🧰 Admin CLI

The installed `seris` binary also provides a small Linux admin CLI.

Examples:

```bash
seris version
seris config path
seris config edit
seris service status
seris service restart
seris service logs --follow
seris self-update v1.0.1
```

Commands that touch the installed config file or the `systemd` service will prompt for `sudo` automatically when needed.

## 📦 GitHub Release Assets

When a GitHub Release is published, `.github/workflows/release-assets.yml` builds and uploads a raw Linux binary plus a bundled Linux installer.

Included assets:

* `seris-<tag>-x86_64-unknown-linux-gnu`
* `seris-<tag>-x86_64-unknown-linux-gnu.tar.gz`
* `install.sh`
* matching `.sha256` files

Bundle contents:

* `seris`
* `install-local.sh`
* `config.example.toml`
* `README.md`

## 🛠️ Automatic Installers

The Linux bootstrap installer downloads the Linux bundle, installs `seris` into `/opt/seris`, creates a `systemd` service, and enables it on boot.

```bash
curl -fsSL https://github.com/nathan2slime/seris/releases/download/<tag>/install.sh | sudo sh
```

Optional version override:

```bash
curl -fsSL https://github.com/nathan2slime/seris/releases/download/<tag>/install.sh | sudo sh -s -- <tag>
```

### Local bundle installers

If you prefer extracting a release bundle manually, run the bundled local installer instead:

* Linux: `sudo ./install-local.sh`

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
