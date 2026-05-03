#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to initialize the `wgpu` GPU abstraction layer
//! without creating a window.
//!
//! It requests a GPU adapter, creates a device and queue, allocates a small
//! buffer, and submits an empty command list to validate the initialized queue.
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::task::Context;
use std::task::Poll;
use std::task::RawWaker;
use std::task::RawWakerVTable;
use std::task::Waker;

fn main() {
    block_on(async_main());
}

async fn async_main() {
    let mut instance_desc =
        wgpu::InstanceDescriptor::new_without_display_handle();
    instance_desc.backends = wgpu::Backends::PRIMARY;
    let instance = wgpu::Instance::new(instance_desc);

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find a compatible GPU adapter");

    let info = adapter.get_info();
    println!("Using adapter: {} ({:?})", info.name, info.backend);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("wgpu example device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            ..Default::default()
        })
        .await
        .expect("Failed to create device");

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Example buffer"),
        size: 4 * 4,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    println!("Created a {}-byte GPU buffer.", buffer.size());
    queue.submit(std::iter::empty());
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

fn noop_waker() -> Waker {
    unsafe fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(ptr::null(), &VTABLE)
    }

    unsafe fn wake_raw(_: *const ()) {}
    unsafe fn wake_by_ref_raw(_: *const ()) {}
    unsafe fn drop_raw(_: *const ()) {}

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone_raw, wake_raw, wake_by_ref_raw, drop_raw);

    unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &VTABLE)) }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
