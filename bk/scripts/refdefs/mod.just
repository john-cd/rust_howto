set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
[unix]
@sort:
  ./sort_refdefs.sh {{root}}

# Sort and deduplicate reference definitions in the central `*-refs.md` files
[windows]
@sort:
  (Get-Content -Path {{root}}\src\refs\crate-refs.md) | Sort-Object -Unique | Set-Content -Path {{root}}\src\refs\crate-refs.md
  (Get-Content -Path {{root}}\src\refs\other-refs.md) | Sort-Object -Unique | Set-Content -Path {{root}}\src\refs\other-refs.md
  (Get-Content -Path {{root}}\src\refs\link-refs.md) | Sort-Object -Unique | Set-Content -Path {{root}}\src\refs\link-refs.md

# List links without corresponding reference definitions and vice versa
[unix]
@check:
  ./check_refdefs.sh {{root}}
