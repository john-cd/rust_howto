
## Basics

- creating and running a runtime, spawning tasks, working with I/O and timers, and handling errors.

### Join

By running all async expressions on the current task, the expressions are able to run concurrently but not in parallel. This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. If parallelism is required, spawn each async expression using `tokio::spawn` and pass the join handle to join!.

### Spawning

## IO

- The I/O section of the website explains how to read and write data asynchronously with Tokio, using streams, codecs, and futures. It also shows how to handle errors and timeouts.
