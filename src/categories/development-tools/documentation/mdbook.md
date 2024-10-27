# mdBook

[![mdbook-github][c-mdbook-github-badge]][c-mdbook-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[mdBook][c-mdbook-github]⮳: a utility to create modern online books{{hi:Online books}} from Markdown files.

```bash
cargo install mdbook{{hi:mdbook}}
cargo install mdbook-hide # optional plugin; many others exist
```

```bash
mdbook serve --open
```

[mdBook documentation][c-mdbook-documentation]⮳

## `mdbook` plugins

[![mdbook_hide-github][c-mdbook_hide-github-badge]][c-mdbook_hide-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

- [mdbook third-party plugins][c-mdbook-third-party-plugins-wiki]
- [mdbook-private][c-mdbook_private-github]
- [mdbook-linkcheck][c-mdbook_linkcheck-github]
- A runner for `mdbook`s to keep your documentation tested: [Byron-termbook][c-termbook-github]

## CD / CI

[GitHub Actions for mdBook][actions-mdbook-github]

{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO: organize, expand

- [ ] consider [mdbook-cmdrun][c-mdbook_cmdrun-github] A mdbook preprocessor for runnning arbitrary (shell) commands in a markdown file
- [ ] consider [mdbook-journal][c-mdbook_journal-website] or [mdbook-tera][c-mdbook_tera-github] for templating
- [ ] review [yapp][c-yapp-github] A mdbook preprocessor that simply replaces text in chapters.
- [ ] review [c-mdbook_toc-github] A preprocessor for mdbook to add inline Table of Contents support.
- [ ] add page TOC ? [mdbook-theme][c-mdbook_theme-github]
- [ ] [alternative][c-mdbook_pagetoc-github]
- [ ] [mdbook-toc][c-mdbook_toc-github]

</div>
