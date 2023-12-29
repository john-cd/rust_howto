fn foo() -> ! {  // <-- empty type
    panic!("This call never returns.");
}

fn main() {
    println!("Will panic");
    foo();
}
