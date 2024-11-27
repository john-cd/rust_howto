# Processor

{{#include processor.incl.md}}

## Check number of logical cpu cores {#check-number-of-logical-cpu-cores}

[![num_cpus][c-num_cpus-badge]][c-num_cpus]{{hi:num_cpus}}
[![num_cpus-crates.io][c-num_cpus-crates.io-badge]][c-num_cpus-crates.io]
[![num_cpus-github][c-num_cpus-github-badge]][c-num_cpus-github]
[![num_cpus-lib.rs][c-num_cpus-lib.rs-badge]][c-num_cpus-lib.rs]
[![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}}{{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in current machine using [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.

```rust,editable
{{#include ../../../deps/tests/cats/hardware_support/cpu_count.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 expand
</div>
