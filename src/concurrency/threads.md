# Threads

## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

The example uses the [crossbeam][crossbeam] crate, which provides data structures and functions for concurrent and parallel programming. [`Scope::spawn`][Scope::spawn] spawns a new scoped thread that is guaranteed to terminate before returning from the closure that passed into [`crossbeam::scope`][crossbeam::scope] function, meaning that you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{{#include ../../deps/examples/crossbeam-spawn.rs}}
```

## Create a parallel pipeline

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example uses the [crossbeam][crossbeam] and [crossbeam-channel][crossbeam-channel] crates to create a parallel pipeline, similar to that described in the ZeroMQ [guide] There is a data source and a data sink, with data being processed by two worker threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using
[`crossbeam_channel::bounded`][crossbeam_channel::bounded]. The producer must be on its own thread because it produces messages faster than the workers can process them (since they sleep for half a second) - this means the producer blocks on the call to
[`crossbeam_channel::Sender::send`][crossbeam_channel::Sender::send] for half a second until one of the workers processes the data in the channel. Also note that the data in the channel is consumed by whichever worker calls receive first, so each message is delivered to a single worker rather than both workers.

Reading from the channels via the iterator
[`crossbeam_channel::Receiver::iter`][crossbeam_channel::Receiver::iter] method will block, either waiting for new messages or until the channel is closed. Because the channels were created within the [`crossbeam::scope`][crossbeam::scope] we must manually close them via `drop` to prevent the entire program from blocking on the worker for-loops. You can think of the calls to `drop` as signaling that no more messages will be sent.

```rust
{{#include ../../deps/examples/crossbeam-complex.rs}}
```

## Pass data between two threads

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of [crossbeam-channel][crossbeam-channel] in a single producer, single consumer (SPSC) setting. We build off the [ex-crossbeam-spawn][ex-crossbeam-spawn] example by using
[`crossbeam::scope`][crossbeam::scope] and [`Scope::spawn`][Scope::spawn] to manage the producer thread. Data is exchanged between the two threads using a [`crossbeam_channel::unbounded`][crossbeam_channel::unbounded] channel, meaning there is no limit to the number of storeable messages. The producer thread sleeps for half a second in between messages.

```rust,editable
{{#include ../../deps/examples/crossbeam-spsc.rs}}
```

## Maintain global mutable state

[![lazy-static-badge]][lazy-static]  [![cat-rust-patterns-badge]][cat-rust-patterns]

Declare global state using [lazy-static][lazy-static]. [lazy-static]
creates a globally available `static ref` which requires a [`Mutex`][Mutex] to allow mutation (also see [`RwLock`][RwLock]). The [`Mutex`][Mutex] wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A [`MutexGuard`][MutexGuard] must be acquired to read or mutate the value stored in a [`Mutex`][Mutex]
[]\

```rust,editable
{{#include ../../deps/examples/global-mut-state.rs}}
```

## Calculate SHA256 sum of iso files concurrently

[![threadpool-badge]][threadpool] [![num-cpus-badge]][num-cpus] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA256 for every file with iso extension in the current directory. A threadpool generates threads equal to the number of cores present in the system found with [`num-cpus::get`][num-cpus::get]. [`Walkdir::new`][walkdir::Walkdir::new] iterates the current directory and calls [`execute`][threadpool-execute] to perform the operations of reading and computing SHA256 hash.

```rust,editable,no_run
{{#include ../../deps/examples/threadpool-walk.rs}}
```

## Draw fractal dispatching work to a thread pool

[![threadpool-badge]][threadpool] [![num-badge]][num] [![num-cpus-badge]][num-cpus] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This example generates an image by drawing a fractal from the [Julia set] with a thread pool for distributed computation.

[![Julia Set]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )

Allocate memory for output image of given width and height with [`ImageBuffer::new`][ImageBuffer::new].
[`Rgb::from_channels`][Rgb::from_channels] calculates RGB pixel values. Create [`ThreadPool`][ThreadPool] with thread count equal to number of cores with [`num-cpus::get`][num-cpus::get].
[`ThreadPool::execute`][ThreadPool::execute] receives each pixel as a separate job.

[`mpsc::channel`][mpsc::channel] receives the jobs and [`Receiver::recv`][Receiver::recv] retrieves them.
[`ImageBuffer::put_pixel`][ImageBuffer::put_pixel] uses the data to set the pixel color.
[`ImageBuffer::save`][ImageBuffer::save] writes the image to `output.png`.

```rust,editable,no_run
{{#include ../../deps/examples/threadpool-fractal.rs}}
```

[ex-crossbeam-spawn]: #spawn-a-short-lived-thread
{{#include ../refs/link-refs.md}}
