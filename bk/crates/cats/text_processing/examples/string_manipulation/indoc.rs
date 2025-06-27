#![allow(dead_code)]
// ANCHOR: example
use indoc::indoc;

fn main() {
    // Without indoc, multi-line strings preserve indentation from source code
    let regular_string = "
        This string preserves all leading whitespace
        which makes it harder to read in source code
        when you want to format your code nicely.
    ";
    println!("Regular string:\n{regular_string}");

    // With indoc, leading whitespace is automatically stripped
    let indoc_string = indoc! {"
        This string is nicely indented in the source code,
        but when printed, the common leading whitespace
        is automatically removed.

        Empty lines are preserved.
          Additional indentation beyond the common prefix
          is also preserved.
    "};
    println!("\nIndoc string:\n{indoc_string}");

    // indoc also works with raw strings
    let raw_indoc = indoc! {r#"
        This raw string can contain "quotes" without escaping.
        It also handles special characters like \n \t without interpreting them.
        All while still removing the common leading whitespace.
    "#};
    println!("\nRaw indoc string:\n{raw_indoc}");

    // Practical example: SQL queries
    let sql_query = indoc! {"
        SELECT users.name, COUNT(orders.id) as order_count
        FROM users
        LEFT JOIN orders
            ON users.id = orders.user_id
        WHERE users.active = true
        GROUP BY users.id
        ORDER BY order_count DESC
        LIMIT 10
    "};
    println!("\nSQL query:\n{sql_query}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
