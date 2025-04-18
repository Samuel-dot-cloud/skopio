#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
DB_CRATE="$ROOT_DIR/crates/db"
SQLX_DESKTOP="$ROOT_DIR/.sqlx-desktop"
SQLX_SERVER="$ROOT_DIR/.sqlx-server"
SQLX_MERGED="$ROOT_DIR/.sqlx-merged"
SQLX_COLLISIONS="$ROOT_DIR/.sqlx-collisions"

echo "==> Preparing for desktop..."
export DATABASE_URL="sqlite://$ROOT_DIR/data/desktop.db"
rm -rf "$SQLX_DESKTOP"
cargo sqlx prepare --workspace -- --manifest-path "$DB_CRATE/Cargo.toml" --features desktop
mv ".sqlx" "$SQLX_DESKTOP"

echo "==> Preparing for server..."
export DATABASE_URL="sqlite://$ROOT_DIR/data/server.db"
rm -rf "$SQLX_SERVER"
cargo sqlx prepare --workspace -- --manifest-path "$DB_CRATE/Cargo.toml" --features server
mv ".sqlx" "$SQLX_SERVER"

echo "==> Checking for collisions..."
comm -12 <(ls "$SQLX_DESKTOP" | sort) <(ls "$SQLX_SERVER" | sort) > "$SQLX_COLLISIONS" || true

if [[ -s "$SQLX_COLLISIONS" ]]; then
  echo "❌ Conflict detected in SQLX cache hashes!"
  cat "$SQLX_COLLISIONS"
  echo "Aborting merge to avoid invalid cache."
  exit 1
fi

echo "==> Merging SQLX caches..."
rm -rf "$SQLX_MERGED"
mkdir -p "$SQLX_MERGED"
cp "$SQLX_DESKTOP"/* "$SQLX_MERGED/"
cp "$SQLX_SERVER"/* "$SQLX_MERGED/"

rm -rf ".sqlx"
cp -r "$SQLX_MERGED" ".sqlx"

echo "===> Cleaning up temporary SQLX folders..."
rm -rf "$SQLX_DESKTOP" "$SQLX_SERVER" "$SQLX_MERGED" "$SQLX_COLLISIONS"

echo "🚀 Merged SQLX cache is now active in .sqlx"
