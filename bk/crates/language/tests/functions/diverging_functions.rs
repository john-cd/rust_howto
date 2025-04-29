// ANCHOR: example
/// This function diverges, meaning it never returns.
/// It uses the `!` (Never) type to indicate this.
fn foo() -> ! {
    panic!("This call never returns.");
}

// This function never returns control.
#[allow(dead_code)]
fn exit_program() -> ! {
    println!("Exiting the process...");
    std::process::exit(1);
}

// This function loops forever.
#[allow(dead_code)]
fn forever() -> ! {
    loop {
        println!("and ever...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    foo(); // This will panic.
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
