# Purpose: Install and run the repository pre-commit hooks.
# Usage: From bk/, run just precommit <recipe>.
#
# TODO Reviewed ??-??-2026

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

## ---- DEFAULT -------------------------------------

root := clean(source_directory() + "/../../..")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- GIT -----------------------------------------
## Commit, skipping pre-commit hooks
#@commit m:
#    git commit --no-verify -m "{{m}}"
## Amend the last commit, skipping pre-commit hooks
#@amend:
#    git commit --amend --no-verify
## ---- PRE-COMMIT -----------------------------------
# [finish - install pre-commit or use cargo-husky](https://github.com/john-cd/rust_howto/issues/1245)?
# Combining just + pre-commit
# pre-commit: https://pre-commit.com/
# based on https://github.com/casey/just/blob/master/examples/pre-commit.just
# review
# The repository's configured hooks run on staged files by default.

# Run configured hooks on staged files.
@fix:
    cd {{ quote(root) }} && pre-commit run

# Run configured hooks on all tracked files.
@fix_all:
    cd {{ quote(root) }} && pre-commit run --all-files

# Install or update pre-commit with pip.
[unix]
@install:
    python3 -m pip install pre-commit

[windows]
@install:
    py -3 -m pip install pre-commit

#npm i -g prettier
# [move to Dockerfile](https://github.com/john-cd/rust_howto/issues/1247)

# FIXME Setup/update pre-commit hooks (optional)
@setup:
    pre-commit install --install-hooks # uninstall: `pre-commit uninstall`

## ---- GIT TOWN -----------------------------------------
# review Git Town configuration
# Optional repository-local configuration for Git Town (https://www.git-town.com/).

# Configure Git Town to use this repository's GitHub remote.
@config_git_town:
    git -C {{ quote(root) }} config --local git-town.forge-type github
    git -C {{ quote(root) }} config --local git-town.hosting-origin-hostname github.com
