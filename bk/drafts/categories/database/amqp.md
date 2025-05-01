# Message Queues (AMQP)

{{#include amqp.incl.md}}

`AMQP` stands for `Advanced Message Queuing Protocol`. It is an open standard messaging protocol that is used to reliably exchange messages between applications or systems, even if they are written in different [programming languages][p-programming-languages] and running on different platforms. `AMQP` ensures that messages are delivered reliably, even if there are network issues or system failures. It offers different levels of delivery guarantees, from "at-most-once" to "exactly-once" delivery.

`RabbitMQ` is an example of a popular message broker that implements the Advanced Message Queuing Protocol. It use cases include microservices communication,
task queues, background job processing, and publish-subscribe (pub-sub).

## Connect to RabbitMQ with `lapin` {#lapin}

[![lapin][c-lapin-badge]][c-lapin] [![lapin-crates.io][c-lapin-crates.io-badge]][c-lapin-crates.io] [![lapin-github][c-lapin-github-badge]][c-lapin-github] [![lapin-lib.rs][c-lapin-lib.rs-badge]][c-lapin-lib.rs]{{hi:lapin}}{{hi:RabbitMQ}}{{hi:AMQP}}{{hi:Mio}}{{hi:Futures}} [![cat-database][cat-database-badge]][cat-database]{{hi:Database interfaces}}

[`lapin`][c-lapin]⮳{{hi:lapin}} is a AMQP client library. It is a pure Rust AMQP 0.9.1 client implementation, that is feature complete, fast, and easy to use.

```rust,editable
{{#include ../../../crates/cats/database/tests/amqp/lapin.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[finish](https://github.com/john-cd/rust_howto/issues/1064)
</div>
