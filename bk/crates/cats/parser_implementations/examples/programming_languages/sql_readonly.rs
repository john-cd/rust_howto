// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Accepts only a single top-level query by parsing SQL before execution.
//!
//! Query syntax does not prove read-only behavior: a `SELECT` can call a
//! function with side effects. Enforce privileges and transaction policy in
//! the database as well.

use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

/// Why a query was rejected.
#[derive(Debug, PartialEq)]
enum Rejection {
    /// The text is not valid SQL.
    Syntax(String),
    /// More or fewer than one statement.
    NotOneStatement(usize),
    /// A statement other than a query, e.g. `DROP TABLE`.
    NotAQuery,
}

/// Returns `Ok(())` only for exactly one query statement.
///
/// # Errors
///
/// Rejects invalid syntax, multiple or empty statements, and non-query
/// statements. A query containing a function such as `nextval` still passes
/// this classifier.
fn check_single_query(sql: &str) -> Result<(), Rejection> {
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, sql)
        .map_err(|e| Rejection::Syntax(e.to_string()))?;
    match statements.as_slice() {
        [Statement::Query(_)] => Ok(()),
        [_] => Err(Rejection::NotAQuery),
        other => Err(Rejection::NotOneStatement(other.len())),
    }
}

/// Demonstrates query classification without executing SQL.
fn main() -> anyhow::Result<()> {
    assert_eq!(
        check_single_query("SELECT id FROM users WHERE age > 18"),
        Ok(())
    );
    assert_eq!(
        check_single_query("DROP TABLE users"),
        Err(Rejection::NotAQuery)
    );
    // The classic injection: a second statement hidden after a semicolon.
    assert_eq!(
        check_single_query("SELECT 1; DROP TABLE users"),
        Err(Rejection::NotOneStatement(2))
    );
    assert!(matches!(
        check_single_query("SELEC 1"),
        Err(Rejection::Syntax(_))
    ));
    println!("only single query statements are accepted");
    Ok(())
}
// ANCHOR_END: example

/// Runs the syntax-classification example without contacting a database.
#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rejects invalid syntax, non-query statements, and multiple statements.
    #[test]
    fn test_sql_readonly_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Shows that query classification alone cannot prove read-only behavior.
    #[test]
    fn accepts_query_syntax_with_a_side_effecting_function() {
        assert_eq!(
            check_single_query("SELECT nextval('example_sequence')"),
            Ok(())
        );
    }
}
