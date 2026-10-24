<!-- TODO Reviewed ??-??-2026 -->

# Configuration Management

[![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

Configuration lets an application change its behavior without changing its source code. A program may read settings from built-in defaults, configuration files, environment variables, and command-line arguments. It should define how those sources combine and report invalid settings before the application starts using them.

## Choose configuration sources

- Use built-in defaults for safe values that apply to most runs.
- Use a configuration file for structured settings that people need to edit or keep together. TOML, JSON, YAML, and INI are common choices; choose a format and parser that fit the project.
- Use environment variables for deployment-specific values and settings supplied by the process environment. [`std::env`][c~std::env~docs]↗{{hi:std::env}} provides direct access. A `.env` file can help local development, but should not be committed when it contains secrets.
- Use command-line arguments for options that belong to one invocation, such as a subcommand, a one-off output path, or an explicit override. [`clap`][c~clap~docs]↗{{hi:clap}} can define and parse these arguments.

A layered application might apply sources in this order: defaults, configuration file, environment variables, then command-line overrides. This is a design choice, not a universal rule. Make precedence explicit, especially when a value appears in more than one source.

## Parse and validate settings

Represent related settings with a Rust type, and deserialize files into that type with [`serde`][c~serde~docs]↗{{hi:serde}} and a format-specific crate such as [`toml`][c~toml~docs]↗{{hi:toml}} or [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}. For multiple sources, a crate such as [`config`][c~config~docs]↗{{hi:config}} can collect and merge values; the application still needs to choose precedence and validate the resulting settings.

Check required values, ranges, and relationships between fields before starting work. Return an actionable error that identifies the setting and source when possible. Avoid silently replacing malformed or missing values with defaults unless that fallback is intentional.

Keep secrets out of source control. Restrict access to configuration files that contain credentials, avoid printing secret values in diagnostics, and use the deployment platform's secret-management facility where available. Environment variables can be a useful delivery mechanism, but they do not replace careful secret handling.

## Configuration examples

{{#include configuration.incl.md}}

## Environment variables

{{#include environment_variables.incl.md}}

## Related topics

### Configuration file formats

- [[toml | TOML]] and [`toml`][c~toml~docs]↗{{hi:toml}}
- [[json | JSON]] and [`serde_json`][c~serde_json~docs]↗{{hi:serde_json}}
- [[yaml | YAML]] and [`serde_yml`][c~serde_yml~docs]↗{{hi:serde_yml}}
- [[ini | INI]] and [`ini`][c~ini~docs]↗{{hi:ini}}

### Command-line arguments

- [`clap`][c~clap~docs]↗{{hi:clap}} for command-line interfaces and per-run overrides

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1195)
</div>
