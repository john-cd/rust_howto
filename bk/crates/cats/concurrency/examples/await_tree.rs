// ANCHOR: example
//! This example demonstrates how to use the `await-tree` crate.
//!
//! `await-tree` is a tool for visualizing the execution of asynchronous tasks
//! in Rust. It helps understand the relationships between tasks and how
//! they are waiting on each other.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! await-tree = "0.3.0" # Or latest
//! ```
use await_tree::ConfigBuilder;
use await_tree::InstrumentAwait;
use await_tree::Registry;
use futures::future::join;
use futures::future::pending;
use itertools::Itertools;
use tokio::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Create a configuration.
    let config = ConfigBuilder::default().verbose(true).build().unwrap();

    // Create a new registry, which can contain multiple await-trees.
    let registry = Registry::new(config);

    // Spawn asynchronous tasks, each in its own await-tree.
    for i in 0_i32..2 {
        // Register an await-tree (identified by a key, here an integer,
        // but it could be a `String` or a custom struct)
        // and its root span.
        let root = registry.register(i, format!("foo {i}"));
        // Spawns a new asynchronous task, instrumenting it with the root span.
        tokio::spawn(root.instrument(foo(i)));
    }

    // Let the tasks run for a while.
    sleep(Duration::from_secs(1)).await;

    // Print the await-trees:
    // foo 0 [1.003s]
    //   baz [1.003s]
    //   bar 0 [1.003s]
    //     pending inside bar 0 [1.003s]
    //
    // foo 1 [1.003s]
    //   baz [1.003s]
    //   bar 1 [1.003s]
    //     pending inside bar 1 [1.003s]
    for (_, tree) in registry
        .collect::<i32>()
        .into_iter()
        .sorted_by_key(|(i, _)| *i)
    {
        println!("{tree}");
    }

    // Alternatively, get a clone of a tree using its key:
    // let tree = registry.get(1).unwrap();

    // It is also possible to collect anonymous await-trees:
    // anonymous background task 1 [998.902ms]
    // anonymous background task 0 [998.902ms]
    for tree in registry.collect_anonymous() {
        println!("{tree}");
    }
}

/// `foo` is an asynchronous function that spawns two other asynchronous
/// functions, `bar` and `baz`, and waits for them to complete.
///
/// # Arguments
/// * `n` - An integer used to identify the task.
async fn foo(n: i32) {
    // Instrument futures with `instrument_await`.

    // Spans of joined futures will be siblings in the tree.
    join(
        bar(n).instrument_await(format!("bar {n}")), /* The span can be a
                                                      * String */
        baz(n).instrument_await("baz"), // or `&'static str`.
    )
    .await;
}

/// `bar` is an asynchronous function that never completes.
async fn bar(n: i32) {
    // `pending()` creates a future which never resolves / never finishes -
    // useful for this example.
    pending::<()>()
        .instrument_await(format!("pending inside bar {n}"))
        .await;
}

/// `baz` is an asynchronous function that spawns an anonymous background task
/// that never completes.
async fn baz(n: i32) {
    // Inside the scope of a registered/instrumented task, we can also directly
    // spawn new tasks with `await_tree::spawn` or `spawn_anonymous`
    // to register them in the same registry. Anonymous tasks don't belong to a
    // specific tree but can be printed via `collect_anonymous`.
    await_tree::spawn_anonymous(
        format!("anonymous background task {n}"),
        pending::<()>(),
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [clean up](https://github.com/john-cd/rust_howto/issues/1416)
