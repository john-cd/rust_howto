# User Directories and Preferred Applications

{{#include user_directories.incl.md}}

The following code examples get platform-specific locations for [[configuration]], cache, and other data.

## Locate User Directories with `dirs` {#dirs}

[![dirs][c~dirs~docs~badge]][c~dirs~docs] [![dirs~crates.io][c~dirs~crates.io~badge]][c~dirs~crates.io] [![dirs~repo][c~dirs~repo~badge]][c~dirs~repo] [![dirs~lib.rs][c~dirs~lib.rs~badge]][c~dirs~lib.rs]{{hi:dirs}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`dirs`][c~dirs~docs]↗{{hi:dirs}} is a low-level library that provides platform-specific standard locations of directories for [[config]], cache and other data on Linux, Windows, macOS and Redox by leveraging the mechanisms defined by the XDG base/user directory specifications on Linux, the "Known Folder" API on [[Windows]], and the Standard Directory guidelines on macOS.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/user_directories/dirs.rs:example}}
```

## Locate User Directories with `directories` {#directories}

[![directories][c~directories~docs~badge]][c~directories~docs] [![directories~crates.io][c~directories~crates.io~badge]][c~directories~crates.io] [![directories~repo][c~directories~repo~badge]][c~directories~repo] [![directories~lib.rs][c~directories~lib.rs~badge]][c~directories~lib.rs]{{hi:directories}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`directories`][c~directories~docs]↗{{hi:directories}} is a mid-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows and macOS by leveraging the mechanisms defined by the XDG base/user directory specifications on Linux, the Known Folder API on Windows, and the Standard Directory guidelines on macOS.

[`directories`][c~directories~docs]↗{{hi:directories}} is a higher-level library than [`dirs`][c~dirs~docs]↗{{hi:dirs}} and can also compute paths for applications.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/user_directories/directories.rs:example}}
```

## Open a File or URL using the Program Configured on your System {#open}

[![open][c~open~docs~badge]][c~open~docs] [![open~crates.io][c~open~crates.io~badge]][c~open~crates.io] [![open~repo][c~open~repo~badge]][c~open~repo] [![open~lib.rs][c~open~lib.rs~badge]][c~open~lib.rs]{{hi:open}}{{hi:open}}{{hi:Xdg-open}}{{hi:Start}}{{hi:Launch}}

[`open`][c~open~docs]↗{{hi:open}} opens a path or [[url | URL]] using the program configured on the system.

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/open.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[user_directories: write](https://github.com/john-cd/rust_howto/issues/362)
</div>
