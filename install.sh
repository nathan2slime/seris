#!/bin/sh

set -eu

REPO_OWNER="nathan2slime"
REPO_NAME="seris"
APP_NAME="seris"
DEFAULT_VERSION="__SERIS_DEFAULT_VERSION__"

say() {
    printf '%s\n' "$1"
}

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        say "[seris-chan] I need '$1' to continue."
        exit 1
    fi
}

detect_arch() {
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)
            printf '%s\n' "x86_64"
            ;;
        *)
            say "[seris-chan] Unsupported architecture: ${arch}"
            exit 1
            ;;
    esac
}

detect_os() {
    os="$(uname -s)"
    case "$os" in
        Linux)
            ;;
        *)
            say "[seris-chan] This installer is for Linux only."
            exit 1
            ;;
    esac
}

resolve_version() {
    if [ "${1:-}" != "" ]; then
        printf '%s\n' "$1"
        return
    fi

    if [ "${SERIS_VERSION:-}" != "" ]; then
        printf '%s\n' "${SERIS_VERSION}"
        return
    fi

    if [ "${DEFAULT_VERSION}" != "__SERIS_DEFAULT_VERSION__" ]; then
        printf '%s\n' "${DEFAULT_VERSION}"
        return
    fi

    require_command curl
    curl -fsSL "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1
}

download_and_install() {
    version="$1"
    arch="$2"
    target="${arch}-unknown-linux-gnu"
    archive="${APP_NAME}-${version}-${target}.tar.gz"
    url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${version}/${archive}"
    temp_dir="$(mktemp -d)"

    cleanup() {
        rm -rf "${temp_dir}"
    }
    trap cleanup EXIT INT TERM

    say "[seris-chan] Downloading ${archive} from release ${version}..."
    curl -fL "${url}" -o "${temp_dir}/${archive}"

    say "[seris-chan] Unpacking the Linux release bundle..."
    tar -xzf "${temp_dir}/${archive}" -C "${temp_dir}"

    package_dir="${temp_dir}/${APP_NAME}-${version}-${target}"
    if [ ! -d "${package_dir}" ]; then
        say "[seris-chan] I could not find the extracted package directory."
        exit 1
    fi

    chmod +x "${package_dir}/install-local.sh"
    say "[seris-chan] Handing things over to the local installer..."
    (
        cd "${package_dir}"
        sh ./install-local.sh
    )
}

require_command uname
require_command mktemp
require_command tar
require_command chmod
require_command rm
require_command sed
require_command head
require_command curl

version="$(resolve_version "${1:-}")"
if [ -z "${version}" ]; then
    say "[seris-chan] I could not determine which release to install."
    exit 1
fi

detect_os
arch="$(detect_arch)"
download_and_install "${version}" "${arch}"
