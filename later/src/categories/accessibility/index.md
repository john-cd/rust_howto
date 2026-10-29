<!-- TODO Reviewed ??-??-2026 -->

# Accessibility

[![cat~accessibility][cat~accessibility~badge]][cat~accessibility]{{hi:Accessibility}}

Accessibility means people can perceive, understand, navigate, and operate an application with different abilities and assistive technology. Treat it as a design and implementation concern from the start: use semantic controls, support keyboard operation, expose meaningful names and state, and give users alternatives to information conveyed only through color, sound, or motion.

The [Web Content Accessibility Guidelines (WCAG)](https://www.w3.org/WAI/standards-guidelines/wcag/) provide testable guidance for web content. Their principles also help when reviewing desktop, terminal, and embedded interfaces, though platform accessibility APIs and requirements differ.

## Web Content

Use native HTML elements for their intended purpose, preserve heading and landmark structure, associate form labels and errors with controls, and provide text alternatives for informative images. Make every action available from a keyboard and keep focus visible and in a logical order. Use ARIA only when native semantics do not express a control's role, name, value, or state; redundant or incorrect ARIA can make an interface harder to use.

When generating or inspecting HTML in Rust, [`html5ever`][c~html5ever~docs]↗{{hi:html5ever}} and [`tl`][c~tl~docs]↗{{hi:tl}} parse HTML, [`css`][c~css~docs]↗{{hi:css}} parses CSS, and [`url`][c~url~docs]↗{{hi:url}} handles URLs. Use these to check generated markup, for example to confirm that images have `alt` text and form controls have labels.

See Web Programming<!-- Cross-book topic: bk/drafts/categories/web-programming/index.md; not included in this preview. -->, HTTP Server<!-- Cross-book topic: bk/drafts/categories/web-programming_http-server/index.md; not included in this preview. --> and HTML<!-- Cross-book topic: bk/drafts/categories/parser-implementations/html.md; not included in this preview. -->.

## Desktop, Mobile, and Embedded GUIs

A visual widget is not automatically available to assistive technology. Each interactive element needs a useful accessible name, role, value, and state, and updates such as validation errors or progress should be announced when appropriate. Check that keyboard focus follows the visual hierarchy and that text scaling, contrast, and reduced-motion preferences remain usable.

GUI crates such as [`iced`][c~iced~docs]↗{{hi:iced}} and [`egui`][c~egui~docs]↗{{hi:egui}} can build responsive interfaces; check their current accessibility support against your requirements.

[`accesskit`][c~accesskit~docs]↗{{hi:accesskit}} provides a cross-platform representation of UI semantics that GUI frameworks can connect to platform accessibility APIs. Framework support varies; verify the framework's current integration and test the built application with the target platform's accessibility tools.

{{#include screen_readers.incl.md}}

See [GUI](../gui/index.md), [GTK](../gui/gtk.md), [Immediate Mode GUI](../gui/immediate_mode_gui.md), [Retained Mode GUI](../gui/retained_mode_gui.md), [Text Layout](../gui/text_layout.md), [UI Layout](../gui/ui_layout.md), and [Web-based GUI](../gui/web_based_gui.md).

## Command-line Interfaces

Make output understandable when read aloud or redirected: use clear labels and stable ordering, keep errors distinct from normal output, and avoid relying on color, cursor position, or decorative characters to communicate meaning. Provide a way to disable color and ensure prompts can be answered without a mouse. Terminal styling crates such as [`termcolor`][c~termcolor~docs]↗{{hi:termcolor}} and [`crossterm`][c~crossterm~docs]↗{{hi:crossterm}} help control presentation, but do not replace accessible wording and behavior.

See Command Line Interface<!-- Cross-book topic: bk/drafts/categories/command-line-interface/index.md; not included in this preview. -->.

## Testing and Review

Automated checks can catch some structural problems, but they do not establish that a complete workflow is accessible. Test important flows with a keyboard alone, inspect focus and text scaling, and use a screen reader on each supported platform. Include people who use assistive technology in usability reviews when possible, and repeat checks after changing navigation or controls.

See Testing<!-- Cross-book topic: bk/drafts/categories/development-tools_testing/index.md; not included in this preview. -->. For language and locale support, see [Internationalization](../internationalization/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
