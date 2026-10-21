#!/usr/bin/env bash
# TODO Reviewed ??-??-2026
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "${script_dir}/fix_recipe_tables.py" "$@"
