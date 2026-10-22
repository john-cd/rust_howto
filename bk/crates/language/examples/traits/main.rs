//! Runs one of the `traits` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod associated_types;
mod blanket_implementations;
mod const_in_traits;
mod extend_external_type;
mod generic_traits;
mod newtype;
mod sealed_trait_pattern;
mod supertraits;
mod trait_bounds;
mod trait_bounds2;
mod trait_bounds_multiple_traits;
mod trait_default_implementation;
mod trait_types;
mod traits;

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
    /// Runs the `associated_types` example.
    #[command(name = "associated_types")]
    AssociatedTypes,
    /// Runs the `blanket_implementations` example.
    #[command(name = "blanket_implementations")]
    BlanketImplementations,
    /// Runs the `const_in_traits` example.
    #[command(name = "const_in_traits")]
    ConstInTraits,
    /// Runs the `extend_external_type` example.
    #[command(name = "extend_external_type")]
    ExtendExternalType,
    /// Runs the `generic_traits` example.
    #[command(name = "generic_traits")]
    GenericTraits,
    /// Runs the `newtype` example.
    #[command(name = "newtype")]
    Newtype,
    /// Runs the `sealed_trait_pattern` example.
    #[command(name = "sealed_trait_pattern")]
    SealedTraitPattern,
    /// Runs the `supertraits` example.
    #[command(name = "supertraits")]
    Supertraits,
    /// Runs the `trait_bounds` example.
    #[command(name = "trait_bounds")]
    TraitBounds,
    /// Runs the `trait_bounds2` example.
    #[command(name = "trait_bounds2")]
    TraitBounds2,
    /// Runs the `trait_bounds_multiple_traits` example.
    #[command(name = "trait_bounds_multiple_traits")]
    TraitBoundsMultipleTraits,
    /// Runs the `trait_default_implementation` example.
    #[command(name = "trait_default_implementation")]
    TraitDefaultImplementation,
    /// Runs the `trait_types` example.
    #[command(name = "trait_types")]
    TraitTypes,
    /// Runs the `traits` example.
    #[command(name = "traits")]
    Traits,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AssociatedTypes => {
                associated_types::run();
            }
            Commands::BlanketImplementations => {
                blanket_implementations::run();
            }
            Commands::ConstInTraits => {
                const_in_traits::run();
            }
            Commands::ExtendExternalType => {
                extend_external_type::run();
            }
            Commands::GenericTraits => {
                generic_traits::run();
            }
            Commands::Newtype => {
                newtype::run();
            }
            Commands::SealedTraitPattern => {
                sealed_trait_pattern::run();
            }
            Commands::Supertraits => {
                supertraits::run();
            }
            Commands::TraitBounds => {
                trait_bounds::run();
            }
            Commands::TraitBounds2 => {
                trait_bounds2::run();
            }
            Commands::TraitBoundsMultipleTraits => {
                trait_bounds_multiple_traits::run();
            }
            Commands::TraitDefaultImplementation => {
                trait_default_implementation::run();
            }
            Commands::TraitTypes => {
                trait_types::run();
            }
            Commands::Traits => {
                traits::run();
            }
        }
    }
}
