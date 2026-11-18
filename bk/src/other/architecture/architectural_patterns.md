<!-- TODO Reviewed ??-??-2026. -->

# Common Design Patterns in Rust

{{#include architectural_patterns.incl.md}}

## Repository Pattern {#repository-pattern}

- Encapsulates data access logic, separating it from business logic.
- Makes it easier to test the business logic by using mock repositories.
- Simplifies code maintenance by centralizing data access logic.
- Easily switches between different data storage implementations.
- Often combined with ORM tools like [`diesel`][c~diesel~docs]↗{{hi:diesel}} or [`sea-orm`][c~sea-orm~docs]↗{{hi:sea-orm}}. See [Query Builders and ORMs](../../categories/database/query_builders_orms.md).

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

Assemble dependencies where the application is configured. This example uses
`shaku` to inject a `Greeter` into an `EnglishConversation`. The module builder
also sets the conversation parameters. A plugin registry serves a different purpose:
discovering implementations registered by different parts of the application.
See [Plugin Registration](index.md#plugin-registration).

```rust,editable
{{#include ../../../crates/other/examples/architecture/di.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Configuration](../../categories/config/index.md): load and manage settings with [`config`][c~config~docs]↗{{hi:config}} or [`dotenv`][c~dotenv~docs]↗{{hi:dotenv}}.
- Logging and monitoring: use [`tracing`][c~tracing~docs]↗{{hi:tracing}} or [`log`][c~log~docs]↗{{hi:log}}; see [Development Tools: Debugging](../../categories/development-tools_debugging/index.md).
- [Recoverable Error Handling](../../standard-library/recoverable_error_handling.md).
- [Testing](../../categories/development-tools_testing/index.md) and quality assurance.
- [Profiling and Performance](../../categories/development-tools/profiling.md).
- Plan deployment and operations around the system's reliability and support requirements.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
