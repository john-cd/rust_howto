# Purpose: Run mdbook-utils and its templates for the book workspace.
# Usage: From bk/, run just utils <recipe>.
#
# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := clean(join(source_directory(), "..", ".."))

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ source_file() }}

## ---- UTILITIES -----------------------------------

help := 'help'
empty := ''

# Run mdbook-utils from the book root. Defaults to its help; pass a command and optional subcommands.
@utils cmd=help *subcmd=empty:
    cd {{ quote(root) }} && mdbook-utils {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}

# Run templ from the book root for crate/category badges and reference definitions; build its binary first if missing.
[unix]
@templ cmd=help *subcmd=empty:
    if [ ! -f {{ quote(root + "/../bin/templ") }} ]; then just -f {{ quote(root + "/../tools/justfile") }} release; fi
    cd {{ quote(root) }} && {{ quote(root + "/../bin/templ") }} {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}

# Run templ from the book root for crate/category badges and reference definitions; build its binary first if missing.
[windows]
@templ cmd=help *subcmd=empty:
    if [[ ! -f {{ quote(root + "/../bin/templ.exe") }} ]]; then just -f {{ quote(root + "/../tools/justfile") }} release; fi
    cd {{ quote(root) }}; {{ quote(root + "/../bin/templ.exe") }} {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}
