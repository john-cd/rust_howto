<!-- TODO Reviewed ??-??-2026 -->
[`AMQP`][wikipedia~AMQP]↗{{hi:AMQP}} (Advanced Message Queuing Protocol) lets clients written in different languages exchange messages through a broker. [`RabbitMQ`][rabbitmq~website]↗{{hi:RabbitMQ}} supports AMQP 0-9-1: publishers send messages to exchanges, which route them to queues for consumers. Consumer acknowledgements and publisher confirms help applications track processing and broker receipt, but a message can be delivered again after a failure; consumers should make processing idempotent rather than assume exactly-once delivery.

[`lapin`][c~lapin~docs]↗{{hi:lapin}} is an asynchronous Rust client for AMQP 0-9-1 and RabbitMQ.

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Connect to RabbitMQ with `lapin`][ex~database~lapin] | [![lapin][c~lapin~docs~badge]][c~lapin~docs] | [![cat~database][cat~database~badge]][cat~database] |

<div class="hidden">
This shared include introduces AMQP, RabbitMQ's queue flow, and the `lapin` client used by the recipe.
</div>
