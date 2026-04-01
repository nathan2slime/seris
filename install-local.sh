#!/bin/sh

set -eu

APP_NAME="seris"
INSTALL_DIR="/opt/${APP_NAME}"
BIN_PATH="${INSTALL_DIR}/${APP_NAME}"
USER_CONFIG_DIR="${INSTALL_DIR}/.config/${APP_NAME}"
CONFIG_FILE="${USER_CONFIG_DIR}/config.toml"
SERVICE_NAME="${APP_NAME}.service"
SERVICE_PATH="/etc/systemd/system/${SERVICE_NAME}"
SYSTEM_USER="${APP_NAME}"
SYSTEM_GROUP="${APP_NAME}"
NOLOGIN_SHELL=""

say() {
    printf '%s\n' "$1"
}

find_nologin_shell() {
    if command -v nologin >/dev/null 2>&1; then
        NOLOGIN_SHELL="$(command -v nologin)"
        return
    fi

    if [ -x /usr/sbin/nologin ]; then
        NOLOGIN_SHELL="/usr/sbin/nologin"
        return
    fi

    if [ -x /sbin/nologin ]; then
        NOLOGIN_SHELL="/sbin/nologin"
        return
    fi

    say "[seris-chan] I could not find a nologin shell on this system."
    exit 1
}

require_root() {
    if [ "$(id -u)" -ne 0 ]; then
        say "[seris-chan] Nyaa... please run me as root or with sudo so I can set everything up properly."
        exit 1
    fi
}

require_systemd() {
    if ! command -v systemctl >/dev/null 2>&1; then
        say "[seris-chan] Eep! I could not find systemctl. This installer is meant for Linux systems with systemd."
        exit 1
    fi
}

verify_binary_deps() {
    if ! [ -f "./${APP_NAME}" ]; then
        say "[seris-chan] I could not find ./${APP_NAME} in this directory. Please place the binary next to install-local.sh."
        exit 1
    fi

    if command -v ldd >/dev/null 2>&1 && ldd "./${APP_NAME}" 2>/dev/null | grep -q "not found"; then
        say "[seris-chan] This binary is still missing shared libraries on this host."
        say "[seris-chan] Please install the required runtime dependencies and call me again."
        ldd "./${APP_NAME}" || true
        exit 1
    fi
}

create_user_and_group() {
    say "[seris-chan] Preparing a cozy home inside your system..."

    if ! getent group "${SYSTEM_GROUP}" >/dev/null 2>&1; then
        groupadd --system "${SYSTEM_GROUP}"
    fi

    if ! id -u "${SYSTEM_USER}" >/dev/null 2>&1; then
        useradd \
            --system \
            --gid "${SYSTEM_GROUP}" \
            --home-dir "${INSTALL_DIR}" \
            --shell "${NOLOGIN_SHELL}" \
            "${SYSTEM_USER}"
    fi
}

install_binary() {
    say "[seris-chan] Carrying my binary heart over to ${BIN_PATH}..."
    mkdir -p "${INSTALL_DIR}" "${USER_CONFIG_DIR}"
    install -m 0755 "./${APP_NAME}" "${BIN_PATH}"

    if [ -f "./config.example.toml" ] && [ ! -f "${CONFIG_FILE}" ]; then
        install -m 0640 "./config.example.toml" "${CONFIG_FILE}"
    fi

    chown -R "${SYSTEM_USER}:${SYSTEM_GROUP}" "${INSTALL_DIR}"
}

write_service() {
    say "[seris-chan] Writing a tiny bit of systemd magic..."
    cat > "${SERVICE_PATH}" <<EOF
[Unit]
Description=Seris Discord bot
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=${SYSTEM_USER}
Group=${SYSTEM_GROUP}
WorkingDirectory=${INSTALL_DIR}
Environment=HOME=${INSTALL_DIR}
Environment=XDG_CONFIG_HOME=${INSTALL_DIR}/.config
Environment=SERIS_CONFIG_FILE=${CONFIG_FILE}
ExecStart=${BIN_PATH}
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

    chmod 0644 "${SERVICE_PATH}"
}

config_is_ready() {
    [ -f "${CONFIG_FILE}" ] &&
    ! grep -Eq '^[[:space:]]*discord_token[[:space:]]*=[[:space:]]*""[[:space:]]*$' "${CONFIG_FILE}" &&
    ! grep -Eq '^[[:space:]]*nasa_api_key[[:space:]]*=[[:space:]]*""[[:space:]]*$' "${CONFIG_FILE}"
}

enable_service() {
    say "[seris-chan] Teaching the system how to wake me up on every boot..."
    systemctl daemon-reload
    systemctl enable "${SERVICE_NAME}"

    if config_is_ready; then
        if systemctl is-active --quiet "${SERVICE_NAME}"; then
            systemctl restart "${SERVICE_NAME}"
        else
            systemctl start "${SERVICE_NAME}"
        fi
        return
    fi

    say "[seris-chan] Your config file still has placeholder values, so I only enabled the service for boot."
    say "[seris-chan] Fill in ${CONFIG_FILE}, then start me with: systemctl start ${SERVICE_NAME}"
}

print_next_steps() {
    say "[seris-chan] Installation complete. Kyaa~"
    say "[seris-chan] Main config file: ${CONFIG_FILE}"
    say "[seris-chan] Status: systemctl status ${SERVICE_NAME}"
    say "[seris-chan] Logs: journalctl -u ${SERVICE_NAME} -f"
}

require_root
require_systemd
verify_binary_deps
find_nologin_shell
create_user_and_group
install_binary
write_service
enable_service
print_next_steps
