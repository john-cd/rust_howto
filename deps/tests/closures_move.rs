use std::thread;

#[test]
fn test() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // `move` forces the closure to take ownership of the values it uses.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
