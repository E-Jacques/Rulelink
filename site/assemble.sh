#!/usr/bin/env bash
# Assembles the published site: the Astro build (site/dist) plus the crate reference
# (target/doc, from `moon run :api-docs`) under /api/. Output: target/site.
# Nothing is published: the folder is what gets uploaded.
set -euo pipefail
cd "$(dirname "$0")/.."

out=target/site
crates="rulelink_model rulelink_registry rulelink_link rulelink_check rulelink_import rulelink_config rulelink_adapter_sdk"

[ -f site/dist/index.html ] || { echo "site/dist is missing: run moon run site:build" >&2; exit 1; }
[ -d target/doc ] || { echo "target/doc is missing: run moon run :api-docs" >&2; exit 1; }

rm -rf "$out"
mkdir -p "$out/api"
cp -R site/dist/. "$out/"
for c in $crates; do
  [ -f "target/doc/$c/index.html" ] || { echo "no reference for $c" >&2; exit 1; }
  cp -R "target/doc/$c" "$out/api/$c"
done
# rustdoc shares its static files, search index and sources between crates.
for d in static.files search.index src trait.impl type.impl; do
  if [ -e "target/doc/$d" ]; then cp -R "target/doc/$d" "$out/api/$d"; fi
done
for f in target/doc/*.js target/doc/*.html target/doc/*.css; do
  if [ -f "$f" ]; then cp "$f" "$out/api/"; fi
done

# Every /api/ link of the site must resolve in the assembled folder.
status=0
while IFS= read -r link; do
  target="$out${link%%#*}"
  case "$target" in */) target="${target}index.html";; esac
  [ -f "$target" ] || { echo "broken link: $link" >&2; status=1; }
done < <(grep -rhoE 'href="/api/[^"]*"' "$out" --include='*.html' --exclude-dir=api | sed -E 's/^href="//; s/"$//' | sort -u)
if [ "$status" = 0 ]; then echo "assembled $out"; fi
exit $status
