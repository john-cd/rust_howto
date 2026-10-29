# Purpose: Install and run the repository git hooks with prek.
# Usage: From bk/, run just precommit <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

## ---- DEFAULT -------------------------------------

root := replace(clean(source_directory() + "/../../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- GIT -----------------------------------------
## Commit, skipping git hooks
#@commit m:
#    git commit --no-verify -m "{{m}}"
## Amend the last commit, skipping git hooks
#@amend:
#    git commit --amend --no-verify
## ---- PREK -----------------------------------------
# Hooks are managed by prek (https://prek.j178.dev) and configured in the
# root prek.toml, the single source of truth (see issue #1245).
# The configured hooks run on staged files by default.

# Run configured hooks on staged files.
@fix:
    cd {{ quote(root) }} && prek run

# Run configured hooks on all tracked files.
@fix_all:
    cd {{ quote(root) }} && prek run --all-files

# Install or update prek (https://prek.j178.dev/installation/).
@install:
    cargo binstall --no-confirm prek || cargo install --locked prek

# Install the git hooks (idempotent). Uninstall with `prek uninstall`.
@setup:
    cd {{ quote(root) }} && prek install --install-hooks

## ---- GIT TOWN -----------------------------------------
# review Git Town configuration
# Optional repository-local configuration for Git Town (https://www.git-town.com/).

# Configure Git Town to use this repository's GitHub remote.
@config_git_town:
    git -C {{ quote(root) }} config --local git-town.forge-type github
    git -C {{ quote(root) }} config --local git-town.hosting-origin-hostname github.com
