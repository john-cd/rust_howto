# Documentation

[The rustdoc book]( https://doc.rust-lang.org/rustdoc/index.html )

[doc.rs]( https://docs.rs/ ): open-source documentation host for Rust crates.

## Documenting your code

1) Add documentation comments to your code.

```rust,editable,ignore
/// This is a doc comment. It is equivalent to the next line.
#[doc = r" This is a doc comment."]
```

`rustdoc` uses the CommonMark Markdown specification.

```rust,editable,ignore
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
```

Any item annotated with `#[doc(hidden)]` will not appear in the documentation.

2) Run `rustdoc src/lib.rs --crate-name <name>` or `cargo doc --open` to create a new directory, `doc` (or `target/doc` when using cargo), with a website inside.

## Module- or crate-level documentation

Use `//!` at the top of the file (instead of `///`) for module-level documentation.

The first lines within `lib.rs` will compose the crate-level documentation front-page.

```rust,editable,ignore
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com
```

- To add a "run" button on your documentation (allowing its execution in the rust playground), use the following attribute:

```rust,editable,ignore
#![doc(html_playground_url = "https://playground.example.com/")]
```

## Badges

[Shield.io]( https://shields.io/ )
