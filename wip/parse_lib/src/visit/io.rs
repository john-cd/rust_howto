//! Conversion from `fmt::Write` to `io::Write`.
//!
//! Provides utilities to adapt `fmt::Write` interfaces to `io::Write` streams.

use std::io;
use std::io::BufWriter;

use fmt2io::Writer;

use super::Visitor;
use crate::ast::*;

impl<'d> Document<'d> {
    /// Adapt from `fmt::Write` to `io::Write`.
    pub fn to_io<'v, IO, F, V>(&'d self, mut io_writer: IO, f: F) -> io::Result<()>
    where
        IO: io::Write,
        V: Visitor<'v>,
        F: FnOnce(&mut Writer<&mut IO>) -> V,
        'd: 'v,
    {
        // The `write` function takes a `io::Write`, converts it to a struct that implements `fmt::Write`
        // and provides it to the closure.
        fmt2io::write(&mut io_writer, |writer| {
            let mut visitor = f(writer);
            self.accept(&mut visitor)?;
            Ok(())
        })?;
        // Surface any I/O errors:
        io_writer.flush()?;
        Ok(())
    }

    /// Create a buffer writer from an IO writer,
    /// convert to fmt::Write, pass to a closure that returns a visitor,
    /// accept the visitor.
    pub fn with_buffer<'v, IO, F, V>(
        &'d self,
        capacity: usize,
        inner_io_writer: IO,
        f: F,
    ) -> io::Result<()>
    where
        IO: io::Write,
        V: Visitor<'v>,
        F: FnOnce(&mut Writer<&mut BufWriter<IO>>) -> V,
        'd: 'v,
    {
        use io::BufWriter;
        use io::Write;
        let mut io_writer = BufWriter::with_capacity(capacity, inner_io_writer);
        fmt2io::write(&mut io_writer, |writer| {
            let mut visitor = f(writer);
            self.accept(&mut visitor)?;
            Ok(())
        })?;
        io_writer.flush()?;
        Ok(())
    }

    /// Adapt from `fmt::Write` to `stdout`.
    /// Accept the visitor returned by the closure.
    pub fn to_stdout<'v, F, V>(&'d self, f: F) -> io::Result<()>
    where
        'd: 'v,
        V: Visitor<'v>,
        F: FnOnce(&mut Writer<&mut io::Stdout>) -> V,
    {
        let io_writer = io::stdout();
        self.to_io(io_writer, f)
    }
}
