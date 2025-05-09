# Configuration Management

[![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Configuration management {{hi:Configuration management}} is the practice of handling application settings. Key goals include:

- Separation of configuration from code, allowing for configuration changes without recompiling the application.
- Support for configuration specific to each environment (development, testing, and production).
- Secure handling of sensitive data e.g. secrets like API keys and database credentials.

## Common Approaches

- [[environment_variables | Environment Variables]] are suitable for simple configurations. The `std::env` module provides access to environment variables.
- [[configuration | Configuration]] files like [[toml | TOML]], [[ini | INI]], [[yaml | YAML]], [[json | JSON]], or RON provide structured and organized configuration. Popular Rust crates for parsing these formats include: [`toml`][c-toml]⮳{{hi:toml}}, [`serde_json`][c-serde_json]⮳{{hi:serde_json}}, `ron`, [`serde`][c-serde]⮳{{hi:serde}}.

FIXME Configuration crates like [`config-rs`][c-config]⮳{{hi:config-rs}}, a powerful and flexible crate for layered configuration. It supports merging configurations from various sources, including files, environment variables, and in-memory data.
Excellent for 12-factor applications.

Command-Line Arguments:
Using crates like `clap` to parse command-line arguments.
Suitable for passing simple configuration options.

## Best Practices

Use a layered configuration approach: Combine multiple sources (e.g., default files, environment variables, command-line arguments) to provide flexibility and override behavior.
Validate configuration: Ensure that configuration values are valid and within expected ranges.
Handle errors gracefully: Provide informative error messages when configuration is invalid or missing.
Secure sensitive data: Avoid storing secrets in version control. Use environment variables or dedicated secret management tools.
Document configuration: Provide clear documentation of all configuration options.

## Configuration Management

{{#include configuration.incl.md}}

## Environment Variables

{{#include environment_variables.incl.md}}

## Related Topics

### Configuration File Formats

- TOML: [`toml`][c-toml]⮳{{hi:toml}}
- YAML: [`serde_yml`][c-serde_yml]⮳{{hi:serde_yml}}, [`yaml-rust`][c-yaml_rust]⮳{{hi:yaml-rust}}
- JSON: [`serde_json`][c-serde_json]⮳{{hi:serde_json}}
- INI: [`ini`][c-ini]⮳{{hi:ini}}

### Configuration Management

- [`config`][c-config]⮳{{hi:config}}: A popular crate that supports multiple formats and merging configurations from different sources (files, environment variables, etc.).
[`serde`][c-serde]⮳{{hi:serde}}: (Not a config crate itself, but essential for serializing and deserializing configuration - data in most cases).
- Environment Variables: `std::env` (for accessing environment variables directly).

## Related Topics {#skip}

- Command-Line Arguments (often used in conjunction with [`config`][c-config]⮳{{hi:config}}): [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}}, [`argh`][c-argh]⮳{{hi:argh}}

Configuration Validation: (Often done manually or with custom functions, but schemars can be used to generate JSON schema for validation).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1195)
</div>
