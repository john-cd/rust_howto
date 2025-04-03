// ANCHOR: example

/// A trait demonstrating the use of associated constants.
trait Example {
    /// An associated constant without a default value.
    ///
    /// Implementors of this trait must provide a value for this constant.
    const CONST_NO_DEFAULT: i32;

    /// An associated constant with a default value.
    ///
    /// Implementors of this trait can optionally override this default.
    const CONST_WITH_DEFAULT: i32 = 99;
}
struct S;

impl Example for S {
    const CONST_NO_DEFAULT: i32 = 0;
}

fn main() {
    println!("{} {}", S::CONST_NO_DEFAULT, S::CONST_WITH_DEFAULT);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
