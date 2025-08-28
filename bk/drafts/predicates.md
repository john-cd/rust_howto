# Predicates

{{#include predicates.incl.md}}

- [predicates][c~predicates~lib.rs]↗: Boolean-valued predicate functions.

Composable first-order predicate functions.

This library implements an interface to "predicates" - boolean-valued functions of one argument. This allows combinatorial logic to be created and assembled at runtime and then used one or more times for evaluating values.

This is really useful when creating filters and checks that can be changed at runtime with user interaction - it allows a clean separation of concerns where the configuration code can be used to build up a predicate, and then that predicate can be given to the code that does the actual filtering without the filtering code knowing anything about user configuration.

```rust,editable
use predicates::prelude::*;

// 1. Define a simple data structure to filter.
#[derive(Debug, Clone)]
struct File {
    name: String,
    size_kb: u64,
    is_readonly: bool,
}

fn main() {
    // 2. Imagine these values come from user input at runtime.
    let name_contains = "report";
    let min_size_kb = 50;

    // 3. Build the predicate dynamically.
    // We start with a base predicate for the filename.
    // The `predicate::str::contains` function creates a predicate that checks
    // if a string contains a specific substring.
    let name_predicate = predicate::str::contains(name_contains)
        .name("name_contains");

    // Next, we create a predicate for the file size.
    // `predicate::ord::gt` checks if a value is greater than the given value.
    let size_predicate = predicate::ord::gt(min_size_kb)
        .name("size_gt");

    // Combine the simple predicates into a complex one.
    // Here, we want files that match BOTH conditions (name AND size).
    // The `.and()` method combines two predicates.
    let combined_filter = name_predicate.and(size_predicate);

    // 4. Create some sample data to test against.
    let files = vec![
        File { name: "monthly_report.txt".into(), size_kb: 120, is_readonly: false },
        File { name: "budget.xlsx".into(), size_kb: 25, is_readonly: false },
        File { name: "final_report_v2.docx".into(), size_kb: 45, is_readonly: false },
        File { name: "archive.zip".into(), size_kb: 250, is_readonly: true },
        File { name: "yearly_report.pdf".into(), size_kb: 300, is_readonly: true },
    ];

    // 5. Use the predicate to filter the data.
    // The `eval` method on a predicate takes a value and returns `true` or `false`.
    // We map each `File` to the values we want to test.
    println!("Filtering for files containing '{}' AND larger than {} KB...", name_contains, min_size_kb);

    let filtered_files: Vec<_> = files
        .into_iter()
        .filter(|file| {
            // The predicate is evaluated for each file.
            // We test the file's name against the first part of our combined predicate
            // and the file's size against the second part.
            combined_filter.eval(&(file.name.as_str(), file.size_kb))
        })
        .collect();

    println!("\nFound files:");
    for file in filtered_files {
        println!("- {:?} KB, '{}'", file.size_kb, file.name);
    }
}
```

## Related Topics {#related-topics .skip}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
