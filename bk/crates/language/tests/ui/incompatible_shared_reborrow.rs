fn main() {
    let mut value = 42;
    let exclusive = &mut value;
    let first = &*exclusive;
    let _mutable_reborrow = &mut *exclusive;
    println!("{first}");
}
