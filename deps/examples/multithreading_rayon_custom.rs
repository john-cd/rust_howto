fn main() {
    // Build the threadpool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    // `install` executes the closure within the threadpool. Any attempts
    // to use join, scope, or parallel iterators will then operate
    // within that threadpool.
    let n = pool.install(|| fib(20));
    println!("{}", n);
}

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    // Conceptually, calling join() is similar to spawning two threads,
    // one executing each of the two closures.
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
    a + b
}
