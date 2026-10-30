<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                                                                                 | Crates                                            | Categories                                                                                         |
| ------------------------------------------------------------------------------------------------------ | ------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| [Embed Build Information][ex~development-tools_build-utils~build-info]                                 | {{#crate build-info}} {{#crate build-info-build}} | {{#categories development-tools="Development tools" development-tools::build-utils="Build Utils"}} |
| [Compile and Link Statically to a Bundled C Library][ex~development-tools_build-utils~cc]              | {{#crate cc}}                                     | {{#categories development-tools="Development tools" development-tools::build-utils="Build Utils"}} |
| [Compile and Link Statically to a Bundled C++ Library][ex~development-tools_build-utils~cpp]           | {{#crate cc}}                                     | {{#categories development-tools="Development tools" development-tools::build-utils="Build Utils"}} |
| [Compile a C Library While Setting Custom Defines][ex~development-tools_build-utils~cc-custom-defines] | {{#crate cc}}                                     | {{#categories development-tools="Development tools" development-tools::build-utils="Build Utils"}} |
| [Generate Rust Source in `OUT_DIR`][ex~development-tools_build-utils~out-dir-codegen]                  | [Cargo build scripts][book~cargo~build-script]    | {{#categories development-tools="Development tools" development-tools::build-utils="Build Utils"}} |

<div class="hidden">
  The build information example uses `build-info` and `build-info-build` to embed build metadata in the binary.
</div>
