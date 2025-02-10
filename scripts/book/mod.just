set unstable
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

_default:
  @just --list book --unsorted --justfile {{justfile()}}
#@just --choose

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
[no-cd]
@_mdbook-build-book:
  cd {{root}}; {{source_directory()}}/build_book.sh {{root}}

[windows]
[no-cd]
@_mdbook-build-book:
  cd {{root}}; mdbook build {{root}}
# TODO P2

# Generate new reference definitions for all crate the book's examples depend on...
@_generate-refdefs:
# TODO P3 mdbook-utils refdefs

# Generate the index and the category page.
@_generate-index-category:
# TODO P2

# Add static assets to book output
[unix]
[script]
@_copystatic:
  root="{{root}}/"
  cp ${root}static/*.* ${root}book/html/

[windows]
@_copystatic:
  copy {{root}}\static\*.* {{root}}\book\html\

# Generate the sitemap.xml file
@_sitemap:
  cd {{root}} && mdbook-utils sitemap

alias s := serve

# Serve the book (incl. link checking)
@serve:
  mdbook serve -p 3000 -n 127.0.0.1 --open {{root}}
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd  {{root}}/book/html ; python3 -m http.server 3000

alias q := quick

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
@quick:
  ./quick.sh {{root}}

[windows]
@quick:
  echo "No support for quick serving while in Windows!"
  mdbook serve -p 3001 -n 127.0.0.1 --open {{root}}
