//! Demonstrates building and applying a composite predicate.

// ANCHOR: example
use predicates::prelude::*;

/// A small file record used to demonstrate runtime filtering.
#[derive(Debug)]
struct File {
    /// The file's name.
    name: String,
    /// The file's size in kibibytes.
    size_kb: u64,
}

/// Filters files by a name substring and a minimum size.
fn main() {
    // Imagine these values come from user input at runtime.
    let name_contains = "report";
    let min_size_kb = 50;

    // Adapt each field predicate to the `File` record.
    let name_contains_predicate = predicate::str::contains(name_contains);
    let name_predicate = predicate::function(|file: &File| {
        name_contains_predicate.eval(file.name.as_str())
    })
    .fn_name("name_contains");
    let min_size_predicate = predicate::gt(min_size_kb);
    let size_predicate = predicate::function(|file: &File| {
        min_size_predicate.eval(&file.size_kb)
    })
    .fn_name("size_gt");

    // Combine the simple predicates to require both conditions.
    let combined_filter = name_predicate.and(size_predicate);

    let files = vec![
        File {
            name: "monthly_report.txt".into(),
            size_kb: 120,
        },
        File {
            name: "budget.xlsx".into(),
            size_kb: 25,
        },
        File {
            name: "final_report_v2.docx".into(),
            size_kb: 45,
        },
        File {
            name: "archive.zip".into(),
            size_kb: 250,
        },
        File {
            name: "yearly_report.pdf".into(),
            size_kb: 300,
        },
    ];

    let filtered_files: Vec<_> = files
        .into_iter()
        .filter(|file| combined_filter.eval(file))
        .collect();

    let matching_names: Vec<_> = filtered_files
        .iter()
        .map(|file| file.name.as_str())
        .collect();
    assert_eq!(matching_names, ["monthly_report.txt", "yearly_report.pdf"]);

    println!(
        "Filtering for files containing '{name_contains}' AND larger than {min_size_kb} KB..."
    );
    println!("\nFound files:");
    for file in filtered_files {
        println!("- {} KB, '{}'", file.size_kb, file.name);
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies that the composed predicate keeps records matching both rules.
    #[test]
    fn test_composite_predicate_filters_files() {
        super::main();
    }
}
