# Lazy Init

[OnceCell]( https://doc.rust-lang.org/core/cell/struct.OnceCell.html ): a cell which can be written to only once.

The corresponding Sync version of OnceCell<T> is `OnceLock<T>`.

```rust
use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| {
        "Hello, World!".to_string()
    });
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}
```


## Older library

[Once Cell]( https://lib.rs/crates/once_cell )

`once_cell` provides two cell-like types, `unsync::OnceCell` and `sync::OnceCell`. A OnceCell might store arbitrary non-Copy types, can be assigned to at most once and provides direct access to the stored contents. The `sync` flavor is thread-safe.

once_cell also has a `Lazy<T>` type, build on top of `OnceCell`:

```rust,ignore
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

// must be static, not const
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
```
