// ANCHOR: example
fn main() {
    // `tonic-build` compiles ProtoBuf files via `prost` and generates service
    // stubs and proto definitions for use with `tonic`.
    // Install `protoc` first: <https://grpc.io/docs/protoc-installation/>.
    tonic_build::compile_protos("proto/helloworld.proto").unwrap();
}
// A more complicated example may be:
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//    tonic_build::configure()
//         .build_server(false) // Disable gRPC server code generation
//         .compile_protos(
//             &["proto/helloworld.proto"],
//             &["proto"],
//         )?;
//    Ok(())
// }
// ANCHOR_END: example
