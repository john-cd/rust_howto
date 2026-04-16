set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- LINK DEFINITION MANAGEMENT -----------------------------------

# (BEWARE: modifies many files; manual editing required) Insert links to categories
[unix]
@insert_category_links:
  ./insert_category_links.sh {{root}}
# [insert preproc directive instead of link. replace link to crates.io category search page to link to category main page.](https://github.com/john-cd/rust_howto/issues/1281)

# (BEWARE: modifies many files; manual editing required) Insert links to crate [`...`][c~<crate name>~docs] into the book's Markdown
[unix]
@insert_crate_links:
  ./insert_crate_links.sh {{root}}
# [fix - one link only; should we have crate landing pages instead of linking to the docs](https://github.com/john-cd/rust_howto/issues/1240)?

# (BEWARE: modifies many files; manual editing required) Insert cross-links (from page to page) [...][p~<page>] into the book's Markdown
[unix]
@cross_link:
  ./cross_link.sh {{root}}

# Suggest reference-style links and reference definitions to replace bare URLs found in the book's markdown (manual review necessary)
[unix]
@suggest:
  ./suggest_links_for_bare_urls.sh {{root}}
# [review suggest_links_for_bare_urls](https://github.com/john-cd/rust_howto/issues/1241)

# (Rough) Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
[unix]
@replace_inline_links:
  ./replace_inline_links.sh {{root}}
# FIXME

## .... GET LINK (MANUAL) ....

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
[unix]
@get pattern:
  ./get_link.sh {{root}} {{pattern}}

[windows]
@get pattern:
  echo "Not implemented for Windows!"
