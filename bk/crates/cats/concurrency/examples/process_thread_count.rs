// TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Prints the current process's thread count when the target supports the query.
fn main() {
    match num_threads::num_threads() {
        Some(count) => println!("current process threads: {count}"),
        None => println!("this target cannot report the process thread count"),
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    #[test]
    fn reported_thread_count_is_positive_or_unavailable() {
        if let Some(count) = num_threads::num_threads() {
            assert!(count.get() > 0);
        }
    }
}
