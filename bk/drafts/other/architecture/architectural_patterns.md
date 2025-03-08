# Common Design Patterns in Rust

{{#include architectural_patterns.incl.md}}

## Repository Pattern

- Encapsulates data access logic, separating it from business logic.
- Makes it easier to test the business logic by using mock repositories.
- Simplifies code maintenance by centralizing data access logic.
- Allows you to easily switch between different data storage implementations.
- Often combined with ORM tools like `diesel` or `sea-orm`. See [[query_builders_orms | Query Builders & ORMs]].

```rust,editable
{{#include ../../../crates/other/tests/architecture/repository.rs:example}}
```

## State Machine Pattern

- Models systems as a collection of states and transitions.
- Useful for handling workflows or processes with clear states.

```rust,editable
{{#include ../../../crates/other/tests/architecture/state_machine.rs:example}}
```

## Dependency Injection

- Promotes modularity by injecting dependencies at runtime or compile time.
- Achievable using crates like `shaku`.
- Rarer in Rust than in other languages like Java.

```rust,editable
{{#include ../../../crates/other/tests/architecture/di.rs:example}}
```

## Related Topics

- [[config | Configuration]] Management: Loading and managing settings with `config` or `dotenv`.
- Logging and Monitoring: Structured logging with `tracing` or `log`. See [[development-tools_debugging | Development Tools: Debugging]].
- [[error_handling | Error Handling]].
- [[testing | Testing]] and Quality Assurance.
- [[performance | Performance]] Optimization.
- Deployment and Operations. See [[devops | Devops]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write. create missing examples
cover `inventory` crate for DI?
</div>
