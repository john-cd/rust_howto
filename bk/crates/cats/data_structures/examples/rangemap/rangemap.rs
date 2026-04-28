// ANCHOR: example
use rangemap::RangeMap;

pub fn main() {
    let mut rm = RangeMap::new();

    rm.insert(0..50, "first half");
    rm.insert(50..100, "second half");

    assert_eq!(rm.get(&25), Some(&"first half"));
    assert_eq!(rm.get(&75), Some(&"second half"));
    assert_eq!(rm.get(&100), None);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
