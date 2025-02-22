# Template Engine

[![cat-template-engine][cat-template-engine-badge]][cat-template-engine]{{hi:Template engines}}

Crates designed to combine templates with data to produce result documents, usually with an emphasis on processing text.

## Create HTML files from a template

{{#include tera.incl.md}}

## Create Markdown fragments from a template

{{#include tinytemplate.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[template-engine/index: write (P1)](https://github.com/john-cd/rust_howto/issues/482)

## Rust Crates for Template Engines

This table lists popular Rust template engines and categorizes them by their approach and features.

| Template Engine | Description | Key Features | Crate Name |
|---|---|---|---|
| String-based Templating (Simple) | Focuses on string interpolation and simple logic within templates. | Easy to learn, good for basic templating. | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}} (can also be used for more advanced cases) |
| String-based Templating (Advanced) | Offers more advanced features like control flow, loops, and expressions. | More powerful, but can be more complex. | [`tera`][c-tera]⮳{{hi:tera}}, [`handlebars`][c-handlebars]⮳{{hi:handlebars}} |
| Type-safe Templating | Leverages Rust's type system for compile-time checking of templates. | Improved safety and reduced runtime errors. | [`askama`][c-askama]⮳{{hi:askama}}, [`maud`][c-maud]⮳{{hi:maud}} |
| HTML Templating | Specifically designed for generating HTML. | Often includes features like HTML escaping and code formatting. | [`maud`][c-maud]⮳{{hi:maud}}, [`yew`][c-yew]⮳{{hi:yew}} (components can act like templates), [`perseus`][c-perseus]⮳{{hi:perseus}} (for static site generation) |
| Markup Templating (General) | Can be used for various markup languages (HTML, XML, etc.). | Flexible and extensible. | [`markup`][c-markup]⮳{{hi:markup}} |
| Logic-less Templating | Emphasizes separation of concerns by minimizing logic within templates. | Cleaner templates, often used with more complex logic handled in Rust code. | [`handlebars`][c-handlebars]⮳{{hi:handlebars}}, [`minijinja`][c-minijinja]⮳{{hi:minijinja}} (can be used in a logic-less way) |
| High-Performance Templating | Optimized for speed and efficiency. | Suitable for applications where performance is critical. | [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}} (benchmarks show good performance) |
| Compile-time Templating | Templates are processed at compile time, reducing runtime overhead. | Faster rendering, but requires recompilation for template changes. | [`askama`][c-askama]⮳{{hi:askama}}, [`maud`][c-maud]⮳{{hi:maud}} |

## Crate Details

* [`minijinja`][c-minijinja]⮳{{hi:minijinja}}: A lightweight and fast templating engine inspired by Jinja2. Good balance of features and performance. Suitable for both simple and more complex scenarios.
* [`tera`][c-tera]⮳{{hi:tera}}: A powerful and flexible templating engine with support for various features, including custom functions and filters. Can be used for both simple and advanced templating needs.
* [`handlebars`][c-handlebars]⮳{{hi:handlebars}}: A logic-less templating engine inspired by Handlebars.js. Good for separating presentation from logic.
* [`askama`][c-askama]⮳{{hi:askama}}: A type-safe and compile-time templating engine. Provides excellent compile-time error checking. Good for applications where safety and performance are critical.
* [`maud`][c-maud]⮳{{hi:maud}}: A fast and type-safe HTML templating engine that generates HTML at compile time. Excellent for web development.
* [`markup`][c-markup]⮳{{hi:markup}}: A general-purpose markup templating engine. Can be used for various markup languages.
* [`yew`][c-yew]⮳{{hi:yew}}: A front-end framework for building web apps with Rust. Components can be used like templates.
* [`perseus`][c-perseus]⮳{{hi:perseus}}: A static site generator that uses templates.

## Choosing a Template Engine

The best template engine for your project depends on your specific requirements:

* Simplicity: For basic templating, [`minijinja`][c-minijinja]⮳{{hi:minijinja}} is a good choice.
* Power and Flexibility: [`tera`][c-tera]⮳{{hi:tera}} offers a wide range of features.
* Type Safety: [`askama`][c-askama]⮳{{hi:askama}} and [`maud`][c-maud]⮳{{hi:maud}} provide compile-time checking.
* Performance: [`minijinja`][c-minijinja]⮳{{hi:minijinja}}, [`tera`][c-tera]⮳{{hi:tera}}, [`askama`][c-askama]⮳{{hi:askama}}, and [`maud`][c-maud]⮳{{hi:maud}} are all performant.
* HTML Generation: [`maud`][c-maud]⮳{{hi:maud}} and [`yew`][c-yew]⮳{{hi:yew}} are well-suited for HTML templating.
* Logic-less Templating: [`handlebars`][c-handlebars]⮳{{hi:handlebars}} and [`minijinja`][c-minijinja]⮳{{hi:minijinja}} (used in a certain way) are good choices.

</div>
