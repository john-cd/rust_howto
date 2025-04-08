// ANCHOR: example
//! # Rhai scripting
//!
//! Rhai is a lightweight, Rust-based scripting language.
//! It allows to embed scripting capabilities in Rust applications.
//!
//! In Cargo.toml, add:
//! ```
//! [dependencies]
//! rhai = "1.21.0" # or latest version
//! ```
use rhai::Engine;
use rhai::EvalAltResult;
use rhai::Scope;

/// Typically, you'll use Rhai as a thin dynamic wrapper layer over Rust code.
/// Here is a simple function that we'll expose as a Rust API into Rhai for scripts to call:
fn add(x: i64, y: i64) -> i64 {
    x + y
}

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create a new `Rhai` scripting engine.
    let mut engine = Engine::new();

    // Register a Rust function to be used in the script.
    engine.register_fn("add", add);
    // With a closure:
    engine.register_fn("foo", |x: i64, y: i64| x * 2 + y * 3);
    // You can also register a custom type with `build_type`.

    // Safety: turn the Engine into an immutable instance.
    // See https://rhai.rs/book/safety/sandbox.html
    let engine = engine;

    // Define a simple script.
    // Review further examples at https://github.com/rhaiscript/rhai/tree/main/scripts
    let script = r#"
        // Define variables:
        let a = 42;
        let b = 58;
        /* Call the registered function */
        let result = add(a, b);
        result
    "#;

    // Evaluate the script (getting a return value).
    match engine.eval::<i64>(script) {
        Ok(result) => println!("The result of the script is: {}", result),
        Err(e) => println!("Error evaluating the script: {:?}", e),
    }
    // Use 'Dynamic' as the return type if you're not sure what it will be.

    // Or run a script:
    let script = "print(40 + 2);";
    engine.run(script)?;

    // Or evaluate or run a script file:
    // let result = engine.eval_file::<i64>("hello_world.rhai".into())?;
    // engine.run_file("hello_world.rhai".into())?;

    // If you only need to evaluate expressions, use `eval_expression*`.
    // `Scope` may be used to pass in variables / constants.
    let mut scope = Scope::new();
    scope.push("x", 42_i64);
    scope.push_constant("SCALE", 10_i64);
    let result: i64 =
        engine.eval_expression_with_scope(&mut scope, "(x + 1) * SCALE")?;
    println!("{}", result);

    // Rhai also allows calling a Rhai-scripted function from Rust via
    // `Engine::call_fn`. Rhai has lots more features: check https://rhai.rs/ and the Rhai book!

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<(), Box<EvalAltResult>> {
    main()?;
    Ok(())
}
