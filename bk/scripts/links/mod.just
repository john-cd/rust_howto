set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- LINK DEFINITION MANAGEMENT -----------------------------------

# (BEWARE: modifies files; manual editing required) Insert links to categories
[unix]
@insert_category_links:
  ./insert_category_links.sh {{root}}
# TODO insert preproc directive instead of link? replace link to crates.io category search page to link to category main page?

# (BEWARE: modifies files; manual editing required) Insert links to crate [`...`][c-<crate name>] into the book's Markdown
[unix]
@insert_crate_links:
  ./insert_crate_links.sh {{root}}
# TODO fix - one link only; should we have crate landing pages instead of linking to the docs?

# (BEWARE: modifies many files; manual editing required) Insert cross-links (from page to page) [...][p-<page>] into the book's Markdown
[unix]
@cross_link:
  ./cross_link.sh {{root}}

# Suggest reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
[unix]
@suggest:
  ./suggest_links_for_bare_urls.sh {{root}}
# TODO P2 review

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
[unix]
@replace_inline_links:
  ./replace_inline_links.sh {{root}}
# TODO P2 review

## .... GET LINK (MANUAL) ....

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
[unix]
@get pattern:
  ./make_link.sh {{root}} {{pattern}}

[windows]
@make_link pattern:
  echo "Not implemented for Windows!"
