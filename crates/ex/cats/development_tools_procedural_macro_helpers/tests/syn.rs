// ANCHOR: example
use quote::quote;
use syn::ItemFn;
fn main() {
    // Example Rust code to parse
    let code = r#"
        fn example_function(x: i32) -> i32 {
            x + 1
        }
    "#;

    // Parse the code into a syntax tree
    let ast: ItemFn = syn::parse_str(code).expect("Failed to parse code");

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
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/743)
