set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

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

# (WIP)(BEWARE: modifies files) Insert potential cross-links into the book's Markdown
[unix]
@cross_link:
  ./suggest_cross_links.sh {{root}}

## .... GET LINK (MANUAL) ....

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
[unix]
@make_link pattern:
  ./make_link.sh {{root}} {{pattern}}

[windows]
@make_link pattern:
  echo "Not implemented for Windows!"
