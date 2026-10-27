<!-- TODO Reviewed ??-??-2026 -->

# Others

{{#include other.incl.md}}

## Search for Rust APIs {#roogle}

[`Roogle`][c~roogle~website]↗ is an API search site that lets you search Rust function signatures. Treat search results as a discovery aid: confirm the item, version, and behavior in the crate's current documentation before using it.

## Reduce a Rust Binary's Size {#binary-minimizer}

Start by measuring a release build for the target you intend to ship. Binary size depends on the target, enabled features, dependency graph, linker, and profile settings, so compare changes under the same conditions.

```sh
cargo build --release
```

Inspect the release profile in `Cargo.toml` and change one option at a time. Link-time optimization and symbol stripping can reduce output size, but may increase build time or remove information needed for debugging. Use [`cargo-bloat`][c~cargo-bloat~docs]↗ to identify large contributors before replacing dependencies or changing application code. See the [Cargo release profile reference][book~cargo~release-profile]↗ and the [compilation performance guide][p~performance].

## Generate Rust Code {#code-generators}

Code generation is useful when source code is repetitive or derived from a stable input such as a schema, protocol definition, or asset list. Choose the mechanism based on when the generated code is needed:

- Use a procedural macro when Rust syntax should be transformed or generated during compilation.
- Use a `build.rs` script or a dedicated generator when output depends on external files or tools.
- Use a project template when generating a starting project for a human to maintain.

Keep inputs and generation steps reproducible: document the command, pin generator versions when practical, and make generated output deterministic. Avoid hand-editing generated files unless the tool explicitly supports that workflow. Review generated changes and include them in version control when downstream users need to build without the generator.

See [procedural macros][p~macros], [Cargo build scripts][book~cargo~build-script]↗, and [Cargo project tools][p~building] for related approaches.

## Scaffold a New Project {#scaffold-a-new-project}

Use a project generator when a starter template can provide useful structure or repeatable boilerplate. Treat the template as source code: review the generated files, dependencies, and configuration before building on them.

| Tool                                                            | Use it when                                                                                                                                            |
| --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [`cargo-generate`][c~cargo-generate~repo]↗{{hi:cargo-generate}} | Start from a Git-hosted template that you control or trust; `cargo generate --git <template-url>` creates the project from it.                         |
| [`kickstart`][c~kickstart~repo]↗{{hi:kickstart}}                | Use a Tera-based template with prompted variables and conditional files when the generated project needs customization.                                |
| [Gerust][gerust~website]↗{{hi:Gerust}}                          | Create an opinionated Rust backend workspace based on Axum and SQLx when that architecture fits; `gerust <project-name>` generates the starting files. |
| [Loco][c~loco-rs~website]↗{{hi:Loco}}                           | Start a batteries-included web application with `loco new`; use `cargo loco generate` (debug builds only) for application components.                  |
| [Rise][rise~repo]↗{{hi:Rise}}                                   | Choose a community template for a web app, CLI, game, or another Rust project.                                                                         |

As of this review, [`Protypo`][protypo~repo]↗{{hi:Protypo}} is a cloud-native headless CMS with one repository commit and no README or release; consider it only for CMS evaluation, not general project scaffolding.

## Related Topics {#related-topics .skip}

- [Rust compilation and linking][p~performance].
- [Cargo subcommands and task runners][p~building].
- [Cargo build-time utilities][p~build-utils].
- [Cargo configuration][p~configuration].
  {{#include refs.incl.md}}
  {{#include ../../../refs/link-refs.md}}
