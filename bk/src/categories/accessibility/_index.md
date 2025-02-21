# Accessibility {#accessibility}

[![cat-accessibility][cat-accessibility-badge]][cat-accessibility]{{hi:Accessibility}}

Assistive technology that helps overcome disabilities and impairments to make software usable by as many people as possible.

## Screen Readers

{{#include screen_readers.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- Configure clippy for better accessibility.

- Core & Tooling:
  - WCAG Basics: Essential links and level explanations.

- Link to Web: `html5ever` / `tl` (HTML parsing), `css-rs` (CSS parsing), `url` (URLs).
  - Semantic HTML: Use elements like <article>, <nav>.
  - Image Alt Text: Good and bad examples, decorative images.
  - Accessible Forms: Labels, errors, keyboard navigation.
  - ARIA Essentials: When and how to use aria-label, etc.
  - Color Contrast: Checking ratios, good/bad examples.
  - Keyboard Navigation: Tab order, focus indicators.

- General:
  - Accessible CLIs: Clear output, screen reader support.
  - Link to I18n: Language considerations.
  - Accessibility Testing Checklist: Manual and automated tests.
  - Custom Accessible Components (GUI): Focus, keyboard, ARIA.

- Link to CLI: termcolor (terminal styling), crossterm (terminal manipulation).
- Link to GUI: GUI crate itself (e.g., `iced`, `egui`).
- Link to: serde (data handling), log/tracing (logging).

</div>
