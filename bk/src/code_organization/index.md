# Code Organization

{{#include index.incl.md}}

Rust's "module system" helps structure your projects effectively. It gives you fine-grained control over code organization, determining which elements are publicly accessible and which remain private, as well as managing the names available in different parts of your program. This system is built upon:

- _Modules_ enable you to organize your code hierarchically, define the scope in which names are valid, and control the privacy of items (via the `pub` keyword).
- _Paths_ provide a way to uniquely identify items within your code, such as structs, functions, or other modules. The `use` keyword creates shortcuts to paths.
- _Crates_ represent a logical unit of code, forming a tree of modules that compiles into either a library or an executable.
- _Packages_, managed by `Cargo`, allow you to build, test, and share your Rust crates.
- _Workspaces_, used for large projects, are a set of packages that share the same dependencies.

## Modules and Paths

{{#include modules.incl.md}}

## Visibility

{{#include visibility.incl.md}}

## `use` Declarations

{{#include use_keyword.incl.md}}

## Dependencies

{{#include dependencies.incl.md}}

## Code Organization by Project Type and Size

{{#include code_organization.incl.md}}

## Naming Conventions

{{#include naming_conventions.incl.md}}

## Related Topics

- [[package_layout | Package Layout]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[incorporate](https://github.com/john-cd/rust_howto/issues/1387)

- [Rust Project Primer](https://rustprojectprimer.com/title.html)

</div>
