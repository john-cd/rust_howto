#!/usr/bin/env bash
# Purpose: Set up this checkout's external Cargo build paths and compiler-cache links.
# Usage: Run with bash scripts/setup-build.sh from the repository root and provide any required arguments.
#
# TODO Reviewed ??-??-2026

set -euo pipefail

repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)
common_git_dir=$(git -C "$repo" rev-parse --path-format=absolute --git-common-dir)
main_repo=$(dirname -- "$common_git_dir")

get_cache_bases() {
    local bases=${1:-${SCCACHE_BASEDIRS:-}}
    append_base() {
        case ":$bases:" in
            *":$1:"*) ;;
            *) bases=${bases:+$bases:}$1 ;;
        esac
    }
    append_base "$main_repo"
    for slot in {a..z}; do
        append_base "$main_repo/.worktrees/$slot"
    done
    local worktrees
    worktrees=$(git -C "$main_repo" worktree list --porcelain)
    while IFS= read -r line; do
        [[ $line == 'worktree '* ]] && append_base "${line#worktree }"
    done <<< "$worktrees"
    printf '%s' "$bases"
}

if [[ ${1:-} == --restart-cache ]]; then
    if pgrep -x 'cargo|rustc' >/dev/null; then
        echo 'Wait for running Cargo/rustc processes before restarting sccache.' >&2
        exit 1
    fi
    export SCCACHE_BASEDIRS=$(get_cache_bases "${2:-${SCCACHE_BASEDIRS:-}}")
    sccache --stop-server || true # An already stopped daemon needs no action.
    sccache --start-server
    sccache --show-stats
    exit
fi

if [[ ${1:-} == --slot ]]; then
    slot=${2:-}
    branch=${3:-}
    if [[ ! $slot =~ ^[a-z]$ || -z $branch ]]; then
        echo 'Specify a slot a through z and a branch.' >&2
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
    export CARGO_TARGET_DIR=${4:-}
    export CARGO_BUILD_BUILD_DIR=${5:-}
fi

identity=main
if [[ $(basename -- "$(dirname -- "$repo")") == .worktrees ]]; then
    identity=$(basename -- "$repo")
fi
if [[ ! $identity =~ ^(main|[a-z])$ ]]; then
    echo 'Use a named .worktrees/a through .worktrees/z slot.' >&2
    exit 1
fi

cache_home=${XDG_CACHE_HOME:-$HOME/.cache}
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

link_directory "$repo/target" "${CARGO_TARGET_DIR:-$cache_home/rust_howto/target/$identity}"
link_directory "$repo/build" "${CARGO_BUILD_BUILD_DIR:-$cache_home/rust_howto/build/$identity}"
link_directory "$repo/.sccache" "${SCCACHE_DIR:-$cache_home/sccache}"
