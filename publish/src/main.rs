use std::error::Error;

use cargo_toml::Manifest;

fn main() -> Result<(), Box<dyn Error>> {
    println!("This crate is a placeholder. It does nothing useful.\n");

    // Let's print parts of the Cargo.toml manifest.

    let cargo_toml_bytes = include_bytes!("../Cargo.toml");
    let manifest = Manifest::from_slice(cargo_toml_bytes)?;
    // Cargo.toml refers to the workspace ../Cargo.toml,
    // therefore some fields are not available until
    // `cargo package` rewrites it with full details.
    let some_fields_unavailable = manifest.needs_workspace_inheritance();

    if let Some(package) = manifest.package {
        println!("Name: {}\n", package.name);
        if some_fields_unavailable {
            return Ok(());
        }
        println!(
            "Description: {}\n",
            package.description.unwrap_or_default().get()?
        );
        println!("Authors: {:?}\n", package.authors.get()?);
        println!(
            "Homepage: {}\n",
            package.homepage.unwrap_or_default().get()?
        );
        println!(
            "Repository: {}\n",
            package.repository.unwrap_or_default().get()?
        );
        println!(
            "Documentation: {}\n",
            package.documentation.unwrap_or_default().get()?
        );
        println!("Keywords: {:?}\n", package.keywords.get()?);
        println!("Categories: {:?}\n", package.categories.get()?);
    }
    Ok(())
}
