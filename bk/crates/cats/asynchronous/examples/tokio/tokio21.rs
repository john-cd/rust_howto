// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates using `tokio::task::LocalSet` to run non-`Send` tasks.
//!
//! Tasks spawned with `tokio::spawn` may move between threads, so their data
//! must be `Send`. A `LocalSet` keeps tasks on the current thread, which allows
//! types such as `Rc` that are not `Send`.

use std::rc::Rc;

use tokio::task;
use tokio::time;

/// Demonstrates using `tokio::task::LocalSet` to run non-`Send` tasks.
#[tokio::main]
async fn main() {
    // Data that is not thread-safe:
    let nonsend_data = Rc::new("world");

    // A set of tasks to be executed on the same thread:
    let local = task::LocalSet::new();

    let nonsend_data2 = nonsend_data.clone();
    local.spawn_local(async move {
        // ...
        println!("hello {nonsend_data2}");
    });

    local.spawn_local(async move {
        time::sleep(time::Duration::from_millis(100)).await;
        println!("goodbye {nonsend_data}");
    });

    // ...

    local.await;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tokio21 example through its entry point.
    #[test]
    fn test_tokio21_example_entry_point() {
        main();
    }

    #[tokio::test]
    async fn local_set_runs_non_send_tasks_to_completion() {
        let data = Rc::new(std::cell::Cell::new(0));
        let local = task::LocalSet::new();
        for i in 1..=3 {
            let data = Rc::clone(&data);
            local.spawn_local(async move {
                data.set(data.get() + i);
            });
        }
        local.await;
        assert_eq!(data.get(), 6);
    }
}
