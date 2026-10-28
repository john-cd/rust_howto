# Purpose: Run spelling checks across book, draft, and future Markdown.
# Usage: From bk/, run just spelling <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## --- SPELL -----------------------------------------

# Check spelling in book, draft, and future Markdown on every platform.
spell:
    cd {{ quote(root + "/..") }}; typos --force-exclude --config {{ quote(root + "/../_typos.toml") }} bk/src bk/drafts later/src

# Run the legacy interactive Aspell checker on Unix.
spell-aspell mode='list':
    ./spellcheck.sh {{ mode }}

aspell-add-word word:
    #!/usr/bin/env bash
    dic="{{ replace(join(source_directory(), "dictionary.txt"), "\\", "/") }}"
    echo "{{ word }}" >> ${dic}
    sort -u -o ${dic} ${dic}
    sed -i '/personal_ws-1.1 en 0 utf-8/d; 1s/^/personal_ws-1.1 en 0 utf-8\n/' ${dic}
