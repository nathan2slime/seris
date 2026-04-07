# Deployment Guide

## Docker

```bash
docker build -t seris .
docker run -it --env-file .env -p 8080:8080 seris
```

The container exposes port `8080` for `/health` and `/ready`, and the TUI is shown when a terminal is attached.

## systemd

Install the release bundle or binary, then manage it with the CLI:

```bash
seris service status
seris service restart
seris service logs --follow
```

The bundled installer places files under `/opt/seris` and configures a `systemd` service.

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

## Recommendations

* Run the container or service with the health endpoint reachable.
* Keep the config file out of the image when using external secrets.
* Monitor `/ready` rather than just process liveness.
* Use an interactive terminal if you want the dashboard UI.
