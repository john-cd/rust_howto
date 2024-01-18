# Tokio

## Basics

- creating and running a runtime, spawning tasks, working with I/O and timers, and handling errors.

### Join

By running all async expressions on the current task, the expressions are able to run concurrently but not in parallel. This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. If parallelism is required, spawn each async expression using `tokio::spawn` and pass the join handle to join!.

### Spawning

## IO

- The I/O section of the website explains how to read and write data asynchronously with Tokio, using streams, codecs, and futures. It also shows how to handle errors and timeouts.

[Current thread runtime][tokio-current-thread-runtime]⮳

equivalent to

```rust
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}
```

[LocalSet][tokio-localset]⮳

In some cases, it is necessary to run one or more futures that do not implement Send and thus are unsafe to send between threads. In these cases, a local task set may be used to schedule one or more !Send futures to run together on the same thread.

```rust
use tokio::{task, time};
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let nonsend_data = Rc::new("world");
    let local = task::LocalSet::new();

    let nonsend_data2 = nonsend_data.clone();
    local.spawn_local(async move {
        // ...
        println!("hello {}", nonsend_data2)
    });

    local.spawn_local(async move {
        time::sleep(time::Duration::from_millis(100)).await;
        println!("goodbye {}", nonsend_data)
    });

    // ...

    local.await;
}
```

{{#include ../refs/link-refs.md}}
