<!-- TODO Reviewed ??-??-2026. -->

A valid derive supplies the greeting name and implements the trait declared by the caller:

```rust,noplayground
use development_tools_procedural_macro_helpers::MyDarlingMacro;

trait MyDarlingTrait {
    fn hello();
}

#[derive(MyDarlingMacro)]
#[my_macro(name = "Ada")]
struct Greeter;

Greeter::hello();
```

The same caller fails to compile if the attribute uses an unknown key:

```rust,compile_fail,noplayground
use development_tools_procedural_macro_helpers::MyDarlingMacro;

trait MyDarlingTrait {
    fn hello();
}

#[derive(MyDarlingMacro)]
#[my_macro(name = "Ada", alias = "by mistake")]
struct Greeter;
```
