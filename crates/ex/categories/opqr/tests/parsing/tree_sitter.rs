// // ANCHOR: example
// use tree_sitter::Language;
// use tree_sitter::Parser;
// use tree_sitter::Query;
// use tree_sitter::QueryCursor;

// // Add to `Cargo.toml`:
// // tree-sitter = "0.24.7" # or latest version
// // tree-sitter-rust = "0.23.2"
// //
// // [build-dependencies]
// // cc="*"

// fn main() -> anyhow::Result<()> {
//     // Load the grammar for a language (e.g., Rust)
//     let rust_grammar: Language = tree_sitter_rust::LANGUAGE();

//     // Create a parser
//     let mut parser = Parser::new();
//     parser
//         .set_language(&rust_grammar)
//         .expect("Error loading Rust grammar");

//     // The Rust code to parse
//     // The source code to parse can be provided either as a string, a slice, a
//     // vector, or as a function that returns a slice. The text can be encoded as
//     // either UTF8 or UTF16.
//     let code = r#"
//         fn main() {
//             let x = 10;
//             println!("x = {}", x);
//         }
//     "#;

//     // Parse the code
//     let tree = parser.parse(code, None).ok_or(anyhow::anyhow!("Error parsing the code"))?;

//     let root_node = tree.root_node();
//     assert_eq!(root_node.kind(), "source_file");
//     assert_eq!(root_node.start_position().column, 0);
//     println!("{}", root_node.to_sexp());

//     // Create a query to find all variable declarations
//     // The query syntax is similar to S-expressions
//     let query_string = r#"
//         (variable_declaration
//             (identifier) @variable)
//     "#;

//     let query = Query::new(&rust_grammar, query_string)?;

//     // Create a query cursor
//     let mut query_cursor = QueryCursor::new();

//     // Execute the query on the syntax tree

// // TODO P1
//     // println!("Variable declarations:");
//     // for m in query_cursor.matches(&query, tree.root_node(), code.as_bytes()) {
//     //     for capture in m.capture {
//     //         // if capture.index as usize == query.capture_index_for_name("variable").unwrap() {
//     //         let node = capture.node;
//     //         let start = node.start_byte();
//     //         let end = node.end_byte();
//     //         let variable_name = &code[start..end];
//     //         println!("- {}", variable_name);
//     //     }
//     // }

//     // Example of using named nodes in the query
//     let query_string_fn_name = r#"
//         (function_item
//             (identifier) @function_name)
//     "#;

//     let query_fn_name = Query::new(&rust_grammar, query_string_fn_name)?;
//     let mut query_cursor_fn = QueryCursor::new();

//     // println!("\nFunction names:");
//     // for m in query_cursor_fn.matches(&query_fn_name, tree.root_node(), code.as_bytes()) {
//     //     // Each match contains the index of the pattern that matched, and a list of captures.
//     //     let capture = m.captures[0];
//     //     let node = capture.node;
//     //     let start = node.start_byte();
//     //     let end = node.end_byte();
//     //     let function_name = &code[start..end];
//     //     println!("- {}", function_name);
//     // }

//     // Example of using predicates
//     let query_string_predicate = r#"
//         (binary_expression
//             (identifier) @left
//             "+"
//             (number_literal) @right
//             (#eq? @left "x"))
//     "#;

//     let query_predicate = Query::new(&rust_grammar, query_string_predicate)?;
//     let mut query_cursor_predicate = QueryCursor::new();

//     let code_with_predicate = r#"
//         fn main() {
//             let x = 10 + 5;
//             let y = 20 + 10;
//         }
//     "#;
//     let tree_with_predicate = parser.parse(code_with_predicate, None).ok_or(anyhow::anyhow!("Error parsing the code with predicate"))?;;

//     // println!("\nBinary expressions where the left side is 'x':");
//     // for m in query_cursor_predicate
//     //     .matches(&query_predicate, tree_with_predicate.root_node() code_with_predicate.as_bytes())
//     // {
//     //     let left_capture = m.captures[0];
//     //     let right_capture = m.captures[1];
//     //     let left_node = left_capture.node;
//     //     let right_node = right_capture.node;
//     //     let left_name =
//     //         &code_with_predicate[left_node.start_byte()..left_node.end_byte()];
//     //     let right_value = &code_with_predicate
//     //         [right_node.start_byte()..right_node.end_byte()];
//     //     println!("- {} + {}", left_name, right_value);
//     // }

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/827)
