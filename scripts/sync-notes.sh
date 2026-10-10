#!/usr/bin/env bash
# Copies the Obsidian vault's notes into notes/ so GitHub Actions can build the site.
# Run before committing: scripts/sync-notes.sh [vault dir, default ../programming]
set -euo pipefail

vault="${1:-../programming}"
dest="notes"

rm -rf "$dest"
mkdir -p "$dest/.obsidian/shiki-themes"
rsync -a --exclude '.*' --exclude 'AGENTS.md' "$vault/" "$dest/"
cp "$vault/.obsidian/shiki-themes/darcula.json" "$dest/.obsidian/shiki-themes/"

echo "synced $(find "$dest" -name '*.md' | wc -l | tr -d ' ') notes from $vault"
