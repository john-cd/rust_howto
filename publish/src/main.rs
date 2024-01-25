use std::error::Error;

use cargo_toml::Manifest;

fn main() -> Result<(), Box<dyn Error>> {
    println!("This crate is a placeholder. It does nothing useful.\n");

    let cargo_toml_bytes = include_bytes!("../Cargo.toml");
    let manifest = Manifest::from_slice(cargo_toml_bytes)?;

    if let Some(package) = manifest.package {
        println!("Name: {}\n", package.name);
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
