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

### Streams

Futures are about a single value that will eventually be produced,  but many event sources naturally produce a stream of values over time.

```rust,ignore
async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` is similar to `Iterator::next`, but returns a
    // type that implements `Future<Output = Option<T>>`.
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

// `for` loops are not usable with Streams, but for imperative-style code, `while let` and the `next`/`try_next` functions can be used:
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;
    let mut sum = 0;
    while let Some(item) = stream.next().await {  // or: .try_next().await?
        sum += item;
    }
    sum
}
```

There are combinator-style methods such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins `try_map`, `try_filter`, and `try_fold`.
To process multiple items from a stream concurrently, use the `for_each_concurrent` and `try_for_each_concurrent` methods.

```rust,ignore
use futures::StreamExt;
use tokio::fs::File;
use tokio::io;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn download_file(url: &str, filename: &str) -> Result {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    let mut file = File::create(filename).await?;
    io::copy(&mut content.as_ref(), &mut file).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result {
    let urls = ["https://www.gutenberg.org/cache/epub/43/pg43.txt"];
    let filenames = ["file1.txt"];

    let futures = urls.iter().zip(filenames.iter()).map( |(url, filename)| download_file(url, filename));

    let fut = futures::stream::iter(futures).for_each_concurrent(4, |fut| async move {
        if let Err(e) = fut.await {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    }); // Download 4 files concurrently

    fut.await;

    println!("Downloaded files successfully!");
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

## Tokio

[Tokio]( https://tokio.rs/ )

[Tokio tutorial]( https://tokio.rs/tokio/tutorial )

[rust-tokio-template]( https://github.com/Finomnis/rust-tokio-template/tree/main ): a template for a tokio-rs app with logging & command line argument parser

[Tokio examples]( https://github.com/tokio-rs/tokio/tree/master/examples )

[Tokio mini-Redis example]( https://github.com/tokio-rs/mini-redis )

Tokio provides multiple variations of the runtime. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.


## Channels for use in async code

Tokio's `sync` module provides channels for using in async code.

### OneShot

`oneshot` sends a single value from a single producer to a single consumer.
This channel is usually used to send the result of a computation to a waiter.

```rust,ignore

```


[Postage](https://lib.rs/crates/postage) is an alternative to `tokio::sync`.


## Alternatives to the Tokio async ecosystem

[async-std]( https://crates.io/crates/async-std ): async version of the Rust standard library. No longer maintained?

[Smol]( https://crates.io/crates/smol )

[Embassy]( https://embassy.dev/ )

[Mio]( https://crates.io/crates/mio ) is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions.
