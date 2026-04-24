# Accessibility

[![cat~accessibility][cat~accessibility~badge]][cat~accessibility]{{hi:Accessibility}}

Assistive technology that helps overcome disabilities and impairments to make software usable by as many people as possible.
Implementing accessibility ensures that applications are usable by a wider audience, including those who rely on screen readers, keyboard navigation, and other assistive tools.
Following guidelines like the [Web Content Accessibility Guidelines (WCAG)](https://www.w3.org/WAI/standards-guidelines/wcag/) provides essential principles and compliance levels for creating accessible content.

## Tooling for Screen Readers

{{#include screen_readers.incl.md}}

## Accessible Web

When building web applications or generating HTML in Rust, ensuring semantic markup and correct structure is fundamental to web accessibility.
Libraries such as [`html5ever`][c~html5ever~docs]↗{{hi:html5ever}} or [`tl`][c~tl~docs]↗{{hi:tl}} can be used for HTML parsing, [`css-rs`][c~css~docs]↗{{hi:css-rs}} for CSS parsing, and [`url`][c~url~docs]↗{{hi:url}} for URLs.

When rendering HTML, it is important to generate output that supports accessibility features:
- **Semantic HTML**: Use semantic elements like \`<article>\` and \`<nav>\` rather than non-semantic \`<div>\` or \`<span>\` elements to give structure and meaning to the content. See [[html | HTML]].
- **Image Alt Text**: Provide appropriate descriptive text for images using \`alt\` attributes, and use empty strings (\`alt=""\`) for purely decorative images.
- **Accessible Forms**: Ensure all form inputs have associated \`<label>\` elements, clearly display errors, and support keyboard navigation.
- **ARIA Essentials**: Use Accessible Rich Internet Applications (ARIA) attributes like \`aria-label\` or \`aria-hidden\` when standard HTML elements are insufficient to describe an element's role or state.
- **Color Contrast**: Verify color contrast ratios to ensure text remains readable for users with visual impairments.
- **Keyboard Navigation**: Ensure a logical tab order and visible focus indicators for all interactive elements.

See:

- [[web-programming | Web Programming]].
- [[web-programming_http-server | HTTP Server]].
- [[web_based_gui | Web-based GUI]].

## Accessible CLIs

Command-line interfaces should also be designed with accessibility in mind. Clear output and appropriate screen reader support are vital. Applications can improve accessibility by avoiding excessive or unlabelled ASCII art, providing well-structured text, and ensuring color schemes are readable or configurable.

Crates for styling and manipulating the terminal include:
- [`termcolor`][c~termcolor~docs]↗{{hi:termcolor}} for terminal styling.
- [`crossterm`][c~crossterm~docs]↗{{hi:crossterm}} for cross-platform terminal manipulation.

See [[command-line-interface | Command Line Interface]].

## Accessible Components (GUI)

Graphical User Interfaces in Rust often utilize crates that natively support or are integrating accessibility features such as focus management, keyboard navigation, and screen reader announcements.
GUI crates like [`iced`][c~iced~docs]↗{{hi:iced}} and [`egui`][c~egui~docs]↗{{hi:egui}} provide mechanisms to build responsive and accessible user interfaces. The [`accesskit`][c~accesskit~docs]↗{{hi:accesskit}} crate is a widely adopted abstraction that helps GUI frameworks integrate natively with platform accessibility APIs.

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
