//! Runs one of the `linear_algebra` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates how to add two matrices using the `ndarray` crate.
mod add_matrices;
/// This example demonstrates how to serialize and deserialize a `DMatrix` using
mod deserialize_matrix;
/// This example demonstrates how to invert a 3x3 matrix using the `nalgebra`
mod invert_matrix;
/// This example demonstrates matrix multiplication using the `ndarray` crate.
mod multiply_matrices;
/// This example demonstrates how to multiply a scalar by a vector and then
mod multiply_scalar_vector_matrix;
/// This example demonstrates matrix decompositions using the `nalgebra` crate.
mod nalgebra_decomposition;
/// This example demonstrates transformations in 3D using the `nalgebra` crate:
mod nalgebra_transformations;
/// This example demonstrates vector operations using the `nalgebra` crate:
mod nalgebra_vectors;

/// Turns linear algebra input into structured syntax data while preserving source locations and
/// reporting malformed input. Analysis and validation code use these routines before applying
/// domain-specific rules.
mod sparse_matrices;
/// This example demonstrates how to compare vectors using the `approx` crate.
mod vector_comparison;
/// Computing the L1 and L2 norms of a vector, and normalizing it, with
mod vector_norm;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `add_matrices` example.
    #[command(name = "add_matrices")]
    AddMatrices,

    /// Runs the `deserialize_matrix` example.
    #[command(name = "deserialize_matrix")]
    DeserializeMatrix,

    /// Runs the `invert_matrix` example.
    #[command(name = "invert_matrix")]
    InvertMatrix,

    /// Runs the `multiply_matrices` example.
    #[command(name = "multiply_matrices")]
    MultiplyMatrices,

    /// Runs the `multiply_scalar_vector_matrix` example.
    #[command(name = "multiply_scalar_vector_matrix")]
    MultiplyScalarVectorMatrix,

    /// Runs the `nalgebra_decomposition` example.
    #[command(name = "nalgebra_decomposition")]
    NalgebraDecomposition,

    /// Runs the `nalgebra_transformations` example.
    #[command(name = "nalgebra_transformations")]
    NalgebraTransformations,

    /// Runs the `nalgebra_vectors` example.
    #[command(name = "nalgebra_vectors")]
    NalgebraVectors,

    /// Runs the `sparse_matrices` example.
    #[command(name = "sparse_matrices")]
    SparseMatrices,

    /// Runs the `vector_comparison` example.
    #[command(name = "vector_comparison")]
    VectorComparison,

    /// Runs the `vector_norm` example.
    #[command(name = "vector_norm")]
    VectorNorm,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::AddMatrices => {
                add_matrices::run();
            }
            Commands::DeserializeMatrix => {
                deserialize_matrix::run()?;
            }
            Commands::InvertMatrix => {
                invert_matrix::run();
            }
            Commands::MultiplyMatrices => {
                multiply_matrices::run();
            }
            Commands::MultiplyScalarVectorMatrix => {
                multiply_scalar_vector_matrix::run();
            }
            Commands::NalgebraDecomposition => {
                nalgebra_decomposition::run();
            }
            Commands::NalgebraTransformations => {
                nalgebra_transformations::run();
            }
            Commands::NalgebraVectors => {
                nalgebra_vectors::run();
            }
            Commands::SparseMatrices => {
                sparse_matrices::run();
            }
            Commands::VectorComparison => {
                vector_comparison::run();
            }
            Commands::VectorNorm => {
                vector_norm::run();
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "add_matrices",
        "deserialize_matrix",
        "invert_matrix",
        "multiply_matrices",
        "multiply_scalar_vector_matrix",
        "nalgebra_decomposition",
        "nalgebra_transformations",
        "nalgebra_vectors",
        "sparse_matrices",
        "vector_comparison",
        "vector_norm",
    ];

    /// Confirms that the command and its arguments are valid.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
