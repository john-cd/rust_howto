# Accessibility

[![cat~accessibility][cat~accessibility~badge]][cat~accessibility]{{hi:Accessibility}}

Assistive technology that helps overcome disabilities and impairments to make software usable by as many people as possible.

- WCAG: Essential links and level explanations.

## Tooling for Screen Readers

{{#include screen_readers.incl.md}}

## Accessible Web

[`html5ever`][c~html5ever~docs]⮳{{hi:html5ever}} / [`tl`][c~tl~docs]⮳{{hi:tl}} (HTML parsing), [`css-rs`][c~css~docs]⮳{{hi:css-rs}} (CSS parsing), [`url`][c~url~docs]⮳{{hi:url}} (URLs).

- Semantic HTML: Use elements like <article>, <nav>. [[html | HTML]].
- Image Alt Text: Good and bad examples, decorative images.
- Accessible Forms: Labels, errors, keyboard navigation.
- ARIA Essentials: When and how to use aria-label, etc.
- Color Contrast: Checking ratios, good/bad examples.
- Keyboard Navigation: Tab order, focus indicators.

See:

- [[web-programming | Web Programming]].
- [[web-programming_http-server | HTTP Server]].
- [[web_based_gui | Web-based GUI]].

## Accessible CLIs

Clear output, screen reader support. See [[command-line-interface | Command Line Interface]].

- [`termcolor`][c~termcolor~docs]⮳{{hi:termcolor}} (terminal styling), [`crossterm`][c~crossterm~docs]⮳{{hi:crossterm}} (terminal manipulation).

See [[command-line-interface | Command Line Interface]].

## Accessible Components (GUI)

Focus, keyboard, ARIA.
GUI crate itself (e.g., [`iced`][c~iced~docs]⮳{{hi:iced}}, [`egui`][c~egui~docs]⮳{{hi:egui}}).

Consult the following sections:

- [[gui | GUI]].
- [[gtk | GTK]].
- [[immediate_mode_gui | Immediate Mode GUI]].
- [[retained_mode_gui | Retained Mode GUI]].
- [[text_layout | Text Layout]].
- [[ui_layout | UI Layout]].
- [[web_based_gui | Web Based GUI]].

## Related Topics

- Language considerations: see I18n / [[internationalization | Internationalization]].
- Accessibility Testing: see [[development-tools_testing | Testing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1221)
</div>
