## Neuroscience

[![cat~science::neuroscience][cat~science::neuroscience~badge]][cat~science::neuroscience]{{hi:Neuroscience}}

Research tools and processing of data related to the brain and nervous system.

The Rust neuroscience ecosystem is still developing, but there's growing interest and activity. Integration with existing neuroscience tools and libraries (often written in Python, MATLAB, or other languages) via FFI (Foreign Function Interface) is a common approach. Many neuroscience applications rely heavily on numerical computation, data analysis, and visualization, so general-purpose Rust crates in those areas are also relevant. Rust's performance is a major advantage for computationally-intensive neuroscience applications, especially for simulations, real-time processing, and large datasets.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Neuroimaging Data Formats | [`nifti`][c~nifti~docs]↗{{hi:nifti}}, [`dicom`][c~dicom~docs]↗{{hi:dicom}} | [`nifti`][c~nifti~docs]↗{{hi:nifti}} handles the NIfTI format, a common neuroimaging data format. [`dicom`][c~dicom~docs]↗{{hi:dicom}} is for DICOM, another crucial format, particularly for MRI. |
| Neuroimaging Analysis | | This area is still relatively nascent in pure Rust. Integration with existing tools via FFI might be common. |
| Computational Neuroscience | | Computational neuroscience often involves simulations and modeling, which can leverage Rust's performance. |
| Electrophysiology Data Analysis | | Analysis of EEG, MEG, and other electrophysiological data is an active area of development. |
| Brain-Computer Interfaces (BCIs) | | BCI development is likely to involve low-level hardware interaction and real-time processing, where Rust's performance can be beneficial. |
| Connectomics | | Analysis of brain networks and connections is an area ripe for development in Rust. |

## Code Examples

{{#include neuroscience.incl.md}}

## Related Topics

Utilize Rust's numerical computation, data analysis, and visualization crates to build neuroscience-specific tools.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Data [[visualization | Visualization]] | | While general-purpose visualization crates can be used, neuroscience-specific visualization tools are less common in pure Rust. |
| Machine Learning and Data [[science | Science]] | Use general ML crates like [`linfa`][c~linfa~docs]↗{{hi:linfa}}, [`tch-rs`][c~tch~docs]↗{{hi:tch-rs}} | Machine learning techniques are widely used in neuroscience, and Rust's ML ecosystem can be leveraged. |
| [[mathematics | Mathematics]] and Numerical Computation | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`statrs`][c~statrs~docs]↗{{hi:statrs}} | These crates are essential for numerical computations commonly used in neuroscience. |
| Data Analysis & [[data-processing | Processing]] | [`polars`][c~polars~docs]↗{{hi:polars}}, [`dataframe`][c~dataframe~docs]↗{{hi:dataframe}} | These crates are useful for working with large datasets. |
| [[simulation | Simulation]] | | |
| FFI | | Leverage existing neuroscience libraries by creating Rust bindings using FFI. See [[development-tools_ffi | Development Tools FFI]], [[external-ffi-bindings | External FFI Bindings]], [[api-bindings | API Bindings]] |

See also Bioinformatics & Genomics. Some neuroscience research overlaps with bioinformatics, so relevant crates from that domain might be applicable.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/959)
[also cover Biology! see lib.rs categorization](https://github.com/john-cd/rust_howto/issues/1197)
</div>
