<!-- TODO Reviewed ??-??-2026 -->

# Message Queues (AMQP)

{{#include amqp.incl.md}}

## Connect to RabbitMQ with `lapin` {#lapin}

[![lapin][c~lapin~docs~badge]][c~lapin~docs] [![lapin~crates.io][c~lapin~crates.io~badge]][c~lapin~crates.io] [![lapin~repo][c~lapin~repo~badge]][c~lapin~repo] [![lapin~lib.rs][c~lapin~lib.rs~badge]][c~lapin~lib.rs]{{hi:lapin}}{{hi:RabbitMQ}}{{hi:AMQP}}{{hi:Mio}}{{hi:Futures}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

[`lapin`][c~lapin~docs]↗{{hi:lapin}} is a pure Rust AMQP 0-9-1 client that integrates with [Tokio][p~tokio].

```rust,editable
{{#include ../../../crates/cats/database/examples/amqp/lapin.rs:example}}
```

Start RabbitMQ locally to run the ignored broker-backed example test:

```sh
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:4-management
```

In another terminal, from `bk/crates`, run:

```sh
cargo test -p database --example amqp --features lapin rabbitmq_round_trip -- --ignored
```

## Process Messages with a Durable NATS JetStream Consumer {#nats-jetstream}

[![async-nats][c~async-nats~docs~badge]][c~async-nats~docs] [![async-nats~crates.io][c~async-nats~crates.io~badge]][c~async-nats~crates.io] [![async-nats~repo][c~async-nats~repo~badge]][c~async-nats~repo] [![async-nats~lib.rs][c~async-nats~lib.rs~badge]][c~async-nats~lib.rs]{{hi:async-nats}}{{hi:NATS}}{{hi:JetStream}} [![cat~database][cat~database~badge]][cat~database]{{hi:Database interfaces}}

JetStream stores published messages in a stream. A durable pull consumer remembers its position on the server, so a restarted worker resumes where the previous one stopped. The recipe publishes with the server's acknowledgement and a `Nats-Msg-Id` header (the server ignores duplicate ids within its duplicate window), pulls bounded batches with a wait time, acknowledges a message only after handling it, terminates malformed messages instead of redelivering them forever, and stops cleanly on Ctrl-C. Because delivery is at least once, the handler records processed order ids so that a redelivered order is acknowledged but not applied twice.

```rust,editable
{{#include ../../../crates/cats/database/examples/nats/jetstream.rs:example}}
```

The unit tests for decoding and idempotent processing run offline. To run the broker-backed test, start a local server with JetStream enabled, then run the ignored test:

```sh
docker run --rm -p 4222:4222 nats:2 -js
```

```sh
NATS_URL=nats://127.0.0.1:4222 cargo test -p database --example nats --features nats jetstream_round_trip -- --ignored
```

## Related Topics {#related-topics .skip}

- [Asynchronous programming][p~async] and [Tokio][p~tokio].
- [Async streams][p~streams].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  The `lapin` recipe now uses a temporary queue, publisher confirms, message verification, consumption, and acknowledgement. Its RabbitMQ integration test stays ignored unless a broker is running; the Docker command and invocation are documented above.
</div>
