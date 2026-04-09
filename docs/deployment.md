# Deployment Guide

## Docker

```bash
docker build -t seris .
docker run -it --env-file .env -p 8080:8080 seris
```

The container exposes port `8080` for `/health` and `/ready`, and the bot writes standard logs to stdout/stderr.

## Local install

Install the release bundle, then run the binary from your user bin directory:

```bash
seris
```

The bundled installer places `seris` in `~/.local/bin` and sets `SERIS_CONFIG_FILE` in `~/.bashrc`.
The default config file lives at `~/.config/seris/config.toml`.
You can update an installed release with `seris self-update` on Linux x86_64.

## GitHub Releases

Published releases include:

* the Linux binary
* a tarball bundle
* `install.sh` for automated installation

Use the installer when you want a hands-off Linux deployment.

## Environment

Required values:

* `SERIS_DISCORD_TOKEN` or `discord_token` in the config file
* `SERIS_NASA_API_KEY` or `nasa_api_key` in the config file
* optional `SERIS_CONFIG_FILE` to override the config path
* optional `SERIS_DB_FILE` to override the SQLite database path

## Recommendations

* Run the container or service with the health endpoint reachable.
* Keep the config file out of the image when using external secrets.
* Monitor `/ready` rather than just process liveness.
* Set `RUST_LOG=info` (or more specific filters) if you want more verbose runtime logs.
* Persisted command stats are stored in the SQLite file, defaulting to `~/.local/share/seris/seris.sqlite3`.
* SQLite connections are pooled internally to reduce contention under load.
