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
                let _ = associated_types::run();
            }
            Commands::BlanketImplementations => {
                let _ = blanket_implementations::run();
            }
            Commands::ConstInTraits => {
                let _ = const_in_traits::run();
            }
            Commands::ExtendExternalType => {
                let _ = extend_external_type::run();
            }
            Commands::GenericTraits => {
                let _ = generic_traits::run();
            }
            Commands::Newtype => {
                let _ = newtype::run();
            }
            Commands::SealedTraitPattern => {
                let _ = sealed_trait_pattern::run();
            }
            Commands::Supertraits => {
                let _ = supertraits::run();
            }
            Commands::TraitBounds => {
                let _ = trait_bounds::run();
            }
            Commands::TraitBounds2 => {
                let _ = trait_bounds2::run();
            }
            Commands::TraitBoundsMultipleTraits => {
                let _ = trait_bounds_multiple_traits::run();
            }
            Commands::TraitDefaultImplementation => {
                let _ = trait_default_implementation::run();
            }
            Commands::TraitTypes => {
                let _ = trait_types::run();
            }
            Commands::Traits => {
                let _ = traits::run();
            }
        }
    }
}
