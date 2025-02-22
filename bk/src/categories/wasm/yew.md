# Yew

{{#include yew.incl.md}}

## Create efficient web applications with `yew` {#yew}

[![yew-website][c-yew-website-badge]][c-yew-website] [![yew][c-yew-badge]][c-yew] [![yew-crates.io][c-yew-crates.io-badge]][c-yew-crates.io] [![yew-github][c-yew-github-badge]][c-yew-github] [![yew-lib.rs][c-yew-lib.rs-badge]][c-yew-lib.rs]{{hi:yew}}{{hi:Javascript}}{{hi:Webasm}}{{hi:Web}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

`yew` is a Rust framework for building multi-threaded front-end web applications with WebAssembly.

Key Features:

- Interactive HTML within Rust: Includes a macro for declaring interactive HTML using Rust expressions (similar to JSX in React).
- High Performance: Yew ensures optimal [performance][p-performance] by minimizing DOM API calls during page rendering and simplifying the process of offloading tasks to background web workers.
- JavaScript Interoperability: Yew supports JavaScript integration, allowing the use of NPM packages and integrating smoothly with existing JavaScript projects.

```rust,editable
{{#include ../../../crates/cats/wasm/tests/yew.rs:example}}
```

### See also

- [What is Yew?][c-yew-website]⮳

- [Example "Real World" app built with Rust + Yew + WebAssembly][c-yew-realworld-example]⮳ [![yew-realworld-example][c-yew-realworld-example-badge]][c-yew-realworld-example]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[yew: write / organize (P2)](https://github.com/john-cd/rust_howto/issues/498)

cover other [WASM][p-wasm] based frameworks
</div>
