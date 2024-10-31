# Processor

{{#include processor.incl.md}}

## Check number of logical cpu cores

[![num_cpus][c-num_cpus-badge]][c-num_cpus]{{hi:num_cpus}}  [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}}{{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in current machine using [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.

```rust
{{#include ../../../deps/tests/cats/hardware_support/cpu_count.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
