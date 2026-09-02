#!/bin/sh
set -eu

bump=${1:-}
case "$bump" in major|minor|patch) ;; *) printf 'Usage: %s major|minor|patch\n' "$0" >&2; exit 2 ;; esac

current=$(sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' Cargo.toml | sed -n '1p')
if [ -z "$current" ]; then
  printf 'Could not read a stable semantic version from Cargo.toml\n' >&2
  exit 1
fi

old_ifs=$IFS
IFS=.
set -- $current
IFS=$old_ifs
major=$1
minor=$2
patch=$3

case "$bump" in
  major) major=$((major + 1)); minor=0; patch=0 ;;
  minor) minor=$((minor + 1)); patch=0 ;;
  patch) patch=$((patch + 1)) ;;
esac

next="$major.$minor.$patch"
temporary=$(mktemp)
trap 'rm -f "$temporary"' EXIT
awk -v version="$next" '
  !updated && /^version = "/ { print "version = \"" version "\""; updated=1; next }
  { print }
' Cargo.toml > "$temporary"
mv "$temporary" Cargo.toml
trap - EXIT
cargo generate-lockfile
printf 'Bumped maplab-wasm from %s to %s\n' "$current" "$next"
