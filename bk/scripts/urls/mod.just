set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working
[unix]
@check:
  ./check_urls.sh {{root}}

# List duplicated URLs (noting that they can't always be avoided).
[unix]
@list_duplicated:
  ./list_duplicated_urls.sh {{root}}
