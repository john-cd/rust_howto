#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `swc_ecma_parser` crate to parse
//! JavaScript code. It creates a simple JavaScript program, parses it, and
//! then prints the parsed Abstract Syntax Tree (AST) to the console.
//!
//! `swc_ecma_parser` is a library for parsing ECMAScript (JavaScript) code.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! swc_ecma_parser = "11.0.0" # Or latest
//! swc_ecma_ast = "8.0"
//! swc_common = "8.0"
//! ```
use swc_common::FileName;
use swc_common::input::StringInput;
use swc_common::sync::Lrc;
use swc_ecma_ast::EsVersion;
use swc_ecma_ast::Program;
use swc_ecma_parser::EsSyntax;
use swc_ecma_parser::Parser;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::lexer::Lexer;

fn main() {
    let cm: Lrc<swc_common::SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("example.js".into()).into(),
        "const a = 1;",
    );

    let lexer = Lexer::new(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let program: Program = parser.parse_program().expect("Failed to parse");

    // Print the parsed program.
    println!("{:#?}", program);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
