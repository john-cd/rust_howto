# User Directories and Preferred Applications

{{#include user_directories_and_preferred_applications.incl.md}}

## Locate User Directories with `dirs` {#dirs}

[![dirs][c~dirs~docs~badge]][c~dirs~docs] [![dirs~crates.io][c~dirs~crates.io~badge]][c~dirs~crates.io] [![dirs~repo][c~dirs~repo~badge]][c~dirs~repo] [![dirs~lib.rs][c~dirs~lib.rs~badge]][c~dirs~lib.rs]{{hi:dirs}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`dirs`][c~dirs~docs]↗{{hi:dirs}} is a tiny, low-level library with a minimal API that provides platform-specific standard locations of directories for [[config | configuration]], cache and other data. It leverages the XDG base/user directory specifications on Linux, the "Known Folder" API on [[Windows]], and the Standard Directory guidelines on macOS.

The following example gets paths to standard home, configuration, data... directories on the file system:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/user_directories/dirs.rs:example}}
```

## Locate User Directories with `directories` {#directories}

[![directories][c~directories~docs~badge]][c~directories~docs] [![directories~crates.io][c~directories~crates.io~badge]][c~directories~crates.io] [![directories~repo][c~directories~repo~badge]][c~directories~repo] [![directories~lib.rs][c~directories~lib.rs~badge]][c~directories~lib.rs]{{hi:directories}}{{hi:App_dirs}}{{hi:Xdg}}{{hi:Path}}{{hi:Folder}}{{hi:Basedir}}

[`directories`][c~directories~docs]↗{{hi:directories}} is a mid-level library that provides platform-specific standard locations of directories for config, cache, and other data on Linux, Windows and macOS.

[`directories`][c~directories~docs]↗{{hi:directories}} is higher-level than [`dirs`][c~dirs~docs]↗{{hi:dirs}}. Use `directories` if you need to compute cache, config, etc. paths for specific applications or projects:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/user_directories/directories.rs:example}}
```

## Open a File or URL using the Program Configured on your System {#open}

[![open][c~open~docs~badge]][c~open~docs] [![open~crates.io][c~open~crates.io~badge]][c~open~crates.io] [![open~repo][c~open~repo~badge]][c~open~repo] [![open~lib.rs][c~open~lib.rs~badge]][c~open~lib.rs]{{hi:open}}{{hi:open}}{{hi:Xdg-open}}{{hi:Start}}{{hi:Launch}}

[`open`][c~open~docs]↗{{hi:open}} opens a path or [[url | URL]] using the default program configured on the system:

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/open.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[directories | Directories]].
- [[paths | Paths]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
