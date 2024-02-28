# Processor

{{#include processor.incl.md}}

## Check number of {{i:logical cpu cores}}

[![num-cpus][num-cpus-badge]][num-cpus]  [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]

Shows the number of logical {{i:CPU cores}} in current machine using [`num-cpus::get`][num-cpus::get].

```rust,editable
{{#include ../../../deps/tests/cpu-count.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
