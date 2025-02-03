set shell := ["bash", "-uc"]

# Set shell for Windows OSs:
# PowerShell Core has some of the Unix shell skills we need, such as || and &&
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ justfile() }}
#@just --choose

# Alias for buildcurrent
[no-cd]
bc:
  @just code buildcurrent

# Alias for code buildpkg
bp pkg:
  @just code buildpkg {{pkg}}

# Alias for code build
b:
  @just code build

# Alias for code buildall
ba:
  @just code buildall

# Alias for code clippyall
ca:
  @just code clippyall

# Format all bin and lib files using rustfmt
f:
  @just code fmt

# Format all packages, and also their local path-based dependencies
fa:
  @just code fmtall

# Format all bin and lib files of the current crate using rustfmt
[no-cd]
fc:
  @just code fmtcurrent

# Alias for code nextest
nt:
  @just code nextest

# Alias for code nextestall
nta:
  @just code nextestall

# Alias for code nextestcurrent
[no-cd]
ntc:
  @just code nextestcurrent

# # Alias for code testall
# ta:
#   @just code testall

# Build and test the code examples
mod code 'scripts/code'

# Build the code documentation
mod docs 'scripts/docs'

# Alias for book
bb:
  @just book buildbook

# Alias for book quick
q:
  @just book quick

# Alias for book serve
s:
  @just book serve

# Build and serve the book
mod book 'scripts/book'

# Manage heading anchors
mod anchors 'scripts/anchors'

# Manage recipe tables
mod recipes 'scripts/recipes'

# Manage crates / dependencies
mod deps 'scripts/deps'

# Manage include statements
mod includes 'scripts/includes'

# Manage code examples
mod examples 'scripts/examples'

# Manage reference definitions
mod refdefs 'scripts/refdefs'

# Manage links
mod links 'scripts/links'

# Manage external URLs
mod urls 'scripts/urls'

# Manage indexes
mod indices 'scripts/indices'

# Hide or unhide chapters
mod chapters 'scripts/chapters'

# Manage the main table of contents
mod toc 'scripts/toc'

# Alias for utils
u *args='':
  @just utils {{args}}

# Utilities
mod utils 'scripts/utils'

spell:
  @just utils spell

lnk pattern:
  @just utils make_link {{pattern}}

# Manage Docker Compose and DockerHub
mod docker 'scripts/docker'

# Manage GitHub
mod gh 'scripts/gh'

# Manage pre-commit
mod? precommit 'scripts/precommit'

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
