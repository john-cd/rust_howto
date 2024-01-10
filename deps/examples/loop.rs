fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // `continue` and loop labels also exist:
            // https://doc.rust-lang.org/book/ch03-05-control-flow.html
        }
    };
    println!("{}", result);
}
