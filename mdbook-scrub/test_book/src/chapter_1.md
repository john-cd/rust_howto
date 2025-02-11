# Chapter 1

## Hidden section

<div class="hidden">
THIS IS NOT SEEN.
</div>

## Includes

{{#include ./included.md}}

## Hidden Includes

### Include a file

{{#include _hidden.md}}

### Partially include a file - third line only

{{#include _hidden.md:3}}

### Partially include a file - up to line 10

{{#include _hidden.md::10}}

### Partially include a file - from line 2

{{#include _hidden.md:2:}}

### Partially include a file - between lines 2 and 10

{{#include _hidden.md:2:10}}

### Partially include a file into your book using anchors

{{#include _hidden.md:component}}

### Including a file but initially hiding all except specified lines

{{#rustdoc_include_hidden.md:2}}

### Inserting runnable Rust files

{{#playground _hidden.md}}

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
