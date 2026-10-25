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

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
