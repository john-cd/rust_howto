// TODO Reviewed ??-??-2026.

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

/// Parses a JavaScript program and prints the resulting syntax tree.
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
    println!("{program:#?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the swc ecma parser example through its entry point.
    #[test]
    fn test_swc_ecma_parser_example_entry_point() {
        main();
    }

    /// Parses a snippet into a program, or returns the parse error.
    fn parse(source: &'static str) -> Result<Program, String> {
        let cm: Lrc<swc_common::SourceMap> = Default::default();
        let fm = cm
            .new_source_file(FileName::Custom("test.js".into()).into(), source);
        let lexer = Lexer::new(
            Syntax::Es(EsSyntax::default()),
            EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        );
        Parser::new_from(lexer)
            .parse_program()
            .map_err(|e| format!("{e:?}"))
    }

    /// A `const` declaration is parsed as a script with one statement.
    #[test]
    fn parses_a_const_declaration() {
        let program = parse("const a = 1;").expect("valid JavaScript");
        let Program::Script(script) = program else {
            panic!("expected a script");
        };
        assert_eq!(script.body.len(), 1);
    }

    /// Invalid JavaScript is reported as an error.
    #[test]
    fn rejects_invalid_syntax() {
        assert!(parse("const = ;").is_err());
    }
}
