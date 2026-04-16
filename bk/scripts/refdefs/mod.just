set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

set tempdir := "."

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

# Collect reference definitions scattered in the Markdown files (excluding the refdefs files) and print them
[unix]
collect:
  #!/usr/bin/env bash
  set -uo pipefail
  root="$(realpath {{root}})/"
  for file in $(find ${root}src ${root}drafts ${root}later -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "*.incl.md" -not -name "*-refs.md" \))
  do
    rg --no-line-number --no-filename -e '^\[[^]]+\]: .*$' "$file" || true
    # remove the wayward refdefs - careful: sed -i -E '/^\[.+?\]: .*$/d' "$file"
  done
  echo "DONE"

# Update the label of a reference-style link and its reference definition.
[unix]
relabel old new:
  #!/usr/bin/env bash
  set -uo pipefail
  root="$(realpath {{root}})/"
  for file in $(find ${root}src ${root}drafts ${root}later -type f \( -name "*.md" -not -name "SUMMARY.md" \))
  do
    sed -i -e 's<\]\['"{{old}}"'\]<]['"{{new}}"']<gp' -e 's<^\['"{{old}}"'\]:<['"{{new}}"']:<g' "$file"
  done
  echo "DONE"
