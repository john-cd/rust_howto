//! Build script to compile gRPC service definitions at compile time.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instruct Cargo to re-run this script if the proto file changes.
    println!("cargo:rerun-if-changed=proto/helloworld.proto");

    // Compile the helloworld.proto file.
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
