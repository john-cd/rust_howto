trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
}
struct S;

impl Example for S {
    const CONST_NO_DEFAULT: i32 = 0;
}

fn main() {
    println!("{} {}", S::CONST_NO_DEFAULT, S::CONST_WITH_DEFAULT);
}
