# Processor

{{#include processor.incl.md}}

## Check the number of logical cpu cores {#check-number-of-logical-cpu-cores}

[![num_cpus][c-num_cpus-badge]][c-num_cpus] [![num_cpus-crates.io][c-num_cpus-crates.io-badge]][c-num_cpus-crates.io] [![num_cpus-github][c-num_cpus-github-badge]][c-num_cpus-github] [![num_cpus-lib.rs][c-num_cpus-lib.rs-badge]][c-num_cpus-lib.rs]{{hi:num_cpus}}{{hi:Cpus}}{{hi:Cores}}{{hi:Cpu}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}}{{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in the current machine using [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.

```rust,editable
{{#include ../../../crates/ex/cats/hardware_support/tests/cpu_count.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[processor: expand (P2)](https://github.com/john-cd/rust_howto/issues/399)

</div>
