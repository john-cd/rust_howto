# Category Badges

TODO: NOT IMPLEMENTED

The `rust_howto` book uses custom directives to insert category badges.

Any left-over directives should be scrubbed by `mdbook-scrub` if the corresponding
configuration toggle is set (which is the default).


- {{!cat mathematics}} -> [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}
- {{!cat no-std}} -> [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
- {{!cat no_std}}
- {{!cat mathematics }}
- {{ !cat mathematics}}
- {{!cat: mathematics}}
- {{!cat : mathematics}}

## Category Badges with Multiple categories

- {{!cat mathematics science }}
