# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working.
[unix]
@check:
  ./check_urls.sh {{root}}

[windows]
@check:
  ./check_urls.sh {{root}}

# List duplicated URLs (noting that they can't always be avoided).
[unix]
@list_duplicated:
  ./list_duplicated_urls.sh {{root}}

[windows]
@list_duplicated:
  ./list_duplicated_urls.sh {{root}}
