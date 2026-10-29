# Purpose: Check external URLs and identify duplicate references.
# Usage: From bk/, run just urls <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working; exit nonzero on any failure.
@check:
    ./check_urls.sh {{ root }}

# Same as check, but report failures without failing (advisory).
@check_advisory:
    ./check_urls.sh --advisory {{ root }}

# Run the check_urls.sh fixtures (no network access).
@test:
    bash ./test_check_urls.sh

# List duplicated URLs (noting that they can't always be avoided).
@list_duplicated:
    ./list_duplicated_urls.sh {{ root }}
