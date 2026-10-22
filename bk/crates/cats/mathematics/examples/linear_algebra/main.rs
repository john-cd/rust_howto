// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod add_matrices;
mod deserialize_matrix;
mod invert_matrix;
mod multiply_matrices;
mod multiply_scalar_vector_matrix;
mod nalgebra_decomposition;
mod nalgebra_transformations;
mod nalgebra_vectors;
mod vector_comparison;
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
    /// Runs the `vector_comparison` example.
    #[command(name = "vector_comparison")]
    VectorComparison,
    /// Runs the `vector_norm` example.
    #[command(name = "vector_norm")]
    VectorNorm,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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
