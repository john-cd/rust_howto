<!-- TODO Reviewed ??-??-2026. -->

# Runtime Recipe Readiness

Windows checks in [#2377](https://github.com/john-cd/rust_howto/issues/2377)
use the workspace lockfile and Rust 1.100.0. A build or an available command
does not establish an interactive or service result.

| Workflow | Result | Next verification |
| --- | --- | --- |
| Embedded PyO3 | Executed with CPython 3.15.2. The launch exited successfully after adding the Python installation directory to PATH. | The separate Maturin/devcontainer workflow remains [#2211](https://github.com/john-cd/rust_howto/issues/2211). |
| Wasmtime typed component | Built the guest for wasm32-wasip2 and called it from the Windows host. The run printed “Hello, Ferris!” and passed its typed error and logger callback assertions. | The review used direct rustc after sccache could not launch the compiler for web-sys. |
| Yew and Leptos | Trunk 0.22.2 built both standalone counters with Yew 0.23.0, Leptos 0.8.23, and wasm-bindgen 0.2.130. Browser clicks in Edge 157 exercised increment, decrement, threshold visibility, and Leptos parity. | The check served the built sites over loopback HTTP. |
| Docker | Both daemon-dependent tests passed against Docker Engine 29.9.3 in Linux container mode. Bollard ran its temporary container, and Testcontainers sent a Redis PING and observed PONG. | Use the documented Docker endpoint for your daemon. |
| Kubernetes | The controller used explicit kubeconfig credentials against kind v0.33.0 and Kubernetes v1.35.1. It created the owned Deployment, reported one ready replica, and Kubernetes removed the Deployment after the Echo was deleted. Five offline tests passed. | The disposable cluster was removed after the run. |
| OpenCV | All five native tests passed in a Debian 13 Linux container using OpenCV 4.10.0 and crate 0.102.0. They covered grayscale, Canny, an image-file round trip, and a Haar cascade check on a blank frame. | The classifier used the frontal-face XML supplied by Debian's `opencv-data` package. |
| AWS | Credential-dependent scenarios remain unverified in this pass. | Use a dedicated account and the documented narrow permissions for the chosen recipe. |

Run the verified embedding example from the repository root in a PowerShell
session with the intended Python on PATH:

```powershell
$env:PYTHONHOME = (python -c "import sys; print(sys.base_prefix)").Trim()
$env:PYO3_PYTHON = Join-Path $env:PYTHONHOME 'python.exe'
$env:PATH = $env:PYTHONHOME + ';' + $env:PATH
just cargo run --manifest-path later/crates/Cargo.toml --locked -p development_tools_ffi --example python -- pyo3
```

For the Windows Wasmtime retry, the wrapper overrides apply only to this command.

```console
just RUSTC_WRAPPER= CARGO_BUILD_RUSTC_WRAPPER= cargo run --manifest-path later/crates/Cargo.toml --locked -p wasm --features wasmtime --example wasm_standalone_runtimes -- wasmtime_component
```

## Browser, Docker, and OpenCV Checks

- Build the [Yew](src/categories/wasm/yew.md#start-a-browser-application) and
  [Leptos](src/categories/wasm/leptos.md#client-side-rendering) counters with
  `trunk build` or start each with `trunk serve`. In Yew, go from 0 to 6 and
  back to 5 and check that the threshold appears and disappears. In Leptos,
  verify 0/even, 1/odd, and 0/even, then increase to 6 to check the threshold message.
- From the repository root, run the live local Docker tests with
  `just cargo test --manifest-path bk/crates/Cargo.toml --locked -p virtualization --features containers --example using_containers -- --ignored`.
  Both tests passed again after the move to the book workspace, against Docker Desktop's Linux daemon (Engine 29.9.3).
- Follow the [disposable cluster walkthrough](src/other/cloud/kubernetes.md#verify-reconciliation-in-a-disposable-cluster)
  to run the controller with an explicit kubeconfig, verify status, and check
  garbage collection.
- On Linux with the SDK installed, run
  `cargo test --manifest-path later/crates/Cargo.toml --locked -p computer_vision --features opencv --example opencv`.
  To include the classifier test on Debian 13, install `opencv-data`, set
  `OPENCV_HAAR_CASCADE=/usr/share/opencv4/haarcascades/haarcascade_frontalface_default.xml`,
  and append `-- --include-ignored`.
