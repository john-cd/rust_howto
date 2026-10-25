<!-- TODO Reviewed ??-??-2026 -->

## Biology and Neuroscience

[![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~science::neuroscience][cat~science::neuroscience~badge]][cat~science::neuroscience]{{hi:Neuroscience}}

Biology studies living systems; neuroscience is one area that studies the brain and nervous system. Rust can help build reliable tools for processing large sequence and imaging datasets, implementing analysis algorithms, and running simulations. The wider scientific category also includes [[science | machine learning and scientific computing]].

The Rust ecosystem has domain-specific crates for common biological data formats and sequence operations, alongside general-purpose array, statistics, and data-processing libraries. Rust can also interoperate with established Python and R tools through FFI when a workflow depends on those ecosystems.

| Topic | Rust Crates (Examples) | Use cases |
|---|---|---|
| Bioinformatics and genomics | [`bio`][rust-bio~docs]↗{{hi:Rust-Bio}}, [`noodles`][noodles~docs]↗{{hi:noodles}} | Read common sequence and alignment formats, then build parsing, indexing, or analysis pipelines. |
| Neuroimaging formats | [`nifti`][c~nifti~docs]↗{{hi:nifti}}, [`dicom`][c~dicom~docs]↗{{hi:dicom}} | Read and write NIfTI and DICOM images; mind coordinate systems and measurement units. |
| Scientific data and computation | [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`polars`][c~polars~docs]↗{{hi:polars}} | Transform multidimensional measurements and tabular datasets. |
| Neuroscience analysis | [`nifti`][c~nifti~docs]↗{{hi:nifti}} and general-purpose numerical crates | Process imaging volumes and build simulations; connect to specialist tools where needed. |

## Code Examples

{{#include neuroscience.incl.md}}

The example generates a synthetic three-dimensional volume and writes it as a NIfTI file. It demonstrates file handling only; the generated values are not measured biological data.

## Related Topics

Use Rust's numerical, data-processing, and visualization crates to build biology and neuroscience tools.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Data [[visualization | Visualization]] | | Plotting libraries can help inspect measurements and analysis results. |
| Machine Learning and Data [[science | Science]] | [`linfa`][c~linfa~docs]↗{{hi:linfa}}, [`tch-rs`][c~tch~docs]↗{{hi:tch-rs}} | General machine-learning libraries can support classification, prediction, and image analysis. |
| [[mathematics | Mathematics]] and numerical computation | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}, [`ndarray`][c~ndarray~docs]↗{{hi:ndarray}}, [`statrs`][c~statrs~docs]↗{{hi:statrs}} | Use established numerical types and methods for scientific calculations. |
| Data Analysis & [[data-processing | Processing]] | [`polars`][c~polars~docs]↗{{hi:polars}}, [`dataframe`][c~dataframe~docs]↗{{hi:dataframe}} | These crates are useful for working with large datasets. |
| [[simulation | Simulation]] | | |
| FFI | | Leverage existing neuroscience libraries by creating Rust bindings using FFI. See [[development-tools_ffi | Development Tools FFI]], [[external-ffi-bindings | External FFI Bindings]], [[api-bindings | API Bindings]] |

Biological data requires careful handling of identifiers, units, coordinate systems, and provenance. Validate format assumptions against the data producer and domain documentation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/959)
[also cover Biology! see lib.rs categorization](https://github.com/john-cd/rust_howto/issues/1197)
</div>
