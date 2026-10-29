<!-- TODO Reviewed ??-??-2026 -->

# Common Design Patterns in Rust

{{#include architectural_patterns.incl.md}}

## Repository Pattern {#repository-pattern}

- Encapsulates data access logic, separating it from business logic.
- Makes it easier to test the business logic by using mock repositories.
- Simplifies code maintenance by centralizing data access logic.
- Easily switches between different data storage implementations.
- Often combined with ORM tools like [`diesel`][c~diesel~docs]↗{{hi:diesel}} or [`sea-orm`][c~sea-orm~docs]↗{{hi:sea-orm}}. See Query Builders & ORMs<!-- Cross-book topic: bk/drafts/categories/database/query_builders_orms.md; not included in this preview. -->.

```rust,editable
{{#include ../../../crates/other/examples/architecture/repository.rs:example}}
```

## State Machine Pattern {#state-machine-pattern}

- Models systems as a collection of states and transitions.
- Useful for handling workflows or processes with clear states.

```rust,editable
{{#include ../../../crates/other/examples/architecture/state_machine.rs:example}}
```

## Dependency Injection {#dependency-injection}

- Promotes modularity by injecting dependencies at runtime or compile time.
- Achievable using crates like [`shaku`][c~shaku~docs]↗{{hi:shaku}}.
- Rarer in Rust than in other languages like Java.

```rust,editable
{{#include ../../../crates/other/examples/architecture/di.rs:example}}
```

## Related Topics {#related-topics .skip}

- Configuration<!-- Cross-book topic: bk/drafts/categories/config/index.md; not included in this preview. --> Management: Loading and managing settings with [`config`][c~config~docs]↗{{hi:config}} or [`dotenv`][c~dotenv~docs]↗{{hi:dotenv}}.
- Logging and Monitoring: Structured logging with [`tracing`][c~tracing~docs]↗{{hi:tracing}} or [`log`][c~log~docs]↗{{hi:log}}. See Development Tools: Debugging<!-- Cross-book topic: bk/drafts/categories/development-tools_debugging/index.md; not included in this preview. -->.
- Error Handling<!-- Cross-book topic: bk/drafts/categories/rust-patterns/error_handling/error_handling.md; not included in this preview. -->.
- Testing<!-- Cross-book topic: bk/drafts/categories/development-tools_testing/testing.md; not included in this preview. --> and Quality Assurance.
- Performance<!-- Cross-book topic: bk/drafts/categories/development-tools_cargo-plugins/performance.md; not included in this preview. --> Optimization.
- Deployment and Operations. See [Devops](../devops/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write. create missing examples. cover [`inventory`][c~inventory~docs]↗{{hi:inventory}} crate for DI?
</div>
