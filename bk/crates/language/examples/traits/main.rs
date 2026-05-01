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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "associated_types")]
    AssociatedTypes,
    #[command(name = "blanket_implementations")]
    BlanketImplementations,
    #[command(name = "const_in_traits")]
    ConstInTraits,
    #[command(name = "extend_external_type")]
    ExtendExternalType,
    #[command(name = "generic_traits")]
    GenericTraits,
    #[command(name = "newtype")]
    Newtype,
    #[command(name = "sealed_trait_pattern")]
    SealedTraitPattern,
    #[command(name = "supertraits")]
    Supertraits,
    #[command(name = "trait_bounds")]
    TraitBounds,
    #[command(name = "trait_bounds2")]
    TraitBounds2,
    #[command(name = "trait_bounds_multiple_traits")]
    TraitBoundsMultipleTraits,
    #[command(name = "trait_default_implementation")]
    TraitDefaultImplementation,
    #[command(name = "trait_types")]
    TraitTypes,
    #[command(name = "traits")]
    Traits,
}

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
