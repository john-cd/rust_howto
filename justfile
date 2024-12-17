# Set shell for Windows OSs:
# PowerShell Core has some of the Unix shell skills we need, such as || and &&
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{ justfile() }}
#@just --choose

# Alias for code build
b:
  @just code build

# Alias for code buildall
ba:
  @just code buildall

# Alias for code clippyall
ca:
  @just code clippyall

# Alias for code fmtall
f:
  @just code fmtall

# Alias for code fmtall
fa:
  @just code fmtall

# Alias for code nextestall
nta:
  @just code nextestall

# Alias for code test
t:
  @just code test

# Alias for code testall
ta:
  @just code testall

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

create_issues_for_examples:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{justfile_directory()}} -type f -name "*.rs" )
  do
    base=$(basename $file)
    rel=$(realpath --relative-to={{justfile_directory()}}/deps/tests $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~^ *// *TODO(.*)$~\1~pg' $file)
    do
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix (P1)}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/deps/tests/${rel})" --label "code example,auto")
      echo "$issue_url"
      # Escape [ and ]
      sed -E -i 's~TODO('${todo}')~[\1]('${issue_url}')~' $file
      sleep 3
    done
  done

moveline:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{justfile_directory()}} -type f -name "*.rs" )
  do
   cat <(grep -vE '\s*//\s*\[.*' $file) <(grep -E '\s*//\s*\[.*' $file) > /tmp/temp.rs
   mv -f /tmp/temp.rs $file
  done
