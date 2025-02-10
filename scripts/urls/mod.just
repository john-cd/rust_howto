set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

_default:
  @just --list urls --unsorted --justfile {{justfile()}}
#@just --choose

## ---- URL MANAGEMENT -----------------------------------

# Check that URLs (to external websites) are valid and working
[unix]
@check:
  ./check_urls.sh {{root}}

# List duplicated URLs (noting that they can't always be avoided).
[unix]
@list_duplicated:
  ./list_duplicated_urls.sh {{root}}
