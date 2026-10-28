<!-- TODO Reviewed ??-??-2026 -->

# Encoding

This crate contains code examples for the **Encoding** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `base64`, `bincode`, `bytemuck`, `byteorder`, `calamine`.

## Examples Covered

- **Binary Encoders** (`examples/binary_encoders/`): Demonstrates concepts like `bincode`, `capnp`, `ciborium`, `flatbuffers`, `foo_capnp`, `my_game_generated`, `prost`, `rmp_serde`.
- **Csv** (`examples/csv/`): Demonstrates concepts like `delimiter`, `filter`, `invalid`, `read`, `read1`, `serde_serialize`, `serialize`, `transform`.
- **Excel** (`examples/excel/`): Demonstrates concepts like `calamine`, `xlsxwriter`.
- **No External Schema File** (`examples/no_external_schema_file/`): Demonstrates concepts like `postcard`, `rkyv`.
- **Serde** (`examples/serde/`): Demonstrates concepts like `json`, `monostate`, `serde`, `serde_ignored`, `serde_json`.
- **String Encoding** (`examples/string_encoding/`): Demonstrates concepts like `base64`, `form_urlencoded`, `hex`, `hex_crate`, `percent_encode`, `percent_encoding`, `url_encode`.
- **Toml Encoding** (`examples/toml_encoding/`): Demonstrates concepts like `toml`, `toml1`.
- **Typecasts** (`examples/typecasts/`): Demonstrates concepts like `bytemuck`, `primitive`, `zerocopy`.
- **Standalone Examples**: `endian_byte`.

## Usage

To check or run examples in this crate:

```console
cargo check --package encoding --examples
cargo test --package encoding
```

Code examples correspond to chapters in `bk/src/categories/encoding` or related book sections.
