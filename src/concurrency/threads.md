# Threads

## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

The example uses the [crossbeam] crate, which provides data structures and functions
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{#include ../../../deps/examples/crossbeam-spawn.rs}
```

## Create a parallel pipeline

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example uses the [crossbeam] and [crossbeam-channel] crates to create
a parallel pipeline, similar to that described in the ZeroMQ [guide]
There is a data source and a data sink, with data being processed by two worker
threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using
[`crossbeam_channel::bounded`]. The producer must be on its own thread because
it produces messages faster than the workers can process them (since they sleep
for half a second) - this means the producer blocks on the call to
`[crossbeam_channel::Sender::send]` for half a second until one of the workers
processes the data in the channel. Also note that the data in the channel is
consumed by whichever worker calls receive first, so each message is delivered
to a single worker rather than both workers.

Reading from the channels via the iterator
[`crossbeam_channel::Receiver::iter`] method will block, either waiting
for new messages or until the channel is closed. Because the channels were
created within the [`crossbeam::scope`], we must manually close them via `drop`
to prevent the entire program from blocking on the worker for-loops. You can
think of the calls to `drop` as signaling that no more messages will be sent.

```rust
{#include ../../../deps/examples/crossbeam-complex.rs}
```

## Pass data between two threads

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of [crossbeam-channel] in a single producer, single
consumer (SPSC) setting. We build off the [ex-crossbeam-spawn] example by using
[`crossbeam::scope`] and [`Scope::spawn`] to manage the producer thread. Data is
exchanged between the two threads using a [`crossbeam_channel::unbounded`]
channel, meaning there is no limit to the number of storeable messages. The
producer thread sleeps for half a second in between messages.

```rust,editable
{#include ../../../deps/examples/crossbeam-spsc.rs}
```

## Maintain global mutable state

[![lazy-static-badge]][lazy-static] [![cat-rust-patterns-badge]][cat-rust-patterns]

Declare global state using [lazy-static]. [lazy-static]
creates a globally available `static ref` which requires a [`Mutex`]
to allow mutation (also see [`RwLock`]). The [`Mutex`] wrap ensures
the state cannot be simultaneously accessed by multiple threads, preventing
race conditions. A [`MutexGuard`] must be acquired to read or mutate the
value stored in a [`Mutex`].

```rust,editable
{#include ../../../../deps/examples/global-mut-state.rs}
```

## Calculate SHA256 sum of iso files concurrently

[![threadpool-badge]][threadpool] [![num-cpus-badge]][num-cpus] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA256 for every file with iso extension in the
current directory. A threadpool generates threads equal to the number of cores
present in the system found with [`num-cpus::get`].  [`Walkdir::new`] iterates
the current directory and calls [`execute`] to perform the operations of reading
and computing SHA256 hash.

```rust,editable,no_run
{#include ../../../deps/examples/threadpool-walk.rs}
```

## Draw fractal dispatching work to a thread pool

[![threadpool-badge]][threadpool] [![num-badge]][num] [![num-cpus-badge]][num-cpus] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This example generates an image by drawing a fractal from the [Julia set]
with a thread pool for distributed computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

Allocate memory for output image of given width and height with [`ImageBuffer::new`].
[`Rgb::from_channels`] calculates RGB pixel values.
Create [`ThreadPool`] with thread count equal to number of cores with [`num-cpus::get`].
[`ThreadPool::execute`] receives each pixel as a separate job.

[`mpsc::channel`] receives the jobs and [`Receiver::recv`] retrieves them.
[`ImageBuffer::put_pixel`] uses the data to set the pixel color.
[`ImageBuffer::save`] writes the image to `output.png`.

```rust,editable,no_run
{#include ../../../deps/examples/threadpool-fractal.rs}
```

[crossbeam-channel]: https://docs.rs/crate/crossbeam-channel/
[ex-crossbeam-spawn]: #spawn-a-short-lived-thread
[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn
[`crossbeam_channel::unbounded`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.unbounded.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[`crossbeam_channel::bounded`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.bounded.html
[`crossbeam_channel::Receiver::iter`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/struct.Receiver.html#method.iter
[`crossbeam_channel::Sender::send`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/struct.Sender.html#method.send
[guide]: http://zguide.zeromq.org/page:all#Divide-and-Conquer
[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::put_pixel`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.put_pixel
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`mpsc::channel`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`num-cpus::get`]: https://docs.rs/num-cpus/*/num-cpus/fn.get.html
[`Receiver::recv`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
[`Rgb::from_channels`]: https://docs.rs/image/*/image/struct.Rgb.html#method.from_channels
[`ThreadPool`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html
[`ThreadPool::execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`Walkdir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new
[Julia set]: https://en.wikipedia.org/wiki/Julia_set
{{#include ../refs/link-refs.md}}
