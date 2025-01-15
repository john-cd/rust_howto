// ANCHOR: example
// ANCHOR_END: example
#![allow(dead_code)]

mod my {

    use std::marker::PhantomData;
    use std::marker::Send;
    use std::marker::Sync;

    // This struct
    // The fields are private and unreachable outside of the module.
    pub struct MyStruct<'a> {
        ptr: *const i32,
        // We want MyStruct to behave like a &i32
        phantom: PhantomData<&'a i32>,
    }

    // SAFETY: By implementing Send for MyStruct,
    // we declare it can be sent across threads safely.
    unsafe impl Send for MyStruct<'_> {}

    // SAFETY: By implementing Sync for MyStruct,
    // we declare it can be shared across threads safely.
    unsafe impl Sync for MyStruct<'_> {}

    impl<'a> MyStruct<'a> {
        pub fn new(value: &'a i32) -> Self {
            // We store the reference as a raw pointer, which is !Sync and !Send
            // by itself.
            let ptr: *const i32 = std::ptr::from_ref(value); // OR: as *const _;
            // A reference is just a pointer that is assumed to be aligned, not
            // null, and pointing to memory containing a valid value
            // of `i32` in this case. Thus the pointer made from it
            // is as well.
            assert!(!ptr.is_null());
            assert!(ptr.is_aligned());
            println!("The address is {ptr:p}");
            MyStruct {
                ptr,
                phantom: PhantomData,
            }
        }
    }

    impl Clone for MyStruct<'_> {
        fn clone(&self) -> Self {
            Self {
                ptr: self.ptr,
                phantom: PhantomData,
            }
        }
    }

    use std::ops::Deref;

    // Implement Deref to make it a smart pointer
    impl Deref for MyStruct<'_> {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            // Dereference the pointer and create a reference
            // SAFETY:
            unsafe { &*self.ptr }
        }
    }
}

fn main() {
    // FIXME
    // let value = 42;
    // // let reference = &value;
    // // println!("The addres of value is {reference:p}");

    // let my_struct = my::MyStruct::new(&value);

    // // Spawn multiple threads to demonstrate that MyStruct is Send and Sync.
    // let handles: Vec<_> = (0..10)
    //     .map(|_| {
    //         // Move the struct into the thread (via the Clone trait)
    //         let my_clone = my_struct.clone();
    //         std::thread::spawn(move || {
    //             // MyStruct dereferences to i32 (via the Deref trait)
    //             let val: i32 = *my_clone;
    //             println!("Data: {}", val);
    //             assert_eq!(val, 42);
    //         })
    //     })
    //     .collect();

    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

#[test]
fn test() {
    main();
}
// [P1 add example for Send / Sync custom impl](https://github.com/john-cd/rust_howto/issues/93)
// Implement a simplified Mutex instead?
// https://doc.rust-lang.org/src/std/sync/mutex.rs.html

// Implementing Vec: <https://doc.rust-lang.org/nomicon/vec/vec.html>
// https://nyanpasu64.gitlab.io/blog/an-unsafe-tour-of-rust-s-send-and-sync/
// https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html

// REFERENCES:
// https://doc.rust-lang.org/std/primitive.reference.html

// POINTERS:
// https://doc.rust-lang.org/reference/types/pointer.html
// https://doc.rust-lang.org/std/ptr/index.html#functions
// <https://doc.rust-lang.org/std/ptr/struct.NonNull.html>
// https://doc.rust-lang.org/std/fmt/trait.Pointer.html

// SEND / SYNC:
// <https://doc.rust-lang.org/nomicon/send-and-sync.html>
// <https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html>

// PHANTOM DATA:
// https://doc.rust-lang.org/nomicon/phantom-data.html
// https://doc.rust-lang.org/std/marker/struct.PhantomData.html
