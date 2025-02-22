# Configuration

[![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Facilitate configuration management{{hi:Configuration management}} for applications.

## Configuration Management

{{#include configuration.incl.md}}

## Environment Variables

{{#include environment_variables.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 link:

Configuration File Formats:

TOML: [`toml`][c-toml]⮳{{hi:toml}}
YAML: [`serde_yaml`][c-serde_yaml]⮳{{hi:serde_yaml}}, [`yaml-rust`][c-yaml_rust]⮳{{hi:yaml-rust}}
JSON: [`serde_json`][c-serde_json]⮳{{hi:serde_json}}
INI: [`ini`][c-ini]⮳{{hi:ini}}

Configuration Management:

[`config`][c-config]⮳{{hi:config}}: A popular crate that supports multiple formats and merging configurations from different sources (files, environment variables, etc.).
[`serde`][c-serde]⮳{{hi:serde}}: (Not a config crate itself, but essential for serializing and deserializing configuration data in most cases).
Environment Variables: `std::env` (for accessing environment variables directly).

Command-Line Arguments (Often used in conjunction with [`config`][c-config]⮳{{hi:config}}): [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}}, [`argh`][c-argh]⮳{{hi:argh}}

Configuration Validation: (Often done manually or with custom functions, but schemars can be used to generate JSON schema for validation).

</div>
