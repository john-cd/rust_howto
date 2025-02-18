# Hashmaps {#hashmaps}

{{#include hashmaps.incl.md}}

[![std][c-std-badge]][c-std]{{hi:std}}

HashMap is a key-value data structure. It allows you to store data in an unordered collection, where each element is identified by a unique key. This makes HashMap an excellent choice for lookups, insertions, and deletions based on keys.

All of the hashmap{{hi:Hashmap}} keys{{hi:Keys}} must have the same type as each other, and all of the values{{hi:Values}} must have the same type.

```rust,editable
{{#include ../../crates/standard_library/tests/other/hashmaps.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[hashmaps: add (P1)](https://github.com/john-cd/rust_howto/issues/622)
link to data structures
</div>
