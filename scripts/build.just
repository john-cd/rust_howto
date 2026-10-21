# TODO Reviewed ??-??-2026
# Imported by every workspace entry point, including root fan-out commands.
build_repo := parent_directory(source_directory())
build_slot := if file_name(parent_directory(build_repo)) == ".worktrees" { file_name(build_repo) } else { "main" }
build_cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
build_target_base := if os_family() == "windows" { "D:/target/rust_howto" } else { join(build_cache_home, "rust_howto/target") }
build_intermediate_base := if os_family() == "windows" { "C:/code_build/rust_howto" } else { join(build_cache_home, "rust_howto/build") }
build_inherited_target := env("CARGO_TARGET_DIR", "")
build_inherited_intermediate := env("CARGO_BUILD_BUILD_DIR", "")
export CARGO_TARGET_DIR := env("CARGO_TARGET_DIR", join(build_target_base, build_slot))
export CARGO_BUILD_BUILD_DIR := env("CARGO_BUILD_BUILD_DIR", join(build_intermediate_base, build_slot))
export RUSTC_WRAPPER := "sccache"
export CARGO_INCREMENTAL := "0"
export SCCACHE_DIR := env("SCCACHE_DIR", if os_family() == "windows" { "D:/sccache" } else { join(build_cache_home, "sccache") })
build_inherited_cache_bases := env("SCCACHE_BASEDIRS", "")
# Longest matching base wins, so each slot is normalized like the main checkout.
export SCCACHE_BASEDIRS := replace(build_repo, "\\", "/")

# Connect direct Cargo commands to the same persistent directories as just.
[windows]
build-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{build_repo}}/scripts/setup-build.ps1"

# Refresh the shared daemon's base directories while no Rust build is running.
[windows]
cache-setup:
    pwsh.exe -NoLogo -NoProfile -File "{{build_repo}}/scripts/setup-build.ps1" -RestartCache -ExistingBaseDirs "{{build_inherited_cache_bases}}"

# Create a named branch in a known slot; existing slots are preserved for reuse.
[windows]
worktree slot branch:
    pwsh.exe -NoLogo -NoProfile -File "{{build_repo}}/scripts/setup-build.ps1" -Slot "{{slot}}" -Branch "{{branch}}" -TargetDir "{{build_inherited_target}}" -BuildDir "{{build_inherited_intermediate}}"

[unix]
build-setup:
    bash "{{build_repo}}/scripts/setup-build.sh"

[unix]
cache-setup:
    bash "{{build_repo}}/scripts/setup-build.sh" --restart-cache "{{build_inherited_cache_bases}}"

[unix]
worktree slot branch:
    bash "{{build_repo}}/scripts/setup-build.sh" --slot "{{slot}}" "{{branch}}" "{{build_inherited_target}}" "{{build_inherited_intermediate}}"
