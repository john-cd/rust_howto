<!-- TODO Reviewed ??-??-2026 -->

# VS Code Configuration

This repository contains several independent Cargo workspaces rather than one workspace at the repository root. Configure `rust-analyzer` to load the workspaces you use, and keep the list aligned with the checkouts that are present on your machine.

Install the [`rust-analyzer` extension][rust-analyzer~plugin]↗. For Rust and repository toolchain setup, see [Develop with VS Code][ex~contributing~using-vs-code].

## Choose a settings file

Use VS Code's **Preferences: Open User Settings (JSON)** command to edit settings that apply to all your projects. To share settings for this repository, use **Preferences: Open Workspace Settings (JSON)**; VS Code stores single-folder workspace settings in `.vscode/settings.json` at the repository root. Workspace settings override user settings. See [VS Code settings][vscode~settings~docs]↗ for the available settings scopes and file locations.

## Configure `rust-analyzer` {#rust-analyzer-configuration}

### Link the Cargo workspaces

`rust-analyzer` can discover Cargo projects automatically. Set `rust-analyzer.linkedProjects` when you want to select the projects explicitly. The setting replaces automatic discovery, so keep every desired project in the array or remove the setting to return to automatic discovery. Paths below are relative to the repository root:

```json
{
  "rust-analyzer.linkedProjects": [
    "bk/crates/Cargo.toml",
    "later/crates/Cargo.toml",
    "mdbook-utils/Cargo.toml",
    "playground/Cargo.toml",
    "publish/Cargo.toml",
    "tools/Cargo.toml",
    "xmpl/Cargo.toml"
  ],
  "search.exclude": {
    "**/target/**": true,
    "**/build/**": true
  }
}
```

Remove an entry if that project is not checked out locally. To reduce startup work, list only the workspace you are changing and add the others when needed. `search.exclude` keeps generated build files out of VS Code's file search; it does not change Rust analysis.

### Reduce background work

By default, `rust-analyzer` runs Cargo checks on save and build scripts for more accurate analysis. If background checks use too many resources, you can temporarily disable them in the settings file:

```json
{
  "rust-analyzer.checkOnSave": false
}
```

This removes on-save Cargo diagnostics. Run the repository's `just` checks from the matching workspace when you need compiler or Clippy results.

### Useful optional settings

The following settings are optional preferences, not requirements. Add the ones you want to your settings file:

```json
{
  "rust-analyzer.restartServerOnConfigChange": true,
  "rust-analyzer.cargo.buildScripts.enable": false,
  "rust-analyzer.initializeStopped": true,
  "rust-analyzer.imports.granularity.group": "item",
  "rust-analyzer.imports.granularity.enforce": true,
  "rust-analyzer.imports.preferPrelude": true,
  "rust-analyzer.inlayHints.lifetimeElisionHints.enable": "always",
  "rust-analyzer.inlayHints.closureReturnTypeHints.enable": "always",
  "rust-analyzer.completion.fullFunctionSignatures.enable": true,
  "rust-analyzer.assist.emitMustUse": true,
  "rust-analyzer.statusBar.showStatusBar": "always"
}
```

The `rust-analyzer` settings reference lists more: the remaining `inlayHints.*` hints (binding mode, closure captures, discriminants, expression adjustments, closing braces, implicit drops), `completion.*` options, `hover.show.traitAssocItems` and `diagnostics.styleLints.enable`.

On Windows, the user-level `settings.json` is under `%APPDATA%\Code\User` (typically `C:\Users\<user_name>\AppData\Roaming\Code\User`).

## Troubleshooting

- If a workspace is missing, confirm its `Cargo.toml` path and check the `linkedProjects` array. Remember that an explicit array disables project auto-discovery.
- If settings are not applied, open the correct User or Workspace settings file from the Command Palette and check for JSON errors. VS Code provides completion and validation for settings in `settings.json`.
- Run **Rust Analyzer: Status** from the Command Palette to inspect the loaded projects and server state. After changing project paths, run **Rust Analyzer: Restart Server**.
- Consult the current [`rust-analyzer` configuration reference][rust-analyzer~configuration~book]↗ for supported options and defaults.

## Related Topics {#related-topics .skip}

- [[development_environment_setup | Development Environment Setup]].
- [[repository_structure | Repository Structure]].
- [[text-editors | Text Editors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
