<!-- TODO Reviewed ??-??-2026 -->

# Common Architectural Patterns in Rust

{{#include common_architectures.incl.md}}

## Layered Architecture {#layered-architecture}

A layered architecture is a traditional software design pattern that organizes an application into distinct horizontal layers, like Presentation, Business Logic, and Data Access. Each layer performs a specific role, and they are arranged in a hierarchical order. It encourages separation of concerns, modularity, and testability.

Use layers when the application has distinct input, domain, and persistence responsibilities that benefit from independent tests. The example passes a repository into a service and uses an in-memory implementation, so the business layer can run without a database. Crates such as [`axum`][c~axum~docs]↗{{hi:axum}} and [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}} can support the presentation and persistence layers. Define boundaries around meaningful responsibilities: extra pass-through layers add navigation and maintenance costs without necessarily improving isolation.

```rust,editable
{{#include ../../../crates/other/examples/architecture/layered_architecture.rs:example}}
```

## Microservices Architecture {#microservices}

A microservices architecture divides a system into independently deployable services around business capabilities, each owning its data and communicating through network APIs. In Rust, [`axum`][c~axum~docs]↗{{hi:axum}} can build HTTP services and [`tonic`][c~tonic~docs]↗{{hi:tonic}} can build gRPC services. Use this structure when separate teams or workloads need independent deployment and scaling; account for network failures, data consistency, and operational overhead, which a modular monolith avoids. [Hydro][hydro~website]↗ is a distributed dataflow framework that generates binaries for a multi-machine program, a related but different way to build distributed systems.

See Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. -->, Web Programming: HTTP Server<!-- Cross-book topic: bk/drafts/categories/web-programming_http-server/index.md; not included in this preview. -->, APIs<!-- Cross-book topic: bk/drafts/categories/web-programming_http-client/apis.md; not included in this preview. -->, and AMQP<!-- Cross-book topic: bk/drafts/categories/database/amqp.md; not included in this preview. -->.

## Hexagonal Architecture (Ports and Adapters) {#hexagonal-architecture}

Hexagonal architecture keeps domain logic independent of I/O through ports, usually Rust traits, with adapters implementing those traits for a database, HTTP client, or user interface. Use it when core behavior needs isolated tests or integrations may change; small applications may not need separate adapter layers. Crates such as [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}} and [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} can provide concrete adapters, while Rust's trait system defines the ports. The example injects an in-memory adapter for tests and a second adapter into the same service.

```rust,editable
{{#include ../../../crates/other/examples/architecture/hexagonal.rs:example}}
```

See [Cross Platform](../cross-platform/index.md) development.

## Event-Driven Architecture {#event-driven-architecture}

In event-driven architecture, producers publish events for consumers to handle independently rather than issuing direct synchronous calls. Use it for asynchronous workflows or fan-out when components need temporal decoupling, while planning for ordering, duplicate delivery, and observability. [`tokio::sync::broadcast`][c~tokio~docs]↗{{hi:tokio}} provides in-process fan-out, but a receiver that falls behind can report lagged messages; the example compares it with a custom bounded bus that applies backpressure. Neither is a durable message broker; systems commonly use Kafka or RabbitMQ for durable delivery between services.

```rust,editable
{{#include ../../../crates/other/examples/architecture/event_bus.rs:example}}
```

See Asynchronous<!-- Cross-book topic: bk/drafts/categories/asynchronous/index.md; not included in this preview. --> and AMQP<!-- Cross-book topic: bk/drafts/categories/database/amqp.md; not included in this preview. -->.

## Actor Model {#actor-model}

The actor model is a conceptual model of concurrent computation that treats "actors" as the universal primitives of [concurrency][p~concurrency].
Actors encapsulate state, behavior, and a mailbox. [Actors][p~actors] communicate exclusively by sending and receiving [asynchronous][p~asynchronous] messages. They are like independent entities that can receive and process messages.

An actor can own its state and process messages sequentially, avoiding locks around that state. Shared resources outside the actor may still require synchronization. Use actors for independent, stateful entities (sessions, devices, game objects) that need isolation and supervision. Libraries like [`actix`][c~actix~docs]↗{{hi:actix}} support actor-based designs.

See the Actors<!-- Cross-book topic: bk/drafts/categories/concurrency/_actors.md; not included in this preview. --> chapter for more details.

## Command Query Responsibility Segregation (CQRS) {#cqrs}

Command Query Responsibility Segregation (CQRS) separates operations that change state from those that read it. Different models can help when write-side business rules and read-side query needs differ, but the separation alone does not guarantee better performance or scaling. If read models update asynchronously, callers must account for the delay between a successful command and its query result. The [`cqrs-es`][architecture~cqrs-es] crate provides CQRS and event-sourcing building blocks; the in-memory example illustrates the pattern without a durable store.

```rust,editable
{{#include ../../../crates/other/examples/architecture/cqrs.rs:example}}
```

## Pipelines/Streams {#pipelines}

Pipelines pass data through ordered stages that each transform or filter the input, making the flow of processing explicit. Use iterator adapters for a single-threaded transformation or [`futures`][c~futures~docs]↗{{hi:futures}} streams and [`tokio`][c~tokio~docs]↗{{hi:tokio}} channels for asynchronous or concurrent stages. Bounded channels provide backpressure between stages, but the application must still bound its inputs and collected outputs. The example feeds and drains the bounded pipeline concurrently so the producer cannot deadlock before output collection begins.

```rust,editable
{{#include ../../../crates/other/examples/architecture/pipeline.rs:example}}
```

See Data Parallelism<!-- Cross-book topic: bk/drafts/categories/concurrency/data_parallelism.md; not included in this preview. -->, [Data Processing](../data-processing/index.md).

## Event Sourcing {#event-sourcing}

Event sourcing stores an ordered sequence of domain events as the source of truth, then rebuilds current or historical state by replaying them. It is useful when audit history or rebuilding read models matters, and is often paired with CQRS. Event versions, duplicate delivery, concurrency, retention, and privacy need deliberate handling; snapshots reduce replay work but do not replace the log. The [`cqrs-es`][architecture~cqrs-es] crate includes event-store abstractions and an in-memory store for experiments, while production use needs durable persistence.

```rust,editable
{{#include ../../../crates/other/examples/architecture/event_sourcing.rs:example}}
```

## Plugin Architecture {#plugin-architecture}

A plugin architecture separates a host's stable extension points from implementations that provide optional behavior. Use it when applications need to select capabilities independently, such as importers or command handlers. Rust traits and explicit registration work for plugins compiled with the host; [`inventory`][c~inventory~docs]↗{{hi:inventory}} can collect registrations, while [`libloading`][architecture~libloading]↗ can load native libraries across a separately defined ABI. Plugins run with the host process's privileges, so untrusted code needs a separate isolation boundary.

## Entity-Component-System Architecture {#entity-component-system}

An entity-component system (ECS) represents objects as entities with components, while systems query and update matching component data. Use it when many game or simulation objects combine reusable data and behavior; scheduling and shared-data access still need coordination. [`bevy_ecs`][architecture~bevy-ecs]↗ provides an ECS independently of the full Bevy engine. See [Game Development](../../categories/game-development/index.md) for related engines and ECS libraries.

## Related Topics {#related-topics .skip}

- [Cloud](../cloud/index.md).
- Concurrency<!-- Cross-book topic: bk/drafts/categories/concurrency/index.md; not included in this preview. -->.
- [Data Processing](../data-processing/index.md).
- Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. -->.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
