//! Criterion benchmarks for high-throughput utilities in `tool_lib`.
//!
//! Measures:
//! - Published recipe link extraction regex scanning across table rows.
//! - Crate block parsing and rendering via `TinyTemplate` across modes.
//! - Markdown template generation (alphabetical sections, category badges, index anchors).

#![allow(
    deprecated,
    reason = "crates_io_api::Crate uses deprecated license field"
)]

use std::collections::BTreeMap;
use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use regex::Regex;
use tool_lib::GenerationMode;
use tool_lib::RecipeLink;
use tool_lib::create_alphabetical_crate_page_section;
use tool_lib::create_category_badge;
use tool_lib::create_crate_block_badges_or_refdefs;
use tool_lib::create_index_anchors;

/// Sample recipe table row containing title, recipe link, and crate badge references.
const SAMPLE_RECIPE_ROW: &str = "\
| [Build a full web application][ex~web-framework~axum] | \
[![axum][c~axum~docs~badge]][c~axum~docs] \
[![tokio][c~tokio~docs~badge]][c~tokio~docs] \
[![serde][c~serde~docs~badge]][c~serde~docs] |";

/// Constructs mock `crates_io_api::Crate` data for testing crate block parsing.
fn mock_crate_data(name: &str) -> crates_io_api::Crate {
    crates_io_api::Crate {
        name: name.to_owned(),
        description: Some("A high-performance utility crate for benchmarking.".to_owned()),
        license: None,
        documentation: Some(format!("https://docs.rs/{name}/latest/{name}/")),
        homepage: Some("https://example.com/project".to_owned()),
        repository: Some("https://github.com/example-user/project.git".to_owned()),
        id: name.to_owned(),
        downloads: 1_234_567,
        recent_downloads: Some(89_012),
        categories: None,
        keywords: None,
        versions: None,
        max_version: "1.0.0".to_owned(),
        max_stable_version: Some("1.0.0".to_owned()),
        links: crates_io_api::CrateLinks {
            version_downloads: String::new(),
            versions: None,
            owners: String::new(),
            owner_team: String::new(),
            owner_user: String::new(),
            reverse_dependencies: String::new(),
        },
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        exact_match: Some(true),
    }
}

/// Benchmarks regex extraction of published recipe rows, matching the logic in
/// `collect_published_recipe_links`.
fn benchmark_published_recipe_regexes(c: &mut Criterion) {
    let recipe_pattern = Regex::new(r"\|\s*\[(?P<title>.*?)\]\[(?P<reference>ex~[^\]]+)\]\s*\|")
        .expect("recipe regex compiles");
    let crate_badge_pattern = Regex::new(
        r"\[!\[(?P<name>[^\]]+)\]\[c~(?P<badge>[^~\]]+)~docs~badge\]\]\[c~(?P<link>[^\]]+)~docs\]",
    )
    .expect("crate badge regex compiles");
    let crate_directive_pattern = Regex::new(r"\{\{\s*#crate\s*:?\s+(?P<name>[^\s}]+)[^}]*\}\}")
        .expect("crate directive regex compiles");

    let mut group = c.benchmark_group("published_recipes_regexes");

    group.bench_function("parse_recipe_row", |b| {
        b.iter(|| {
            let line = black_box(SAMPLE_RECIPE_ROW);
            let recipe = recipe_pattern.captures(line);
            let mut badges = Vec::new();
            for badge in crate_badge_pattern.captures_iter(line) {
                badges.push(badge["name"].to_owned());
            }
            let mut directives = Vec::new();
            for directive in crate_directive_pattern.captures_iter(line) {
                directives.push(directive["name"].to_owned());
            }
            black_box((recipe, badges, directives));
        });
    });

    group.finish();
}

/// Benchmarks crate block template parsing and rendering across different modes.
fn benchmark_crate_block_parsing(c: &mut Criterion) {
    let crate_data = mock_crate_data("serde_json");
    let mut group = c.benchmark_group("crate_block_parsing");

    group.bench_function("crate_block_badges", |b| {
        b.iter(|| {
            let rendered = create_crate_block_badges_or_refdefs(
                black_box(&crate_data),
                GenerationMode::CrateBlock,
            )
            .expect("renders crate block badges");
            black_box(rendered);
        });
    });

    group.bench_function("crate_description", |b| {
        b.iter(|| {
            let rendered = create_crate_block_badges_or_refdefs(
                black_box(&crate_data),
                GenerationMode::CrateDescription,
            )
            .expect("renders crate description");
            black_box(rendered);
        });
    });

    group.bench_function("crate_block_refdefs", |b| {
        b.iter(|| {
            let rendered = create_crate_block_badges_or_refdefs(
                black_box(&crate_data),
                GenerationMode::CrateBlockRefdefs,
            )
            .expect("renders crate block refdefs");
            black_box(rendered);
        });
    });

    group.finish();
}

/// Benchmarks markdown template generation utilities (alphabetical crate lists, category badges,
/// index anchors).
fn benchmark_markdown_template_generation(c: &mut Criterion) {
    let mut recipe_links = BTreeMap::new();
    recipe_links.insert(
        "anyhow".to_owned(),
        vec![RecipeLink {
            title: "Error Handling with Anyhow".to_owned(),
            reference: "ex~error-handling~anyhow".to_owned(),
        }],
    );
    let crate_names = vec!["anyhow", "arc-swap", "arrayvec", "atomic-waker", "axum"];
    let keywords = vec!["serde", "json", "serialization", "parsing", "web"];

    let mut group = c.benchmark_group("markdown_template_generation");

    group.bench_function("alphabetical_page_section", |b| {
        b.iter(|| {
            let section = create_alphabetical_crate_page_section(
                black_box("A"),
                black_box(crate_names.clone()),
                black_box(&recipe_links),
            )
            .expect("renders alphabetical crate page section");
            black_box(section);
        });
    });

    group.bench_function("category_badge", |b| {
        b.iter(|| {
            let badge = create_category_badge(
                black_box("Development tools"),
                black_box("development-tools"),
            )
            .expect("renders category badge");
            black_box(badge);
        });
    });

    group.bench_function("index_anchors", |b| {
        b.iter(|| {
            let anchors =
                create_index_anchors(black_box(keywords.clone())).expect("renders index anchors");
            black_box(anchors);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_published_recipe_regexes,
    benchmark_crate_block_parsing,
    benchmark_markdown_template_generation
);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    /// Runs the tool_lib benchmark entry point under test harness.
    #[test]
    fn test_tool_lib_bench_entry_point() {
        let mut criterion = criterion::Criterion::default().configure_from_args();
        super::benchmark_published_recipe_regexes(&mut criterion);
        super::benchmark_crate_block_parsing(&mut criterion);
        super::benchmark_markdown_template_generation(&mut criterion);
    }
}
