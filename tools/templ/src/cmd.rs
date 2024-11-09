pub(crate) struct Config {
    pub verbose: bool,
}

/// The command that the end user selected
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    Badges(CratesCmdArgs),
    Rbe(ConceptsCmdArgs),
    CategoriesForCrateBadge(CratesCmdArgs),
    CategoryBadges(CategoriesCmdArgs),
    Info(CratesCmdArgs),
}

/// The command arguments for multiple commands
/// List of crates
#[derive(Debug)]
pub(crate) struct CratesCmdArgs {
    pub names: Vec<String>,
}

/// The `rbe` subcommand arguments
/// List of concepts / RBE chapters
#[derive(Debug)]
pub(crate) struct ConceptsCmdArgs {
    pub concepts: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct CategoriesCmdArgs {
    pub categories: Vec<String>,
}
