# TODO Reviewed ??-??-2026

set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ source_file() }}

## --- SPELL -----------------------------------------

# Check spelling in book, draft, and future Markdown on every platform.
spell:
    cd {{ quote(root + "/..") }}; typos --force-exclude --config {{ quote(root + "/../_typos.toml") }} bk/src bk/drafts later/src

# Run the legacy interactive Aspell checker on Unix.
[unix]
spell-aspell mode='list':
    ./spellcheck.sh {{ mode }}

[unix]
aspell-add-word:
    #!/usr/bin/env bash
    dic={{ join(source_directory(), "dictionary.txt") }};
    echo "{{ word }}" >> ${dic}
    sort -u -o ${dic} ${dic}
    sed -i '/personal_ws-1.1 en 0 utf-8/d; 1s/^/personal_ws-1.1 en 0 utf-8\n/' ${dic}
