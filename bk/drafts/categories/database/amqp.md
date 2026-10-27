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

## Related Topics {#related-topics .skip}

- [Asynchronous programming][p~async] and [Tokio][p~tokio].
- [Async streams][p~streams].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  The `lapin` recipe now uses a temporary queue, publisher confirms, message verification, consumption, and acknowledgement. Its RabbitMQ integration test stays ignored unless a broker is running; the Docker command and invocation are documented above.
</div>
