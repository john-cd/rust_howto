// ANCHOR: example
use bloomfilter::Bloom;

fn main() {
    let items_count = 100;
    let mut bloom = Bloom::new_for_fp_rate(items_count, 0.01).unwrap();

    bloom.set(&"item1");
    bloom.set(&"item2");

    assert!(bloom.check(&"item1"));
    assert!(bloom.check(&"item2"));
    assert!(!bloom.check(&"item3"));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
