set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

_default:
  @just --list links --unsorted --justfile {{justfile()}}
#@just --choose

## ---- LINK DEFINITION MANAGEMENT -----------------------------------

# Suggest reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
[unix]
@suggest:
  ./suggest_links_for_bare_urls.sh {{root}}
# TODO P2

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
[unix]
@replace_inline_links:
  ./replace_inline_links.sh {{root}}
# TODO P2
