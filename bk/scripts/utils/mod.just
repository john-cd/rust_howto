set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(join(source_directory(), "..", ".."))

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ source_file() }}

## ---- UTILITIES -----------------------------------

help := 'help'
empty := ''

# Call mdbook-utils to manage links, ref definitions, etc...
@utils cmd=help *subcmd=empty:
    cd {{ quote(root) }} && mdbook-utils {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}

# [clarify mdbook-utils](https://github.com/john-cd/rust_howto/issues/1254)

# Run the templating tool e.g to create badges and reference definitions for a given crate or category
[unix]
@templ cmd=help *subcmd=empty:
    if [ ! -f {{ quote(root + "/../bin/templ") }} ]; then just -f {{ quote(root + "/../tools/justfile") }} release; fi
    cd {{ quote(root) }} && {{ quote(root + "/../bin/templ") }} {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}

# [clarify templ](https://github.com/john-cd/rust_howto/issues/1254)

[windows]
@templ cmd=help *subcmd=empty:
    if (-not (Test-Path -LiteralPath {{ quote(root + "/../bin/templ.exe") }})) { just -f {{ quote(root + "/../tools/justfile") }} release }
    Set-Location -LiteralPath {{ quote(root) }}; & {{ quote(root + "/../bin/templ.exe") }} {{ quote(cmd) }}{{ if subcmd == "" { "" } else { " " + quote(subcmd) } }}
