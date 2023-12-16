# Async

[Are we async yet?]( https://areweasyncyet.rs/ )

[Asynchronous Programming in Rust (book)]( https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html )

- The most fundamental traits, types and functions, such as the `Future` trait are provided by the standard library. A future represents an asynchronous computation that might not have finished yet.
- The `async`/`await` syntaxic sugar is supported directly by the Rust compiler.
- Many utility types, macros and functions are provided by the `futures` crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as `Tokio` and `async-std`. Most async applications, and some async crates, depend on a specific runtime.
- In most cases, prefer `Tokio` - see [The State of Async Rust: Runtimes]( https://corrode.dev/blog/async/ )

```rust,ignore
// async transforms a block of code into a state machine that implements a trait called Future.
async fn foo() -> u8 { 5 }  // `foo()` returns a type that implements `Future<Output = u8>`.

fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;  // `foo().await` will result in a value of type `u8`.
        x + 5
    }
}

async fn first_task() -> SomeStruct { /* ... */ }
async fn second_task_1(&s: SomeStruct ) { /* ... */ }
async fn second_task_2() { /* ... */ }

async fn do_something() { // The value returned by async fn is a Future.

    let s = first_task().await; // use `.await` to prevent blocking the thread

    let f1 = second_task_1(&s);
    let f2 = second_task_2();
    futures::join!(f1, f2);     // `join!` is like `.await` but can wait for multiple futures concurrently.
}

fn main() {
    let future = do_something(); // Futures are lazy - nothing is happening until driven to completion by .await, block_on...

    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread. See `Tokio`.
    futures::executor::block_on(future); // `future` is run and "hello, world!" is printed
}
```

## Futures crate

[Futures crate]( https://crates.io/crates/futures )

### Select

Polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

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

//Polls multiple futures simultaneously, returning a tuple of all results once complete.
assert_eq!(join!(foo(1), foo(2)), (1, 2)); // `join!` is variadic, so you can pass any number of futures

let futures = vec![foo(1), foo(2), foo(3)];
// Create a future which represents a collection of the outputs of the futures given.
assert_eq!(futures::future::join_all(futures).await, [1, 2, 3]);
```

### Map, then, either, flatten

```rust,ignore
use futures::future::FutureExt;  // An extension trait for Futures that provides a variety of convenient adapters.

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

## Async traits

The async working group's headline goal for 2023 is to stabilize a "minimum viable product" (MVP) version of async functions in traits.
In the meanwhile, use the [Async trait crate]( https://github.com/dtolnay/async-trait ).

```rust,ignore
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}
```
