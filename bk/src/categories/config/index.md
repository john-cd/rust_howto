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

TOML: `toml`
YAML: `serde_yaml`, `yaml-rust`
JSON: `serde_json`
INI: `ini`

Configuration Management:

`config`: A popular crate that supports multiple formats and merging configurations from different sources (files, environment variables, etc.).
`serde`: (Not a config crate itself, but essential for serializing and deserializing configuration data in most cases).
Environment Variables: `std::env` (for accessing environment variables directly).

Command-Line Arguments (Often used in conjunction with `config`): `clap`, `structopt`, `argh`

Configuration Validation: (Often done manually or with custom functions, but schemars can be used to generate JSON schema for validation).

</div>
