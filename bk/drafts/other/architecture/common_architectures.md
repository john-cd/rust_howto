# Common Architectural Patterns in Rust

{{#include common_architectures.incl.md}}

## Layered Architecture {#layered-architecture}

A layered architecture is a traditional software design pattern that organizes an application into distinct horizontal layers, like Presentation, Business Logic, and Data Access. Each layer performs a specific role, and they are arranged in a hierarchical order. It encourages separation of concerns, modularity, and testability.

```rust,editable
{{#include ../../../crates/other/tests/architecture/layered_architecture.rs:example}}
```

## Microservices Architecture {#microservices}

A microservices architecture structures an application as a collection of small, independent services, built around business capabilities.
Each microservice

- is a self-contained unit that performs a specific business function;
- can be developed, deployed, and scaled independently;
- typically manages its own [database][p-database], promoting data independence;
- and can use different programming languages, frameworks, and databases, allowing teams to choose the best tools for each job.

Microservices communicate with each other through well-defined APIs, promoting loose coupling.

Crates like [`axum`][c-axum]⮳{{hi:axum}}, [`warp`][c-warp]⮳{{hi:warp}} or [`tonic`][c-tonic]⮳{{hi:tonic}} can help build such services.

### Key Technologies {#skip2}

- Communication: Microservices communicate over the network, commonly via REST or [[_grpc | gRPC]].
- Service Discovery: Use a service registry (e.g., `Consul`, `etcd`) to dynamically discover microservice locations.
- API Gateway: Use an API gateway (e.g., `Kong`) to handle routing, [authentication][p-authentication], and other cross-cutting concerns.
- Message Queues: Use message queues (e.g., `RabbitMQ`, `Kafka`) for [asynchronous][p-asynchronous] communication.
- [Containerization][p-containerization]: Use `Docker` and `Kubernetes` for [containerization][p-containerization] and orchestration.
- Logging and Monitoring: Implement central logging and monitoring with `Fluentd`; [`Elasticsearch`][c-elasticsearch]⮳{{hi:Elasticsearch}}, `Logstash`, and `Kibana`; `Graylog`, `Splunk`, `Prometheus` and/or `Grafana`.
- [Error Handling][p-error-handling] and Resilience: Implement retry mechanisms, circuit breakers, and other resilience patterns.
- Configuration Management: externalize configuration and secrets e.g. with `Vault`.
- Authentication/Authorization: Implement security with `JWT` (JSON Web Tokens) or `OAuth 2.0` / `OpenID Connect` (OIDC), using the aforementioned API Gateway or a sidecar proxy (e.g. `Envoy`, `Istio`).
- Tracing: Implement distributed tracing to track requests across microservices, using `OpenTelemetry`, `Jaeger`, or `Zipkin`.

See [[web-programming | Web Programming]], [[web-programming_http-server | Web Programming: HTTP Server]], [[apis | APIs]], and [[amqp | AMQP]].

## Hexagonal Architecture (Ports and Adapters) {#hexagonal-architecture}

The hexagonal architecture, also known as the "Ports and Adapters" architecture, is a software design pattern that emphasizes separating the core business logic of an application from its external dependencies. It enables testability, flexibility, separation of concerns, and dependency inversion.

### Core Concepts {#skip4}

The "Ports and Adapters" architecture divides the system into three main layers:

- Core (Business Logic): The heart of the application, independent of external systems. It contains the domain model.
- Ports: Interfaces that define how the core interacts with external systems.
- Adapters: Implementations of ports to connect the core with specific external systems (e.g., [databases][p-databases], APIs, UIs).

This [architecture][p-architecture] enables the core application to function independently of infrastructure and technology specifics.

### Advantages {#skip5}

- Easy to swap out external systems (e.g., changing a [database][p-database] or UI framework) without affecting core logic.
- Promotes clean and maintainable code.
- Ideal for systems where clear boundaries and independence from external systems are important.

### Use Cases {#skip6}

- Complex systems requiring flexibility.
- Traditional enterprise applications with a focus on maintainability and modularity.
- Systems requiring strict adherence to domain-driven design principles.

See [[cross-platform | Cross Platform]] development.

## Event-Driven Architecture {#event-driven-architecture}

In an Event-driven architecture (EDA), applications communicate by producing and consuming events. Instead of direct, synchronous requests, systems react to changes in state represented by these events.

- Enable loose coupling and scalability by structuring systems around events.
- Promote responsiveness and resilience in distributed systems.

### Core Concepts {#skip8}

- Event Producers and Consumers: Systems or components communicate by emitting and responding to events.
- Event Bus/Queue: Acts as a mediator for events, often using tools like `Kafka`, `RabbitMQ`, or other message brokers.
- [Asynchronous][p-asynchronous] Communication: Components operate independently and interact via events.
- Often implemented with libraries like [`tokio`][c-tokio]⮳{{hi:tokio}} or [`async-std`][c-async_std]⮳{{hi:async-std}}.

See [[asynchronous | Asynchronous]] and [[amqp | AMQP]].

### Advantages {#skip9}

- Highly scalable and responsive, as components can work independently.
- Naturally supports [asynchronous][p-asynchronous] workflows and real-time data processing.
- Encourages microservices architecture by isolating components.

### Use Cases {#skip}

- Systems with high throughput or real-time requirements (e.g., financial systems, IoT platforms, social media apps).
- Large-scale distributed systems where scalability and availability are key.

## Actor Model {#actor-model}

The actor model is a conceptual model of concurrent computation that treats "actors" as the universal primitives of [concurrency][p-concurrency].
Actors encapsulate state, behavior, and a mailbox. [Actors][p-actors] communicate exclusively by sending and receiving [asynchronous][p-asynchronous] messages. They are like independent entities that can receive and process messages.

Libraries like [`actix`][c-actix]⮳{{hi:actix}} support actor-based designs.

See the [[_actors |  Actors]] chapter for more details.

## Command Query Responsibility Segregation (CQRS) {#cqrs}

Command Query Responsibility Segregation (CQRS) is an architectural pattern that separates read and write operations. This separation allows for independent optimization of each operation, leading to improved performance, scalability, and security. It is useful for systems with high read/write demands or complex business rules.

```rust,editable
{{#include ../../../crates/other/tests/architecture/cqrs.rs:example}}
```

## Pipelines/Streams {#pipelines}

Pipeline/stream architecture is a design pattern focused on processing a continuous flow of data through a series of sequential steps. It's particularly relevant in scenarios requiring real-time or near real-time data processing.

See [[data_parallelism | Data Parallelism]], [[data-processing | Data Processing]].

## Event Sourcing {#event-sourcing}

Event sourcing focuses on storing a sequence of events that represent changes to the state of an application, rather than storing the current state itself. In essence, it's about recording "what happened" rather than "what is." It is useful in systems requiring auditability or history reconstruction.

## Related Topics

- [[cloud | Cloud]].
- [[concurrency | Concurrency]].
- [[data-processing | Data Processing]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write / detail every architecture](https://github.com/john-cd/rust_howto/issues/1230)

- [Hydro](https://hydro.run/docs/hydro)

</div>
