<!-- TODO Reviewed ??-??-2026. -->

# Computer Vision

The [OpenCV chapter](../../../src/categories/computer-vision/opencv.md) covers
grayscale conversion, Canny edges, and a Haar cascade. The native recipes run
on Linux with OpenCV headers, libclang, and pkg-config installed.

From `bk/crates`:

```console
cargo run --locked -p computer_vision --features opencv --example opencv -- opencv
cargo test --locked -p computer_vision --features opencv --example opencv
```

For the `faces` subcommand and the ignored classifier test, set
`OPENCV_HAAR_CASCADE` to a readable cascade XML file.
