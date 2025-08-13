# Modules and Paths

{{#include modules.incl.md}}

## Structure your Code into Modules {#modules}

[![Rust by example - Modules][book~rust-by-example~mod~badge]][book~rust-by-example~mod]{{hi:mod}}

Modules{{hi:Modules}} are Rust's way of organizing code within a crate (a binary or a library){{hi:Crate}}. Modules are containers that help:

- Group related code - keep functions, structs, enums, etc., that work together in one place,
- Control visibility - decide which parts of your code are public (usable outside the module) and which are private implementation details,
- Create namespaces - avoid naming conflicts by putting items into distinct scopes.

Modules form a tree that originates in the "crate root file" (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate). In the crate root file, you can declare modules. In turn, in each module, you can nest other modules, and so on.

To declare a module, use the [`mod`][keyword~mod]{{hi:mod}} keyword. A module can be written inline, in the same file than its parent, by using curly brackets: `mod <module_name> { ... }` or it may be defined in a separate file or folder, by inserting a semicolon after its declaration: `mod <module_name>;`:

```rust,noplayground
// Module in separate file: `a_file.rs` or `a_file/mod.rs` - see below.
mod a_file;

// Inline module.
mod inline_module {
    // The module's items are written within the brackets.
    fn some_function() {
      // ...
    }
}
```

Note that modules (and items within) are private by default. Review [[visibility | visibility rules]].

## Split your Code into Several Files {#code-files}

Each crate has a crate root file{{hi:Crate root file}} (typically `main.rs` or `lib.rs` in the `src` folder). You may write all your code in that file, if it is very short. For non-trivial projects, you will write your code in multiple files organized into multiple folders.

Write `mod module_name;` to declare a module that has its code in a separate file (note the semicolon at the end and the lack of `{ }`). The compiler then looks for specific files:

- `module_name.rs`,
- `module_name/mod.rs` (older style),

and effectively insert the contents in the parent module, as if it were an inline module.

You can nest (inline or external-file) modules as you wish:

```rust,noplayground
// Inline module.
mod a {

    // The module code is in `a/b.rs` or to `a/b/mod.rs`.
    // `b.rs` or `mod.rs` can in turn contain modules.
    mod b;

    // Nested inline module.
    mod c {
    // ...
    }
}
```

The "older style" conveniently groups all files for a module and its submodules into one folder, but results in a proliferation of `mod.rs` files.

Note the following:

- Adding `.rs` files to your source code folder does not automatically incorporate the code in your crate. You must add explicit `mod` statements. Editors like 'VS Code' will not analyze your code or display hints while typing if you forget to do so!

- The [`mod`][keyword~mod]{{hi:mod}} statement must be added to the _parent_ file, not to the file that contains the module itself.
- Modules (and items within) are private by default. Use the [`pub`][keyword~pub]{{hi:pub}} keyword - see [[visibility | visibility rules]].

- It is possible (but confusing) to override the name and path of the file where a module is stored, using the [path attribute][book~rust-reference~path-attribute]↗.

## Access Items Within Modules via Paths {#paths}

Paths{{hi:Path}} let you use items (like functions, structs, enums, modules, etc.) within your Rust code, when those items are defined in different modules. Paths consist of a sequence of one or more segments separated by `::`, where each segment is an item (often a module), an alias, or a keyword like `super`, `self`, or `crate`:{{hi:super}}{{hi:self}}{{hi:crate}}

```rust,noplayground
a_module                          // Path to a module.
a_module::a_function              // Path to a function in a module.
a_module::Struct1                 // Path to a struct in a module. Also works for enums, constants, etc.

a_module::nested_module           // Path to a nested module.
a_module::nested_module::b_function // Path to a function in a nested module.

super                             // Path to the parent module.
super::brother_module             // Path to another module declared in the parent module.

crate::first_level::second_level  // Path to a nested module, starting from the crate root.
```

Note that [[visibility | visibility]] rules apply. A path is valid only if all of its segments are accessible from the location of use. See the section below. Aliases are covered in [[use_keyword | [`use`][keyword~use]{{hi:use}} keyword]] chapter.{{hi:use}}

Going into more details, there are two main kinds of paths: relative and absolute paths. Relative paths start from the current module you are writing code in:

```rust,noplayground
/// Inline module declaration.
mod a_module {
    pub fn some_function() {
        // ...
    }
}

fn call_something_in_a_module() {
    // Call `some_function` using a relative path:
    a_module::some_function();

    // Relative paths can also start with a `self` keyword,
    // which refers to the current module, but it is very often elided:
    // The above is equivalent to:
    self::a_module::some_function();
}
// The inner function was marked public with `pub`,
// so that it could be accessed. See visibility rules below.
```

The [`super`][keyword~super]{{hi:super}} keyword is used to refer to the parent module:

```rust,noplayground
mod a {
  fn in_a() {
    // Call a function in the parent module of `a`:
    super::in_parent_module();
  }
}

fn in_parent_module() {
  // ...
}
```

Absolute paths start from the root of your crate (the top-level module, usually defined in `src/lib.rs` or `src/main.rs`). You use the keyword `crate` followed by `::` to begin an absolute path.

```rust,editable,noplayground
// In the crate root:
mod module_a {
  pub mod submodule_b {
    pub fn some_function() {
      // ...
    }
  }
}

// Elsewhere in the code:
crate::module_a::submodule_b::some_function();
```

{{i:Absolute paths}} are infrequently seen but useful for disambiguation, when a module name is the same than an external dependency.

Paths can be used to refer to elements within containers other than modules: structs, enums, traits, etc.

```rust,noplayground
a_module::Enum1::Variant1         // Path to a variant in an enum, itself in a module.
TypeName::a_function()            // Path to an associated function within a type (e.g. a struct, an enum).
<ImplType as Trait>::AssocType    // Path to an associated type within a trait.
TypeName::CONSTANT_NAME           // Path to an associated constant within a type's `impl` block.
```

## Use Modules to Hide Implementation Details {#hide-implementation-details}

{{i:Modules}} provides encapsulation, meaning they hide items within from their parent, unless the items are explicitly made public.

- Most items, including modules and items within, are private by default.{{hi:Private by default}}
- Use the [`pub`][keyword~pub]{{hi:pub}} keyword to make them public.{{hi:pub}}
- All items, public or private, can be accessed from within the _same_ module or child modules.
- Items cannot be accessed from their parent, unless they are public.
- Items cannot be accessed from another module, unless all items in the path from that module to the item are accessible (public, if traversed in the parent -> child direction; private or public, if traversed in the child -> parent direction or in the same module).

The following example summarizes the visibility rules when it comes to modules. See the [[visibility | Visibility]] and [[use_keyword | `use` keyword]] chapters for complete details.

```rust,editable
{{#include ../../crates/code_organization/examples/modules/module_visibility.rs:example}}
```

## Related Topics {#related-topics}

- [[package_layout | Package Layout]].

## References {#references}

- [Defining Modules to Control Scope and Privacy (Rust book)][book~rust~ch07-02-defining-modules-to-control-scope-and-privacy]↗.
- [Paths for Referring to an Item in the Module Tree (Rust book)][book~rust~ch07-03-paths-for-referring-to-an-item-in-the-module-tree]↗.
- A [clear explanation of Rust's module system][rust-module-system~website]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
