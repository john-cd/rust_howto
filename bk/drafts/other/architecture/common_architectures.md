## Common Architectural Patterns in Rust

## Layered Architecture

- Separation into layers like Presentation, Business Logic, and Data Access.
- Encourages modularity and testability.

## Microservices Architecture

- Breaks applications into smaller, independent services.
- Scalable and resilient, commonly paired with REST or gRPC.
- Crates like `warp` or `tonic` can help build such services.

## Hexagonal Architecture (Ports and Adapters)

### Goals

- Decouple business logic from external systems.
- Enable testability and flexibility by abstracting dependencies through interfaces.

### Core Concepts

- Separation of Concerns: Divides the system into three main layers:
  - Core (Business Logic): The heart of the application, independent of external systems.
  - Ports: Interfaces that define how the core interacts with external systems.
  - Adapters: Implementations of ports to connect the core with specific external systems (e.g., databases, APIs, UIs).

### Advantages

- Easy to swap out external systems (e.g., changing a database or UI framework) without affecting core logic.
- Promotes clean and maintainable code.
- Ideal for systems where clear boundaries and independence from external systems are important.

### Use Cases

- Complex systems requiring flexibility.
- Traditional enterprise applications with a focus on maintainability and modularity.
- Systems requiring strict adherence to domain-driven design principles.

## Event-Driven Architecture

### Goals

- Enable loose coupling and scalability by structuring systems around events.
- Promote responsiveness and resilience in distributed systems.

### Core Concepts

- Event Producers and Consumers: Systems or components communicate by emitting and responding to events.
- Event Bus/Queue: Acts as a mediator for events, often using tools like Kafka, RabbitMQ, or other message brokers.
- Asynchronous Communication: Components operate independently and interact via events.
- Often implemented with libraries like `tokio` or `async-std`.

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

## Command Query Responsibility Segregation (CQRS)

- Separates read and write operations into distinct models.
- Useful for systems with high read/write demands or complex business rules.

## Pipelines/Streams

- Processes data sequentially through a series of stages.
- Common in data-intensive applications, implemented with crates like `futures` or `rayon`.

## Event Sourcing

- Stores the state of a system as a sequence of events.
- Useful in systems requiring auditability or history reconstruction.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / detail every architecture
link to other chapters
</div>
