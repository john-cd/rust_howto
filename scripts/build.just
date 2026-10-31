# Purpose: Set shared Cargo output and compiler-cache defaults for repository workspaces.
# Usage: Imported by each workspace justfile; use that workspace's just recipes.
#
# TODO Reviewed ??-??-2026.

build_repo := parent_directory(source_directory())
build_parent := parent_directory(build_repo)
build_sccache_base := if file_name(build_parent) == ".worktrees" { parent_directory(build_parent) } else { build_repo }
build_slot := if build_parent =~ '^([A-Za-z]:)?[/\\]*$' { "main" } else if file_name(build_parent) == ".worktrees" { file_name(build_repo) } else { "main" }
build_cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
build_target_base := if os_family() == "windows" { "D:/target/rust_howto" } else { join(build_cache_home, "cargo-target/rust_howto") }
build_intermediate_base := if os_family() == "windows" { "C:/code_build/rust_howto" } else { join(build_cache_home, "cargo-build/rust_howto") }
build_setup_ps1 := join(build_repo, "scripts/setup-build.ps1")
build_setup_sh := join(build_repo, "scripts/setup-build.sh")
build_inherited_target := join(build_target_base, build_slot)
build_inherited_intermediate := join(build_intermediate_base, build_slot)

export CARGO_TARGET_DIR := build_inherited_target
export CARGO_BUILD_TARGET_DIR := build_inherited_target
export CARGO_BUILD_BUILD_DIR := build_inherited_intermediate
# Workspace recipes avoid sccache and enable incremental compilation; direct Cargo uses the config wrapper with incrementals off.
export RUSTC_WRAPPER := ""
export CARGO_BUILD_RUSTC_WRAPPER := ""
export CARGO_INCREMENTAL := "1"
export SCCACHE_DIR := if os_family() == "windows" { "D:/sccache" } else { join(build_cache_home, "sccache") }
export SCCACHE_CACHE_SIZE := "20G"

build_inherited_cache_bases := env("SCCACHE_BASEDIRS", "")
build_cache_separator := if os_family() == "windows" { ";" } else { ":" }
export SCCACHE_BASEDIRS := if build_inherited_cache_bases == "" { build_sccache_base } else { build_inherited_cache_bases + build_cache_separator + build_sccache_base }

# Run an ad hoc Cargo command with this checkout's shared build environment.
[positional-arguments]
@cargo +args:
    cargo "$@"

# Connect direct Cargo commands to the same persistent directories as just.
[windows]
build-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_setup_ps1 }}"

# Refresh the shared daemon's base directories while no Rust build is running.
[windows]
cache-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_setup_ps1 }}" -RestartCache -ExistingBaseDirs "{{ build_inherited_cache_bases }}"

# Create a named branch in a known slot; existing slots are preserved for reuse.
[windows]
worktree slot branch:
    pwsh.exe -NoLogo -NoProfile -File "{{ build_setup_ps1 }}" -Slot "{{ slot }}" -Branch "{{ branch }}" -TargetDir "{{ join(build_target_base, slot) }}" -BuildDir "{{ join(build_intermediate_base, slot) }}"

[unix]
build-setup:
    bash "{{ build_setup_sh }}"

[unix]
cache-setup:
    bash "{{ build_setup_sh }}" --restart-cache "{{ build_inherited_cache_bases }}"

[unix]
worktree slot branch:
    bash "{{ build_setup_sh }}" --slot "{{ slot }}" "{{ branch }}" "{{ join(build_target_base, slot) }}" "{{ join(build_intermediate_base, slot) }}"
