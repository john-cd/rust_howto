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
`[crossbeam_channel::Sender::send`] for half a second until one of the workers
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

[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[crossbeam-channel]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/index.html
[`crossbeam_channel::bounded`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.bounded.html
[`crossbeam_channel::Receiver::iter`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/struct.Receiver.html#method.iter
[`crossbeam_channel::Sender::send`]: https://docs.rs/crossbeam-channel/*/crossbeam_channel/struct.Sender.html#method.send
[guide]: http://zguide.zeromq.org/page:all#Divide-and-Conquer
