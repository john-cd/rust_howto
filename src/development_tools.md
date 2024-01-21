# Development Tools

## Versioning

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse and increment a version string][ex-semver-increment] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Parse a complex version string][ex-semver-complex] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check if given version is pre-release][ex-semver-prerelease] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Find the latest version satisfying given range][ex-semver-latest] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [Check external command version for compatibility][ex-semver-command] | [![semver-badge]][semver] | [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

## Build Time

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Compile and link statically to a bundled C library][ex-cc-static-bundled] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile and link statically to a bundled C++ library][ex-cc-static-bundled-cpp] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [Compile a C library while setting custom defines][ex-cc-custom-defines] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |

[ex-semver-increment]: tools/versioning.md#parse-and-increment-a-version-string
[ex-semver-complex]: tools/versioning.md#parse-a-complex-version-string
[ex-semver-prerelease]: tools/versioning.md#check-if-given-version-is-pre-release
[ex-semver-latest]: tools/versioning.md#find-the-latest-version-satisfying-given-range
[ex-semver-command]: tools/versioning.md#check-external-command-version-for-compatibility
[ex-cc-static-bundled]: tools/build_tools.md#compile-and-link-statically-to-a-bundled-c-library
[ex-cc-static-bundled-cpp]: tools/build_tools.md#compile-and-link-statically-to-a-bundled-c-library-1
[ex-cc-custom-defines]: tools/build_tools.md#compile-a-c-library-while-setting-custom-defines
{{#include refs/link-refs.md}}