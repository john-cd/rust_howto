# Purpose: Set shared Cargo output and compiler-cache defaults for repository workspaces.
# Usage: Imported by each workspace justfile; use that workspace's just recipes.
#
# TODO Reviewed ??-??-2026.

build_repo := parent_directory(source_directory())
build_parent := parent_directory(build_repo)
build_slot := if build_parent =~ '^([A-Za-z]:)?[/\\]*$' { "main" } else if file_name(build_parent) == ".worktrees" { file_name(build_repo) } else { "main" }
build_cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
build_target_base := if os_family() == "windows" { "D:/target/rust_howto" } else { join(build_cache_home, "rust_howto/target") }
build_intermediate_base := if os_family() == "windows" { "C:/code_build/rust_howto" } else { join(build_cache_home, "rust_howto/build") }
build_inherited_target := env("CARGO_TARGET_DIR", "")
build_inherited_intermediate := env("CARGO_BUILD_BUILD_DIR", "")

export CARGO_TARGET_DIR := env("CARGO_TARGET_DIR", join(build_target_base, build_slot))
export CARGO_BUILD_BUILD_DIR := env("CARGO_BUILD_BUILD_DIR", join(build_intermediate_base, build_slot))
# sccache is opt-in: set RUSTC_WRAPPER=sccache (incremental compilation is then disabled automatically).
# By default Cargo's incremental compilation is used, which rebuilds fastest in a warm slot.
export RUSTC_WRAPPER := env("RUSTC_WRAPPER", "")
export CARGO_INCREMENTAL := env("CARGO_INCREMENTAL", if env("RUSTC_WRAPPER", "") == "" { "1" } else { "0" })
export SCCACHE_DIR := env("SCCACHE_DIR", if os_family() == "windows" { "D:/sccache" } else { join(build_cache_home, "sccache") })

build_inherited_cache_bases := env("SCCACHE_BASEDIRS", "")
build_cache_separator := if os_family() == "windows" { ";" } else { ":" }

# Preserve caller-supplied roots and add this checkout as a fallback.
export SCCACHE_BASEDIRS := if build_inherited_cache_bases == "" { replace(build_repo, "\\", "/") } else { build_inherited_cache_bases + build_cache_separator + replace(build_repo, "\\", "/") }

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
    pwsh.exe -NoLogo -NoProfile -File "{{ build_repo }}/scripts/setup-build.ps1" -RestartCache -ExistingBaseDirs "{{ build_inherited_cache_bases }}"

# Create a named branch in a known slot; existing slots are preserved for reuse.
[windows]
worktree slot branch:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_repo }}/scripts/setup-build.ps1" -Slot "{{ slot }}" -Branch "{{ branch }}" -TargetDir "{{ build_inherited_target }}" -BuildDir "{{ build_inherited_intermediate }}"

[unix]
build-setup:
    bash "{{ build_repo }}/scripts/setup-build.sh"

[unix]
cache-setup:
    bash "{{ build_repo }}/scripts/setup-build.sh" --restart-cache "{{ build_inherited_cache_bases }}"

[unix]
worktree slot branch:
    bash "{{ build_repo }}/scripts/setup-build.sh" --slot "{{ slot }}" "{{ branch }}" "{{ build_inherited_target }}" "{{ build_inherited_intermediate }}"
