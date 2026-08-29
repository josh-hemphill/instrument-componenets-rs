#!/usr/bin/env bash
# Idempotent Cloud Agent bootstrap: Rust toolchain, Deno, .NET 8 SDK, docs deps.
set -euo pipefail

readonly DOTNET_CHANNEL="8.0"
readonly DOTNET_INSTALL_DIR="/usr/local/dotnet"
readonly DENO_INSTALL_DIR="/usr/local"
readonly WINDOWS_TARGET="x86_64-pc-windows-gnu"
readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

log() { printf '\n[install] %s\n' "$1"; }

# Rust: dependencies require the 2024 edition (rustc >= 1.85), so track the
# latest stable and add the components/target the CI matrix exercises.
log "Updating Rust stable toolchain"
rustup update stable
rustup default stable
rustup component add clippy rustfmt
rustup target add "${WINDOWS_TARGET}"

# Deno powers the shared-table / dialect / registry code generators.
if command -v deno >/dev/null 2>&1; then
  log "Deno already installed ($(deno --version | head -n1))"
else
  log "Installing Deno"
  curl -fsSL https://deno.land/install.sh | sudo env DENO_INSTALL="${DENO_INSTALL_DIR}" sh -s -- -y
fi

# .NET 8 SDK builds and tests the C# port under dotnet/.
if command -v dotnet >/dev/null 2>&1; then
  log ".NET SDK already installed ($(dotnet --version))"
else
  log "Installing .NET ${DOTNET_CHANNEL} SDK"
  curl -fsSL https://dot.net/v1/dotnet-install.sh -o /tmp/dotnet-install.sh
  chmod +x /tmp/dotnet-install.sh
  sudo /tmp/dotnet-install.sh --channel "${DOTNET_CHANNEL}" --install-dir "${DOTNET_INSTALL_DIR}"
  sudo ln -sf "${DOTNET_INSTALL_DIR}/dotnet" /usr/local/bin/dotnet
fi

# Python packages for the MkDocs documentation site.
log "Installing MkDocs documentation dependencies"
pip install --user --disable-pip-version-check -r "${REPO_ROOT}/docs-site/requirements.txt"

# Warm dependency caches so the first agent action is fast.
log "Fetching Rust dependencies"
cargo fetch --manifest-path "${REPO_ROOT}/Cargo.toml"

log "Restoring .NET dependencies"
DOTNET_NOLOGO=1 DOTNET_CLI_TELEMETRY_OPTOUT=1 dotnet restore "${REPO_ROOT}/dotnet/InstrumentComponents.sln"

log "Bootstrap complete"
