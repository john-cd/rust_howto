<!-- TODO Reviewed ??-??-2026 -->

# Watching for Changes

{{#include watching_for_changes.incl.md}}

| Task                                       | Tool                                | Why                                                                 |
| ------------------------------------------ | ----------------------------------- | ------------------------------------------------------------------- |
| Rerun common Cargo checks in a terminal UI | [Bacon][c~bacon~repo]               | Cargo-aware tasks such as `check`, `test`, and `clippy`.            |
| Watch files and run any command            | [Watchexec][c~watchexec~repo]       | General command runner for custom build, test, or script workflows. |
| Keep an existing watcher script running    | [`cargo-watch`][c~cargo-watch~repo] | Archived; retain only for legacy workflows.                         |
| Read compiler diagnostics with less noise  | [`cargo-limit`][c~cargo-limit~repo] | Filters and prioritizes Cargo output; it does not watch files.      |

Many [IDEs][p~ides] already watch files and can run Cargo tasks automatically. Check the editor's build and test settings before adding another background process.

## Use Bacon for Cargo tasks {#bacon}

[Bacon][c~bacon~repo] is a terminal UI that watches a Cargo workspace and reruns a selected task. Install it with `cargo install --locked bacon`, then launch the default check task with `bacon`; choose `bacon test` to rerun tests or `bacon clippy` to run Clippy. Bacon is useful when you want a persistent view of diagnostics while editing.

## Use Watchexec for arbitrary commands {#watchexec}

[Watchexec][c~watchexec~repo] watches files and restarts a command when they change. Install the CLI with `cargo install --locked watchexec-cli`. For example, `watchexec --restart --watch src --watch Cargo.toml -- cargo test` reruns a Cargo test command when Rust source or the manifest changes. Add watched paths or filters that match the inputs used by your own build script.

Choose Watchexec for commands that are not Cargo tasks or for a custom sequence of tools. Limit watched paths so generated files and build outputs do not trigger unnecessary rebuild loops.

## Keep `cargo-watch` for legacy workflows {#cargo-watch}

The [`cargo-watch`][c~cargo-watch~repo] repository is archived and read-only; its maintainers recommend Bacon or Watchexec for new setups. Existing scripts can still use commands such as `cargo watch -x test`, but avoid relying on it for new workflows because bug fixes and compatibility updates are no longer expected.

## `cargo limit` {#cargo-limit}

[![cargo-limit][c~cargo-limit~docs~badge]][c~cargo-limit~docs]{{hi:cargo-limit}}
[![cargo-limit~crates.io][c~cargo-limit~crates.io~badge]][c~cargo-limit~crates.io]
[![cargo-limit~repo][c~cargo-limit~repo~badge]][c~cargo-limit~repo]
[![cargo-limit~lib.rs][c~cargo-limit~lib.rs~badge]][c~cargo-limit~lib.rs]

[cargo-limit][c~cargo-limit~crates.io]↗ is [Cargo][p~cargo] with less noise: warnings are skipped until errors are fixed, Neovim integration, etc.

- errors have highest priority.
- they never appear in the middle of warnings.
- warnings are skipped by default until errors are fixed.
- external path dependencies' warnings are skipped by default.
- all messages come in reversed order by default to avoid extra scrolling.
- messages are grouped by filenames.
- number of messages can be limited.
- after encountering first error the rest of build time is limited by default.
- files can be automatically opened in your text editor on affected lines.

Use `cargo-limit` with Bacon, Watchexec, or a manual Cargo command when its output filtering fits the workflow.

## Related Topics {#related-topics .skip}

For build and compiler settings, see [Reduce Compilation Duration](../development-tools/compilation/reduce_compilation_duration.md). For Cargo task runners and test execution, see [Performance](performance.md) and [Test Runners](../development-tools_testing/test_runners.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
