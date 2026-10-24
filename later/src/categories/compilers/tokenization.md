# Tokenizing Input with Logos

A lexer turns source text into a sequence of tokens that a parser can consume. [logos][c~logos~docs]↗{{hi:logos}} derives an efficient lexer from token definitions and regular expressions.

The example recognizes identifiers and an addition operator, skips spaces, and asserts the token sequence.

{{#include ../../../crates/cats/compilers/examples/tokenization/tokenization.rs:example}}
