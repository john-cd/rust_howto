mod bincode;
// [review NOW](https://github.com/john-cd/rust_howto/issues/1234)
#[cfg(target_os = "linux")]
mod capnp;
mod ciborium;
mod flatbuffers;
mod prost;
mod protobuf;
mod rmp_serde;
