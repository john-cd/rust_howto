# VS Code Configuration

TODO document rust-analyzer settings that works with large codebases like this book.

## `rust-analyzer` Configuration {#rust-analyzer-configuration}

Main `settings.json` under e.g. `C:\Users\<user_name>\AppData\Roaming\Code\User`

`rust-analyzer` may not automatically discover all `cargo` workspaces. Add the following to your `.vscode/settings.json` file:

```json
{
    "rust-analyzer.linkedProjects": [
        "bk/crates/Cargo.toml",
        "playground/Cargo.toml",
        "publish/Cargo.toml",
        "tools/Cargo.toml",
        "wip/Cargo.toml",
        "xmpl/Cargo.toml"
    ],
    // More configuration settings...
}
```

Other configuration options include the following:

```json
"rust-analyzer.restartServerOnConfigChange": true,
"rust-analyzer.cargo.buildScripts.enable": false,
```


```json
"rust-analyzer.checkOnSave": true,

"rust-analyzer.showDependenciesExplorer": false,
"rust-analyzer.cargo.buildScripts.rebuildOnSave": false,
```

```json
"rust-analyzer.statusBar.clickAction": "stopServer",
"rust-analyzer.statusBar.showStatusBar": "always",
```

```json
"rust-analyzer.diagnostics.styleLints.enable": true,

"rust-analyzer.imports.granularity.group": "item",
"rust-analyzer.imports.granularity.enforce": true,
"rust-analyzer.imports.preferPrelude": true,

"rust-analyzer.inlayHints.bindingModeHints.enable": true,
"rust-analyzer.inlayHints.closureCaptureHints.enable": true,
"rust-analyzer.inlayHints.closureReturnTypeHints.enable": "always",
"rust-analyzer.inlayHints.discriminantHints.enable": "always",
"rust-analyzer.inlayHints.expressionAdjustmentHints.enable": "always",
"rust-analyzer.inlayHints.lifetimeElisionHints.enable": "always",
"rust-analyzer.inlayHints.closingBraceHints.minLines": 1,
"rust-analyzer.inlayHints.genericParameterHints.lifetime.enable": true,
"rust-analyzer.inlayHints.genericParameterHints.type.enable": true,
"rust-analyzer.inlayHints.implicitDrops.enable": true,
"rust-analyzer.inlayHints.rangeExclusiveHints.enable": true,

"rust-analyzer.assist.emitMustUse": true,
"rust-analyzer.assist.preferSelf": true,

"rust-analyzer.completion.fullFunctionSignatures.enable": true,
"rust-analyzer.completion.privateEditable.enable": true,
"rust-analyzer.completion.termSearch.enable": true,

"rust-analyzer.hover.show.traitAssocItems": 5,

"rust-analyzer.completion.limit": 25,
"rust-analyzer.completion.postfix.enable": false,
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
