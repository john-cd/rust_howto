#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Integer types:
    let a: i8 = -128; // 8-bit signed integer.
    let b: u8 = 255; // 8-bit unsigned integer.
    let c: i16 = 32767; // 16-bit signed integer.
    let d: u16 = 65535; // 16-bit unsigned integer.
    let e: i32 = 2147483647; // 32-bit signed integer (default for integers).
    let f: u32 = 4294967295; // 32-bit unsigned integer.
    let g: i64 = 9223372036854775807; // 64-bit signed integer.
    let h: u64 = 18446744073709551615; // 64-bit unsigned integer.
    let i: i128 = 170141183460469231731687303715884105727; // 128-bit signed integer.
    let j: u128 = 340282366920938463463374607431768211455; // 128-bit unsigned integer.
    let k: isize = 92233720; // Pointer-sized signed integer (depends on architecture).
    let l: usize = 18446740; // Pointer-sized unsigned integer (depends on architecture).

    // Type inference.
    let m = 42; // The compiler infers `i32` by default.

    // Floating-point types.
    let n: f32 = 3.1; // 32-bit float.
    let o: f64 = 2.7; // 64-bit float (default for floats).

    // Boolean type.
    let p: bool = true;
    let q: bool = false;

    // Character type (Unicode scalar value).
    let r: char = 'a';
    let s: char = '😀'; // Unicode emoji.
    let t: char = '∞'; // Unicode symbol.

    // Print all values.
    println!("Integers:");
    println!("i8: {a}");
    println!("u8: {b}");
    println!("i16: {c}");
    println!("u16: {d}");
    println!("i32: {e}");
    println!("u32: {f}");
    println!("i64: {g}");
    println!("u64: {h}");
    println!("i128: {i}");
    println!("u128: {j}");
    println!("isize: {k}");
    println!("usize: {l}");
    println!("inferred i32: {m}");

    println!("\nFloating-point:");
    println!("f32: {n}");
    println!("f64: {o}");

    println!("\nBooleans:");
    println!("true: {p}");
    println!("false: {q}");

    println!("\nCharacters:");
    println!("ASCII: {r}");
    println!("Emoji: {s}");
    println!("Symbol: {t}");

    // Type operations:
    println!("\nType operations:");
    println!("Integer addition: {a} + {} = {}", 100, a.wrapping_add(100)); // Using `wrapping_add` to avoid overflow panic.
    println!("Float multiplication: {n} * {o} = {}", (n as f64) * o);
    println!("Boolean AND: {p} && {q} = {}", p && q);
    println!("Boolean OR: {p} || {q} = {}", p || q);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
