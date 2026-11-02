#!/usr/bin/env bash
# Verify docs/components/api pages stay in sync with packages/leptos.
#
# Checks:
#   1. every docs/components/api/<name>.md maps to an existing packages/leptos/<name> crate
#      (meta pages listed in META_PAGES are exempt)
#   2. no page mentions the stale crate names `shadcn-ui-leptos` / `shadcn_ui_leptos`
#   3. (warning only) crates in packages/leptos without an API page
#
# Usage: build/scripts/check-docs-api.sh   (exit code 1 on failure)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
API_DIR="$ROOT/docs/components/api"
PKG_DIR="$ROOT/packages/leptos"
META_PAGES=("DRIFT" "README" "index")

fail=0

is_meta() {
  local n="$1" m
  for m in "${META_PAGES[@]}"; do [[ "$n" == "$m" ]] && return 0; done
  return 1
}

for page in "$API_DIR"/*.md; do
  name="$(basename "$page" .md)"
  is_meta "$name" && continue
  if [[ ! -f "$PKG_DIR/$name/Cargo.toml" ]]; then
    echo "ERROR: $page has no matching crate at packages/leptos/$name" >&2
    fail=1
  fi
done

pages=()
for page in "$API_DIR"/*.md; do
  is_meta "$(basename "$page" .md)" || pages+=("$page")
done
if grep -nE 'shadcn-ui-leptos|shadcn_ui_leptos' "${pages[@]}" >&2; then
  echo "ERROR: stale crate names found above (use leptos-shadcn-* / leptos_shadcn_*)" >&2
  fail=1
fi

for crate in "$PKG_DIR"/*/; do
  name="$(basename "$crate")"
  [[ -f "$crate/Cargo.toml" ]] || continue
  [[ -f "$API_DIR/$name.md" ]] || echo "WARN: packages/leptos/$name has no docs/components/api/$name.md" >&2
done

if [[ $fail -eq 0 ]]; then
  echo "docs/components/api: OK"
fi
exit $fail
