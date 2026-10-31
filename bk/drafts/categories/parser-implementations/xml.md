<!-- TODO Reviewed ??-??-2026 -->

# XML Parsing

{{#include xml.incl.md}}

Choose a streaming parser when documents may be large or should be processed incrementally. Use a tree parser when convenient random access matters. XML parsers differ in their support for namespaces, encodings, and document models; check the selected crate's behavior against the input you accept.

## `xml-rs` {#xml-rs}

[![xml~website][c~xml~website~badge]][c~xml~website] {{#badges xml}} {{hi:Sax}} {{hi:Parsing}} {{hi:Writer}} {{hi:xml}} {{hi:Parser}} {{#categories parser-implementations="Parser implementations"}}

[`xml`][c~xml~docs]↗{{hi:xml}} is an XML library written in pure Rust.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xml.rs:example}}
```

## `quick-xml` {#quick-xml}

{{#badges quick-xml asynchronous=Asynchronous encoding=Encoding parser-implementations="Parser implementations" parsing="Parsing tools"}}{{hi:Html}}{{hi:Parser}}{{hi:Serde}}{{hi:Writer}}{{hi:Xml}}

[`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} is a high-performance XML reader and writer. [`quick-xml`][c~quick-xml~docs]↗{{hi:quick-xml}} is fast for streaming.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/quick_xml.rs:example}}
```

## `xmlparser` {#xmlparser}

{{#badges xmlparser parser-implementations="Parser implementations"}}{{hi:Xml}}{{hi:Tokenizer}}{{hi:Parser}}

[`xmlparser`][c~xmlparser~docs]↗{{hi:xmlparser}} is a pull-based, zero-allocation XML parser.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xmlparser.rs:example}}
```

## `xml5ever` {#xml5ever}

[![xml5ever~website][c~xml5ever~website~badge]][c~xml5ever~website] {{#badges xml5ever}} {{hi:Parser}} {{hi:Parsing}} {{hi:Xml}} {{hi:Xml5}} {{#categories parser-implementations="Parser implementations",web-programming="Web programming"}}

[`xml5ever`][c~xml5ever~docs]↗{{hi:xml5ever}} is a push-based streaming parser for XML.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/xml5ever.rs:example}}
```

## Parse XML as a read-only tree with `roxmltree` {#roxmltree}

{{#badges roxmltree parser-implementations="Parser implementations"}}{{hi:Parser}}{{hi:Tree}}{{hi:Xml}}{{hi:Dom}}

[`roxmltree`][c~roxmltree~docs]↗{{hi:roxmltree}} represents an XML as a read-only tree. It is good for simple parsing.

```rust,editable
{{#include ../../../crates/cats/parser_implementations/examples/xml/roxmltree.rs:example}}
```

## Other Options {#other-options .skip}

[`minidom`][c~minidom~docs]↗{{hi:minidom}} builds a DOM tree.

## Related Topics {#related-topics .skip}

- [Parsing Utilities](../parsing/parsing.md) covers parsers for custom grammars.
- [HTML and CSS](html.md) covers parsing web documents and selecting elements.
- [Web Scraping](../web-programming/scraping.md) demonstrates extracting content from web pages.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
