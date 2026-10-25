<!-- TODO Reviewed ??-??-2026 -->

# Syntax Highlighting

{{#include syntax_highlighting.incl.md}}

TextMate grammars (`syntect`) and real parse trees (`tree-sitter`) are the two common approaches to highlighting code. The `tree-sitter` recipe is tracked in [issue #827](https://github.com/john-cd/rust_howto/issues/827).

## Highlight Code with `syntect` {#syntect}

[![syntect][c~syntect~docs~badge]][c~syntect~docs] [![syntect~crates.io][c~syntect~crates.io~badge]][c~syntect~crates.io]{{hi:syntect}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Syntax highlighting}}

[`syntect`][c~syntect~docs]↗{{hi:syntect}} highlights text with Sublime Text / TextMate grammars and themes, producing HTML or terminal escape codes. Loading the bundled `SyntaxSet` and `ThemeSet` is slow, so load them once (for example in a `LazyLock`). The `default-fancy` feature selects a pure-Rust regex engine instead of linking Oniguruma. If the HTML comes from user-supplied code, sanitize it; see [[html_sanitization | Sanitize HTML]].

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/syntect_html.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing Utilities]].
- [[ides | Editors and IDEs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
