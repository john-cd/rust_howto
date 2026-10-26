#!/usr/bin/env bash
# Purpose: Run the Python tool that updates recipe tables and their titles.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "${script_dir}/fix_recipe_tables.py" "$@"
