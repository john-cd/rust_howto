# Concurrency 

## Thread-based

### Spawn, join

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }});

    let thread_two = thread::spawn(|| { /* ... */ });
    // more stufff

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
```

When the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running. 


### [Scoped threads]( https://doc.rust-lang.org/std/thread/fn.scope.html )

```rust,ignore
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

// Our error type needs to be `Send` to be used in a channel
fn read_contents<T: AsRef<Path>>(file: T) -> Result<String, Box<dyn Error + Send>> {
    todo!()
}

fn main() {
    let (tx, rx) = mpsc::channel(); // to share state between threads, consider using a channel

    thread::scope(|scope| {  // Creates a “fork-join” scope
        scope.spawn(|| {
            println!("hello from the first scoped thread");
            let contents = read_contents("foo.txt");
            tx.send(contents).unwrap();
        });
        scope.spawn(|| {
            println!("hello from the second scoped thread");
            let contents = read_contents("bar.txt");
            tx.send(contents).unwrap();
        });
    });
    // No join; spawned threads get joined automatically once the scope ends!

    // Receive messages from the channel
    println!("hello from the main thread");
    for received in rx {
        println!("Got: {:?}", received);
    }
}
```


## Rayon - parallel processing

[Rayon docs]( https://docs.rs/rayon/latest/rayon/ )


### Parallel iteration

Convert `.iter()` or `iter_mut()` or `into_iter()` into `par_iter()` or `par_iter_mut()` or `into_par_iter()` to execute in parallel.

```rust, ignore
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|i| i * i)
         .sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut()
         .for_each(|p| *p += 1);
}
```


### Parallel sorting

```rust,ignore
let mut v = [-5, 4, 1, -3, 2];
v.par_sort();
```


### [Rayon threadpool]( https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html# )

```rust,ignore
 fn main() {
        // Build the threadpool
        let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
        // `install` executes the closure within the threadpool. Any attempts to use join, scope, or parallel iterators will then operate within that threadpool.
        let n = pool.install(|| fib(20)); 
        println!("{}", n);
   }

   fn fib(n: usize) -> usize {
        if n == 0 || n == 1 {
            return n;
        }
        // Conceptually, calling join() is similar to spawning two threads, one executing each of the two closures. 
        let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
        return a + b;
    }
```


## Message passing and shared state

The Rust standard library provides channels for message passing and smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts. 


## See Also

[Concurrency]( https://doc.rust-lang.org/book/ch16-00-concurrency.html )

[Message Passing]( https://doc.rust-lang.org/book/ch16-02-message-passing.html )

[crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)

[Parking Lot]( https://crates.io/crates/parking_lot ): more compact and efficient implementations of the standard synchronization primitives.

[Dashmap]( https://docs.rs/dashmap/5.3.3/dashmap/struct.DashMap.html# )

[Rayon]( https://github.com/rayon-rs/rayon ) for parallel data processing
