<!-- TODO Reviewed ??-??-2026 -->

[`AMQP`][wikipedia~AMQP]↗{{hi:AMQP}} (Advanced Message Queuing Protocol) lets clients written in different languages exchange messages through a broker. [`RabbitMQ`][rabbitmq~website]↗{{hi:RabbitMQ}} supports AMQP 0-9-1: publishers send messages to exchanges, which route them to queues for consumers. Consumer acknowledgements and publisher confirms help applications track processing and broker receipt, but a message can be delivered again after a failure; consumers should make processing idempotent rather than assume exactly-once delivery.

[`lapin`][c~lapin~docs]↗{{hi:lapin}} is an asynchronous Rust client for AMQP 0-9-1 and RabbitMQ.

[`async-nats`][c~async-nats~docs]↗{{hi:async-nats}} is the official asynchronous client for [NATS][nats~website]↗{{hi:NATS}}, including its JetStream persistence layer.

Which tool fits? An in-process channel (`std::sync::mpsc`, `tokio::sync::mpsc`, `flume`) moves messages between tasks of one process and loses them when it exits. RabbitMQ routes messages through exchanges to durable queues with per-message acknowledgements. NATS Core is a very fast fire-and-forget publish-subscribe system with at-most-once delivery; NATS JetStream adds persistent streams and durable consumers with explicit acknowledgements (at-least-once delivery). Apache Kafka, with a partitioned replayable log and a different client ecosystem, is the usual choice for very high-throughput event streams. None of these gives exactly-once processing on its own: make handlers idempotent.

| Recipe                                                                                | Crates                | Categories                                     |
| ------------------------------------------------------------------------------------- | --------------------- | ---------------------------------------------- |
| [Connect to RabbitMQ with `lapin`][ex~database~lapin]                                 | {{#crate lapin}}      | {{#categories database="Database interfaces"}} |
| [Process messages with a durable NATS JetStream consumer][ex~database~nats-jetstream] | {{#crate async-nats}} | {{#categories database="Database interfaces"}} |

<div class="hidden">
  This shared include introduces AMQP, RabbitMQ's queue flow, and the `lapin` client used by the recipe.
</div>
