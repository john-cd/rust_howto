set unstable
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list book --unsorted --justfile {{justfile()}}
#@just --choose

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
buildbook: _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
[no-cd]
_mdbook-build-book:
  cd {{justfile_directory()}}; {{source_directory()}}/build_book.sh {{justfile_directory()}}

[windows]
[no-cd]
_mdbook-build-book:
  cd {{justfile_directory()}}; mdbook build {{justfile_directory()}}
# TODO P2

# Generate new reference definitions for all crate the book's examples depend on...
_generate-refdefs:
# TODO P3 mdbook-utils refdefs

# Generate the index and the category page.
_generate-index-category:
# TODO P2

# Add static assets to book output
[unix]
[script]
_copystatic:
  root="{{justfile_directory()}}/"
  cp ${root}static/*.* ${root}book/html/

[windows]
_copystatic:
  copy {{justfile_directory()}}\static\*.* {{justfile_directory()}}\book\html\

# Generate the sitemap.xml file
_sitemap:
  cd {{justfile_directory()}} && mdbook-utils sitemap

alias s := serve

# Serve the book (incl. link checking)
serve:
  mdbook serve -p 3000 -n 127.0.0.1 --open {{justfile_directory()}}
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd  {{justfile_directory()}}/book/html ; python3 -m http.server 3000

alias q := quick

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
quick:
  ./quick.sh {{justfile_directory()}}

[windows]
quick:
  echo "No support for quick serving while in Windows!"
  mdbook serve -p 3001 -n 127.0.0.1 --open {{justfile_directory()}}
