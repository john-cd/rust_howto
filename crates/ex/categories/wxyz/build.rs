// ANCHOR: example
fn main() {
    tonic_build::compile_protos("proto/helloworld.proto").unwrap();
}
// ANCHOR_END: example
