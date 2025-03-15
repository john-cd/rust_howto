# Scrub Hidden Sections - Don't Include Hidden Files

## The Following <div> Should Be Hidden

<div class="hidden">
THIS SHOULD NOT BE SEEN.
</div>


## The Following Should Be Included

{{#include ./included.txt}}


## These Includes Should Be Hidden

### Include a File

```rust
{{#include _hidden.rs}}
```

### Partially Include a File - Third Line Only

```rust
{{#include _hidden.rs:3}}
```

### Partially Include a File - Up to Line 5

```rust
{{#include _hidden.rs::5}}
```

### Partially Include a File - from Line 2

```rust
{{#include _hidden.rs:2:}}
```

### Partially Include a File - Between Lines 2 and 5

```rust
{{#include _hidden.rs:2:5}}
```

### Partially Include a File into Your Book Using Anchors

```rust
{{ #include _hidden.rs:component }}
```

### Including a File, but Initially Hiding All Except Specified Lines

```rust
{{#rustdoc_include _hidden.rs:2}}
```

### Inserting Runnable Rust Files

{{#playground _hidden.rs}}

### Inserting Runnable Rust Files with Attributes

{{#playground _hidden.rs editable}}
