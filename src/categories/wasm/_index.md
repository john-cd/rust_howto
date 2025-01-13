# WebAssembly (WASM)

[![cat-wasm][cat-wasm-badge]][cat-wasm]

Tooling targeting WebAssembly or manipulating WebAssembly.

## What is WASM?

WebAssembly (abbreviated WASM) is a binary instruction format that is designed to be a portable compilation target for programming languages. WASM binaries typically have a `.wasm` file extension. The textual representation of WebAssembly has a `.wat` file extension.

WebAssembly was created primarily to address the limitations of JavaScript (performance, security) and the desire to bring other programming languages to the Web.

- JavaScript, while versatile, can struggle with computationally intensive tasks. WebAssembly code is designed to be small and fast, running at near-native speeds within the browser.
- WASM is a low-level, assembly-like language: It is designed to be a compilation target for other languages like C, C++, Rust. This means you can write code in these languages and compile it into WebAssembly for execution in the browser or outside of it. WASM allows developers to leverage their existing codebases and skills. This opens up the web to a wider range of developers and enables the reuse of existing libraries and tools.
- WASM is designed to be platform-independent, running consistently across different browsers and operating systems.
- WASM operates within a secure sandboxed environment, mitigating potential security risks.

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

WASM can be used to build virtual machines or emulators for other architectures, enabling execution of legacy software or providing sandboxed environments.

## `yew`

{{#include yew.incl.md}}

## Others

{{#include others.incl.md}}

## See also

[Rust and WebAssembly (book)][book-rustwasm]{{hi:WASM}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
