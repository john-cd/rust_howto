#![allow(unused)]
// ANCHOR: example

// Use `compile_error` to provide custom error messages at compile time:
macro_rules! must_be_an_identifier {
    ($x:ident) => {
        // Your code goes here.
    };
    ($_:tt) => {
        // Causes compilation to fail with the given error message.
        // It is the compiler-level form of `panic!`,
        // but emits an error during compilation rather than at runtime.
        compile_error!("This macro only accepts an identifier.");
    };
}

// Trick to ensure that a literal passed as an argument is numeric.
macro_rules! must_be_numeric {
    ($lit:literal) => {
        // This block acts as a compile-time gate.
        {
            // This operation will fail to compile if `$lit`
            // is not a number. The compiler
            // knows that you can't add a string to an integer.
            // We use a `const` block to ensure this check happens at compile
            // time.
            const _: () = {
                let _ = 0 + $lit;
            };
        }
    };
}

// Trick to ensure that a literal passed as an argument is a string.
macro_rules! must_be_string {
    ($lit:literal) => {
        // This block acts as a compile-time gate.
        {
            // We use a const block to force this check to happen at compile
            // time.
            const _: () = {
                // This helper function *only* accepts a string slice.
                const fn must_be_string(_s: &str) {}

                // This line will only compile if `$literal` is a string
                // literal. Any other literal type (numeric, char,
                // bool) will cause a "mismatched types" error.
                must_be_string($lit);
            };
        }
    };
}

// Trick to ensure that only certain types can be processed by a macro.
// 1. Define a trait that describes the action we want to perform.
// This creates a common interface for different types to implement.
trait Processable {
    fn process(&self) {}
}

// 2. Create a helper macro to implement our trait for all integer types at once.
// This avoids a lot of repetitive boilerplate code.
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

// 3. Run the helper macro for all primitive integer types.
impl_processable_for_integer!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

macro_rules! integer_only {
    ( $e:expr ) => {
        // Static dispatch. This compiles only if there is an `impl` for the type of the expression.
        $e.process();
    }
}

fn main() {

    must_be_an_identifier!(foo);
    //must_be_an_identifier!(42);

    must_be_numeric!(42);
    // ERROR: when_numeric!("42");

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
