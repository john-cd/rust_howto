## `Cow` Alternatives {#cow-alternatives}

While `Cow` is a great choice for optimizing memory usage when dealing with a mix of borrowed and owned data, there are alternatives.

Instead of e.g. `Cow<str>`, you can use:

- `Arc<str>`, an atomically reference-counted shared string slice, to be used in multithreaded environments,
- `Rc<str>`, a single-threaded reference-counted string slice,
- `Box<str>` for heap-allocated string slices.

```rust,editable
{{#include ../../crates/standard_library/tests/cow/cow_alternatives.rs:example}}
```

## Arc::make_mut {#arc_make_mut}

If you need reference-counting, note that `Rc::make_mut` and `Arc::make_mut` can provide clone-on-write functionality as well.

```rust,editable
{{#include ../../crates/standard_library/tests/arc/arc_make_mut.rs:example}}
```

---

## Other Strings {#other-strings}

- Use a string type with a small-string optimization, if you need to return small formatted strings.
- If you need to use a lot of strings, and copy them around, you probably want string interning.
- Use `ustr` for interning global identifiers and hashmap keys.
