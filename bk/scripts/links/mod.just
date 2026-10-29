# Purpose: Insert and normalize cross-links and crate or category links in book Markdown.
# Usage: From bk/, run just links <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- LINK DEFINITION MANAGEMENT -----------------------------------

# Preview category links using book landing pages where available; the default is a dry run.
@insert_category_links:
    ./insert_category_links.sh {{ quote(root) }}

# Apply category links after reviewing the dry-run diff.
@insert_category_links_apply:
    ./insert_category_links.sh {{ quote(root) }} --apply

# Category badge preprocessor directives are tracked in #248.

# (BEWARE: modifies many files; manual editing required) Insert links to crate [`...`][c~<crate name>~docs] into the book's Markdown
@insert_crate_links:
    ./insert_crate_links.sh {{ root }}

# fix - one link only; should we have crate landing pages instead of linking to the docs?

# (BEWARE: modifies many files; manual editing required) Insert cross-links (from page to page) [...][p~<page>] into the book's Markdown
@cross_link:
    ./cross_link.sh {{ root }}

# Suggest reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
@suggest:
    ./suggest_links_for_bare_urls.sh {{ root }}

# review suggest_links_for_bare_urls

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
@replace_inline_links:
    ./replace_inline_links.sh {{ root }}

## .... GET LINK (MANUAL) ....

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
@get pattern:
    ./get_link.sh {{ root }} {{ pattern }}
