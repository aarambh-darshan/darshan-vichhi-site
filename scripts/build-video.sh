#!/usr/bin/env bash
# Rebuilds the intro video (src/video, Rust -> WebAssembly) into assets/video/.
#
# One-time setup:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --locked --version "$(scripts/build-video.sh --wasm-bindgen-version)"
#   (optional, smaller output)  brew install binaryen   /   sudo apt install binaryen
#
# Then:  scripts/build-video.sh        # builds and writes assets/video/intro_video.js + intro_video_bg.wasm
# The narration (assets/video/intro.mp3) and loader.js are committed and not touched by this script.
set -euo pipefail
cd "$(dirname "$0")/.."

wb_version() {
  awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[version =\"]/, "", $0); print; exit }' Cargo.lock
}
if [[ "${1:-}" == "--wasm-bindgen-version" ]]; then wb_version; exit 0; fi

have="$(wasm-bindgen --version 2>/dev/null | awk '{print $2}' || true)"
want="$(wb_version)"
if [[ "$have" != "$want" ]]; then
  echo "wasm-bindgen CLI $want is required (found: ${have:-none}). Install it with:" >&2
  echo "  cargo install wasm-bindgen-cli --locked --version $want" >&2
  exit 1
fi

cargo build --profile wasm-release --lib --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir assets/video --out-name intro_video \
  target/wasm32-unknown-unknown/wasm-release/darshan_vichhi_site.wasm

if command -v wasm-opt >/dev/null 2>&1; then
  tmp="$(mktemp)"
  if wasm-opt -Oz --enable-bulk-memory --enable-sign-ext --enable-nontrapping-float-to-int --enable-mutable-globals \
      --enable-reference-types --enable-multivalue assets/video/intro_video_bg.wasm -o "$tmp"; then
    mv "$tmp" assets/video/intro_video_bg.wasm
    chmod 644 assets/video/intro_video_bg.wasm
  else
    echo "wasm-opt failed; keeping the unoptimized module" >&2
    rm -f "$tmp"
  fi
fi
ls -l assets/video
