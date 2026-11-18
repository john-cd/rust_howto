<!-- TODO Reviewed ??-??-2026. -->

# Neuroscience and Neuroimaging

{{#include neuroscience.incl.md}}

## Simulate a Leaky Integrate-and-Fire Neuron {#lif}

The leaky integrate-and-fire model is a compact way to explore how input current and membrane properties affect spike timing. This deterministic example uses Euler steps and checks both suprathreshold and subthreshold inputs.

```rust,editable
{{#include ../../../crates/cats/science_neuroscience/examples/neuroscience/leaky_integrate_fire.rs:example}}
```

## Read Brain-Imaging Data {#imaging}

### Generate a Synthetic NIfTI Volume {#neuro}

[`nifti`][c~nifti~docs]↗{{hi:nifti}} reads and writes NIfTI-1 volumes. With its `ndarray_volumes` feature, a volume can be converted to an `ndarray` for analysis. The example below generates a synthetic volume and writes it to a temporary file; its values are test data, not measured brain scans.

```rust,editable
{{#include ../../../crates/cats/science_neuroscience/examples/neuroscience/neuro.rs:example}}
```

The example's volume is synthetic. For an acquired volume, preserve voxel spacing, orientation, and the affine transform that maps voxel indices to the physical coordinate system; do not infer millimeters from array indices alone.

## Estimate a Frequency in a Sampled Signal {#fft-signal}

[`rustfft`][c~rustfft~docs]↗{{hi:rustfft}} computes a discrete Fourier transform. This example creates a two-second synthetic 10 Hz signal sampled at 128 Hz, applies a Hann window, and finds the strongest positive-frequency bin. The bin spacing is 0.5 Hz; this demonstrates the processing steps, not clinical interpretation.

```rust,editable
{{#include ../../../crates/cats/science_neuroscience/examples/neuroscience/fft_signal.rs:example}}
```

For recorded signals, retain the sampling rate and units for every channel, and document filtering, windowing, and normalization. The frequency spectrum depends on those choices and on gaps or artifacts in the recording.

For clinical imaging files, [`dicom`][c~dicom~docs]↗{{hi:dicom}} is the DICOM-rs umbrella crate. Its `object` module reads DICOM objects and its `pixeldata` module converts pixel data to image or array forms. A DICOM study commonly consists of multiple objects, while NIfTI is often used for a single research volume. DICOM-rs 0.10.0 was released in June 2026 and remains under active development; review its [release history][dicom-rs~releases]↗{{hi:DICOM-rs releases}} when selecting a version.

## Analyze EEG and Other Signals {#signals}

EDF and EDF+ files store sampled signals and, for EDF+, annotations and recording timing. [`european-data-format`][c~european-data-format~docs]↗{{hi:european-data-format}} reads and writes EDF and EDF+; [`edfplus`][c~edfplus~docs]↗{{hi:edfplus}} is another EDF+ reader and writer. As of this review, their latest listed releases are 0.2.0 (March 2026) and 0.1.0 (July 2025), respectively. Both are pre-1.0, so check supported EDF features and release activity before choosing one for a project.

For spectral analysis, [`rustfft`][c~rustfft~docs]↗{{hi:rustfft}} provides pure-Rust FFTs. Plan a transform with `FftPlanner`, then process signal samples in a buffer. Convert sample indices to time using the recording's sampling rate, and document windowing and normalization when interpreting spectral magnitudes. RustFFT 6.4.1 was released in September 2025.

## Related Topics {#related-topics .skip}

- [Science and machine learning](../science/index.md).
- [Computer vision](https://john-cd.com/rust_howto/categories/computer-vision/opencv.html).
- [Data visualization](https://john-cd.com/rust_howto/categories/visualization/visualization.html).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
