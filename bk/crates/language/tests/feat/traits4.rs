// ANCHOR: example
trait MyHash {
    fn myhash(&self) -> u64;
}

impl MyHash for i64 {
    fn myhash(&self) -> u64 {
        *self as u64
    }
}

fn main() {
    let x = 1i64;
    println!("{}", x.myhash());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
