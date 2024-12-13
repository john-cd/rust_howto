set windows-shell := [ "cmd.exe", "/c" ]

[no-exit-message]
_default:
  @just --choose
#@just --list --unsorted --justfile {{justfile()}}

# Build and test the code examples
mod code 'just/code.just'

# Build and serve the book
mod book 'just/book.just'

# Build the code documentation
mod docs 'just/docs.just'

# Manage heading anchors
mod anchors 'just/anchors.just'

# Manage recipe tables
mod recipes 'just/recipes.just'

# Manage dependencies
mod deps 'just/deps.just'

# Manage include statements
mod includes 'just/includes.just'

# Manage code examples
mod examples 'just/examples.just'

# Manage reference definitions
mod refdefs 'just/refdefs.just'

# Manage links
mod links 'just/links.just'

# Manage external URLs
mod urls 'just/urls.just'

# Manage indexes
mod indices 'just/indices.just'

# Hide or unhide chapters
mod chapters 'just/chapters.just'

# Manage the main table of contents
mod toc 'just/toc.just'

# Manage heavy tests
mod heavy 'just/heavy.just'

# Utilities
mod tools 'just/tools.just'

# Search the references using a crate name or label fragment and return the refdefs / URLs and reference-style links
lnk pattern:
  {{justfile_directory()}}/scripts/refdefs/search.sh {{pattern}}

## ---- CLEAN ------------------------------------------

# Clean Cargo's `target` and mdbook's `book` directories
clean: &&_clean
  cargo clean
  mdbook clean

[unix]
_clean:
  rm --recursive --force ./doctest_cache/

[windows]
_clean:
  if exist .doctest_cache rmdir /s /q .doctest_cache

## ---- PRE-PUSH -----------------------------------

# Prepare for git push: spell sortrefs fmtall clean clippyall testall _builddocall buildbook
# prep: spell sortrefs fmtall clean clippyall testall docs::_builddocall buildbook
