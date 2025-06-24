#![allow(dead_code)]
// // ANCHOR: example
// //! Example of using the `flate2` crate.
// //!
// //! `flate2` is a stream compression/decompression library.
// //! It provides support for compression and decompression of DEFLATE-based
// //! streams:
// //! - the DEFLATE format itself,
// //! - the zlib format,
// //! - gzip.
// //!
// //! It supports several different backends, controlled through its features.

// use std::fs::File;
// use std::io;
// use std::io::BufReader;
// // Import the prelude for common I/O traits.
// use std::io::prelude::*;
// use std::path::Path;

// use flate2::Compression;

// /// Compress bytes.
// ///
// /// Encoders in `flate2::write` implement the `Write` interface
// /// and takes a stream of uncompressed data, writing the compressed data
// /// to the wrapped writer.
// fn compress_bytes(raw_bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
//     let writer = Vec::new();
//     let mut e = flate2::write::ZlibEncoder::new(writer,
// Compression::default());     e.write_all(raw_bytes)?;
//     let compressed_bytes = e.finish()?;
//     Ok(compressed_bytes)
// }

// /// Decompress bytes.
// ///
// /// This function takes a vector of bytes, decompresses it, and returns the
// /// decompressed string.
// fn decompress_bytes(compressed_bytes: Vec<u8>) -> io::Result<String> {
//     let mut gz = flate2::read::ZlibDecoder::new(&compressed_bytes[..]);
//     let mut s = String::new();
//     gz.read_to_string(&mut s)?;
//     Ok(s)
// }

// /// Use a buffered file to compress contents into a Vec<u8>.
// /// This function reads a file, compresses its contents, and returns the
// /// compressed data in a vector.
// fn compress_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
//     let f = File::open(path)?;
//     let b = BufReader::new(f);
//     // When read from, it reads uncompressed data from the underlying BufRead
//     // and provides the compressed data:
//     let mut z = flate2::bufread::ZlibEncoder::new(b, Compression::fast());
//     let mut buffer = Vec::new();
//     z.read_to_end(&mut buffer)?;
//     Ok(buffer)
// }

// fn main() -> anyhow::Result<()> {
//     let raw_bytes = b"Hello";
//     let compressed_bytes = compress_bytes(raw_bytes)?;
//     println!("{:?}", compressed_bytes);

//     let decompressed_string = decompress_bytes(compressed_bytes)?;

//     let compressed = compress_file("temp/hello_world.txt")?;
//     println!("{:?}", compressed);

//     Ok(())
// }
// // ANCHOR_END: example
// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [read / write a file NOW](https://github.com/john-cd/rust_howto/issues/1009)
