# Markdown

{{#include markdown.incl.md}}

[`pulldown-cmark`][c~pulldown_cmark~docs]⮳{{hi:pulldown-cmark}} is CommonMark compliant. [`comrak`][c~comrak~docs]⮳{{hi:comrak}} is another popular choice.

## `pulldown-cmark` {#pulldown-cmark}

[![pulldown-cmark][c~pulldown_cmark~docs~badge]][c~pulldown_cmark~docs] [![pulldown-cmark~crates.io][c~pulldown_cmark~crates.io~badge]][c~pulldown_cmark~crates.io] [![pulldown-cmark~github][c~pulldown_cmark~github~badge]][c~pulldown_cmark~github] [![pulldown-cmark~lib.rs][c~pulldown_cmark~lib.rs~badge]][c~pulldown_cmark~lib.rs]{{hi:pulldown-cmark}}{{hi:CommonMark}}{{hi:Markdown}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`pulldown-cmark`][c~pulldown_cmark~docs]⮳{{hi:pulldown-cmark}} is a pull parser for CommonMark.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/markdown/pulldown_cmark.rs:example}}
```

## `markdown` {#markdown}

[![markdown][c~markdown~docs~badge]][c~markdown~docs] [![markdown~crates.io][c~markdown~crates.io~badge]][c~markdown~crates.io] [![markdown~github][c~markdown~github~badge]][c~markdown~github] [![markdown~lib.rs][c~markdown~lib.rs~badge]][c~markdown~lib.rs]{{hi:markdown}}{{hi:CommonMark}}{{hi:markdown}}{{hi:Parse}}{{hi:Render}}{{hi:Tokenize}} [![cat~compilers][cat~compilers~badge]][cat~compilers]{{hi:Compilers}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`markdown`][c~markdown~docs]⮳{{hi:markdown}} is a CommonMark compliant markdown parser in Rust with ASTs and extensions.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/markdown/markdown.rs:example}}
```

## `comrak` {#comrak}

[![comrak][c~comrak~docs~badge]][c~comrak~docs] [![comrak~crates.io][c~comrak~crates.io~badge]][c~comrak~crates.io] [![comrak~github][c~comrak~github~badge]][c~comrak~github] [![comrak~lib.rs][c~comrak~lib.rs~badge]][c~comrak~lib.rs]{{hi:comrak}}{{hi:CommonMark}}{{hi:Markdown}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`comrak`][c~comrak~docs]⮳{{hi:comrak}} is a 100% CommonMark-compatible GitHub Flavored Markdown parser and formatter.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/markdown/comrak.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[markdown: write](https://github.com/john-cd/rust_howto/issues/442)
</div>
