#!/usr/bin/env bash
# Download cetz_core.wasm (~100KB) from the typst package store and verify
# its SHA-256. Required for `cargo build -p locus-dsl` to compile the
# diagram subsystem (the WASM is referenced by cetz at typst-compile time).
#
# Usage: scripts/fetch-diagram-assets.sh
# Idempotent: skips download if file exists with the expected hash.

set -euo pipefail

CETZ_VERSION="0.5.0"
PKG_URL="https://packages.typst.org/preview/cetz-${CETZ_VERSION}.tar.gz"
DEST_DIR="crates/dsl/assets/cetz-${CETZ_VERSION}/cetz-core"
DEST_FILE="${DEST_DIR}/cetz_core.wasm"
EXPECTED_SHA256="793f0e18ff47e9886220f3ad4b8240d77026ff616c8f245ee6930fce4fed4c09"

mkdir -p "$DEST_DIR"

if [[ -f "$DEST_FILE" ]]; then
  actual=$(sha256sum "$DEST_FILE" | awk '{print $1}')
  if [[ "$actual" == "$EXPECTED_SHA256" ]]; then
    echo "cetz_core.wasm already present and verified."
    exit 0
  fi
  echo "cetz_core.wasm exists but hash mismatch — re-downloading."
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo "Downloading cetz ${CETZ_VERSION} from ${PKG_URL}..."
curl -fsSL "$PKG_URL" -o "$tmp/cetz.tar.gz"
tar -xzf "$tmp/cetz.tar.gz" -C "$tmp"

if [[ ! -f "$tmp/cetz-core/cetz_core.wasm" ]]; then
  echo "ERROR: cetz_core.wasm missing from package archive" >&2
  exit 1
fi

actual=$(sha256sum "$tmp/cetz-core/cetz_core.wasm" | awk '{print $1}')
if [[ "$actual" != "$EXPECTED_SHA256" ]]; then
  echo "ERROR: SHA-256 mismatch" >&2
  echo "  expected: $EXPECTED_SHA256" >&2
  echo "  actual:   $actual" >&2
  echo "If cetz published a new $CETZ_VERSION, update EXPECTED_SHA256 in this script after verifying." >&2
  exit 1
fi

cp "$tmp/cetz-core/cetz_core.wasm" "$DEST_FILE"
echo "Wrote $DEST_FILE ($(stat -c%s "$DEST_FILE") bytes)"
