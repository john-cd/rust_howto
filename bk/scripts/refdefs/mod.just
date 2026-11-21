# Purpose: Sort and validate reference definitions in the book source.
# Usage: From bk/, run just refdefs <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]
set tempdir := "."

[unix]
set script-interpreter := ["bash"]

[windows]
set script-interpreter := ["C:/Program Files/Git/bin/bash.exe"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- REFERENCE DEFINITION MANAGEMENT -----------------------------------

# Sort and deduplicate reference definitions in the central `*-refs.md` files; remove the last / from URLs
@sort:
    ./sort_refdefs.sh {{ root }}

# List links without corresponding reference definitions and vice versa. `book_tool refdefs-audit`
# reports include-scoped definition provenance and unused entries but does not report undefined labels.
@check:
    ./check_refdefs.sh {{ root }}

# Collect reference definitions scattered in the Markdown files (excluding the refdefs files) and print them
[script]
collect:
    set -uo pipefail
    root="$(realpath {{ root }})/"
    for file in $(find ${root}src ${root}../later/src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "*.incl.md" -not -name "*-refs.md" \))
    do
      rg --no-line-number --no-filename -e '^\[[^]]+\]: .*$' "$file" || true
      # remove the wayward refdefs - careful: sed -i -E '/^\[.+?\]: .*$/d' "$file"
    done
    echo "DONE"

# Update the label of a reference-style link and its reference definition.
[script]
relabel old new:
    set -uo pipefail
    root="$(realpath {{ root }})/"
    for file in $(find ${root}src ${root}../later/src -type f \( -name "*.md" -not -name "SUMMARY.md" \))
    do
      sed -i -e 's<\]\['"{{ old }}"'\]<]['"{{ new }}"']<gp' -e 's<^\['"{{ old }}"'\]:<['"{{ new }}"']:<g' "$file"
    done
    echo "DONE"
