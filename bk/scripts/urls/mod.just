# Purpose: Check external URLs and identify duplicate references.
# Usage: From bk/, run just urls <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- URL MANAGEMENT -----------------------------------

# Check URLs across Markdown, TOML, and YAML; lychee also supplies status classes and advisory
# mode that `book_tool link-check` does not expose.
@check:
    ./check_urls.sh {{ root }}

# Same file scope and diagnostics as `check`, but report failures without failing (advisory).
@check_advisory:
    ./check_urls.sh --advisory {{ root }}

# Run the check_urls.sh fixtures (no network access).
@test:
    bash ./test_check_urls.sh

# Count raw URL duplicates in global reference files. `mdbook-utils duplicate-links` scans parsed
# Markdown links and writes a different report.
@list_duplicated:
    ./list_duplicated_urls.sh {{ root }}
