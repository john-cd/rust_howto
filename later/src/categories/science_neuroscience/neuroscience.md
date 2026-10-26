<!-- TODO Reviewed ??-??-2026 -->

# Neuroscience and Neuroimaging

{{#include neuroscience.incl.md}}

## Generate a Synthetic NIfTI Volume {#neuro}

This example writes a small synthetic 3D volume to a temporary NIfTI file.

```rust,editable
{{#include ../../../crates/cats/science_neuroscience/examples/neuroscience/neuro.rs:example}}
```

## Simulate a Leaky Integrate-and-Fire Neuron {#lif}

```rust,editable
{{#include ../../../crates/cats/science_neuroscience/examples/neuroscience/leaky_integrate_fire.rs:example}}
```

## Related Topics {#related-topics .skip}

- Neuroimaging formats such as NIfTI and DICOM.
- Signal analysis, including EEG/EDF readers and FFTs.
- Machine learning for biology.
- `nifti` and `dicom` ecosystem tooling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[neuro: write](https://github.com/john-cd/rust_howto/issues/476)
cover [`nifti`][c~nifti~docs]↗{{hi:nifti}}, [`dicom`][c~dicom~docs]↗{{hi:dicom}}?
</div>
