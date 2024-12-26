set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list refdefs --unsorted --justfile {{justfile()}}
#@just --choose

## ---- REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
[unix]
sort:
  ./sort_refdefs.sh

# Sort and deduplicate reference definitions in the central `*-refs.md` files
[windows]
sort:
  (Get-Content -Path {{justfile_directory()}}\src\refs\crate-refs.md) | Sort-Object -Unique | Set-Content -Path {{justfile_directory()}}\src\refs\crate-refs.md
  (Get-Content -Path {{justfile_directory()}}\src\refs\other-refs.md) | Sort-Object -Unique | Set-Content -Path {{justfile_directory()}}\src\refs\other-refs.md
  (Get-Content -Path {{justfile_directory()}}\src\refs\link-refs.md) | Sort-Object -Unique | Set-Content -Path {{justfile_directory()}}\src\refs\link-refs.md

# List links without corresponding reference definitions and vice versa
[unix]
check:
  ./check_refdefs.sh
