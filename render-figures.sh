#!/usr/bin/env bash
# Renders each figure src/<dir>/images/<name>.tex to <name>.svg beside it.
#
# Each .tex is a standalone document (open it in Overleaf or run `latex` on it
# to preview). This compiles it through pgf's dvisvgm driver and keeps text as
# real text with embedded WOFF2 fonts, so browsers render it crisply. Only
# figures whose .tex is newer than their .svg are rebuilt; pass --all to
# rebuild everything. Needs TeX Live (latex, dvisvgm).
#
# Usage: ./render-figures.sh [--all] [src/path/to/images/name.tex ...]
set -euo pipefail

cd "$(dirname "$0")"

# On-page size relative to TeX size: 1.35 brings 10pt TeX text to the 18px
# body text size.
SCALE=1.35

force=false
if [[ ${1:-} == --all ]]; then
  force=true
  shift
fi

if [[ $# -gt 0 ]]; then
  sources=("$@")
else
  mapfile -t sources < <(find src -path '*/images/*.tex' | sort)
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

rendered=0
for tex in "${sources[@]}"; do
  svg=${tex%.tex}.svg
  if ! $force && [[ -f $svg && $svg -nt $tex ]]; then
    continue
  fi

  # The driver must be chosen before \documentclass loads TikZ.
  { echo '\def\pgfsysdriver{pgfsys-dvisvgm.def}'; cat "$tex"; } > "$work/fig.tex"
  if ! (cd "$work" && latex -interaction=nonstopmode -halt-on-error fig.tex > fig.log 2>&1); then
    echo "error: $tex failed to compile:" >&2
    grep -A5 '^!' "$work/fig.log" >&2 || tail -20 "$work/fig.log" >&2
    exit 1
  fi
  # papersize keeps the standalone border; --no-styles puts the font on each
  # <text> instead of in shared CSS classes.
  dvisvgm --font-format=woff2 --bbox=papersize --no-styles -o "$work/fig.svg" "$work/fig.dvi" > /dev/null 2>&1

  perl -pe "for my \$a ('width', 'height') { s/(<svg[^>]*? \$a=')([\d.]+)pt'/\$1.sprintf('%.2f', \$2 * $SCALE).\"pt'\"/e }" \
    "$work/fig.svg" > "$svg"
  echo "rendered $svg"
  rendered=$((rendered + 1))
done

echo "$rendered figure(s) rendered"
