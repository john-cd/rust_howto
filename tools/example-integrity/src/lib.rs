//! Read-only chapter, include, and example integrity inventory.
//!
//! TODO Reviewed ??-??-2026.

/// Repository-wide chapter, include, and Rust target integrity checks.
mod audit;

/// Behavior and test-convention survey for Rust example entry points.
mod test_survey;

/// Generates and checks the book's example index and derived reference definitions.
mod example_index;

/// Stable, path-aware report records for the integrity inventory.
mod model;

pub use audit::build_report;
pub use example_index::check_example_index;
pub use example_index::check_example_refdefs;
pub use example_index::generate_example_index;
pub use example_index::generate_example_refdefs;
pub use model::Report;
pub use test_survey::DEFAULT_TEST_ROOTS;
pub use test_survey::ExampleTestSurvey;
pub use test_survey::survey_example_tests;
