//! Parsers for elements of the AST.
//!
//! The parsers do not cover all corner cases of the CommonMark spec, just what we need.

// `nom` documentation: <https://github.com/rust-bakery/nom/tree/main/doc>
// List of parsers and combinators: <https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md>
//
// A nom parser has the following signature:
//
// `fn parser(input: I) -> IResult<I, O, E>;`
//
// with `IResult` defined as:
//
// ```rust
// pub type IResult<I, O, E=(I,ErrorKind)> = Result<(I, O), Err<E>>;
//
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum Needed {
//   Unknown,
//   Size(u32)
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum Err<E> {
//   Incomplete(Needed),
//   Error(E),
//   Failure(E),
// }
// ```
//
// It can have the following values:
//
// - A correct result `Ok((I,O))` with the first element being the remaining of the input (not parsed yet), and the second the output value;
// - An error `Err(Err::Error(c))` with `c` an error that can be built from the input position and a parser specific error;
// - An error `Err(Err::Incomplete(Needed))` indicating that more input is necessary. `Needed` can indicate how much data is needed;
// - An error `Err(Err::Failure(c))`. It works like the Error case, except it indicates an unrecoverable error: We cannot backtrack and test another parser.
//
// Most of the times you can ignore the error type and use the default:
// `fn parser(input: I) -> IResult<I, O>;`.
//
// `nom` combinators like `take_while` return a function. That function is a
// parser, to which we can pass the input.

// TODO
// mod ast;
// mod autolink;
// mod code;
// mod hidden;
// mod images;
// mod link_destination;
// mod links;
// mod refdefs;
// mod url;
// mod whitespace;
