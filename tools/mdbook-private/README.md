# Local mdbook-private port

This is a repository-local port of [RealAtix/mdbook-private](https://github.com/RealAtix/mdbook-private)
0.2.3 to mdBook 0.5. Upstream uses mdBook 0.4 and cannot read the 0.5 book protocol.
The port uses `mdbook-preprocessor`, typed configuration, `Book.items`, and the public
`SectionNumber::new` constructor. It preserves removal, styling, notice, and chapter-prefix
settings. Invalid settings fail the build. Draft chapters are retained.

Build it with `just release` from `tools/`; `bk/book.toml` selects `../bin/mdbook-private`
explicitly, so an older binary on PATH cannot bypass the compatible implementation.
`just bb` builds both local preprocessors when needed.

The adapted source remains under the Mozilla Public License 2.0 (see LICENSE).
