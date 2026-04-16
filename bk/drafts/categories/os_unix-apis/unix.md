# Unix APIs

{{#include unix.incl.md}}{{hi:Operating systems::Unix APIs}}

## Bind to Unix-specific APIs with `rustix` {#rustix}

[![rustix][c~rustix~docs~badge]][c~rustix~docs]{{hi:rustix}}
[![rustix~crates.io][c~rustix~crates.io~badge]][c~rustix~crates.io]
[![rustix~repo][c~rustix~repo~badge]][c~rustix~repo]
[![rustix~lib.rs][c~rustix~lib.rs~badge]][c~rustix~lib.rs]
[![cat~date-and-time][cat~date-and-time~badge]][cat~date-and-time]{{hi:Date and time}}
[![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}
[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]{{hi:Unix APIs}}

[`rustix`][c~rustix~docs]↗{{hi:rustix}} offers efficient and safe POSIX / *nix / Winsock syscall-like APIs. It uses idiomatic Rust types: refs, slices, Results instead of raw pointers, safe wrappers around raw file descriptors, [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} instead of bare integer flags, and several other conveniences.

```rust,editable
{{#include ../../../crates/cats/os_unix_apis/examples/unix/rustix.rs:example}}
```

## Bind to Unix-specific APIs with `nix` {#nix}

[![nix][c~nix~docs~badge]][c~nix~docs]{{hi:nix}}
[![nix~crates.io][c~nix~crates.io~badge]][c~nix~crates.io]
[![nix~repo][c~nix~repo~badge]][c~nix~repo]
[![nix~lib.rs][c~nix~lib.rs~badge]][c~nix~lib.rs]
[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]{{hi:Unix APIs}}

[`nix`][c~nix~docs]↗{{hi:nix}} provides bindings to the various *nix system [functions][p~functions] (Unix, Linux, MacOS, etc.).

```rust,editable
{{#include ../../../crates/cats/os_unix_apis/examples/unix/nix.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[unix: write](https://github.com/john-cd/rust_howto/issues/436)
</div>
