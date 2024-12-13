_default:
  @just --list book --unsorted --justfile {{justfile()}}
#@just --choose

## ---- BOOK BUILDING -----------------------------------

alias b := buildbook
alias q := quick
alias s := serve

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
buildbook: _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
_mdbook-build-book:
  {{justfile_directory()}}//scripts/build_book.sh

[windows]
_mdbook-build-book:
  mdbook build
# TODO P2

# Generate new reference definitions for all crate the book's examples depend on...
_generate-refdefs:
# TODO P3 mdbook-utils refdefs

# Generate the index and the category page.
_generate-index-category:
# TODO P2

# Add static assets to book output
[unix]
_copystatic:
  cp {{justfile_directory()}}/static/*.* {{justfile_directory()}}/book/html/

[windows]
_copystatic:
  copy {{justfile_directory()}}\static\*.* {{justfile_directory()}}\book\html\

# Generate the sitemap.xml file
_sitemap:
  mdbook-utils sitemap

# Serve the book (incl. link checking)
serve:
  mdbook serve -p 3000 -n 127.0.0.1 --open
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd book/html ; python3 -m http.server 3000

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
quick:
  {{justfile_directory()}}/scripts/quick.sh

[windows]
quick:
  echo "No support for quick serving while in Windows!"
  mdbook serve -p 3001 -n 127.0.0.1 --open
