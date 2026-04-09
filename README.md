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
* Plugin-based command registry
* SQLite-backed persistence with a connection pool
* Benchmark helpers for critical paths
* Minimal Docker footprint
* Fast startup, low memory usage
* Standard runtime logs for bot lifecycle and errors

---

## 🧭 Commands

All commands are **slash commands** (`/`).

1. **Ping**
   `/ping` — Confirms responsiveness.

2. **Clear Messages**
   `/clear` — Removes messages (restricted permissions).

3. **NASA – Astronomy Picture of the Day**
   `/nasa apod` — Displays NASA’s daily image or video.

4. **Random Anime**
   `/anime random` — Suggests an anime title.

5. **Utility Commands**
   `/about` — Shows bot metadata.
   `/uptime` — Shows process uptime.
   `/stats` — Shows persisted command stats.

## ⚡ Benchmarking

Lightweight benchmark-style checks live as ignored tests, including memory sampling.

Run them with:

```bash
cargo test --locked --all-features -- --ignored --nocapture
```

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

Seris reads application settings from TOML, with environment fallbacks for secrets. `SERIS_CONFIG_FILE` still overrides the config file path.

Build flags:

* `bot` - Discord bot functionality

Environment variable fallbacks are also supported for secrets:

* `SERIS_DISCORD_TOKEN`
* `SERIS_NASA_API_KEY`
* `SERIS_DB_FILE`

Monitoring:

* `GET /health` on port `8080` returns `200` when the process is up.
* `GET /ready` returns `200` after Discord is connected and `503` otherwise.
* The Docker image includes a `HEALTHCHECK` against `/ready`.

---

## 🧱 Architecture

```mermaid
flowchart LR
  Discord[Discord Gateway] --> Handler[Serenity event handler]
  Handler --> Commands[Slash commands]
  Commands --> Embeds[Embed builders]
  Commands --> Services[API clients]
  Services --> Jikan[Jikan random anime/manga]
  Services --> NASA[NASA APOD]
  Handler --> Health[Health state]
  Health --> HealthHTTP[HTTP /health and /ready]
  HealthHTTP --> Docker[Docker healthcheck]
```

The bot keeps the command layer thin: commands call services, services fetch API data, and embeds format the response for Discord. SQLite uses a small connection pool so reads and writes do not serialize on a single handle.

See also:

* `docs/architecture.md`
* `docs/api-endpoints.md`

---

## 🛟 Troubleshooting

* If the bot exits immediately, confirm `discord_token` and `nasa_api_key` are set in the config file or environment.
* If `/ready` keeps returning `503`, the Discord client has not connected yet or lost its shard connection.
* If Docker reports an unhealthy container, confirm port `8080` is exposed and no other process is binding it.
* If the binary cannot find its config, confirm `SERIS_CONFIG_FILE` is set or that `~/.config/seris/config.toml` exists.

---

## 🚀 Deployment

Platform-specific deployment notes live in `docs/deployment.md`.

## ▶️ Running Locally

```bash
cargo run --release
```

Seris prints standard logs to the terminal by default. Use `RUST_LOG` to adjust verbosity.
To update an installed binary, run `seris self-update` on Linux x86_64.

## 🖥️ Runtime Signals

* `/health` - liveness check on port `8080`
* `/ready` - Discord readiness check on port `8080`
* Logs include startup, connection, shutdown, and command failure events

## 📦 GitHub Release Assets

When a GitHub Release is published, `.github/workflows/release-assets.yml` builds and uploads a raw Linux binary plus a bundled Linux installer.

Included assets:

* `seris-<tag>-x86_64-unknown-linux-gnu`
* `seris-<tag>-x86_64-unknown-linux-gnu.tar.gz`
* `install.sh`
* matching `.sha256` files

Bundle contents:

* `seris`
* `config.example.toml`
* `README.md`

## 🛠️ Automatic Installers

The Linux bootstrap installer downloads the Linux bundle, installs `seris` into `~/.local/bin`, and wires the config path through `SERIS_CONFIG_FILE` in `~/.bashrc`.
The default config file lives at `~/.config/seris/config.toml`.

```bash
curl -fsSL https://github.com/nathan2slime/seris/releases/download/<tag>/install.sh | sh
```

Optional version override:

```bash
curl -fsSL https://github.com/nathan2slime/seris/releases/download/<tag>/install.sh | sh -s -- <tag>
```

The installer adds these lines to `~/.bashrc` if they are missing:

```bash
export PATH="$HOME/.local/bin:$PATH"
export SERIS_CONFIG_FILE="$HOME/.config/seris/config.toml"
```

Source `~/.bashrc` or open a new shell after installation.

---

## 📚 API Endpoints

The external and internal endpoints used by Seris are documented in `docs/api-endpoints.md`.

---

## 🐳 Docker (Minimal Image)

Seris is built to run in **ultra-minimal containers**.

### Build

```bash
docker build -t seris .
```

### Run

```bash
docker run -it --env-file .env -p 8080:8080 seris
```

* Image size: **~4–6 MB**
* Static binary
* No shell, no package manager
* Runs as non-root
* Prints logs to stdout/stderr

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
