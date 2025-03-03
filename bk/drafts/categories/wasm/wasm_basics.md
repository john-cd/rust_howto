# WASM Basics

## What is WASM?

WebAssembly (abbreviated WASM) is a low-level assembly-like language with a compact binary format that can be run in modern web browsers.

WebAssembly was created primarily to address the limitations of JavaScript (performance, security...) and the desire to bring other programming languages to the Web.

- JavaScript, while versatile, can struggle with computationally intensive tasks. WebAssembly code is small and fast, running at near-native speeds within the browser.
- WebAssembly is designed to complement and run alongside JavaScript — using the WebAssembly JavaScript APIs, you can load WebAssembly modules into a JavaScript app and share functionality between the two.
- WASM is a low-level, assembly-like language: It is designed to be a compilation target for other languages like C, C++, and Rust. This means you can write code in these languages and compile it into WebAssembly for execution in the browser (and nowadays, outside of it). WASM allows developers to leverage their existing codebases and skills. This opens up the web to a wider range of developers and enables the reuse of existing libraries and tools.
- WASM is designed to be platform-independent, running consistently across different browsers and operating systems.
- WASM operates within a secure sand-boxed environment, mitigating security risks.

WebAssembly binaries typically have a [`.wasm`][book-rustwasm]⮳{{hi:.wasm}} file extension. The textual representation of WebAssembly has a `.wat` file extension.

In essence, WebAssembly bridges the gap between high-level programming languages and the Web, enabling developers to create powerful and performant web applications.

## WASM Use Cases

Inside the browser:

- Gaming: Powering complex game logic and graphics.
- Data visualization and analysis: Handling heavy data processing tasks.
- Audio/video processing: Enabling real-time audio and video manipulation.
- Image Recognition: Real-time image processing and recognition.
- Music Applications: Streaming and caching music.
- Cryptography: Performing cryptographic operations efficiently.
- Peer-to-Peer Applications: Collaborative editing, decentralized and centralized applications.
- Virtual/Augmented Reality (VR/AR): Handling the demanding computations required for immersive experiences.
- Scientific Visualization and Simulation.

While WebAssembly was initially designed for the web, its benefits like portability, performance, and security have led to its adoption in various non-browser environments:

- Server-Side Compute: Running untrusted code on the server side,
  - Serverless Computing,
  - Distributed computations across multiple nodes.
  - Edge Computing,
- Cross-Platform Development,
- Embedded Systems and IoT,
- Hybrid Native Apps: Combining native and web technologies on mobile devices,
- Plugin systems for e.g. desktop applications, extending their functionality with secure and isolated modules.

WASM can be used to build virtual machines or emulators for other architectures, enabling execution of legacy software or providing sand-boxed environments.

See [webassembly.org](https://webassembly.org) and [madewithwebassembly.com](https://madewithwebassembly.com/).

### Game Development using WASM

Rust can be used for game development targeting the web via WASM. [`winit`][c-winit]⮳{{hi:winit}} handles window creation, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} provides access to WebGPU, and [`bevy`][c-bevy]⮳{{hi:bevy}} is a Rust game engine that can compile to WASM.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO polish
- What is WebAssembly?
- Rust-WASM Ecosystem
- WebAssembly's Binary Format
- WASM Text Representation (WAT)

Application Areas
- Web Applications (e.g., Single Page Applications)
- Image, Video, and Audio Processing
- Game Development
- Scientific Computing and Simulations
- Data Visualization and Rendering (e.g., charts, maps)

</div>
