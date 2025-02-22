# WebAssembly (WASM)

[![cat-wasm][cat-wasm-badge]][cat-wasm]

Tooling targeting WebAssembly or manipulating WebAssembly.

## What is WASM?

WebAssembly (abbreviated WASM) is a binary instruction format that is designed to be a portable compilation target for programming languages. WASM binaries typically have a [`.wasm`][book-rustwasm]⮳{{hi:.wasm}} [`.wasm`][cat-wasm]⮳{{hi:.wasm}} [`.wasm`][c-wasmer]⮳{{hi:.wasm}} [`.wasm`][c-wasmtime]⮳{{hi:.wasm}}  file extension. The textual representation of WebAssembly has a `.wat` file extension.

WebAssembly was created primarily to address the limitations of JavaScript (performance, security) and the desire to bring other programming languages to the Web.

- JavaScript, while versatile, can struggle with computationally intensive tasks. WebAssembly code is designed to be small and fast, running at near-native speeds within the browser.
- WASM is a low-level, assembly-like language: It is designed to be a compilation target for other languages like C, C++, Rust. This means you can write code in these languages and compile it into WebAssembly for execution in the browser or outside of it. WASM allows developers to leverage their existing codebases and skills. This opens up the web to a wider range of developers and enables the reuse of existing libraries and tools.
- WASM is designed to be platform-independent, running consistently across different browsers and operating systems.
- WASM operates within a secure sand-boxed environment, mitigating potential security risks.

In essence, WebAssembly bridges the gap between high-level programming languages and the Web, enabling developers to create powerful and performant web applications.

### Use cases

- Gaming: Powering complex game logic and graphics.
- Data visualization and analysis: Handling heavy data processing tasks.
- Audio/video processing: Enabling real-time audio and video manipulation.
- Cryptography: Performing cryptographic operations efficiently.
- Virtual/Augmented Reality (VR/AR): Handling the demanding computations required for immersive experiences.

While WebAssembly (Wasm) was initially designed for the web, its benefits like portability, performance, and security have led to its adoption in various non-web environments:

- Serverless Computing,
- Embedded Systems and IoT,
- Edge Computing,
- Cross-Platform Development,
- Plugin systems for e.g. desktop applications, extending their functionality with secure and isolated modules.

WASM can be used to build virtual machines or emulators for other architectures, enabling execution of legacy software or providing sand-boxed environments.

## `yew`

{{#include yew.incl.md}}

## WASM Runtimes

{{#include wasm_runtimes.incl.md}}

## See also

[Rust and WebAssembly (book)][book-rustwasm]{{hi:WASM}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 review](https://github.com/john-cd/rust_howto/issues/970)

This table outlines common WebAssembly (WASM) development tasks and relevant Rust crates.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Compiling Rust to WASM | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}}, [`cargo-wasi`][c-cargo_wasi]⮳{{hi:cargo-wasi}} | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} is essential for bridging between Rust and JavaScript, allowing you to interact with the browser's APIs from your Rust/WASM code. [`cargo-wasi`][c-cargo_wasi]⮳{{hi:cargo-wasi}} is used for compiling to WASI (WebAssembly System Interface), which is useful for running WASM outside of the browser (e.g., on servers or embedded devices). |
| WASM Runtime (in Rust) | [`wasmi`][c-wasmi]⮳{{hi:wasmi}}, [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} | [`wasmi`][c-wasmi]⮳{{hi:wasmi}} is a WASM interpreter written in Rust. [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} provides bindings for the WASI API, enabling your WASM code to interact with the host environment when running outside the browser. |
| Interacting with JavaScript from Rust/WASM | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} |  [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} handles the complexities of passing data between Rust/WASM and JavaScript. |
| Calling Rust/WASM functions from JavaScript | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} |  [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} also facilitates calling Rust/WASM functions from JavaScript. |
| Managing WASM Memory | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} (implicitly), [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} | [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} helps manage WASM memory. [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} is a memory allocator designed for WASM. |
| Web Frameworks (using WASM) | [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, [`leptos`][c-leptos]⮳{{hi:leptos}}  | These frameworks enable building complex web applications with Rust/WASM. They provide component-based architectures and other tools for structuring web apps. |
| Game Development (using WASM) | [`winit`][c-winit]⮳{{hi:winit}} (windowing), [`wgpu`][c-wgpu]⮳{{hi:wgpu}} (WebGPU bindings), [`bevy`][c-bevy]⮳{{hi:bevy}} (game engine - can target WASM) | Rust can be used for game development targeting the web via WASM. [`winit`][c-winit]⮳{{hi:winit}}  handles window creation, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} provides access to WebGPU, and [`bevy`][c-bevy]⮳{{hi:bevy}} is a Rust game engine that can compile to WASM. |
| Server-Side WASM (WASI) | [`cargo-wasi`][c-cargo_wasi]⮳{{hi:cargo-wasi}}, [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} |  WASI allows you to run WASM code on servers or other environments outside the browser. |
| Debugging WASM | Browser developer tools, [`console_error_panic_hook`][c-console_error_panic_hook]⮳{{hi:console_error_panic_hook}} | Browser developer tools can be used to debug WASM. [`console_error_panic_hook`][c-console_error_panic_hook]⮳{{hi:console_error_panic_hook}} helps with better error reporting in the browser console. |
| Testing WASM | [`wasm-bindgen-test`][c-wasm_bindgen_test]⮳{{hi:wasm-bindgen-test}} | [`wasm-bindgen-test`][c-wasm_bindgen_test]⮳{{hi:wasm-bindgen-test}} provides utilities for testing your Rust/WASM code. |
| Size Optimization | [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}, [`twiggy`][c-twiggy]⮳{{hi:twiggy}}, [`wasm-opt`][c-wasm_opt]⮳{{hi:wasm-opt}} |  [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} can help reduce WASM binary size. [`twiggy`][c-twiggy]⮳{{hi:twiggy}} is a tool for analyzing WASM binaries. [`wasm-opt`][c-wasm_opt]⮳{{hi:wasm-opt}} is a tool for optimizing WASM code. |

## Key Considerations

- [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} is fundamental for almost all Rust/WASM development that interacts with the browser.
- [`WASI`][c-wasi]⮳{{hi:WASI}}: WASI is important if you want to run your WASM code outside of the browser.
- Web Frameworks: Frameworks like [`yew`][c-yew]⮳{{hi:yew}}, [`seed`][c-seed]⮳{{hi:seed}}, and [`leptos`][c-leptos]⮳{{hi:leptos}}  simplify web application development with Rust/WASM.
- Performance:  Rust's performance characteristics translate well to WASM, making it suitable for performance-sensitive web applications.
- Debugging: Browser developer tools are essential for debugging WASM.
- Size Optimization:  WASM binary size is important for web performance.  Use tools like [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}, [`twiggy`][c-twiggy]⮳{{hi:twiggy}}, and [`wasm-opt`][c-wasm_opt]⮳{{hi:wasm-opt}} to reduce size.

## Workflow

A typical Rust/WASM development workflow involves writing Rust code, using [`wasm-bindgen`][c-wasm_bindgen]⮳{{hi:wasm-bindgen}} to create bindings to JavaScript, compiling the Rust code to WASM, and then using JavaScript to load and interact with the WASM module in a web page.

</div>
