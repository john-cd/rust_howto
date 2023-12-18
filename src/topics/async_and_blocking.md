# Mixing Async and Blocking Code

## Calling blocking code from async code

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound computation, call to a synchronous IO API, use the `spawn_blocking` function, use `rayon`, or spawn a dedicated thread.

See [Async: What is blocking? blog post]( https://ryhl.io/blog/async-what-is-blocking/ ).

## Tokio spawn_blocking

Use [`spawn_blocking`]( https://docs.rs/tokio/1.35.0/tokio/task/fn.spawn_blocking.html ) to run a _small portion_ of synchronous code.

```rust
#[tokio::main]
async fn main() {
    // This is running on Tokio. We may not block here.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a thread where blocking is fine.
        println!("Inside spawn_blocking");
    });

    blocking_task.await.unwrap();
}
```

## Using the `rayon` crate

```rust,ignore
async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation on this thread...

        // ...or compute the sum on multiple rayon threads.
        let sum = nums.par_iter().sum();

        // Send the result back to Tokio.
        let _ = tx.send(sum);
    });

    // Wait for the rayon task.
    rx.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}
```

### Spawn a dedicated thread

If a blocking operation keeps running forever, you should run it on a dedicated thread.

```rust,ignore
async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    std::thread::spawn(move || {
        // Perform an expensive computation on this thread...
        let sum = nums.sum();

        // Send the result back to Tokio.
        let _ = tx.send(sum);
    });

    // Wait for the rayon task.
    rx.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}
```

## Call async code from blocking code

[Bridging with sync code]( https://tokio.rs/tokio/topics/bridging )

In other cases, it may be easier to structure the application as largely synchronous, with smaller or logically distinct asynchronous portions. For instance, a GUI application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[futures-executor](https://docs.rs/futures-executor/latest/futures_executor/index.html) includes a minimal executor. The [block_on](https://docs.rs/futures-executor/latest/futures_executor/fn.block_on.html) function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust
async fn do_something() { }

fn main() {
    let future = do_something(); // Futures are lazy - nothing is happening until driven to completion by .await, block_on...

    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread. See `Tokio`.
    futures::executor::block_on(future); // `future` is run and "hello, world!" is printed
}
```

### Using the Tokio runtime directly

```rust
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    // Do something time-consuming while the async background tasks execute.
    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    // Wait for all of them to complete.
    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        runtime.block_on(handle).unwrap();
    }
}

// example async code to excute
async fn my_bg_task(i: u64) {
    // By subtracting, the tasks with larger values of i sleep for a
    // shorter duration.
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);

    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;

    println!("Task {} stopping.", i);
}
```
