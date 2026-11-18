<!-- TODO Reviewed ??-??-2026. -->

# User Directories and Preferred Applications

{{#include user_directories_and_preferred_applications.incl.md}}

## Locate User Directories {#dirs}

{{#badges dirs}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`dirs`][c~dirs~docs]↗{{hi:dirs}} is a tiny, low-level library with a minimal API that provides platform-specific standard locations of directories for [configuration](../config/index.md), cache, and other data. It follows the XDG base/user directory specifications on Linux, the "Known Folder" API on [Windows](../os_windows-apis/windows.md), and the Standard Directory guidelines on macOS.

The following example gets paths to standard (home, configuration, data, cache, executables) directories on the file system:

From `bk/crates`, run `cargo test --locked -p filesystem --example user_directories`. The output depends on the local home directory and operating system.

```rust,ignore,noplayground
{{#include ../../../crates/cats/filesystem/examples/user_directories/dirs.rs:example}}
```

## Compute the Location of Cache, Config, or Data Directories for a Specific Application {#directories}

{{#badges directories}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`directories`][c~directories~docs]↗{{hi:directories}} is a mid-level library that provides platform-specific standard locations of directories for config, cache, and other data on Linux, Windows and macOS.

[`directories`][c~directories~docs]↗{{hi:directories}} is higher-level than [`dirs`][c~dirs~docs]↗{{hi:dirs}}. Use `directories` if you need to compute cache, config, etc. paths for specific applications or projects:

```rust,editable,noplayground
{{#include ../../../crates/cats/filesystem/examples/user_directories/directories.rs:example}}
```

## Open a File or URL using the Program Configured on your System {#open}

{{#badges open}}{{hi:open}}{{hi:Xdg-open}}{{hi:Start}}{{hi:Launch}}

[`open`][c~open~docs]↗{{hi:open}} opens a path or [URL](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/web-programming/url.md) using the default program configured on the system:

From `bk/crates`, run `cargo run --locked -p filesystem --example open`. Create `example.txt` in `bk/crates` first. The example opens the file and a URL using the system's default applications.

```rust,ignore,noplayground
{{#include ../../../crates/cats/filesystem/examples/open.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directories | Directories]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
