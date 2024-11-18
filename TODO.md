# TODO

- [ ] review each .md file in turn - WIP
- [ ] add examples where needed - WIP
- [ ] replace current badge markdown mess by a preprocessor command
- [ ] review unused reference definitions (without corresponding links in the markdown) - WIP
- [ ] modify index of crates to point to the relevant recipes
- [ ] add badge refdefs for links to RBE book in lang / standard_lib; use `just templ rbe` command - WIP

## Tools

- [ ] finish mdbook preprocessor - {#badge <crate_name> <extra_cat1>...}, {#categories <cat1>,<cat2>... }
- [ ] finish autogen and tool_lib/src/tera --> generate index of examples? recipe tables? new subchapter? - WIP
- [ ] finish git hook setup with cargo husky - why are they not installed?
- [ ] consolidate templ / crate_indices / autogen / mdbook-utils tools and respective libs

## Links

## Markdown

## Examples

- [ ] fix commented examples - listen_unused, backtrace, rate_limited, paginated
- [ ] rewrite rest_post.rs so that a username and password are not required?
- [ ] finish to review ignore / no_run examples - add noplayground if playground does not have the dependency
- [ ] Make addt'l examples out of mdbook-utils, crate_indices, templ, clean, autogen...
- [ ] integrate clap builder example
- [ ] fix failing examples in deps/drafts
- [ ] tower_http example polish; other examples have been checked against the rust playground
- [ ] fix leaky tests when using nextest on Windows
- [ ] add shuttle.rs example

## Others

- [ ] pull request to little book of rust books
- [ ] Consider adding project to [goodfirstissue.dev][goodfirstissue-website]
- [ ] review GA / GSC - issues with redirects??
- [ ] license
- [ ] add licenses to thanks page ?
- [ ] update thanks
- [ ] Search within the book improvements - see search.md; Algolia
- [ ] explore bacon / review bacon.toml
- [ ] review deny.toml
- Contributing
- Code of conduct
