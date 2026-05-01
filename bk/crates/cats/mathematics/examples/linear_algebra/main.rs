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

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AddMatrices => {
                let _ = add_matrices::run();
            }
            Commands::DeserializeMatrix => {
                let _ = deserialize_matrix::run();
            }
            Commands::InvertMatrix => {
                let _ = invert_matrix::run();
            }
            Commands::MultiplyMatrices => {
                let _ = multiply_matrices::run();
            }
            Commands::MultiplyScalarVectorMatrix => {
                let _ = multiply_scalar_vector_matrix::run();
            }
            Commands::NalgebraDecomposition => {
                let _ = nalgebra_decomposition::run();
            }
            Commands::NalgebraTransformations => {
                let _ = nalgebra_transformations::run();
            }
            Commands::NalgebraVectors => {
                let _ = nalgebra_vectors::run();
            }
            Commands::VectorComparison => {
                let _ = vector_comparison::run();
            }
            Commands::VectorNorm => {
                let _ = vector_norm::run();
            }
        }
    }
}
