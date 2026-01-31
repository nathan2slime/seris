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

## 🔐 Environment Variables

Create a `.env` file in the project root:

```env
DISCORD_TOKEN=""
RUST_LOG="info"
DATABASE_URL="sqlite::memory:seris"
NASA_API_KEY=""
```

* `DISCORD_TOKEN`: Discord bot token
* `RUST_LOG`: Log level
* `DATABASE_URL`: Database connection
* `NASA_API_KEY`: Required for NASA commands

---

## ▶️ Running Locally

```bash
cargo run --release
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
