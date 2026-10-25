<!-- TODO Reviewed ??-??-2026 -->

Architecture is the set of important choices about a system's structure, boundaries,
runtime behavior, and the qualities it must provide. It is not a one-time diagramming
phase or a plan that must be completed before coding. Revisit the decisions as the team
learns from prototypes, implementation, and operation. Scale the process to the risks:
write down the reasoning that will help people make or change consequential decisions,
and avoid maintaining detail that nobody uses.

The activities below are a repeatable loop. They can happen in order for a new system,
or be revisited independently when an existing system changes.

## Define objectives and requirements {#define-objectives-and-requirements}

Start with the problem the system must solve and the people who care about the result.
Agree on its purpose, scope, users, external systems, constraints, and what is out of
scope. Include business or research goals, regulatory and organizational constraints,
budget, delivery dates, deployment environments, and existing systems that must remain
compatible.

Separate functional requirements (what the system does) from quality requirements
(how well it must work). Pick a small number of the most important quality goals, then
make them verifiable with scenarios. For example: when a user imports a large file on a
supported laptop, the application completes within an agreed time and memory budget.
Specify the workload, environment, response, and measure so the team can test whether
the goal was met. If a stakeholder has not supplied a threshold, record the assumption
and resolve it before the decision depends on it.

## Map system context and boundaries {#system-context-and-boundaries}

Show who uses the system, which external systems it calls, and the data or control that
crosses each boundary. A system context diagram is a useful first view. Add a container
view when it helps explain the independently running applications and data stores inside
the system. In the [C4 model][c4~model~diagrams]↗, a container is an application or data
store, not necessarily an operating-system or Docker container. Use only the views that
help a reader answer a real question; a diagram is optional when a concise description
is clearer.

For Rust systems, also sketch the important package, crate, and module boundaries. A
Cargo workspace is a convenient way to organize packages, but it does not by itself
define the right architecture. Identify the public interfaces, dependency direction,
data ownership, and which component is responsible for each rule. Avoid circular crate
dependencies and boundaries that only rename folders without reducing coupling.

## Design components and interfaces {#design-components-and-interfaces}

Partition the system around cohesive responsibilities and stable interfaces. Describe
the main components, their inputs and outputs, how they communicate, and where state is
owned. Consider runtime and data-flow views for behavior that crosses components, such
as a request path, background job, event stream, or failure recovery sequence.

Choose a structure that fits the constraints: a library, a modular application, a
client/server system, or a set of independently deployed services. Start with the
simplest design that meets the requirements and leaves credible room for expected
change. Do not introduce a service boundary, abstraction, or dependency-injection layer
without a concrete need for independent deployment, ownership, substitution, or
testing.

In Rust, make ownership and error behavior part of the interface design. Decide which
crate owns mutable state, what callers may access, how errors cross API boundaries, and
whether work is synchronous or asynchronous. Use Rust's type system to encode useful
invariants, while still designing explicit validation, authorization, resource limits,
and failure handling.

## Evaluate architecture and technology options {#evaluate-options}

List plausible alternatives before selecting frameworks, storage, communication
protocols, or deployment models. Compare them against the agreed requirements and
constraints, including operational burden, team experience, ecosystem maturity,
security, portability, performance, and the cost of changing course. A decision matrix
can make priorities visible, but scores should explain the reasoning rather than imply
more precision than the evidence supports.

Use [[common_architectures]] and
[[architectural_patterns]] as a vocabulary for evaluating
options, not as recipes to apply automatically. For each important option, state what it
improves, what it makes harder, and which quality goals or constraints drive the choice.
Delay a decision when evidence is missing and a small experiment can reduce uncertainty.

## Record decisions and trade-offs {#record-decisions}

Write an Architecture Decision Record (ADR) for decisions that affect system structure,
important quality goals, dependencies, interfaces, or construction techniques. A short
ADR should let a future maintainer understand why the team chose an approach and what
it costs. Include:

- A short title and current status, such as proposed, accepted, deprecated, or
  superseded.
- The context, including the problem, constraints, assumptions, and decision criteria.
- The decision and the main alternatives considered.
- The expected consequences, including disadvantages, risks, and follow-up work.
- Links to related requirements, decisions, code, or validation evidence.

Store ADRs with the project so changes can be reviewed alongside code. Update the
status when the decision changes and link to its replacement. Record the important
decisions; do not turn every local implementation choice into architecture paperwork.
The [arc42 architecture decision guidance][arc42~decisions]↗ and its
[documentation template][arc42~template]↗ provide examples of lightweight records and
other useful architecture views.

## Validate the design and its risks {#validate-the-design}

Check the design against the highest-priority scenarios before relying on it. Use the
cheapest useful evidence: a small prototype, an integration test, a load test, a
benchmark, a deployment rehearsal, a threat review, or a failure-injection exercise.
State the environment and the result, and revisit the decision if the evidence misses
the target. The [arc42 quality requirements guide][arc42~quality~requirements]↗ shows
how to express measurable usage and change scenarios.

### Security and trust boundaries {#security-and-trust}

Identify untrusted inputs, sensitive data, credentials, privileged operations, and
external dependencies. Decide where validation and authorization happen, how secrets
are supplied, what data is retained, and how components behave when a dependency is
unavailable. Use established cryptographic and authentication libraries rather than
designing custom primitives. Include security assumptions in the relevant tests and
reviews.

### Errors and failure semantics {#error-and-failure-semantics}

For each interface, decide which failures are expected, which are retryable, and what
information callers need to recover or report the problem. In Rust, use `Result` and
domain-specific errors at reusable library boundaries; add application context where
errors reach a user or operator. Define timeouts, cancellation, backpressure, and
partial-success behavior for asynchronous or distributed operations.

### Performance and resource limits {#performance-and-resource-limits}

Measure the workloads named in the quality scenarios. Set explicit budgets for latency,
throughput, memory, storage, or startup time where they matter. Profile before
optimizing, and account for contention, allocation, concurrency limits, and the cost of
external calls. Keep a benchmark or production metric only when it helps the team
notice a regression or make a decision.

## Plan deployment and operations {#deployment-and-operations}

Describe how the system is built, configured, released, deployed, observed, and
recovered. Identify runtime dependencies, configuration and secret sources, data
migrations, health signals, backup and restore expectations, and ownership for
operational response. Decide how version compatibility, rollbacks, and partial outages
will be handled. Use the project's [[building | build]] and deployment guidance and the
[CI/CD chapter][p~github-actions].

For Rust applications, document target platforms and required system libraries, the
Cargo features used for each build, and how binaries or libraries are packaged. Check
that the architecture can be built and exercised in the environments it promises to
support.

## Evolve the architecture with the implementation {#evolve-architecture}

Review architectural assumptions when requirements, dependencies, operating
conditions, or measured behavior change. Compare the implemented system with the
documented boundaries, update diagrams and ADR status when they are stale, and record
new risks or deferred work. Add architecture checks to code review or CI only when they
protect a boundary that the team has agreed to preserve.

Keep the working architecture set small and findable. It often includes the problem
scope and quality scenarios, a context or deployment view, the important component and
data flows, a few key ADRs, and open risks with their validation plan. The right set is
the one that helps this team explain, test, operate, and safely change its system.

## Rust crates for common concerns {#crates-for-common-concerns}

Treat these as starting points to evaluate against your requirements, not as defaults.

| Concern                    | Crates and tools                                                                                                                          |
| -------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Web services               | [`axum`][c~axum~docs]↗{{hi:axum}}, [`rocket`][c~rocket~docs]↗{{hi:rocket}}                                                                |
| Command-line tools         | [`clap`][c~clap~docs]↗{{hi:clap}}                                                                                                         |
| Configuration              | [`config`][c~config~docs]↗{{hi:config}}, [`dotenvy`][c~dotenvy~docs]↗{{hi:dotenvy}}                                                       |
| Databases                  | [`sqlx`][c~sqlx~docs]↗{{hi:sqlx}}, [`diesel`][c~diesel~docs]↗{{hi:diesel}}                                                                |
| Logging and tracing        | [`tracing`][c~tracing~docs]↗{{hi:tracing}}, [`log`][c~log~docs]↗{{hi:log}}                                                                |
| Errors                     | [`thiserror`][c~thiserror~docs]↗{{hi:thiserror}} for library error types, [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} for application context |
| Cryptography               | The [RustCrypto](https://github.com/RustCrypto)↗ crates (see [[cryptography                                                               | Cryptography]]), [`ring`][c~ring~docs]↗{{hi:ring}} |
| Mocking                    | [`mockall`][c~mockall~docs]↗{{hi:mockall}}                                                                                                |
| Benchmarking and profiling | [`criterion`][c~criterion~docs]↗{{hi:criterion}}, `perf`, `valgrind`                                                                      |
| Data parallelism           | [`rayon`][c~rayon~docs]↗{{hi:rayon}}                                                                                                      |

An example `src/` layout for a modular application (adapt it; do not copy it blindly):

```txt
src/
├── main.rs       (entry point)
├── lib.rs        (library code)
├── config/       (configuration loading and parsing)
├── services/     (business logic)
├── models/       (data models)
├── api/          (REST or other interfaces)
└── utils/        (helpers)
tests/            (integration tests)
```

## Related topics {#related-topics .skip}

[Building a SaaS with Rust and Next.js][blog~building-a-saas-with-rust-and-next-js]↗{{hi:SaaS}}
