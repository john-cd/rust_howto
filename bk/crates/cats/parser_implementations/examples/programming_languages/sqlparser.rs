#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `sqlparser` crate to parse SQL
//! statements, extract information from them, and regenerate the original SQL
//! text from the parsed Abstract Syntax Tree (AST).
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! sqlparser = { version = "0.54.0", features = ["visitor"] }
//! ```
use std::ops::ControlFlow;

use sqlparser::ast::SetExpr;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() -> anyhow::Result<()> {
    let sql = "SELECT * FROM users WHERE age > 18";

    let dialect = GenericDialect {}; // Or AnsiDialect, PostgreSqlDialect, etc.

    // Create a parser for a `Dialect`:
    let mut parser = Parser::new(&dialect).try_with_sql(sql)?;
    // You may configure the parser with e.g.
    // `.with_recursion_limit(n).with_options(options)`.

    // Parse potentially multiple statements; tokenize the sql string and sets
    // this parser's state to parse the resulting tokens.
    let statements = parser.parse_statements()?;

    // You may also use `parse_sql`:
    // let statements = Parser::parse_sql(
    //   &dialect, "SELECT * FROM foo"
    // )?;

    for statement in statements.clone() {
        // `statement` is a top-level construct: SELECT, INSERT, CREATE, etc.
        match statement {
            // SELECT statment.
            Statement::Query(query) => match *query.body {
                // SELECT .. FROM .. HAVING (no ORDER BY or set operations).
                SetExpr::Select(select) => {
                    println!("SELECT statement:");
                    println!("  Projection: {:?}", select.projection);
                    println!("  From: {:?}", select.from);
                    println!("  Where: {:?}", select.selection);
                }
                _ => println!("Not a SELECT statement"),
            },
            _ => println!("Not a Query statement"),
        }
    }

    // The original SQL text can be generated from the AST
    // (Abstract Syntax Tree).
    assert_eq!(statements[0].to_string(), sql);

    // You may also visit all statements, expressions, or tables.
    // You can also implement a custom `Visitor`.
    let mut visited = vec![];
    let _ = sqlparser::ast::visit_statements(&statements, |stmt| {
        visited.push(format!("Statement: {}", stmt));
        ControlFlow::<()>::Continue(())
    });
    println!("{:?}", visited);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
