_default:
  @just --list links --unsorted --justfile {{justfile()}}
#@just --choose

## ---- LINK DEFINITION MANAGEMENT -----------------------------------

# Outputs reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
suggest_links_for_bare_urls:
  {{justfile_directory()}}/scripts/links/suggest_links_for_bare_urls.sh
# TODO P2

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
replace_inline_links:
  {{justfile_directory()}}/scripts/links/replace_inline_links.sh
# TODO P2
