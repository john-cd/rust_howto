//! Build script to compile gRPC service definitions at compile time.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instruct Cargo to re-run this script if the proto file changes.
    println!("cargo:rerun-if-changed=proto/helloworld.proto");

    let protoc = protoc_bin_vendored::protoc_bin_path()?; // TODO review
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    // Compile the helloworld.proto file.
    tonic_prost_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
