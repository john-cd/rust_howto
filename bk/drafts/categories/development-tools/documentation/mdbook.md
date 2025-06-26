# `mdBook`

{{#include mdbook.incl.md}}

## Write Online Books with `mdBook` {#mdbook}

[![mdbook~github][c~mdbook~github~badge]][c~mdbook~github]{{hi:mdbook}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]

[mdBook][c~mdbook~github]⮳ is a utility to create modern online [books][p~books]{{hi:Online books}} from Markdown files.

```bash
cargo install mdbook
```

```bash
mdbook serve --open
```

## Let Readers Execute Sample Code in the Rust Playground {#playground}

[![Rust by example - Playground][book~rust-by-example~playground~badge]][book~rust-by-example~playground] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[Playground (Rust by example)][book~rust-by-example~playground]{{hi:Rust by example}}⮳.

## Augment `mdbook` with Plugins {#skip1}

[`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} has a large number of [third-party plugins][c~mdbook~third-party-plugins~wiki]⮳.

### Check Links with `mdbook-linkcheck` {#mdbook-linkcheck}

[![mdbook-linkcheck][c~mdbook_linkcheck~docs~badge]][c~mdbook_linkcheck~docs] [![mdbook-linkcheck~crates.io][c~mdbook_linkcheck~crates.io~badge]][c~mdbook_linkcheck~crates.io] [![mdbook-linkcheck~github][c~mdbook_linkcheck~github~badge]][c~mdbook_linkcheck~github] [![mdbook-linkcheck~lib.rs][c~mdbook_linkcheck~lib.rs~badge]][c~mdbook_linkcheck~lib.rs]{{hi:mdbook-linkcheck}}

[`mdbook-linkcheck`][c~mdbook_linkcheck~docs]⮳{{hi:mdbook-linkcheck}} is a backend for [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}}, which will check your links for you.

### Hide Entire Chapters with `mdbook-private` {#mdbook-private}

[![mdbook-private][c~mdbook_private~docs~badge]][c~mdbook_private~docs] [![mdbook-private~crates.io][c~mdbook_private~crates.io~badge]][c~mdbook_private~crates.io] [![mdbook-private~github][c~mdbook_private~github~badge]][c~mdbook_private~github] [![mdbook-private~lib.rs][c~mdbook_private~lib.rs~badge]][c~mdbook_private~lib.rs]{{hi:mdbook-private}}

[`mdbook-private`][c~mdbook_private~docs]⮳{{hi:mdbook-private}} is a [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} pre-processor that controls visibility of private chapters and sections within them.

### Hide Pages with `mdbook-hide` {#mdbook-hide}

[![mdbook-hide][c~mdbook_hide~docs~badge]][c~mdbook_hide~docs] [![mdbook-hide~crates.io][c~mdbook_hide~crates.io~badge]][c~mdbook_hide~crates.io] [![mdbook-hide~github][c~mdbook_hide~github~badge]][c~mdbook_hide~github] [![mdbook-hide~lib.rs][c~mdbook_hide~lib.rs~badge]][c~mdbook_hide~lib.rs]{{hi:mdbook-hide}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]

[`mdbook-hide`][c~mdbook_hide~docs]⮳{{hi:mdbook-hide}} is a pre-processor for [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} that adds support for hidden chapters.

```bash
cargo install mdbook-hide
```

## Deploy Your Book or Documentation via a CD / CI Pipeline {#cd-ci}

`GitHub Actions` is a continuous integration and continuous delivery (CI/CD) platform that allows you to automate your build, test, and deployment pipeline.

[GitHub Actions for mdBook][actions-mdbook~github]{{hi:Github actions}} allows you to build your site with [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}}. Linux (Ubuntu), macOS, and Windows are supported.

```yaml
name: github pages

on:
  push:
    branches:
      - main
  pull_request:

jobs:
  deploy:
    runs-on: ubuntu-20.04
    concurrency:
      group: ${{ github.workflow }}-${{ github.ref }}
    steps:
      - uses: actions/checkout@v2

      - name: Setup mdBook
        uses: peaceiris/actions-mdbook@v2
        with:
          mdbook-version: '0.4.10'
          # mdbook-version: 'latest'

      - run: mdbook build

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        if: ${{ github.ref == 'refs/heads/main' }}
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./book
```

## Add a Table of Contents to Each Page {#skip1}

### `mdbook-toc` {#mdbook-toc}

[![mdbook-toc][c~mdbook_toc~docs~badge]][c~mdbook_toc~docs] [![mdbook-toc~crates.io][c~mdbook_toc~crates.io~badge]][c~mdbook_toc~crates.io] [![mdbook-toc~github][c~mdbook_toc~github~badge]][c~mdbook_toc~github] [![mdbook-toc~lib.rs][c~mdbook_toc~lib.rs~badge]][c~mdbook_toc~lib.rs]{{hi:mdbook-toc}}

[`mdbook-toc`][c~mdbook_toc~docs]⮳{{hi:mdbook-toc}} is a pre-processor for [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} to add inline Table of Contents support.

### `mdbook-theme` {#mdbook-theme}

[![mdbook-theme][c~mdbook_theme~docs~badge]][c~mdbook_theme~docs] [![mdbook-theme~crates.io][c~mdbook_theme~crates.io~badge]][c~mdbook_theme~crates.io] [![mdbook-theme~github][c~mdbook_theme~github~badge]][c~mdbook_theme~github] [![mdbook-theme~lib.rs][c~mdbook_theme~lib.rs~badge]][c~mdbook_theme~lib.rs]{{hi:mdbook-theme}}{{hi:Ace}}{{hi:Book}}{{hi:Markdown}}{{hi:Rustbook}}{{hi:Theme}}

[`mdbook-theme`][c~mdbook_theme~docs]⮳{{hi:mdbook-theme}} is a pre-processor and a backend to config theme for mdbook, especially creating a pagetoc on the right and setting full color themes from the official ace editor.

### `mdbook-pagetoc` {#mdbook-pagetoc}

[![mdbook-pagetoc][c~mdbook_pagetoc~docs~badge]][c~mdbook_pagetoc~docs] [![mdbook-pagetoc~crates.io][c~mdbook_pagetoc~crates.io~badge]][c~mdbook_pagetoc~crates.io] [![mdbook-pagetoc~github][c~mdbook_pagetoc~github~badge]][c~mdbook_pagetoc~github] [![mdbook-pagetoc~lib.rs][c~mdbook_pagetoc~lib.rs~badge]][c~mdbook_pagetoc~lib.rs]{{hi:mdbook-pagetoc}}{{hi:Toc}}{{hi:Table}}{{hi:Pagetoc}}{{hi:Mdbook}}{{hi:Contents}}

[`mdbook-pagetoc`][c~mdbook_pagetoc~docs]⮳{{hi:mdbook-pagetoc}} is a [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} plugin that provides a table of contents for each page.

## Create Pages from a Template {#skip2}

### Create Pages from a Template with `mdbook-tera` {#mdbook-tera}

[![mdbook-tera][c~mdbook_tera~docs~badge]][c~mdbook_tera~docs] [![mdbook-tera~crates.io][c~mdbook_tera~crates.io~badge]][c~mdbook_tera~crates.io] [![mdbook-tera~github][c~mdbook_tera~github~badge]][c~mdbook_tera~github] [![mdbook-tera~lib.rs][c~mdbook_tera~lib.rs~badge]][c~mdbook_tera~lib.rs]{{hi:mdbook-tera}}{{hi:Pre-processor}}{{hi:Tera}}{{hi:Mdbook}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

[`mdbook-tera`][c~mdbook_tera~docs]⮳{{hi:mdbook-tera}} is a Tera pre-processor for mdBook.

### Replace Text in Chapters with `yapp` {#yapp}

[![mdbook-yapp][c~mdbook_yapp~docs~badge]][c~mdbook_yapp~docs] [![mdbook-yapp~crates.io][c~mdbook_yapp~crates.io~badge]][c~mdbook_yapp~crates.io] [![mdbook-yapp~github][c~mdbook_yapp~github~badge]][c~mdbook_yapp~github] [![mdbook-yapp~lib.rs][c~mdbook_yapp~lib.rs~badge]][c~mdbook_yapp~lib.rs]{{hi:mdbook-yapp}}{{hi:Text}}{{hi:Pre-processor}}{{hi:Mdbook}}{{hi:Replace}}{{hi:Pattern}}[![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}[![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`yapp`][c~yapp~docs]⮳{{hi:yapp}} is a [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} pre-processor that simply replaces text in chapters. Phrases to be replaced with specified content are defined in plain-text configuration file.

### Create a Journal with `mdbook-journal` {#mdbook-journal}

[![mdbook-journal~website][c~mdbook_journal~website~badge]][c~mdbook_journal~website] [![mdbook-journal][c~mdbook_journal~docs~badge]][c~mdbook_journal~docs] [![mdbook-journal~crates.io][c~mdbook_journal~crates.io~badge]][c~mdbook_journal~crates.io] [![mdbook-journal~github][c~mdbook_journal~github~badge]][c~mdbook_journal~github] [![mdbook-journal~lib.rs][c~mdbook_journal~lib.rs~badge]][c~mdbook_journal~lib.rs]{{hi:mdbook-journal}}{{hi:Book}}{{hi:Gitbook}}{{hi:Markdown}}{{hi:Mdbook}}{{hi:Plugin}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~template-engine][cat~template-engine~badge]][cat~template-engine]{{hi:Template engine}}

[`mdbook-journal`][c~mdbook_journal~docs]⮳{{hi:mdbook-journal}} is a journal plugin for [`mdBook`][c~mdbook~docs]⮳{{hi:mdBook}}.

## Other {#skip3}

### Preprocess your Book via Arbitrary Commands with `mdbook-cmdrun` {#mdbook-cmdrun}

[![mdbook-cmdrun][c~mdbook_cmdrun~docs~badge]][c~mdbook_cmdrun~docs] [![mdbook-cmdrun~crates.io][c~mdbook_cmdrun~crates.io~badge]][c~mdbook_cmdrun~crates.io] [![mdbook-cmdrun~github][c~mdbook_cmdrun~github~badge]][c~mdbook_cmdrun~github] [![mdbook-cmdrun~lib.rs][c~mdbook_cmdrun~lib.rs~badge]][c~mdbook_cmdrun~lib.rs]{{hi:mdbook-cmdrun}}{{hi:Mdbook}}{{hi:Pre-processor}}{{hi:Runcmd}}

[`mdbook-cmdrun`][c~mdbook_cmdrun~docs]⮳{{hi:mdbook-cmdrun}} is a [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} pre-processor to run arbitrary commands.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[mdbook: organize, expand NOW](https://github.com/john-cd/rust_howto/issues/299)

- [mdslides][mdslides]

[mdslides]: https://github.com/ferrous-systems/mdslides

## Test Code in Your `mdbook` {#termbook}

[Byron-termbook][c~termbook~github]{{hi:termbook}} is a runner for [`mdbook`][c~mdbook~docs]⮳{{hi:mdbook}} to keep your documentation tested:

## Alternatives {#skip}

[`mdbook-hide`][c~mdbook_hide~github]{{hi:mdbook-hide}}⮳ hides chapters under construction. Install with `cargo install mdbook-hide`. Configure in [`book.toml`][c~mdbook~book.toml]⮳{{hi:book.toml}}.

```toml
# Adds support for hidden chapters. The hidden chapters can be marked by adding a special Markdown comment: <!--hidden-->
# https://github.com/ankitrgadiya/mdbook-hide/
[preprocessor.hide]
hide = true
```

To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`mdbook-keeper`][c~mdbook_keeper~crates.io]{{hi:mdbook-keeper}}⮳. Install with:

```bash
cargo install mdbook_keeper --git https://github.com/tfpk/mdbook_keeper.git
```

</div>
