<!-- TODO Reviewed ??-??-2026 -->
# WASM Basics

{{#include wasm_basics.incl.md}}

## What is WebAssembly? {#What-is-wasm .skip}

WebAssembly (Wasm) is a portable binary instruction format and execution environment designed primarily as a compiler target. Developers usually write Rust, C, C++, or another source language, then compile it to a `.wasm` module. The human-readable text representation uses the `.wat` extension. See [WebAssembly concepts on MDN][mozilla~webassembly-concepts~website]↗.

Wasm is designed to complement JavaScript in web applications. A browser loads a module through JavaScript or generated bindings, and JavaScript and Wasm can call one another through imports and exports. Wasm can run close to native speed for some workloads, but performance depends on the workload, runtime, data transfer, and startup cost. Measure the complete application before choosing it for speed.

The WebAssembly core format does not provide operating-system services or browser APIs by itself. A host runtime supplies imported functions and decides which capabilities a module can use. In a browser, access to the DOM and other Web APIs is mediated by JavaScript bindings. In other environments, runtimes may expose WASI or their own APIs. Wasm's sandbox limits a module's access to its host, while the host's imports and permissions determine what it can do.

## Target environments {#target-environments .skip}

The same `.wasm` format can be used in different runtimes, but a module must match the imports and features that its host provides. A browser module that uses Web APIs is not automatically a standalone WASI program, and a WASI module does not automatically have access to a browser DOM. See [[wasm_standalone_runtimes | WASM Standalone Runtimes]] for examples of non-browser hosts.

## Use cases {#wasm-use-cases .skip}

In browsers, Wasm is useful for compute-heavy or portable components such as image and audio processing, simulations, emulators, and parts of games. It can also run code written in languages other than JavaScript alongside a JavaScript application. It is not a replacement for browser APIs or a guarantee that an application will be faster.

Outside browsers, Wasm can provide a portable module format for plugins, server-side tasks, edge workloads, or embedded systems when the target runtime supports the required imports. Portability depends on the APIs and WebAssembly features used by the module.

Rust game libraries such as [`winit`][c~winit~docs]↗{{hi:winit}}, [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, and [`bevy`][c~bevy~docs]↗{{hi:bevy}} can be used for browser-targeted projects when their selected features and graphics backend support the target.

## Related topics {#related-topics .skip}

- [[wasm_development | WASM Development]]
- [[interfacing_with_javascript | Interfacing with JavaScript]]
- [[web-programming | Web Programming]]
- [[development_tools | Development Tools]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[final review](https://github.com/john-cd/rust_howto/issues/1219)
</div>
