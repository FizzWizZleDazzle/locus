#!/usr/bin/env bash
# Download vendored Typst packages used by the DSL diagram renderer:
#   - @preview/cetz:0.5.0           (LGPL-3.0)  -> crates/dsl/assets/cetz-0.5.0/
#   - @preview/oxifmt:1.0.0         (MIT/Apache) -> crates/dsl/assets/oxifmt-1.0.0/
#
# Both are third-party assets, not our code, so they're fetched at build time
# rather than committed. LICENSE files are committed (in-tree) so license
# metadata is visible without running the script.
#
# Each package is downloaded as a tarball from packages.typst.org and verified
# against a pinned SHA-256 of the .tar.gz. Re-run is idempotent.
#
# Usage: scripts/fetch-diagram-assets.sh

set -euo pipefail

ROOT_DIR="crates/dsl/assets"

fetch_pkg() {
  local name="$1" version="$2" archive_sha="$3"
  local url="https://packages.typst.org/preview/${name}-${version}.tar.gz"
  local dest="$ROOT_DIR/${name}-${version}"
  local sentinel="$dest/.fetched-${archive_sha}"

  if [[ -f "$sentinel" ]]; then
    echo "${name} ${version}: already present"
    return 0
  fi

  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN

  echo "Downloading ${name} ${version} from ${url}..."
  curl -fsSL "$url" -o "$tmp/pkg.tar.gz"

  local actual
  actual=$(sha256sum "$tmp/pkg.tar.gz" | awk '{print $1}')
  if [[ "$actual" != "$archive_sha" ]]; then
    echo "ERROR: ${name} ${version} archive SHA-256 mismatch" >&2
    echo "  expected: $archive_sha" >&2
    echo "  actual:   $actual" >&2
    echo "If upstream republished, update the pin in this script after auditing." >&2
    return 1
  fi

  mkdir -p "$dest"
  tar -xzf "$tmp/pkg.tar.gz" -C "$dest"
  : > "$sentinel"
  echo "Wrote ${name} ${version} -> ${dest}"
}

# Pinned tarball SHA-256s. Bump alongside the version when upgrading.
fetch_pkg cetz   0.5.0 d2714007baa7827b321e16719d50c75bf6a44ca3f2d29ae297e4f01afaf1e91c
fetch_pkg oxifmt 1.0.0 7d17a1fc8ad01740ec3cb2b03c7360a4225ff9318e5710765fa98ea6fd59594f
