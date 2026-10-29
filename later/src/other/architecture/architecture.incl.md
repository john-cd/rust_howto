<!-- TODO Reviewed ??-??-2026 -->

This section connects architectural choices to the Rust code and systems that use
them. In Rust, architecture includes deployable processes and external interfaces as
well as package, crate, and module boundaries, public APIs, dependency direction, and
ownership of state. Organize around responsibilities and changes the system must
support; a folder layout alone does not define an architecture.

## Choose a starting point

| If you want to...                                                                         | Start here                                                        |
| ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Clarify goals, boundaries, quality requirements, alternatives, and decisions              | [Software Architecture Process](software_architecture_process.md) |
| Compare system-level structures such as layered, event-driven, or microservices           | [Common Architectures](common_architectures.md)                   |
| Organize component responsibilities with repository, state-machine, or injection patterns | [Architectural Patterns](architectural_patterns.md)               |

Read the process first when beginning a system or reviewing a major change. Use the
architecture styles to compare broad structures, then choose component patterns only
where they help meet the requirements. The process chapter also covers measurable
quality scenarios, Architecture Decision Records, validation, and operational concerns.

## Describe the architecture

Use a diagram or written view when it makes a boundary or decision easier to discuss.
The [C4 model diagrams][c4~model~diagrams]↗ can show a system's context and its
applications and data stores at different levels of detail. The
[arc42 template][arc42~template]↗ offers a structure for recording context and
requirements, decisions, risks, and other architecture views. Neither resource
requires a team to document every view; choose the detail that answers a stakeholder
or engineering question.

## Related topics

- [Cloud](../cloud/index.md) and deployment environments.
- Concurrency<!-- Cross-book topic: bk/drafts/categories/concurrency/index.md; not included in this preview. --> and asynchronous behavior.
- [Data Processing](../data-processing/index.md) and storage choices.
- Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. --> and service interfaces.
- [Written in Rust](../written-in-rust/index.md) for examples of complete Rust applications.
