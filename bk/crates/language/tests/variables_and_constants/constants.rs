#![allow(dead_code)]
// ANCHOR: example
//! `const` Example.

// Declare a constant in the global scope.
// - The type must be provided.
// - Constant names, like statics, should always be in SCREAMING_SNAKE_CASE.
const LEN: usize = 5;

fn main() {
    // Constants are essentially inlined wherever they are used, meaning that
    // they are bitwise copied wherever they are used.
    println!("The length is: {}", LEN);

    // The above is equivalent to:
    // println!("The maximum points allowed is: {}", 10usize);

    // Constants are always immutable.
    // ERROR: LEN += 1;

    // Constants can be used in array definitions; in patterns; initializers of
    // statics & consts; enum discriminants; etc.
    let arr: [i32; LEN] = [0; LEN];
    println!("arr: {:?}", arr);

    let x = 5;
    match x {
        LEN => {
            println!("x == LEN");
        } // Use in a `match` pattern.
        _ => unreachable!(),
    }

    static MY_LEN: usize = LEN; // Use to initialize a `static`.

    // Use as a "const generic" parameter:
    struct Foo<const N: usize>([i32; N]);
    let _foo = Foo::<LEN>([0; 5]);

    // A constant's value can be calculated from an expression
    // at _compile_ time:
    const SECONDS_IN_ONE_HOUR: u32 = 60 * 60;

    // Of course, not all expressions can be evaluated at compile-time.
    // "Constant expressions" can only include literals, as above;
    // other constants; read-only statics; const parameters; and calls to
    // "const fn" functions.
    const C: usize = LEN + MY_LEN + std::mem::size_of::<u32>(); // `size_of` is `const fn`.

    // The following is prohibited, because the expression's value can only be
    // determined at runtime (strings are allocated on the heap):
    // ERROR: const MESSAGE: String = String::from("Hello");

    // Despite the inlining, it is possible to obtain a reference to a
    // constant. The compiler either creates a temporary local variable or, if
    // possible, promotes it to global static memory (via a mechanism called
    // promotion / lifetime extension). Note that multiple references to the
    // same constant are _not_ guaranteed to refer to the same memory address.
    let refer: &'static usize = &LEN;
    println!("The length again: {}", refer);

    // Let's create a struct that implements a `Drop` destructor:
    struct PrintOnDrop(&'static str);

    impl Drop for PrintOnDrop {
        fn drop(&mut self) {
            // Print the inner string slice.
            println!("{}", self.0);
        }
    }

    // Simply declaring a constant does not result in a call to `Drop`.
    const POD: PrintOnDrop = PrintOnDrop("This message does not appear.");

    // However, variables initialized with a constant follow normal `Drop`
    // behavior.
    const POD2: PrintOnDrop = PrintOnDrop("Dropped.");
    let _pod = POD2;
    let _pod1 = POD2;
    let _pod2 = &POD2;
    println!("Before drop.");
    // When each variable goes out of scope, its destructor is run. The above
    // prints `Dropped.` thrice.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
