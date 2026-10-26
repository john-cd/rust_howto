// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Accepts only a single read-only `SELECT` statement, by parsing the SQL
//! with `sqlparser` before it reaches a database.

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
fn check_read_only(sql: &str) -> Result<(), Rejection> {
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, sql)
        .map_err(|e| Rejection::Syntax(e.to_string()))?;
    match statements.as_slice() {
        [Statement::Query(_)] => Ok(()),
        [_] => Err(Rejection::NotAQuery),
        other => Err(Rejection::NotOneStatement(other.len())),
    }
}

/// Demonstrates accepting a single read-only SQL query.
fn main() -> anyhow::Result<()> {
    assert_eq!(
        check_read_only("SELECT id FROM users WHERE age > 18"),
        Ok(())
    );
    assert_eq!(
        check_read_only("DROP TABLE users"),
        Err(Rejection::NotAQuery)
    );
    // The classic injection: a second statement hidden after a semicolon.
    assert_eq!(
        check_read_only("SELECT 1; DROP TABLE users"),
        Err(Rejection::NotOneStatement(2))
    );
    assert!(matches!(
        check_read_only("SELEC 1"),
        Err(Rejection::Syntax(_))
    ));
    println!("only single SELECT statements are accepted");
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the SQL validation example as a test.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
