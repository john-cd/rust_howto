<!-- TODO Reviewed ??-??-2026. -->

# Runtime Recipe Readiness

Windows checks in [#2377](https://github.com/john-cd/rust_howto/issues/2377)
use the workspace lockfile and Rust 1.100.0. A build or an available command
does not establish an interactive or service result.

| Workflow | Result | Next verification |
| --- | --- | --- |
| Embedded PyO3 | Executed with CPython 3.15.2. The launch exited successfully after adding the Python installation directory to PATH. | The separate Maturin/devcontainer workflow remains [#2211](https://github.com/john-cd/rust_howto/issues/2211). |
| Wasmtime typed component | Installed the guest target. The host build stopped when sccache failed to launch a compiler process for a dependency. | Run the documented command and confirm the greeting, typed errors, and guest log callbacks. |
| Yew and Leptos | Browser interaction unverified. Trunk was unavailable on the review host. | Start the recipe in a browser and exercise each documented update. A server-side render covers different behavior. |
| Containers and Kubernetes | Docker CLI available, but its engine pipe was unavailable. Kubectl had no current context. | Use a disposable local context, verify the generated CRD and reconciliation, then remove the test resources. |
| OpenCV | SDK-dependent examples remain unverified in this pass. | Use the feature and SDK requirements in the OpenCV chapter and check a known image result. |
| AWS | Credential-dependent scenarios remain unverified in this pass. | Use a dedicated account and the documented narrow permissions for the chosen recipe. |

Run the verified embedding example from the repository root in a PowerShell
session with the intended Python on PATH:

```powershell
$env:PYTHONHOME = (python -c "import sys; print(sys.base_prefix)").Trim()
$env:PYO3_PYTHON = Join-Path $env:PYTHONHOME 'python.exe'
$env:PATH = $env:PYTHONHOME + ';' + $env:PATH
just cargo run --manifest-path later/crates/Cargo.toml --locked -p development_tools_ffi --example python -- pyo3
```
