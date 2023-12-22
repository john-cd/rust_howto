# Futures crate

[Futures crate]( https://crates.io/crates/futures ) provides a number of core abstractions for writing asynchronous code.

In most cases, you will use this crate directly only when writing async code intended to work for multiple runtimes.
Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio](./tokio.md) for example.

### Selecting futures

`Select` polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,ignore
fn main() {
    use futures::{
        future::FutureExt, // for `.fuse()`
        pin_mut,
        select,
    };

    async fn task_one() { /* ... */ }
    async fn task_two() { /* ... */ }

    async fn race_tasks() {
        let t1 = task_one().fuse();
        let t2 = task_two().fuse();

        pin_mut!(t1, t2);

        select! {
            () = t1 => println!("task one completed first"),
            () = t2 => println!("task two completed first"),
        }
    }
}
```

### Joining futures

```rust,ignore
async fn foo(i: u32) -> u32 { i }

// The `join!` macro polls multiple futures simultaneously, returning a tuple of all results once complete.
assert_eq!(join!(foo(1), foo(2)), (1, 2)); // `join!` is variadic, so you can pass any number of futures

// `join_all` Create a future which represents a collection of the outputs of the futures given.
let futures = vec![foo(1), foo(2), foo(3)];
assert_eq!(futures::future::join_all(futures).await, [1, 2, 3]);
```

### Map, then, either, flatten

The `futures` crate provides an extension trait that provides a variety of convenient adapters.

```rust,ignore
use futures::future::FutureExt;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let future_of_1 = async { 1 };

    // Map this future’s output to a (possibly) different type, returning a new future of the resulting type.
    let new_future = future.map(|x| x + 3);

    // Chain on a computation for when a future finished, passing the result of the future to the provided closure f.
    let future_of_4 = future_of_1.then(|x| async move { x + 3 });
    assert_eq!(future_of_4.await, 4);

    // Conditional `Either` future
    let x = 6;
    let future = if x > 10 {
        async { true }.left_future()
    } else {
        async { false }.right_future()
    };
    assert_eq!(future.await, false);

    // Flatten nested futures
    let nested_future = async { async { 1 } };
    let future = nested_future.flatten();
    assert_eq!(future.await, 1);
    Ok(())
}
```
