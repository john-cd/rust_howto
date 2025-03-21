set unstable
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- BOOK BUILDING -----------------------------------

alias bb := buildbook

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
@buildbook: _release_mbdbook-scrub _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the book's HTML / JS
[unix]
@_mdbook-build-book:
  echo "Build book in {{root}}"
  cd {{root}}; {{source_directory()}}/build_book.sh {{root}}

[windows]
@_mdbook-build-book:
  echo "Build book in {{root}}"
  cd {{root}}; mdbook build {{root}}

# Generate new reference definitions for all crate the book's examples depend on...
@_generate-refdefs:
# [mdbook-utils refdefs](https://github.com/john-cd/rust_howto/issues/1237)

# Generate the index and the category page.
@_generate-index-category:
# [generate index of crates by category](https://github.com/john-cd/rust_howto/issues/1274)

# Add static assets to book output
[unix]
@_copystatic:
  echo "Copy static files"
  cp {{root}}/static/*.* {{root}}/book/html/

[windows]
@_copystatic:
  echo "Copy static files"
  copy {{root}}\static\*.* {{root}}\book\html\

# Generate the sitemap.xml file
@_sitemap:
  echo "Install mdbook-utils if needed"
  cargo install mdbook-utils
  echo "Generate the sitemap"
  cd {{root}} && mdbook-utils sitemap

# Build / release the mdbook-scrub preprocessor, if needed
[unix]
@_release_mbdbook-scrub:
  echo "Build mdbook-scrub, if needed"
  if [ ! -f {{root}}/../bin/mdbook-scrub ]; then just -f {{root}}/../mdbook-scrub/justfile release; fi

[windows]
@_release_mbdbook-scrub:
  echo "Build mdbook-scrub, if needed"
  if (-Not (Test-Path -Path {{root}}\..\bin\mdbook-scrub.exe )) { just -f {{root}}\..\mdbook-scrub\justfile release }

## ---- BOOK SERVING -----------------------------------

alias s := serve

# Serve the book (incl. link checking)
@serve:
  echo "Serve the book"
  cd {{root}} && mdbook serve -p 3000 -n 127.0.0.1 --open {{root}}
  ## NOTE: can conflict with "port" / EXPOSE in the Docker / Docker Compose configuration
  ## Or use: cd  {{root}}/book/html ; python3 -m http.server 3000

alias q := quick

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
@quick:
  echo "Serve the book (no link checking)"
  cd {{root}} && ./quick.sh {{root}}

[windows]
@quick: && serve
  echo "No support for quick serving while in Windows!"
