# XML Parsing

{{#include xml.incl.md}}

## `xml-rs` {#xml-rs}

[![xml~website][c~xml~website~badge]][c~xml~website] [![xml][c~xml~docs~badge]][c~xml~docs] [![xml~crates.io][c~xml~crates.io~badge]][c~xml~crates.io] [![xml~repo][c~xml~repo~badge]][c~xml~repo] [![xml~lib.rs][c~xml~lib.rs~badge]][c~xml~lib.rs]{{hi:xml}}{{hi:Sax}}{{hi:Parsing}}{{hi:Writer}}{{hi:xml}}{{hi:Parser}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`xml`][c~xml~docs]↗{{hi:xml}} is an XML library written in pure Rust.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xml.rs:example}}
```

## `quick-xml` {#quick-xml}

[![quick-xml][c~quick-xml~docs~badge]][c~quick-xml~docs] [![quick-xml~crates.io][c~quick-xml~crates.io~badge]][c~quick-xml~crates.io] [![quick-xml~repo][c~quick-xml~repo~badge]][c~quick-xml~repo] [![quick-xml~lib.rs][c~quick-xml~lib.rs~badge]][c~quick-xml~lib.rs]{{hi:quick-xml}}{{hi:Html}}{{hi:Parser}}{{hi:Serde}}{{hi:Writer}}{{hi:Xml}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

[`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} is a high-performance XML reader and writer. [`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} is fast for streaming.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/quick-xml.rs:example}}
```

## `xmlparser` {#xmlparser}

[![xmlparser][c~xmlparser~docs~badge]][c~xmlparser~docs] [![xmlparser~crates.io][c~xmlparser~crates.io~badge]][c~xmlparser~crates.io] [![xmlparser~repo][c~xmlparser~repo~badge]][c~xmlparser~repo] [![xmlparser~lib.rs][c~xmlparser~lib.rs~badge]][c~xmlparser~lib.rs]{{hi:xmlparser}}{{hi:Xml}}{{hi:Tokenizer}}{{hi:Parser}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`xmlparser`][c~xmlparser~docs]↗{{hi:xmlparser}} is a pull-based, zero-allocation XML parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xmlparser.rs:example}}
```

## `xml5ever` {#xml5ever}

[![xml5ever~website][c~xml5ever~website~badge]][c~xml5ever~website] [![xml5ever][c~xml5ever~docs~badge]][c~xml5ever~docs] [![xml5ever~crates.io][c~xml5ever~crates.io~badge]][c~xml5ever~crates.io] [![xml5ever~repo][c~xml5ever~repo~badge]][c~xml5ever~repo] [![xml5ever~lib.rs][c~xml5ever~lib.rs~badge]][c~xml5ever~lib.rs]{{hi:xml5ever}}{{hi:Parser}}{{hi:Parsing}}{{hi:Xml}}{{hi:Xml5}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[`xml5ever`][c~xml5ever~docs]↗{{hi:xml5ever}} is a push-based streaming parser for XML.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xml5ever.rs:example}}
```

## Parse XML as a read-only tree with `roxmltree` {#roxmltree}

[![roxmltree][c~roxmltree~docs~badge]][c~roxmltree~docs] [![roxmltree~crates.io][c~roxmltree~crates.io~badge]][c~roxmltree~crates.io] [![roxmltree~repo][c~roxmltree~repo~badge]][c~roxmltree~repo] [![roxmltree~lib.rs][c~roxmltree~lib.rs~badge]][c~roxmltree~lib.rs]{{hi:roxmltree}}{{hi:Parser}}{{hi:Tree}}{{hi:Xml}}{{hi:Dom}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`roxmltree`][c~roxmltree~docs]↗{{hi:roxmltree}} represents an XML as a read-only tree. It is good for simple parsing.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/roxmltree.rs:example}}
```

## Other Options {#other-options .skip}

[`minidom`][c~minidom~docs]↗{{hi:minidom}} builds a DOM tree.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/446)
</div>
