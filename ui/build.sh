#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

DX_OUT=./target/dx/my-settings-ui/release/web/public
WWWROOT=../wwwroot

echo "→ cleaning previous artifacts"
rm -rf "$DX_OUT"
mkdir -p "$WWWROOT"
rm -rf "$WWWROOT"/* "$WWWROOT"/.[!.]* 2>/dev/null || true

echo "→ dx build --release --web"
dx build --release --web

if [ -f ./build.py ]; then
    echo "→ cache-busting index.html"
    python3 build.py "$DX_OUT/index.html"
fi

echo "→ copying to $WWWROOT"
cp -r "$DX_OUT"/* "$WWWROOT/"

echo "✓ WASM SPA скопирован в $WWWROOT"
