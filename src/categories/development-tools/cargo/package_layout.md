# Package Layout

```text
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs                      # The default library file is src/lib.rs.
│   ├── main.rs                     # The default executable file is src/main.rs.
│   └── bin/                        # Other executables can be placed in src/bin/,
│       ├── named-executable.rs     # even in library projects.
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs                   # cargo run --example simple
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/                          # Integration tests go in the tests directory.
    ├── some-integration-tests.rs   # Tests in your src files should be unit tests
    └── multi-file-test/            # and documentation tests.
        ├── main.rs
        └── test_module.rs
```

If you’re building a non-end product, such as a rust library that other rust packages will depend on, put `Cargo.lock` in your `.gitignore`.

- A {{i:package}} is a bundle of one or more crates - as defined by a {{i:`Cargo.toml`}} file
- A {{i:crate}} is the smallest amount of code that the Rust compiler considers at a time.
- A crate can come in one of two forms: a binary crate (must have a function called `main`) or a library crate.
- A package can contain as many {{i:binary crate}}s as you like, but at most only one {{i:library crate}}.
- If a package contains {{i:`src/main.rs`}} and {{i:`src/lib.rs`}}, it has two crates: a binary and a library, both with the same name as the package.
