## Common Architectural Patterns in Rust

## Layered Architecture

- Separation into layers like Presentation, Business Logic, and Data Access.
- Encourages modularity and testability.

```rust,editable
{{#include ../../../crates/other/tests/architecture/layered_architecture.rs:example}}
```

## Microservices Architecture

- Breaks applications into smaller, independent services.
- Scalable and resilient, commonly paired with REST or [[_grpc | gRPC]].
- Crates like `axum`, `warp` or `tonic` can help build such services.

### Key Concepts

- Communication: Microservices communicate over the network.
- Independent Deployment and Scaling: Each microservice can be deployed and scaled independently.
- Technology Agnosticism: Microservices could be written in different languages.
- Service Discovery: Use a service registry (e.g., `Consul`, `etcd`) to dynamically discover microservice locations.
- API Gateway: Use an API gateway (e.g., `Kong`) to handle routing, authentication, and other cross-cutting concerns.
- Message Queues: Use message queues (e.g., `RabbitMQ`, `Kafka`) for asynchronous communication.
- Containerization: Use `Docker` and `Kubernetes` for containerization and orchestration.
- Logging and Monitoring: Implement central logging and monitoring with `Fluentd`; `Elasticsearch`, `Logstash`, and `Kibana`; `Graylog`, `Splunk`, `Prometheus` and/or `Grafana`.
- Error Handling and Resilience: Implement retry mechanisms, circuit breakers, and other resilience patterns.
- Configuration Management: externalize configuration and secrets e.g. with `Vault`.
- Authentication/Authorization: Implement security with `JWT` (JSON Web Tokens) or `OAuth 2.0` / `OpenID Connect` (OIDC), using the aforementioned API Gateway or a sidecar proxy (e.g. `Envoy`, `Istio`).
- Tracing: Implement distributed tracing to track requests across microservices, using `OpenTelemetry`, `Jaeger`, or `Zipkin`.

See [[web-programming | Web Programming]], [[web-programming_http-server | Web Programming: HTTP Server]], [[apis | APIs]], [[amqp | AMQP]].

## Hexagonal Architecture (Ports and Adapters)

- Decouple business logic from external systems.
- Enable testability and flexibility by abstracting dependencies through interfaces (ports).

### Core Concepts

It divides the system into three main layers:

- Core (Business Logic): The heart of the application, independent of external systems.
- Ports: Interfaces that define how the core interacts with external systems.
- Adapters: Implementations of ports to connect the core with specific external systems (e.g., databases, APIs, UIs).

This architecture enables the core application to function independently of infrastructure and technology specifics.

### Advantages

- Easy to swap out external systems (e.g., changing a database or UI framework) without affecting core logic.
- Promotes clean and maintainable code.
- Ideal for systems where clear boundaries and independence from external systems are important.

### Use Cases

- Complex systems requiring flexibility.
- Traditional enterprise applications with a focus on maintainability and modularity.
- Systems requiring strict adherence to domain-driven design principles.

See [[cross-platform | Cross Platform]] development.

## Event-Driven Architecture

- Enable loose coupling and scalability by structuring systems around events.
- Promote responsiveness and resilience in distributed systems.

### Core Concepts

- Event Producers and Consumers: Systems or components communicate by emitting and responding to events.
- Event Bus/Queue: Acts as a mediator for events, often using tools like `Kafka`, `RabbitMQ`, or other message brokers.
- Asynchronous Communication: Components operate independently and interact via events.
- Often implemented with libraries like `tokio` or `async-std`.

See [[asynchronous | Asynchronous]] and [[amqp | AMQP]].

### Advantages

- Highly scalable and responsive, as components can work independently.
- Naturally supports asynchronous workflows and real-time data processing.
- Encourages microservices architecture by isolating components.

### Use Cases

- Systems with high throughput or real-time requirements (e.g., financial systems, IoT platforms, social media apps).
- Large-scale distributed systems where scalability and availability are key.

## Actor Model

- Emphasizes concurrency with isolated actors that communicate via messages.
- Libraries like `actix` support actor-based designs.

See [[_actors |  Actors]].

## Command Query Responsibility Segregation (CQRS)

- Separates read and write operations into distinct models.
- Useful for systems with high read/write demands or complex business rules.

## Pipelines/Streams

- Processes data sequentially through a series of stages.
- Common in data-intensive applications.

See [[data_parallelism | Data Parallelism]], [[data-processing | Data Processing]].

## Event Sourcing

- Stores the state of a system as a sequence of events.
- Useful in systems requiring auditability or history reconstruction.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / detail every architecture
link to other chapters
</div>
