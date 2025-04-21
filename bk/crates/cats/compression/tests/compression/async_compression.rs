// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use std::io::Result as IoResult;

// use bytes::Bytes;
// use flate2::Compression;
// use flate2::read::GzDecoder;
// use flate2::write::GzEncoder;
// use futures::stream::StreamExt;
// use futures::stream::TryStreamExt;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
// use tokio::io::AsyncWriteExt;
// use tokio_util::codec::BytesCodec;
// use tokio_util::codec::FramedRead;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let input_filename = "temp/uncompressed.txt";
//     let compressed_filename = "temp/compressed.gz";
//     let decompressed_filename = "temp/decompressed.txt";

//     // 1. Asynchronously create an uncompressed file.
//     let mut input_file = File::create(input_filename).await?;
//     input_file.write_all(b"This is some sample data.\n").await?;
//     input_file.write_all(b"It has multiple lines.\n").await?;
//     drop(input_file);

//     println!("Created '{}'", input_filename);

//     // 2. Asynchronously compress the file.
//     println!(
//         "Compressing '{}' to '{}'...",
//         input_filename, compressed_filename
//     );
//     compress_file_async(input_filename, compressed_filename).await?;
//     println!("Compression complete.");

//     // 3. Asynchronously decompress the file.
//     println!(
//         "Decompressing '{}' to '{}'...",
//         compressed_filename, decompressed_filename
//     );
//     decompress_file_async(compressed_filename, decompressed_filename).await?;
//     println!("Decompression complete.");

//     // 4. Verify the decompressed content.
//     let original_content = tokio::fs::read_to_string(input_filename).await?;
//     let decompressed_content =
//         tokio::fs::read_to_string(decompressed_filename).await?;

//     if original_content == decompressed_content {
//         println!("Decompressed content matches the original.");
//     } else {
//         eprintln!("Error: Decompressed content does NOT match the
// original!");     }

//     Ok(())
// }

// async fn compress_file_async(
//     source_path: &str,
//     dest_path: &str,
// ) -> IoResult<()> {
//     // Asynchronously opens the file for reading.
//     let source_file = File::open(source_path).await?;
//     // Creates a `FramedRead` around the source file with `BytesCodec` to
// read     // data in chunks of Bytes. The `FramedRead` turns the asynchronous
// File     // (which implements `AsyncRead`) into an asynchronous stream (`impl
//     // Stream<Item = Result<Bytes, io::Error>>`).
//     let mut reader = FramedRead::new(source_file, BytesCodec::new());

//     let dest_file = File::create(dest_path).await?;
//     let mut encoder = GzEncoder::new(dest_file, Compression::default());

//     // Iterates through the asynchronous stream of bytes from the source file
//     // using `reader.next().await`. `reader` implements the Stream trait from
//     // the futures crate. `.next()` asynchronously waits for the next item (a
//     // `Result<Bytes, io::Error>`) from the stream. It returns
//     // `Some(Result<Bytes, io::Error>)` if an item is available and `None`
//     // when the stream is exhausted (end of file).
//     while let Some(result) = reader.next().await {
//         let bytes = result?;
//         // For each chunk of bytes read, it asynchronously writes the chunk
// to         // the `GzEncoder` using `encoder.write_all().await?`. The
// `GzEncoder`         // handles the compression internally as data is written
// to it.         encoder.write_all(&bytes).await?;
//     }

//     // Ensure that all buffered compressed data is written to the destination
//     // file and the gzip stream is properly finalized.
//     encoder.finish().await?;
//     Ok(())
// }

// async fn decompress_file_async(
//     source_path: &str,
//     dest_path: &str,
// ) -> IoResult<()> {
//     let source_file = File::open(source_path).await?;
//     // Creates a `GzDecoder` wrapped around the source file. `GzDecoder` from
//     // `flate2::read` implements `AsyncRead`.
//     let decoder = GzDecoder::new(source_file);
//     // Creates a `FramedRead` around the `GzDecoder` with `BytesCodec` to
// read     // the decompressed data in chunks.
//     let mut reader = FramedRead::new(decoder, BytesCodec::new());

//     let dest_file = File::create(dest_path).await?;
//     let mut writer = dest_file;

//     while let Some(result) = reader.next().await {
//         let bytes = result?;
//         writer.write_all(&bytes).await?;
//     }

//     Ok(())
// }
