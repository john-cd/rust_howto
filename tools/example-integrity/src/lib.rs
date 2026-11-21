//! Read-only chapter, include, and example integrity inventory.
//!
//! TODO Reviewed ??-??-2026.

/// Repository-wide chapter, include, and Rust target integrity checks.
mod audit;

/// Behavior and test-convention survey for Rust example entry points.
mod test_survey;

/// Stable, path-aware report records for the integrity inventory.
mod model;

pub use audit::build_report;
pub use model::Report;
pub use test_survey::DEFAULT_TEST_ROOTS;
pub use test_survey::ExampleTestSurvey;
pub use test_survey::survey_example_tests;
