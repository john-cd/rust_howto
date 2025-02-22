# Scrub hidden sections and don't include hidden files

## The following <div> should be hidden

<div class="hidden">
THIS SHOULD NOT BE SEEN.
</div>


## The following should be included

{{#include ./included.txt}}


## These includes should be hidden

### Include a file

```rust
{{#include _hidden.rs}}
```

### Partially include a file - third line only

```rust
{{#include _hidden.rs:3}}
```

### Partially include a file - up to line 5

```rust
{{#include _hidden.rs::5}}
```

### Partially include a file - from line 2

```rust
{{#include _hidden.rs:2:}}
```

### Partially include a file - between lines 2 and 5

```rust
{{#include _hidden.rs:2:5}}
```

### Partially include a file into your book using anchors

```rust
{{ #include _hidden.rs:component }}
```

### Including a file but initially hiding all except specified lines

```rust
{{#rustdoc_include _hidden.rs:2}}
```

### Inserting runnable Rust files

{{#playground _hidden.rs}}

### Inserting runnable Rust files with attributes

{{#playground _hidden.rs editable}}
