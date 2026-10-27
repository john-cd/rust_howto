#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Expose a Rust type to Rhai, keep state in a `Scope` and sandbox scripts.

use rhai::Engine;
use rhai::EvalAltResult;
use rhai::Scope;

/// A Rust type that scripts can create and modify.
#[derive(Debug, Clone)]
struct Counter {
    value: i64,
}

impl Counter {
    /// Creates a counter starting at zero.
    fn new() -> Self {
        Self { value: 0 }
    }

    /// Increments the stored count.
    fn increment(&mut self) {
        self.value += 1;
    }

    /// Returns the count to the script property getter.
    fn value(&mut self) -> i64 {
        self.value
    }
}

/// Registers a custom type, reuses a `Scope` between runs and limits a script.
fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // Expose `Counter`: constructor, method and property getter.
    engine
        .register_type_with_name::<Counter>("Counter")
        .register_fn("new_counter", Counter::new)
        .register_fn("increment", Counter::increment)
        .register_get("value", Counter::value);

    // Variables persist between runs when the same `Scope` is reused.
    let mut scope = Scope::new();
    engine
        .run_with_scope(&mut scope, "let c = new_counter(); c.increment();")?;
    engine.run_with_scope(&mut scope, "c.increment();")?;
    let value: i64 = engine.eval_with_scope(&mut scope, "c.value")?;
    println!("counter = {value}");
    assert_eq!(value, 2);

    // Sandbox: stop runaway scripts and block `eval`.
    engine.set_max_operations(10_000);
    engine.set_max_call_levels(16);
    engine.disable_symbol("eval");

    let result = engine.run("loop { }");
    let error = result.expect_err("the loop must be stopped");
    assert!(matches!(*error, EvalAltResult::ErrorTooManyOperations(_)));
    println!("infinite loop stopped: {error}");

    Ok(())
}
// ANCHOR_END: example

/// Runs the type registration and limits example.
pub fn run() -> Result<(), Box<EvalAltResult>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rhai types and limits example through its entry point.
    #[test]
    fn test_rhai_types_and_limits_example_entry_point()
    -> Result<(), Box<EvalAltResult>> {
        main()
    }

    /// Checks that disabling `eval` rejects a script that invokes it.
    #[test]
    fn disabled_eval_is_rejected() {
        let mut engine = Engine::new();
        engine.disable_symbol("eval");
        assert!(engine.eval::<i64>("eval(\"40 + 2\")").is_err());
    }
}
