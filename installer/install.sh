#!/usr/bin/env bash
# KelpyShark Installer — Linux / macOS
#
# Usage:
#   curl -fsSL https://github.com/KelpyShark/main/raw/main/installer/install.sh | bash
#
# What this does:
#   1. Checks for Rust/Cargo (installs if missing)
#   2. Clones the KelpyShark repository
#   3. Builds the toolchain from source
#   4. Installs the `kelpy` binary to ~/.kelpyshark/bin
#   5. Adds ~/.kelpyshark/bin to PATH

set -euo pipefail

KELPYSHARK_HOME="${HOME}/.kelpyshark"
KELPYSHARK_BIN="${KELPYSHARK_HOME}/bin"
KELPYSHARK_REPO="https://github.com/kelpyshark/main.git"

echo "KelpyShark Installer"
echo "========================"
echo ""

#  Check prerequisites 

check_command() {
    if ! command -v "$1" &> /dev/null; then
        return 1
    fi
    return 0
}

# Check for git
if ! check_command git; then
    echo -e "[\e[0;31mERROR\e[0m] Git is required but not installed."
    echo "   Please install git first: https://git-scm.com/"
    exit 1
fi

# Check for Rust/Cargo
if ! check_command cargo; then
    echo -e "[\e[0;34mINFO\e[0m] Rust/Cargo not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "${HOME}/.cargo/env"
    echo -e "[\e[0;34mINFO\e[0m] Rust installed."
fi

echo -e "[\e[0;34mINFO\e[0m] Rust version: $(rustc --version)"
echo -e "[\e[0;34mINFO\e[0m] Cargo version: $(cargo --version)"
echo ""

#  Clone / update repository 

TEMP_DIR=$(mktemp -d)
echo -e "[\e[0;34mINFO\e[0m] Cloning KelpyShark..."
git clone --depth 1 "${KELPYSHARK_REPO}" "${TEMP_DIR}/kelpyshark" 2>/dev/null || {
    echo -e "[\e[0;33mWARN\e[0m] Could not clone from remote. Building from local source..."
    TEMP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
}

BUILD_DIR="${TEMP_DIR}/kelpyshark"
if [ ! -f "${BUILD_DIR}/Cargo.toml" ]; then
    BUILD_DIR="${TEMP_DIR}"
fi

#  Build 

echo -e "[\e[0;34mINFO\e[0m] Building KelpyShark..."
cd "${BUILD_DIR}"
cargo build --release --bin kelpyshark --bin ks --bin kshark

#  Install 

echo -e "[\e[0;34mINFO\e[0m] Installing to ${KELPYSHARK_BIN}..."
mkdir -p "${KELPYSHARK_BIN}"
mkdir -p "${KELPYSHARK_HOME}/registry"

cp "${BUILD_DIR}/target/release/kelpyshark" "${KELPYSHARK_BIN}/kelpyshark"
cp "${BUILD_DIR}/target/release/ks"         "${KELPYSHARK_BIN}/ks"
cp "${BUILD_DIR}/target/release/kshark"     "${KELPYSHARK_BIN}/kshark"
chmod +x "${KELPYSHARK_BIN}/kelpyshark"
chmod +x "${KELPYSHARK_BIN}/ks"
chmod +x "${KELPYSHARK_BIN}/kshark"

#  Update PATH 

SHELL_RC=""
if [ -f "${HOME}/.bashrc" ]; then
    SHELL_RC="${HOME}/.bashrc"
elif [ -f "${HOME}/.zshrc" ]; then
    SHELL_RC="${HOME}/.zshrc"
elif [ -f "${HOME}/.profile" ]; then
    SHELL_RC="${HOME}/.profile"
fi

if [ -n "${SHELL_RC}" ]; then
    if ! grep -q "kelpyshark" "${SHELL_RC}" 2>/dev/null; then
        echo "" >> "${SHELL_RC}"
        echo "# KelpyShark" >> "${SHELL_RC}"
        echo "export PATH=\"${KELPYSHARK_BIN}:\$PATH\"" >> "${SHELL_RC}"
        echo -e "[\e[0;34mINFO\e[0m] Added ${KELPYSHARK_BIN} to PATH in ${SHELL_RC}"
    fi
fi

export PATH="${KELPYSHARK_BIN}:${PATH}"

echo ""
echo "   KelpyShark installed successfully!"
echo ""
echo "   You can run programs with any of:"
echo "   kelpyshark run hello.ks"
echo "   kshark run hello.ks"
echo "   ks run hello.ks"
echo ""
echo "   Restart your shell or run:"
echo "   source ${SHELL_RC:-~/.bashrc}"
