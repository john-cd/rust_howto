# Common Design Patterns in Rust

## Repository Pattern

- Encapsulates data access logic, separating it from business logic.
- Often combined with ORM tools like `diesel` or `sea-orm`.

## State Machine Pattern

- Models systems as a collection of states and transitions.
- Useful for handling workflows or processes with clear states.

## Dependency Injection

- Promotes modularity by injecting dependencies at runtime or compile time.
- Achievable using crates like `shaku`.
- Rarer in Rust than in other languages like Java.

## Related Topics

- Configuration Management: Loading and managing settings with `config` or `dotenv`.
- Logging and Monitoring: Structured logging with `tracing` or `log`.
- Error Handling.
- Testing and Quality Assurance.
- Performance Optimization.
- Security.
- Deployment and Operations.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
link to other chapters
</div>
