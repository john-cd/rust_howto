# mdBook

{{#include mdbook.incl.md}}

## mdBook {#mdbook}

[![mdbook-github][c-mdbook-github-badge]][c-mdbook-github]{{hi:mdbook}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[mdBook][c-mdbook-github]⮳: a utility to create modern online books{{hi:Online books}} from Markdown files.

```bash
cargo install mdbook{{hi:mdbook}}
cargo install mdbook-hide # optional plugin; many others exist
```

```bash
mdbook serve --open
```

## Playground {#playground}

[Playground (Rust by example)][book-rust-by-example-playground]{{hi:Rust by example}}⮳ [![Rust by example - Playground][book-rust-by-example-playground-badge]][book-rust-by-example-playground] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

## `mdbook` plugins {#mdbook-plugins}

[![mdbook_hide-github][c-mdbook_hide-github-badge]][c-mdbook_hide-github]{{hi:mdbook-hide}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

- [mdbook third-party plugins][c-mdbook-third-party-plugins-wiki]
- [mdbook-private][c-mdbook_private-github]{{hi:mdbook-private}}
- [mdbook-linkcheck][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}
- A runner for `mdbook`s to keep your documentation tested: [Byron-termbook][c-termbook-github]{{hi:termbook}}

## CD / CI {#cd-ci}

[GitHub Actions for mdBook][actions-mdbook-github]{{hi:Github actions}}

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO P1 organize, expand

- [ ] consider [mdbook-cmdrun][c-mdbook_cmdrun-github]{{hi:mdbook-cmdrun}} A mdbook preprocessor for runnning arbitrary (shell) commands in a markdown file
- [ ] consider [mdbook-journal][c-mdbook_journal-website]{{hi:mdbook-journal}} or [mdbook-tera][c-mdbook_tera-github]{{hi:mdbook-tera}} for templating
- [ ] review [c-mdbook_toc-github]{{hi:mdbook-toc}} A preprocessor for mdbook to add inline Table of Contents support.
- [ ] add page TOC ? [mdbook-theme][c-mdbook_theme-github]{{hi:mdbook-theme}}
- [ ] [alternative][c-mdbook_pagetoc-github]{{hi:mdbook-pagetoc}}
- [ ] [mdbook-toc][c-mdbook_toc-github]{{hi:mdbook-toc}}

## `yapp` {#yapp}

[![mdbook-yapp][c-mdbook_yapp-badge]][c-mdbook_yapp] [![mdbook-yapp-crates.io][c-mdbook_yapp-crates.io-badge]][c-mdbook_yapp-crates.io] [![mdbook-yapp-github][c-mdbook_yapp-github-badge]][c-mdbook_yapp-github] [![mdbook-yapp-lib.rs][c-mdbook_yapp-lib.rs-badge]][c-mdbook_yapp-lib.rs]{{hi:mdbook-yapp}}{{hi:Text}}{{hi:Preprocessor}}{{hi:Mdbook}}{{hi:Replace}}{{hi:Pattern}}[![cat-template-engine][cat-template-engine-badge]][cat-template-engine]{{hi:Template engine}}[![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

A mdBook preprocessor for simple text replacements

</div>
