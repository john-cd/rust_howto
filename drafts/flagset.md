## Flagset

[![flagset][c-flagset-badge]][c-flagset]{{hi:flagset}}
[![flagset-crates.io][c-flagset-crates.io-badge]][c-flagset-crates.io]
[![flagset-github][c-flagset-github-badge]][c-flagset-github]
[![flagset-lib.rs][c-flagset-lib.rs-badge]][c-flagset-lib.rs]

FlagSet is a new, ergonomic approach to handling flags that combines the best of existing crates like bitflags and enumflags without their downsides.

```rust
use flagset::{FlagSet, flags};
use std::os::raw::c_int;

flags! {
    enum FlagsA: u8 {
        Foo,
        Bar,
        Baz,
    }

    enum FlagsB: c_int {
        Foo,
        Bar,
        Baz,
    }
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
