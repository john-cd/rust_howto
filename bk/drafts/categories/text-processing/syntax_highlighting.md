<!-- TODO Reviewed ??-??-2026 -->

# Syntax Highlighting

{{#include syntax_highlighting.incl.md}}

TextMate grammars (`syntect`) and real parse trees (`tree-sitter`) are two approaches to highlighting code. Tree-sitter grammars are language-specific, so the example below uses [`tree-sitter-json`][tree-sitter-json~crates.io] to classify JSON keys and values by their parsed node kinds.

## Highlight Code with `syntect` {#syntect}

[![syntect][c~syntect~docs~badge]][c~syntect~docs] [![syntect~crates.io][c~syntect~crates.io~badge]][c~syntect~crates.io]{{hi:syntect}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Syntax highlighting}}

[`syntect`][c~syntect~docs]↗{{hi:syntect}} highlights text with Sublime Text / TextMate grammars and themes, producing HTML or terminal escape codes. Loading the bundled `SyntaxSet` and `ThemeSet` is slow, so load them once (for example in a `LazyLock`). The `default-fancy` feature selects a pure-Rust regex engine instead of linking Oniguruma. If the HTML comes from user-supplied code, sanitize it; see [[html_sanitization | Sanitize HTML]].

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/syntect_html.rs:example}}
```

## Classify Code into Highlight Spans with `tree-sitter` {#tree-sitter-spans}

[![tree-sitter][c~tree-sitter~docs~badge]][c~tree-sitter~docs] [![tree-sitter~crates.io][c~tree-sitter~crates.io~badge]][c~tree-sitter~crates.io]{{hi:tree-sitter}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Syntax highlighting}}

[`tree-sitter`][c~tree-sitter~docs]↗{{hi:tree-sitter}} builds a concrete syntax tree, so a highlighter can classify tokens by their role (a string literal, a number, a keyword) instead of matching regular expressions. The example walks the tree and collects a byte range per class. After an edit, describe the change with `Tree::edit` and pass the old tree to the next `parse` call: Tree-sitter reuses unchanged subtrees, which keeps highlighting fast in an editor. For production use, see the `tree-sitter-highlight` crate, which runs the grammar's `highlights.scm` query. Grammars are separate crates, such as `tree-sitter-rust`.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/text_analysis/tree_sitter_json_spans.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing Utilities]].
- [[ides | Editors and IDEs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
