#![allow(dead_code)]
// ANCHOR: example
/// Finds the maximum value in an array using `crossbeam` for concurrency.
fn main() {
    let arr = &[1, 25, -4, 10];

    // Find the maximum value.
    let max = find_max(arr);
    assert_eq!(max, Some(25));
    println!("The maximum is {:?}", max);
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    // If the array is small enough, find the maximum sequentially.
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    // Split the array into two halves.
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    // Use crossbeam to spawn two threads to find the maximum in each half.
    crossbeam::scope(|s| {
        // Spawn a thread to find the maximum in the left half.
        let thread_l = s.spawn(|_| find_max(left));
        // Spawn a thread to find the maximum in the right half.
        let thread_r = s.spawn(|_| find_max(right));

        // Wait for the threads to finish and get the maximum from each half.
        // The `?` operator propagates errors if any of the threads return
        // `None`.
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
