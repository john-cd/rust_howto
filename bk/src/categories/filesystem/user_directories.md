# User directories

{{#include user_directories.incl.md}}

Get platform-specific locations for configuration, cache, and other data

## `dirs` {#dirs}

[![dirs][c-dirs-badge]][c-dirs] [![dirs-crates.io][c-dirs-crates.io-badge]][c-dirs-crates.io] [![dirs-github][c-dirs-github-badge]][c-dirs-github] [![dirs-lib.rs][c-dirs-lib.rs-badge]][c-dirs-lib.rs]{{hi:dirs}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

`dirs` is a low-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows, macOS and Redox by leveraging the mechanisms defined by the XDG base/user directory specifications on Linux, the Known Folder API on Windows, and the Standard Directory guidelines on macOS.

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/user_directories/dirs.rs:example}}
```

## `directories` {#directories}

[![directories][c-directories-badge]][c-directories] [![directories-crates.io][c-directories-crates.io-badge]][c-directories-crates.io] [![directories-github][c-directories-github-badge]][c-directories-github] [![directories-lib.rs][c-directories-lib.rs-badge]][c-directories-lib.rs]{{hi:directories}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

`directories` is a mid-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows and macOS by leveraging the mechanisms defined by the XDG base/user directory specifications on Linux, the Known Folder API on Windows, and the Standard Directory guidelines on macOS.

`directories` is a higher-level library than `dirs` and can also compute paths for applications.

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/user_directories/directories.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[user_directories: write (P1)](https://github.com/john-cd/rust_howto/issues/362)

- find a spot for the following:

## `open` {#open}

[![open][c-open-badge]][c-open] [![open-crates.io][c-open-crates.io-badge]][c-open-crates.io] [![open-github][c-open-github-badge]][c-open-github] [![open-lib.rs][c-open-lib.rs-badge]][c-open-lib.rs]{{hi:open}}{{hi:open}}{{hi:Xdg-open}}{{hi:Start}}{{hi:Launch}}

Open a path or URL using the program configured on the system.

```rust,editable
{{#include ../../../crates/cats/filesystem/tests/open.rs:example}}
```

</div>
