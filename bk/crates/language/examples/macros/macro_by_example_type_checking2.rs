#![allow(unused)]
// ANCHOR: example

// 1. If you want to allow expressions but ensure they are numeric, you can
//    enforce type checking:
macro_rules! ensure_numeric_expression {
    ($val:expr) => {{
        let _: f64 = $val; // Forces `$val` to be coercible to `f64`.
        println!("Numeric value: {}", $val);
    }};
}
// You may also use:
// let _ = 0 + $val;

// 2. Accept only literals and rejects expressions or non-numeric values.
macro_rules! ensure_numeric_literal {
    ($val:literal) => {{
        let _num: f64 = $val as f64;
        println!("Numeric value: {}", _num);
    }};
    ($val:expr) => {
        compile_error!("Argument must be a numeric literal.");
    };
}

// 3. Ensure that a literal passed as an argument is a string.
macro_rules! must_be_string {
    ($lit:literal) => {
        // This block acts as a compile-time gate.
        {
            const _: () = {
                // This helper function *only* accepts a string slice.
                const fn must_be_string(_s: &str) {}

                // This line will only compile if `$lit` is a string
                // literal. Any other literal type (numeric, char,
                // bool) will cause a "mismatched types" error.
                must_be_string($lit);
            };
        }
    };
}

// 4. Ensure that only certain types can be processed by a macro.
//    - Define a trait that describes the action we want to perform. This
//      creates a common interface for different types to implement.
trait Processable {
    fn process(&self) {}
}

// B. Create a helper macro to implement our trait for e.g. all integer types at
//    once. This avoids a lot of repetitive boilerplate code.
macro_rules! impl_processable_for_integer {
    // `$t:ty` matches a type. `$(...),*` means "repeat for zero or more, separated by commas".
    ($($t:ty),*) => {
        $(
            // This is the implementation of the trait above.
            // It will be generated for every type passed into the macro.
            impl Processable for $t { }
        )*
    };
}

// C. Run the helper macro for e.g. all primitive integer types.
impl_processable_for_integer!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

// D. Call the trait method.
// This compiles only if there is an `impl` for the type of the expression.
macro_rules! integer_only {
    ( $e:expr ) => {
        $e.process();
    };
}

fn main() {
    ensure_numeric_expression!(2.0 + 2.0);
    // ERROR ensure_numeric_expression!("text");

    ensure_numeric_literal!(42);
    ensure_numeric_literal!(1.234);
    // ERROR ensure_numeric_literal!(2 + 2);
    // ERROR ensure_numeric_literal!("hello");

    must_be_string!("42");
    // ERROR: must_be_string!(42);

    integer_only!(41 + 1);
    // ERROR: integer_only!(4.2_f64);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
