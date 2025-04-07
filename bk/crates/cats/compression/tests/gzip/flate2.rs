// // ANCHOR: example
// //! Example of using the `flate2` crate.
// //!
// //! `flate2` is a DEFLATE-based stream compression/decompression library.

// use std::fs::File;
// use std::io;
// use std::io::BufReader;
// use std::io::prelude::*; // Import the prelude for common I/O traits.

// use flate2::Compression;

// /// Compress bytes.
// fn compress() -> anyhow::Result<Vec<u8>> {
//     // `ZlibEncoder` implements the `Write` interface
//     // and takes a stream of uncompressed data, writing the compressed data
//     // to the wrapped writer.
//     let mut e =
//         flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
//     e.write_all(b"foo")?;
//     e.write_all(b"bar")?;
//     let compressed_bytes = e.finish()?;
//     println!("{:?}", compressed_bytes);
//     Ok(compressed_bytes)
// }

// /// Decompress bytes using `GzDecoder`.
// /// This function takes a vector of bytes, decompresses it, and returns the
// decompressed string. fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
//     let mut gz = flate2::read::GzDecoder::new(&bytes[..]);
//     let mut s = String::new();
//     gz.read_to_string(&mut s)?;
//     println!("{}", s);
//     Ok(s)
// }

// /// Use a buffered file to compress contents into a Vec<u8>.
// /// This function reads a file, compresses its contents, and stores the
// compressed data in a vector. fn compress_file() -> anyhow::Result<()> {
//     let f = File::open("temp/hello_world.txt")?;
//     let b = BufReader::new(f);
//     let mut z = flate2::bufread::ZlibEncoder::new(b, Compression::fast());
//     let mut buffer = Vec::new();
//     z.read_to_end(&mut buffer)?;
//     println!("{:?}", buffer);
//     Ok(())
// }

// fn main() -> anyhow::Result<()> {
//     compress()?;
//     compress_file()?;
//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [read / write a file NOW](https://github.com/john-cd/rust_howto/issues/1009)
