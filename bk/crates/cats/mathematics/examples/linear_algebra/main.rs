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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add_matrices")]
    AddMatrices,
    #[command(name = "deserialize_matrix")]
    DeserializeMatrix,
    #[command(name = "invert_matrix")]
    InvertMatrix,
    #[command(name = "multiply_matrices")]
    MultiplyMatrices,
    #[command(name = "multiply_scalar_vector_matrix")]
    MultiplyScalarVectorMatrix,
    #[command(name = "nalgebra_decomposition")]
    NalgebraDecomposition,
    #[command(name = "nalgebra_transformations")]
    NalgebraTransformations,
    #[command(name = "nalgebra_vectors")]
    NalgebraVectors,
    #[command(name = "vector_comparison")]
    VectorComparison,
    #[command(name = "vector_norm")]
    VectorNorm,
}

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
