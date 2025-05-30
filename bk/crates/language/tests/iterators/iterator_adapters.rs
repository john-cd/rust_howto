// ANCHOR: example
fn main() {
    let numbers = [1, 2, 3];

    // `map` applies a closure to each element, producing a new iterator.
    // `collect` transforms the iterator back into a collection. It is required,
    // because iterators are lazy.
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled); // Output: Doubled numbers: [2, 4, 6]

    // `filter` keeps elements that satisfy a predicate (a closure that returns
    // a boolean).
    let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even numbers: {:?}", even); // Output: Even numbers: [2]

    // `take` takes the first `n` elements of an iterator.
    let first_two: Vec<_> = numbers.iter().take(2).collect();
    println!("First two: {:?}", first_two); // Output: First two: [1, 2]

    // `skip` skips the first `n` elements of an iterator.
    let after_two: Vec<_> = numbers.iter().skip(2).collect();
    println!("After skipping two: {:?}", after_two); // Output: After skipping two: [3]

    // `chain` concatenates two iterators.
    let more_numbers = [4, 5];
    let combined: Vec<_> = numbers.iter().chain(more_numbers.iter()).collect();
    println!("Combined two vectors: {:?}", combined);

    // `zip` combines two iterators element-wise into pairs.
    let letters = ['a', 'b', 'c'];
    let zipped: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    println!("Zipped numbers and letters: {:?}", zipped); // Output: Zipped numbers and letters: [(1, 'a'), (2, 'b'), (3, 'c')]

    // `enumerate` adds an index to each element.
    let indexed: Vec<_> = numbers.iter().enumerate().collect();
    println!("Indexed numbers: {:?}", indexed); // Output: Indexed numbers: [(0, 1), (1, 2), (2, 3)]

    // Chain multiple adapters together.
    let even_doubled: Vec<_> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("Even numbers doubled: {:?}", even_doubled); // Output: Even numbers doubled: [4, 8]
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
