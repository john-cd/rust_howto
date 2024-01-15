# Strings

## String type

```rust,editable
{{#include ../../deps/examples/strings.rs}}
```

## Placeholders

```rust,editable
{{#include ../../deps/examples/strings2.rs}}
```

Use `{:?}` to use the `Debug` output format (annotate type with `#[derive(Debug)]`) or `{:#?}` for pretty print.

Also use `dbg!(&rect1);` for debug output (returns ownership of the expression’s value).

## String concatenation

Here are several common methods to concatenate Strings:

```rust,editable

#[macro_use(concat_string)]
extern crate concat_string;

#[macro_use(concat_strs)]
extern crate concat_strs;

static DATE: &str = "2024-01-15";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

fn main() {
    let datetime = &[DATE, T, TIME].concat();

    let datetime = &[DATE, TIME].join(T);

    let datetime = &[DATE, T, TIME].join("");

    let datetime = &[DATE, T, TIME].join("");

    let list = [DATE, T, TIME];
    let datetime: String = list.iter().map(|x| *x).collect();

    let list = vec![DATE, T, TIME];
    let datetime: String = list.iter().map(|x| *x).collect();

    let datetime = &format!("{}{}{}", DATE, T, TIME);

    let datetime = &format!("{DATE}{T}{TIME}");

    let mut datetime = String::new();
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);

    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(DATE));
    datetime.push(String::from(T));
    datetime.push(String::from(TIME));
    let datetime = datetime.join("");

    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push_str(T); // or 'T'
    datetime.push_str(TIME);

    let datetime = &(String::from(DATE) + &String::from(T) + &String::from(TIME));

    let datetime = &(String::from(DATE) + T + TIME);

    let datetime = &(DATE.to_owned() + T + TIME);

    let datetime = &(DATE.to_string() + T + TIME);

    let datetime = concat_string!(DATE, T, TIME);

    let datetime = &concat_strs!(DATE, T, TIME);
}
```

Examples from [concatenation_benchmarks-rs]( https://github.com/hoodie/concatenation_benchmarks-rs )

{{#include ../refs/link-refs.md}}
