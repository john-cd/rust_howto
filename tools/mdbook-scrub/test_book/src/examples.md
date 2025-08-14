# Example Directive

The `rust_howto` book uses custom directives to insert examples.

Any left-over directives should be scrubbed by `mdbook-scrub`
if the corresponding configuration toggle is set (which is the default).

- {{#example some_example}}
- {{#example some_example }}
- {{# example some_example}}
- {{ #example some_example}}

- Incomplete directives:
  - {{#example}}
  - {{#example }}
