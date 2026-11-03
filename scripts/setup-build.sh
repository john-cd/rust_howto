#!/usr/bin/env bash
# Purpose: Set up this checkout's external Cargo build paths and compiler-cache links.
# Usage: Run with bash scripts/setup-build.sh from the repository root and provide any required arguments.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail
export SCCACHE_CACHE_SIZE=35G

repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)
common_git_dir=$(git -C "$repo" rev-parse --path-format=absolute --git-common-dir)
main_repo=$(dirname -- "$common_git_dir")
target_override=
build_override=

if [[ ${1:-} == --restart-cache ]]; then
    if pgrep -x 'cargo|rustc' >/dev/null; then
        echo 'Wait for running Cargo/rustc processes before restarting sccache.' >&2
        exit 1
    fi
    export SCCACHE_BASEDIRS="$(just --justfile "$repo/justfile" --evaluate build_sccache_basedirs)"
    sccache --stop-server || true # An already stopped daemon needs no action.
    sccache --start-server
    sccache --show-stats
    exit
fi

if [[ ${1:-} == --slot ]]; then
    slot=${2:-}
    branch=${3:-}
    if [[ ! $slot =~ ^[abc]$ || -z $branch ]]; then
        echo 'Specify a slot a, b, or c and a branch.' >&2
        exit 1
    fi
    checkout=$main_repo/.worktrees/$slot
    if [[ -e $checkout || -L $checkout ]]; then
        echo "Slot $slot already exists. Inspect its status, then reuse it without deleting it." >&2
        exit 1
    fi
    git -C "$main_repo" check-ref-format --branch "$branch"
    if git -C "$main_repo" show-ref --verify --quiet "refs/heads/$branch"; then
        git -C "$main_repo" worktree add "$checkout" "$branch"
    else
        git -C "$main_repo" worktree add -b "$branch" "$checkout" HEAD
    fi
    repo=$checkout
    # Keep explicit user overrides, not just's exports for the parent checkout.
    target_override=${4:-}
    build_override=${5:-}
fi

identity=main
if [[ $(basename -- "$(dirname -- "$repo")") == .worktrees ]]; then
    identity=$(basename -- "$repo")
fi
if [[ ! $identity =~ ^(main|[abc])$ ]]; then
    echo 'Use only .worktrees/a, .worktrees/b, or .worktrees/c.' >&2
    exit 1
fi

cache_home=${XDG_CACHE_HOME:-$HOME/.cache}
code_target_root=${CODE_TARGET_ROOT:-$cache_home/cargo-target}
code_build_root=${CODE_BUILD_ROOT:-$cache_home/cargo-build}
code_sccache_root=${CODE_SCCACHE_ROOT:-${SCCACHE_DIR:-$cache_home/sccache}}
link_directory() {
    local link=$1 destination=$2
    if [[ $destination != /* ]]; then
        echo "Expected an absolute cache path: $destination" >&2
        exit 1
    fi
    if [[ -e $link || -L $link ]]; then
        if [[ ! -L $link || $(readlink -- "$link") != "$destination" ]]; then
            echo "Preserving existing $link; expected a symlink to $destination." >&2
            exit 1
        fi
    else
        mkdir -p -- "$destination"
        ln -s -- "$destination" "$link"
    fi
    printf '%s -> %s\n' "$link" "$destination"
}

link_directory "$repo/target" "${target_override:-$code_target_root/rust_howto/$identity}"
link_directory "$repo/build" "${build_override:-$code_build_root/rust_howto/$identity}"
link_directory "$repo/.sccache" "$code_sccache_root"
