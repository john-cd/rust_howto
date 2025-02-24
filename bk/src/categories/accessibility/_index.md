# Accessibility {#accessibility}

[![cat-accessibility][cat-accessibility-badge]][cat-accessibility]{{hi:Accessibility}}

Assistive technology that helps overcome disabilities and impairments to make software usable by as many people as possible.

## Screen Readers

{{#include screen_readers.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

Configure `clippy` for better accessibility.

## Core & Tooling

- WCAG Basics: Essential links and level explanations.

## Web

[`html5ever`][c-html5ever]⮳{{hi:html5ever}} / [`tl`][c-tl]⮳{{hi:tl}} (HTML parsing), [`css-rs`][c-css]⮳{{hi:css-rs}} (CSS parsing), [`url`][c-url]⮳{{hi:url}} (URLs).

[[web-programming | Web Programming]]
[[web-programming_http-server | HTTP Server]]
[[web_based_gui | Web Based GUI]]

- Semantic HTML: Use elements like <article>, <nav>. [[html | HTML]]
- Image Alt Text: Good and bad examples, decorative images.
- Accessible Forms: Labels, errors, keyboard navigation.
- ARIA Essentials: When and how to use aria-label, etc.
- Color Contrast: Checking ratios, good/bad examples.
- Keyboard Navigation: Tab order, focus indicators.

## General

- Accessible CLIs: Clear output, screen reader support. [[command-line-interface | Command Line Interface]]
- I18n: Language considerations. [[internationalization | Internationalization]]
- Accessibility Testing Checklist: Manual and automated tests.
- Custom Accessible Components (GUI): Focus, keyboard, ARIA. [[gui | GUI]]

## CLI

`termcolor` (terminal styling), `crossterm` (terminal manipulation).

## GUI

GUI crate itself (e.g., [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}).

## See also

`serde` (data handling) [[encoding | Encoding]]

`log`/`tracing` (logging)
[[log | Log]]
[[tracing | Tracing]]
[[tracing_alternatives | Tracing Alternatives]]

</div>
