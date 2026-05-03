#![allow(dead_code)]
// ANCHOR: example
//! A no-alloc example using `core` only.
//!
//! This example runs in a `#![no_std]` context and formats text into a
//! fixed-size stack buffer without heap allocations.
use core::fmt;
use core::fmt::Write;

fn main() {
    let mut buffer = ArrayString::new();
    write!(
        &mut buffer,
        "no_alloc core formatting: {} + {} = {}",
        2,
        3,
        2 + 3
    )
    .unwrap();

    assert_eq!(buffer.as_str(), "no_alloc core formatting: 2 + 3 = 5");
}

struct ArrayString {
    buf: [u8; 64],
    len: usize,
}

impl ArrayString {
    const fn new() -> Self {
        Self {
            buf: [0; 64],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

impl Write for ArrayString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let new_len = self.len + bytes.len();
        if new_len > self.buf.len() {
            return Err(fmt::Error);
        }
        self.buf[self.len..new_len].copy_from_slice(bytes);
        self.len = new_len;
        Ok(())
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
}
