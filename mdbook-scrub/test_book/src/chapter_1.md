# Chapter 1

## Hidden section

<div class="hidden">
THIS IS NOT SEEN.
</div>

## Includes

{{#include ./included.md}}

## Hidden Includes

### Include a file

```rust
{{#include _hidden.rs}}
```

### Partially include a file - third line only

```rust
{{#include _hidden.rs:3}}
```

### Partially include a file - up to line 5

```rust
{{#include _hidden.rs::5}}
```

### Partially include a file - from line 2

```rust
{{#include _hidden.rs:2:}}
```

### Partially include a file - between lines 2 and 5

```rust
{{#include _hidden.rs:2:5}}
```

### Partially include a file into your book using anchors

```rust
{{#include _hidden.rs:component}}
```

### Including a file but initially hiding all except specified lines

```rust
{{#rustdoc_include  _hidden.rs:2}}
```

### Inserting runnable Rust files

{{#playground _hidden.rs}}

### Inserting runnable Rust files with attributes

{{#playground _hidden.rs editable}}

## Code blocks

```rust,ignore
fn main() {}
```

## Links

- [Inline]( https://www.rust-lang.org/what/cli )
- [Reference-style][ref]
- [Link to Github][github]
- [shortcut]
- [collapsed][]
- <https://www.rust-lang.org/> autolink
- [invalid-ref][invalid-ref]

## Reference definitions

[ref]: https://rust-cli.github.io/book/index.html
[github]: https://github.com/john-cd
[shortcut]: https://rust-cli.github.io/book/in-depth/human-communication.html
[collapsed]: https://rust-cli.github.io/book/in-depth/signals.html
[invalid-ref]: http:://a.com

{{#include ./refs.md}}
