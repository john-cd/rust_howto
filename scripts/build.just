# Purpose: Set shared Cargo output and compiler-cache defaults for repository workspaces.
# Usage: Imported by each workspace justfile; use that workspace's just recipes.
#
# TODO Reviewed ??-??-2026.

# Storage-root overrides: CODE_REPOS_ROOT names the canonical checkout root for tools;
# CODE_TEMP_ROOT holds temporary files;
# CODE_PACKAGES_ROOT stores package-manager data; CODE_TOOLS_ROOT stores portable tools.
# CODE_TARGET_ROOT stores compiled artifacts, CODE_BUILD_ROOT stores intermediate Cargo files
# (required to be on NTFS on Windows), and CODE_SCCACHE_ROOT selects the compiler cache.
# SCCACHE_DIR is retained as a fallback when CODE_SCCACHE_ROOT is unset.
build_repo := parent_directory(source_directory())
build_parent := parent_directory(build_repo)
build_slot := if build_parent =~ '^([A-Za-z]:)?[/\\]*$' { "main" } else if file_name(build_parent) == ".worktrees" { file_name(build_repo) } else { "main" }
build_cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
build_code_repos_root := env("CODE_REPOS_ROOT", if os_family() == "windows" { "D:/code" } else { join(home_directory(), "code") })
build_code_temp_root := env("CODE_TEMP_ROOT", if os_family() == "windows" { if path_exists("D:/temp") == "true" { "D:/temp" } else { "C:/temp" } } else { env("TMPDIR", "/tmp") })
build_code_packages_root := env("CODE_PACKAGES_ROOT", if os_family() == "windows" { "D:/packages" } else { join(home_directory(), ".local/share") })
build_code_tools_root := env("CODE_TOOLS_ROOT", if os_family() == "windows" { "C:/tools" } else { join(home_directory(), ".local/tools") })
build_code_target_root := env("CODE_TARGET_ROOT", if os_family() == "windows" { "D:/target" } else { join(build_cache_home, "cargo-target") })
build_code_build_root := env("CODE_BUILD_ROOT", if os_family() == "windows" { "C:/code_build" } else { join(build_cache_home, "cargo-build") })
build_code_sccache_root := env("CODE_SCCACHE_ROOT", env("SCCACHE_DIR", if os_family() == "windows" { "D:/sccache" } else { join(build_cache_home, "sccache") }))
build_target_base := join(build_code_target_root, "rust_howto")
build_intermediate_base := join(build_code_build_root, "rust_howto")
build_inherited_target := env("CARGO_TARGET_DIR", "")
build_inherited_intermediate := env("CARGO_BUILD_BUILD_DIR", "")

build_target_dir := join(build_target_base, tooling_slot)
build_intermediate_dir := join(build_intermediate_base, tooling_slot)
# Native Just paths; validate slots before exporting build directories.
tooling_slot := if build_slot =~ '^(main|[ab])$' { build_slot } else { error("Invalid worktree slot: " + build_slot) }
tooling_main := if file_name(parent_directory(build_repo)) == ".worktrees" { parent_directory(parent_directory(build_repo)) } else { build_repo }
build_sccache_basedirs := trim(shell('''
set -euo pipefail
root=$1
printf '%s' "$root"
git -C "$root" -c core.fsmonitor=false worktree list --porcelain | while IFS= read -r line; do
    case "$line" in
        "worktree $root/.worktrees/"*)
            slot=${line#"worktree $root/.worktrees/"}
            if [[ $slot =~ ^[ab]$ ]]; then
                printf '%s%s/.worktrees/%s' "$2" "$root" "$slot"
            fi
            ;;
    esac
done
''', replace(tooling_main, "\\", "/"), if os() == "windows" { ";" } else { ":" }))

export CARGO_TARGET_DIR := build_target_dir
export CARGO_BUILD_TARGET_DIR := build_target_dir
host_build_volume := if os() == "windows" { trim(shell('pwsh.exe -NoLogo -NoProfile -File "$1" -BuildRoot "$2"', join(build_repo, "scripts/cargo-host.ps1"), build_code_build_root)) } else { "ok" }
export CARGO_BUILD_BUILD_DIR := if host_build_volume == "ok" { build_intermediate_dir } else { error("Cargo build root must use NTFS") }
export RUSTC_WRAPPER := "sccache"
export CARGO_BUILD_RUSTC_WRAPPER := "sccache"
export CARGO_INCREMENTAL := "0"
export SCCACHE_DIR := build_code_sccache_root
export SCCACHE_CACHE_SIZE := "35G"

export SCCACHE_BASEDIRS := build_sccache_basedirs
export TEMP := if os() == "windows" { build_code_temp_root } else { env("TEMP", "/tmp") }
export TMP := if os() == "windows" { build_code_temp_root } else { env("TMP", "/tmp") }
export TMPDIR := if os() == "windows" { build_code_temp_root } else { env("TMPDIR", "/tmp") }

# Run an ad hoc Cargo command with this checkout's shared build environment.
[positional-arguments]
@cargo +args:
    cargo "$@"

# Connect direct Cargo commands to the same persistent directories as just.
[windows]
build-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_repo }}/scripts/setup-build.ps1"

# Refresh the shared daemon's base directories while no Rust build is running.
[windows]
cache-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_repo }}/scripts/setup-build.ps1" -RestartCache

# Create a named branch in a known slot; existing slots are preserved for reuse.
[windows]
worktree slot branch:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_repo }}/scripts/setup-build.ps1" -Slot "{{ slot }}" -Branch "{{ branch }}" -TargetDir "{{ build_inherited_target }}" -BuildDir "{{ build_inherited_intermediate }}"

# Connect this checkout to its external Cargo output and compiler-cache directories.
[unix]
build-setup:
    bash "{{ build_repo }}/scripts/setup-build.sh"

# Restart the shared sccache daemon with the registered worktree base directories.
[unix]
cache-setup:
    bash "{{ build_repo }}/scripts/setup-build.sh" --restart-cache

# Create a named worktree slot and configure its external build directories.
[unix]
worktree slot branch:
    bash "{{ build_repo }}/scripts/setup-build.sh" --slot "{{ slot }}" "{{ branch }}" "{{ build_inherited_target }}" "{{ build_inherited_intermediate }}"
# Print the active worktree paths and compiler-cache configuration.
build-env:
    @printf 'WORKTREE_DIR=%s\n' {{quote(build_repo)}}
    @printf 'WORKTREE_SLOT=%s\n' {{quote(tooling_slot)}}
    @printf 'REPO_ROOT=%s\n' {{quote(tooling_main)}}
    @printf 'CODE_TARGET_ROOT=%s\n' {{quote(build_code_target_root)}}
    @printf 'CODE_BUILD_ROOT=%s\n' {{quote(build_code_build_root)}}
    @printf 'CODE_SCCACHE_ROOT=%s\n' {{quote(build_code_sccache_root)}}
    @printf 'CODE_TEMP_ROOT=%s\n' {{quote(build_code_temp_root)}}
    @printf 'BRANCH=%s\n' "$(git -C {{quote(build_repo)}} -c core.fsmonitor=false branch --show-current)"
    @if test -z "$(git -C {{quote(build_repo)}} -c core.fsmonitor=false status --porcelain)"; then echo STATUS=clean; else echo STATUS=dirty; fi
    @printf 'CARGO_TARGET_DIR=%s\n' {{quote(CARGO_TARGET_DIR)}}
    @printf 'CARGO_BUILD_TARGET_DIR=%s\n' {{quote(CARGO_BUILD_TARGET_DIR)}}
    @printf 'CARGO_BUILD_BUILD_DIR=%s\n' {{quote(CARGO_BUILD_BUILD_DIR)}}
    @printf 'RUSTC_WRAPPER=%s\n' {{quote(RUSTC_WRAPPER)}}
    @printf 'CARGO_BUILD_RUSTC_WRAPPER=%s\n' {{quote(CARGO_BUILD_RUSTC_WRAPPER)}}
    @printf 'CARGO_INCREMENTAL=%s\n' {{quote(CARGO_INCREMENTAL)}}
    @printf 'CARGO_BUILD_JOBS=%s\n' {{quote(env("CARGO_BUILD_JOBS", ""))}}
    @printf 'SCCACHE_DIR=%s\n' {{quote(SCCACHE_DIR)}}
    @printf 'SCCACHE_CACHE_SIZE=%s\n' {{quote(SCCACHE_CACHE_SIZE)}}
    @printf 'SCCACHE_BASEDIRS=%s\n' {{quote(SCCACHE_BASEDIRS)}}
    @printf 'TEMP=%s\n' {{quote(TEMP)}}
    @printf 'TMP=%s\n' {{quote(TMP)}}
    @printf 'TMPDIR=%s\n' {{quote(TMPDIR)}}

export CODE_REPOS_ROOT := build_code_repos_root
export CODE_PACKAGES_ROOT := build_code_packages_root
export CODE_TOOLS_ROOT := build_code_tools_root
export CODE_TARGET_ROOT := build_code_target_root
export CODE_BUILD_ROOT := build_code_build_root
export CODE_TEMP_ROOT := build_code_temp_root
export CARGO_HOME := env("CARGO_HOME", if os() == "windows" { join(build_code_packages_root, "cargo") } else { join(home_directory(), ".cargo") })
export NUGET_PACKAGES := env("NUGET_PACKAGES", join(build_code_packages_root, "nuget"))
export npm_config_cache := env("npm_config_cache", join(build_code_packages_root, "npm"))

# Configure the host fallback; regular recipes retain their separate checkout slots.
configure-cargo-host:
    pwsh -NoLogo -NoProfile -File {{quote(join(build_repo, "scripts/cargo-host.ps1"))}} -BuildRoot {{quote(build_code_build_root)}} -ConfigureHost
