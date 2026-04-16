#![allow(unused)]
// ANCHOR: example
// Import our custom function-like macro.
use proc_macros::sql;

fn main() {
    // Function-like procedural macros are invoked using `!`.
    let user_query = sql!(SELECT * FROM users WHERE id = 1);
    println!("Query result: {user_query}");

    let product_query =
        sql!(INSERT INTO products (name, price) VALUES ("Widget", 9.99));
    println!("Query result: {product_query}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
