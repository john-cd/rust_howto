// ANCHOR: example
#![allow(clippy::approx_constant)]
use core::cmp::Ordering;

use rug::Float;
use rug::Integer;
use rug::Rational;
use rug::float::Round;
use rug::ops::DivAssignRound;

// The `rug` crate performs operations with arbitrary precision numbers.

fn main() {
    // Working with large integers:
    let a = Integer::from(123456789012345678901234567890_u128);
    let b = Integer::from(987654321098765432109876543210_u128);

    // Or parse a string:
    let s = "123456789012345678901234567890";
    let c = s.parse::<Integer>().unwrap();

    // Casting to primitive types (None if not possible)
    assert_eq!(c.to_u32(), None);

    // Operations on two borrowed numbers result in an incomplete-computation
    // value that has to be assigned to a new value.
    let a_b_ref = &a + &b;
    let sum = Integer::from(a_b_ref);
    println!("Sum of integers: {}", sum);

    // Most methods have three versions:
    // The first method consumes the operand.
    // The second method has a “_mut” suffix and mutates the operand.
    // The third method has a “_ref” suffix and borrows the operand.
    // The returned item is an incomplete-computation value that can be assigned
    // to an Integer.
    let a = Integer::from(-15);
    let _abs_a = a.abs();
    let mut b = Integer::from(-16);
    b.abs_mut();
    let c = Integer::from(-17);
    let r = c.abs_ref();
    let _abs_c = Integer::from(r);

    // You can also mix Integer and primitive integer types; the result will be
    // an Integer.

    // Working with rationals:
    let r1 = Rational::from((1, 3));
    let r2 = Rational::from((2, 3));
    let sum_rational = Rational::from(&r1 + &r2);
    println!("Sum of rationals: {}", sum_rational);

    // Working with floating-point numbers:
    // Float is a multi-precision floating-point number
    // with arbitrarily large precision and correct rounding.
    // The primitive `f64` has a precision of 53.
    let mut f1 = Float::with_val(53, 3.141_592_653_589_793);
    let f2 = Float::with_val(53, 2.718_281_828_459_045);

    let dir = f1.div_assign_round(3.0, Round::Down);
    // The direction of the rounding is returned.
    // Since we rounded down, direction is Ordering::Less
    assert_eq!(dir, Ordering::Less);

    let product_ref = &f1 * &f2;
    let product = Float::with_val(53, product_ref);
    println!("Product of floats: {}", product);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
