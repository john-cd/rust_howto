mod allow_dead_code;
mod attributes_deprecated;
mod attributes_derive;
mod attributes_early_development;
mod attributes_must_use;
mod attributes_production;
mod cfg_if;
mod conditional_compilation;

fn main() {
    attributes_production::main();
}
