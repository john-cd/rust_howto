// ANCHOR: example
use anyhow::Context;
use quote::quote;
use syn::Expr;
use syn::ItemFn;
use syn::Result;
// `syn`` takes Rust code as input (typically a TokenStream, which represents a
// sequence of tokens in Rust code) and transforms it into an Abstract Syntax
// Tree (AST). This AST is represented by a set of syn data structures that
// mirror the grammatical structure of Rust. For example, you'll find structs in
// syn representing items (like functions, structs, enums), expressions,
// statements, attributes, and more. You can then easily traverse the AST,
// inspect different parts of the code and modify it.

// TODO
fn simple() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    Ok(())
}

// TODO
fn m() -> anyhow::Result<()> {
    // Example Rust code to parse
    let code = r#"
        fn example_function(x: i32) -> i32 {
            x + 1
        }
    "#;

    // Parse the code into a syntax tree
    // Parse a string of Rust code into the chosen syntax tree node.
    let ast: ItemFn = syn::parse_str(code).context("Failed to parse code")?;

    // Manipulate the syntax tree (e.g., change the function body)
    let new_body = quote! {
        {
            println!("Function called with: {}", x);
            x + 2
        }
    };

    let new_fn = ItemFn {
        block: Box::new(
            syn::parse2(new_body).expect("Failed to parse new body"),
        ),
        ..ast
    };

    // Generate the new code
    let generated_code = quote! {
        #new_fn
    };

    // Print the generated code
    println!("{}", generated_code);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    simple()?;
    m()?;
    Ok(())
}

// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [P1](https://github.com/john-cd/rust_howto/issues/743)
// See https://github.com/dtolnay/syn/tree/master/examples
