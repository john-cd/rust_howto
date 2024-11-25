// ANCHOR: example
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running Linux!");
}

// And this function only gets compiled if the target OS is *not*
// linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running Linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        // alternative: use cfg!
        println!("Yes. It's definitely Linux!");
    } else {
        println!("Yes. It's definitely *not* Linux!");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
